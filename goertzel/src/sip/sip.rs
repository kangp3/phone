use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, LazyLock};
use std::time::{SystemTime, UNIX_EPOCH};

use itertools::Itertools;
use local_ip_address::local_ip;
use md5::{Md5, Digest};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rsip::headers::auth::{Algorithm, AuthQop};
use rsip::headers::{auth, CallId, ContentLength, Expires, MaxForwards, UserAgent};
use rsip::param::OtherParam;
use rsip::prelude::ToTypedHeader;
use rsip::typed::{Allow, Authorization, CSeq, Contact, ContentType, From, MediaType, To, Via};
use rsip::{Auth, Header, Headers, Method, Param, Request, Response, Scheme, SipMessage, StatusCode, Transport, Uri, Version};
use sdp_rs::{MediaDescription, SessionDescription};
use tokio::sync::{mpsc, RwLock, RwLockWriteGuard};
use vec1::Vec1;


const MESSAGE_CHANNEL_SIZE: usize = 64;

const USERNAME: &str = "1100";
const PASSWORD: &str = "SW2fur7facrarac";
const REALM: &str = "asterisk";
const USER_AGENT: &str = "Frandline";
const UA_VERSION: &str = "0.1.0";
// Branch should always be prefixed with magic string z9hG4bK
// https://www.ietf.org/rfc/rfc3261.txt (8.1.1.7)
const BRANCH_PREFIX: &str = "z9hG4bK";
const MAX_FORWARDS: u32 = 70;

// TODO(peter): Make these configurable
pub static SERVER_ADDR: LazyLock<SocketAddr> = LazyLock::new(
    || SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 12, 182)), 5060)
);
pub static CLIENT_ADDR: LazyLock<SocketAddr> = LazyLock::new(
    || SocketAddr::new(local_ip().unwrap(), 5060)
);
pub static MY_URI: LazyLock<Uri> = LazyLock::new(
    || Uri{
        scheme: Some(Scheme::Sip),
        auth: Some(Auth{
            user: USERNAME.into(),
            password: None,
        }),
        host_with_port: (*SERVER_ADDR).into(),
        ..Default::default()
    }
);

pub static TXN_MAILBOXES: LazyLock<Arc<RwLock<HashMap::<String, mpsc::Sender<SipMessage>>>>> = LazyLock::new(|| {
    Arc::new(RwLock::new(HashMap::new()))
});

fn md5(s: String) -> String {
    let mut hasher = Md5::new();
    hasher.update(s);
    format!("{:x}", hasher.finalize())
}

pub async fn register(out_ch: mpsc::Sender<SipMessage>) -> Result<(), Box<dyn Error>> {
    let txn_mailboxes = TXN_MAILBOXES.clone();

    let mut txn = {
        let mailboxes = txn_mailboxes.write().await;
        Txn::new(out_ch, mailboxes)
    };
    let msg = SipMessage::Request(txn.register_request());
    txn.tx_ch.send(msg).await?;
    let msg = txn.rx_ch.recv().await.ok_or("closed rx channel")?;
    if let SipMessage::Response(r) = msg {
        let mut opaque = None;
        let mut nonce = String::new();
        for header in r.headers {
            match header {
                Header::WwwAuthenticate(h) => {
                    let h = h.typed()?;
                    opaque = h.opaque;
                    nonce = h.nonce;
                    break;
                },
                _ => {},
            }
        }
        let msg = SipMessage::Request({
            let mut req = txn.register_request();
            txn.add_auth_to_request(&mut req, opaque, nonce);
            req
        });
        txn.tx_ch.send(msg).await?;
        let msg = txn.rx_ch.recv().await.ok_or("closed rx channel")?;
        match msg {
            SipMessage::Request(_) => Err("expected 200 response to authed register, got request")?,
            SipMessage::Response(r) =>
                (r.status_code == StatusCode::OK).then_some(()).ok_or("response status not 200")?,
        }
    }
    Ok(())
}

// TODO(peter): Implement Drop on this to make sure mailboxes get cleared out
pub struct Txn {
    pub tx_ch: mpsc::Sender<SipMessage>,
    pub rx_ch: mpsc::Receiver<SipMessage>,

    cseq: u32,
    call_id: CallId,
    from_tag: Param,
    to_tag: Option<Param>,
}

// TODO(peter): Ask about &mut impl Rng vs &mut ThreadRng (this didn't work)
fn rand_chars(rng: &mut impl Rng, len: usize) -> String {
    rng.sample_iter(&Alphanumeric).take(len).map(char::from).collect()
}

fn ms_since_epoch() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

fn micros_since_epoch() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros()
}

impl Txn {
    pub fn new(tx_ch: mpsc::Sender<SipMessage>, mut mailboxes: RwLockWriteGuard<'_, HashMap<String, mpsc::Sender<SipMessage>>>) -> Self {
        let (rx_send_ch, rx_ch) = mpsc::channel(MESSAGE_CHANNEL_SIZE);
        let mut rng = thread_rng();
        let call_id = CallId::from(format!("{}/{}", ms_since_epoch(), rand_chars(&mut rng, 16)));
        let from_tag = Param::Tag(rand_chars(&mut rng, 16).into());
        let to_tag = None;

        mailboxes.insert(call_id.to_string(), rx_send_ch);

        Txn {
            tx_ch,
            rx_ch,

            cseq: 0,
            call_id,
            from_tag,
            to_tag,
        }
    }

    pub fn from_req(req: Request, tx_ch: mpsc::Sender<SipMessage>, mut mailboxes: RwLockWriteGuard<'_, HashMap<String, mpsc::Sender<SipMessage>>>) -> Result<Self, Box<dyn Error>> {
        let (rx_send_ch, rx_ch) = mpsc::channel(MESSAGE_CHANNEL_SIZE);

        let mut cseq: Option<u32> = None;
        let mut call_id: Option<CallId> = None;
        let mut from_tag: Option<Param> = None;
        let mut to_tag: Option<Param> = None;

        for header in req.headers {
            match header {
                Header::CallId(h) => {
                    call_id = Some(h.into());
                }
                Header::CSeq(h) => {
                    let h = h.typed()?;
                    cseq = Some(h.seq);
                }
                Header::From(h) => {
                    let h = h.typed()?;
                    for param in h.params {
                        match param {
                            Param::Tag(t) => {
                                from_tag = Some(t.into());
                            }
                            _ => {}
                        }
                    }
                }
                Header::To(h) => {
                    let h = h.typed()?;
                    for param in h.params {
                        match param {
                            Param::Tag(t) => {
                                to_tag = Some(t.into());
                            }
                            _ => {}
                        }
                    }
                }
                _ => {},
            }
        }

        let cseq = cseq.ok_or("missing cseq")?;
        let call_id = call_id.ok_or("missing call_id")?;
        let from_tag = from_tag.ok_or("missing from tag")?;

        match mailboxes.entry(call_id.to_string()) {
            Entry::Occupied(_) => Err("mailbox already exists in map")?,
            Entry::Vacant(e) => e.insert(rx_send_ch),
        };

        Ok(Txn {
            rx_ch,
            tx_ch,

            cseq,
            call_id,
            from_tag,
            to_tag,
        })
    }

    pub fn response_to(&self, req: Request, status_code: StatusCode, body: Vec<u8>) -> Response {
        let mut headers: Headers = Default::default();
        for header in req.headers {
            match header {
                h@Header::CallId(_) |
                h@Header::CSeq(_) |
                h@Header::From(_) |
                h@Header::To(_) |
                h@Header::Via(_) => headers.push(h),
                _ => {},
            }
        }
        headers.push(ContentLength::from(body.len() as u32).into());

        Response{
            status_code,
            version: Version::V2,
            headers,
            body,
        }
    }

    fn new_request(&mut self, method: Method, body: Vec<u8>) -> Request {
        self.cseq += 1;
        let branch: String = format!("{}{}", BRANCH_PREFIX, rand_chars(&mut thread_rng(), 32));

        let mut headers: Headers = Default::default();
        headers.push(CSeq{ seq: self.cseq, method }.into());
        headers.push(Via{
            version: Version::V2,
            transport: Transport::Udp,
            uri: Uri {
                host_with_port: (*CLIENT_ADDR).into(),
                ..Default::default()
            },
            params: vec![
                Param::Branch(branch.into()),
                Param::Other(OtherParam::from("rport"), None)
            ],
        }.into());
        headers.push(UserAgent::from(format!("{}/{}", USER_AGENT, UA_VERSION)).into());
        headers.push(self.call_id.clone().into());
        headers.push(Contact{
            display_name: Some(USERNAME.into()),
            uri: Uri {
                scheme: Some(Scheme::Sip),
                host_with_port: (*CLIENT_ADDR).into(),
                auth: Some(Auth{
                    user: USERNAME.into(),
                    password: None,
                }),
                ..Default::default()
            },
            params: vec![Param::Q("1".into())],
        }.into());
        headers.push(MaxForwards::from(MAX_FORWARDS).into());
        headers.push(ContentLength::from(body.len() as u32).into());

        Request {
            method,
            uri: Uri {
                scheme: Some(Scheme::Sip),
                host_with_port: (*SERVER_ADDR).into(),
                ..Default::default()
            },
            version: Version::V2,
            headers,
            body,
        }
    }

    fn new_request_from_to(&mut self, method: Method, from: Uri, to: Uri, body: Vec<u8>) -> Request {
        let mut req = self.new_request(method, body);
        req.headers.push(From{
            display_name: None,
            uri: from,
            params: vec![self.from_tag.clone()],
        }.into());
        req.headers.push(To{
            display_name: None,
            uri: to,
            params: vec![self.to_tag.clone()].into_iter().filter_map(|t| t).collect(),
        }.into());

        req
    }

    pub fn add_auth_to_request(&self, req: &mut Request, opaque: Option<String>, nonce: String) {
        let cnonce = format!("{}/{}", ms_since_epoch(), rand_chars(&mut thread_rng(), 16));
        // TOOD(peter): Actually track this?
        let nc = 1;

        let ha1 = md5(format!("{}:{}:{}", USERNAME, REALM, PASSWORD));
        let ha2 = md5(format!("{}:{}:{}", req.method, req.uri.scheme.as_ref().unwrap_or(&Scheme::Sip), (*SERVER_ADDR)));
        let response = md5(format!("{}:{}:{:08x}:{}:auth:{}", ha1, nonce, nc, cnonce, ha2));

        req.headers.push(Authorization{
            scheme: auth::Scheme::Digest,
            username: USERNAME.into(),
            realm: REALM.into(),
            nonce,
            uri: Uri {
                scheme: Some(Scheme::Sip),
                host_with_port: (*SERVER_ADDR).into(),
                ..Default::default()
            },
            response,
            algorithm: Some(Algorithm::Md5),
            opaque,
            qop: Some(AuthQop::Auth { cnonce, nc }),
        }.into());
        req.headers.push(Expires::from(3600).into());
    }

    pub fn register_request(&mut self) -> Request {
        let mut req = self.new_request_from_to(Method::Register, (*MY_URI).clone(), (*MY_URI).clone(), vec![]);
        req.headers.push(Allow::from(Method::all()).into());
        req
    }

    pub fn invite_request(&mut self, to: Uri) -> Request {
        let sess_id = micros_since_epoch().to_string();
        let body = SessionDescription{
            version: sdp_rs::lines::Version::V0,
            origin: sdp_rs::lines::Origin{
                username: "-".to_string(),
                sess_id: sess_id.clone(),
                sess_version: sess_id,
                nettype: sdp_rs::lines::common::Nettype::In,
                addrtype: sdp_rs::lines::common::Addrtype::Ip4,
                unicast_address: (*CLIENT_ADDR).ip(),
            },
            session_name: sdp_rs::lines::SessionName::from(USER_AGENT.to_string()),
            session_info: None,
            uri: None,
            emails: vec![],
            phones: vec![],
            connection: Some(sdp_rs::lines::Connection{
                nettype: sdp_rs::lines::common::Nettype::In,
                addrtype: sdp_rs::lines::common::Addrtype::Ip4,
                connection_address: (*CLIENT_ADDR).ip().into(),
            }),
            bandwidths: vec![],
            times: Vec1::new(sdp_rs::Time{
                active: sdp_rs::lines::Active { start: 0, stop: 0 },
                repeat: vec![],
                zone: None,
            }),
            key: None,
            attributes: vec![],
            media_descriptions: vec![MediaDescription{
                media: sdp_rs::lines::Media{
                    media: sdp_rs::lines::media::MediaType::Audio,
                    port: 19512,
                    num_of_ports: None,
                    proto: sdp_rs::lines::media::ProtoType::RtpAvp,
                    fmt: vec![0, 101].into_iter().map(|v| v.to_string()).join(" "),
                },
                info: None,
                connections: vec![],
                bandwidths: vec![],
                key: None,
                attributes: vec![
                    sdp_rs::lines::Attribute::Rtpmap(sdp_rs::lines::attribute::Rtpmap {
                        payload_type: 0,
                        encoding_name: "PCMU".into(),
                        clock_rate: 8000,
                        encoding_params: None,
                    }),
                    sdp_rs::lines::Attribute::Rtpmap(sdp_rs::lines::attribute::Rtpmap {
                        payload_type: 101,
                        encoding_name: "telephone-event".into(),
                        clock_rate: 8000,
                        encoding_params: None,
                    }),
                    sdp_rs::lines::Attribute::Other("fmtp".into(), Some("101 0-16".into())),
                    sdp_rs::lines::Attribute::Ptime(20.0),
                    sdp_rs::lines::Attribute::Maxptime(140.0),
                    sdp_rs::lines::Attribute::Sendrecv,
                ],
            }],
        };
        let mut req = self.new_request_from_to(
            Method::Invite,
            (*MY_URI).clone(),
            to.clone(),
            body.to_string().into_bytes(),
        );
        req.uri = to;
        req.headers.push(ContentType(MediaType::Sdp(vec![])).into());
        req
    }

    pub fn ack_request(&mut self, resp: Response) -> Request {
        let mut req = self.new_request(Method::Ack, vec![]);
        for header in resp.headers {
            match header {
                h@Header::From(_) |
                h@Header::To(_) => {
                    req.headers.push(h);
                },
                _ => {},
            }
        }
        req
    }

    pub fn cancel_request(&mut self, to: Uri) -> Request {
        let req = self.new_request_from_to(Method::Cancel, (*MY_URI).clone(), to, vec![]);
        req
    }
}
