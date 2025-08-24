use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::sync::{Arc, LazyLock};
use std::time::{SystemTime, UNIX_EPOCH};

use itertools::Itertools;
use local_ip_address::local_ip;
use md5::{Digest, Md5};
use rand::distr::Alphanumeric;
use rand::prelude::*;
use rand::rngs::StdRng;
use rand::{rng, Rng};
use rsip::headers::auth::{Algorithm, AuthQop};
use rsip::headers::{self, auth, CallId, ContentLength, Expires, MaxForwards, UserAgent};
use rsip::param::{OtherParam, Tag};
use rsip::typed::{Allow, Authorization, CSeq, Contact, ContentType, From, MediaType, To, Via};
use rsip::{prelude::*, StatusCodeKind};
use rsip::{
    Auth, Header, Headers, HostWithPort, Method, Param, Request, Response, Scheme, SipMessage,
    StatusCode, Transport, Uri, Version,
};
use sdp_rs::{MediaDescription, SessionDescription};
use tokio::sync::{broadcast, mpsc, RwLock, RwLockWriteGuard};
use tracing::debug;
use vec1::Vec1;

const MESSAGE_CHANNEL_SIZE: usize = 64;

const REALM: &str = "asterisk";
const USER_AGENT: &str = "Frandline";
const UA_VERSION: &str = "0.1.0";
// Branch should always be prefixed with magic string z9hG4bK
// https://www.ietf.org/rfc/rfc3261.txt (8.1.1.7)
const BRANCH_PREFIX: &str = "z9hG4bK";
const MAX_FORWARDS: u32 = 70;

pub const SERVER_NAME: &str = "pbx.frandline.com";
pub const SERVER_PORT: u16 = 5061;
pub static USERNAME: LazyLock<String> = LazyLock::new(|| env::var("SIP_USERNAME").unwrap());
static PASSWORD: LazyLock<String> = LazyLock::new(|| env::var("SIP_PASSWORD").unwrap());
pub static SERVER_ADDR: LazyLock<SocketAddr> =
    LazyLock::new(|| SocketAddr::from_str(&env::var("SIP_SERVER_ADDRESS").unwrap()).unwrap());
pub static FRANDLINE_PBX_ADDR: LazyLock<HostWithPort> =
    LazyLock::new(|| HostWithPort::try_from(SERVER_NAME).unwrap());
pub static CLIENT_ADDR: LazyLock<SocketAddr> =
    LazyLock::new(|| SocketAddr::new(local_ip().unwrap(), 5060));
pub static MY_URI: LazyLock<Uri> = LazyLock::new(|| Uri {
    scheme: Some(Scheme::Sip),
    auth: Some(Auth {
        user: (*USERNAME).clone(),
        password: None,
    }),
    host_with_port: (*SERVER_ADDR).into(),
    ..Default::default()
});
pub static SIPS_URI: LazyLock<Uri> = LazyLock::new(|| Uri {
    scheme: Some(Scheme::Sips),
    auth: Some(Auth {
        user: (*USERNAME).clone(),
        password: None,
    }),
    host_with_port: (*FRANDLINE_PBX_ADDR).clone(),
    ..Default::default()
});

pub static TXN_MAILBOXES: LazyLock<Arc<RwLock<HashMap<String, broadcast::Sender<SipMessage>>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

// Responses not tied to a transaction
pub fn response_to(req: &Request, status_code: StatusCode) -> SipMessage {
    let mut headers: Headers = Default::default();
    for header in req.headers().clone() {
        match header {
            h @ Header::CallId(_)
            | h @ Header::CSeq(_)
            | h @ Header::From(_)
            | h @ Header::To(_)
            | h @ Header::Via(_) => headers.push(h),
            _ => {}
        }
    }

    SipMessage::Response(Response {
        status_code,
        version: Version::V2,
        headers,
        body: vec![],
    })
}

// Requests not tied to a transaction
pub fn ack_to(resp: &Response) -> SipMessage {
    let branch: String = format!("{}{}", BRANCH_PREFIX, rand_chars(&mut rng(), 32));

    let mut headers: Headers = Default::default();
    for header in resp.headers().clone() {
        match header {
            h @ Header::CallId(_)
            | h @ Header::CSeq(_)
            | h @ Header::From(_)
            | h @ Header::To(_) => headers.push(h),
            _ => {}
        }
    }
    headers.push(
        Via {
            version: Version::V2,
            transport: Transport::Tcp,
            uri: Uri {
                host_with_port: (*CLIENT_ADDR).into(),
                ..Default::default()
            },
            params: vec![
                Param::Branch(branch.into()),
                Param::Other(OtherParam::from("rport"), None),
            ],
        }
        .into(),
    );
    headers.push(UserAgent::from(format!("{}/{}", USER_AGENT, UA_VERSION)).into());
    headers.push(ContentLength::from(0).into());

    rsip::SipMessage::Request(Request {
        method: Method::Ack,
        uri: Uri {
            scheme: Some(Scheme::Sip),
            host_with_port: (*SERVER_ADDR).into(),
            ..Default::default()
        },
        version: Version::V2,
        headers,
        body: vec![],
    })
}

fn md5(s: String) -> String {
    let mut hasher = Md5::new();
    hasher.update(s);
    format!("{:x}", hasher.finalize())
}

pub async fn register(
    out_ch: mpsc::Sender<(SocketAddr, SipMessage)>,
) -> Result<(), Box<dyn Error>> {
    let txn_mailboxes = TXN_MAILBOXES.clone();

    let mut txn = {
        let mailboxes = txn_mailboxes.write().await;
        Txn::new(out_ch, mailboxes)
    };
    let mut rx_ch = txn.rx_ch.subscribe();
    let msg = SipMessage::Request(txn.register_request());
    txn.tx_ch.send(((*SERVER_ADDR).clone(), msg)).await?;
    let msg = rx_ch.recv().await?;
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
                }
                _ => {}
            }
        }
        let msg = SipMessage::Request({
            let mut req = txn.register_request();
            txn.add_auth_to_request(&mut req, opaque, nonce);
            req
        });
        txn.tx_ch.send(((*SERVER_ADDR).clone(), msg)).await?;
        let msg = rx_ch.recv().await?;
        match msg {
            SipMessage::Request(_) => Err("expected 200 response to authed register, got request")?,
            SipMessage::Response(r) => (r.status_code == StatusCode::OK)
                .then_some(())
                .ok_or("response status not 200")?,
        }
    }
    Ok(())
}

// TODO(peter): Implement Drop on this to make sure mailboxes get cleared out
#[derive(Clone)]
pub struct Txn {
    pub tx_ch: mpsc::Sender<(SocketAddr, SipMessage)>,
    pub rx_ch: broadcast::Sender<SipMessage>,

    cseq: u32,
    call_id: CallId,
    from_tag: Tag,
    to_tag: Option<Tag>,
}

// TODO(peter): Ask about &mut impl Rng vs &mut ThreadRng (this didn't work)
fn rand_chars(rng: &mut impl Rng, len: usize) -> String {
    rng.sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

fn ms_since_epoch() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn micros_since_epoch() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros()
}

impl Txn {
    pub fn new(
        tx_ch: mpsc::Sender<(SocketAddr, SipMessage)>,
        mut mailboxes: RwLockWriteGuard<'_, HashMap<String, broadcast::Sender<SipMessage>>>,
    ) -> Self {
        let (rx_ch, _) = broadcast::channel(MESSAGE_CHANNEL_SIZE);
        let mut rng = rng();
        let call_id = CallId::from(format!("{}/{}", ms_since_epoch(), rand_chars(&mut rng, 16)));
        let from_tag = rand_chars(&mut rng, 16).into();
        let to_tag = None;

        mailboxes.insert(call_id.to_string(), rx_ch.clone());

        Txn {
            tx_ch,
            rx_ch,

            cseq: 0,
            call_id,
            from_tag,
            to_tag,
        }
    }

    pub fn from_req(
        req: Request,
        tx_ch: mpsc::Sender<(SocketAddr, SipMessage)>,
        mut mailboxes: RwLockWriteGuard<'_, HashMap<String, broadcast::Sender<SipMessage>>>,
    ) -> Result<Self, Box<dyn Error>> {
        let (rx_ch, _) = broadcast::channel(MESSAGE_CHANNEL_SIZE);

        let cseq = req.cseq_header()?.seq()?;
        let call_id = req.call_id_header()?.clone();
        let from_tag = req.from_header()?.tag()?.ok_or("missing from tag")?;
        let to_tag = req.to_header()?.tag()?;

        match mailboxes.entry(call_id.to_string()) {
            Entry::Occupied(_) => Err("mailbox already exists in map")?,
            Entry::Vacant(e) => e.insert(rx_ch.clone()),
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

    pub fn response_to(
        &mut self,
        req: Request,
        status_code: StatusCode,
        body: Vec<u8>,
    ) -> Result<(SocketAddr, SipMessage), Box<dyn Error>> {
        self.cseq = req.cseq_header()?.seq()?;

        let mut headers: Headers = Default::default();
        for header in req.headers().clone() {
            match header {
                h @ Header::CallId(_)
                | h @ Header::CSeq(_)
                | h @ Header::From(_)
                | h @ Header::Via(_) => headers.push(h),
                ref h @ Header::To(ref to) => match to.tag()? {
                    Some(_) => headers.push(h.clone()),
                    None => {
                        let tag = match &self.to_tag {
                            Some(tag) => tag.clone(),
                            None => {
                                let mut rng = rng();
                                let tag: Tag = rand_chars(&mut rng, 16).into();
                                self.to_tag = Some(tag.clone());
                                tag
                            }
                        };
                        headers.push(Header::To(to.clone().with_tag(tag)?));
                    }
                },
                _ => {}
            }
        }
        headers.push(
            Contact {
                display_name: Some((*USERNAME).clone()),
                uri: Uri {
                    scheme: Some(Scheme::Sip),
                    host_with_port: (*CLIENT_ADDR).into(),
                    auth: Some(Auth {
                        user: (*USERNAME).clone(),
                        password: None,
                    }),
                    ..Default::default()
                },
                params: vec![Param::Q("1".into())],
            }
            .into(),
        );
        headers.push(ContentLength::from(body.len() as u32).into());

        // let host_with_port = {
        //     let uri = match req.contact_headers().first() {
        //         Some(contact) => contact.uri()?,
        //         None => req.via_header()?.uri()?,
        //     };
        //     uri.host_with_port
        // };
        // let addr = SocketAddr::new(host_with_port.host.try_into()?, *host_with_port.port.unwrap_or(rsip::Port::new(5060_u16)).value());
        Ok((
            (*SERVER_ADDR).clone(),
            SipMessage::Response(Response {
                status_code,
                version: Version::V2,
                headers,
                body,
            }),
        ))
    }

    pub fn sdp_response_to(
        &mut self,
        req: Request,
        status_code: StatusCode,
        sdp: SessionDescription,
    ) -> Result<(SocketAddr, SipMessage), Box<dyn Error>> {
        let (addr, mut resp) = self.response_to(req, status_code, sdp.to_string().into_bytes())?;
        resp.headers_mut()
            .push(ContentType(MediaType::Sdp(vec![])).into());
        Ok((addr, resp))
    }

    fn new_request(&mut self, method: Method, body: Vec<u8>) -> Request {
        self.cseq += 1;
        let branch: String = format!("{}{}", BRANCH_PREFIX, rand_chars(&mut rng(), 32));

        let mut headers: Headers = Default::default();
        headers.push(
            CSeq {
                seq: self.cseq,
                method,
            }
            .into(),
        );
        headers.push(
            Via {
                version: Version::V2,
                transport: Transport::Udp,
                uri: Uri {
                    host_with_port: (*CLIENT_ADDR).into(),
                    ..Default::default()
                },
                params: vec![
                    Param::Branch(branch.into()),
                    Param::Other(OtherParam::from("rport"), None),
                ],
            }
            .into(),
        );
        headers.push(UserAgent::from(format!("{}/{}", USER_AGENT, UA_VERSION)).into());
        headers.push(self.call_id.clone().into());
        headers.push(
            Contact {
                display_name: Some((*USERNAME).clone()),
                uri: Uri {
                    scheme: Some(Scheme::Sip),
                    host_with_port: (*CLIENT_ADDR).into(),
                    auth: Some(Auth {
                        user: (*USERNAME).clone(),
                        password: None,
                    }),
                    ..Default::default()
                },
                params: vec![Param::Q("1".into())],
            }
            .into(),
        );
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

    fn new_request_from_to(
        &mut self,
        method: Method,
        from: Uri,
        to: Uri,
        body: Vec<u8>,
    ) -> Request {
        let mut req = self.new_request(method, body);
        req.headers.push(
            From {
                display_name: None,
                uri: from,
                params: vec![self.from_tag.clone().into()],
            }
            .into(),
        );
        req.headers.push(
            To {
                display_name: None,
                uri: to,
                params: vec![self.to_tag.clone()]
                    .into_iter()
                    .filter_map(|t| t.map(|t| Param::Tag(t)))
                    .collect(),
            }
            .into(),
        );

        req
    }

    pub fn add_auth_to_request(&self, req: &mut Request, opaque: Option<String>, nonce: String) {
        let cnonce = format!("{}/{}", ms_since_epoch(), rand_chars(&mut rng(), 16));
        // TODO(peter): Actually track this?
        let nc = 1;

        let ha1 = md5(format!("{}:{}:{}", *USERNAME, REALM, *PASSWORD));
        let ha2 = md5(format!(
            "{}:{}:{}",
            req.method,
            req.uri.scheme.as_ref().unwrap_or(&Scheme::Sip),
            (*SERVER_ADDR)
        ));
        let response = md5(format!(
            "{}:{}:{:08x}:{}:auth:{}",
            ha1, nonce, nc, cnonce, ha2
        ));

        req.headers.push(
            Authorization {
                scheme: auth::Scheme::Digest,
                username: (*USERNAME).clone(),
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
            }
            .into(),
        );
        req.headers.push(Expires::from(3600).into());
    }

    pub fn sdp(&self, sess_id: String) -> SessionDescription {
        SessionDescription {
            version: sdp_rs::lines::Version::V0,
            origin: sdp_rs::lines::Origin {
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
            connection: Some(sdp_rs::lines::Connection {
                nettype: sdp_rs::lines::common::Nettype::In,
                addrtype: sdp_rs::lines::common::Addrtype::Ip4,
                connection_address: (*CLIENT_ADDR).ip().into(),
            }),
            bandwidths: vec![],
            times: Vec1::new(sdp_rs::Time {
                active: sdp_rs::lines::Active { start: 0, stop: 0 },
                repeat: vec![],
                zone: None,
            }),
            key: None,
            attributes: vec![],
            media_descriptions: vec![MediaDescription {
                media: sdp_rs::lines::Media {
                    media: sdp_rs::lines::media::MediaType::Audio,
                    port: 19512, // TODO(peter): Choose and set up an RTP port
                    num_of_ports: None,
                    proto: sdp_rs::lines::media::ProtoType::RtpAvp,
                    fmt: vec![0].into_iter().map(|v| v.to_string()).join(" "),
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
                    sdp_rs::lines::Attribute::Ptime(20.0),
                    sdp_rs::lines::Attribute::Maxptime(140.0),
                    sdp_rs::lines::Attribute::Sendrecv,
                ],
            }],
        }
    }

    pub fn sdp_from(&self, req: Request) -> Result<SessionDescription, Box<dyn Error>> {
        let sdp = SessionDescription::from_str(std::str::from_utf8(&req.body)?)?;
        let sess_id = sdp.origin.sess_id;
        Ok(self.sdp(sess_id))
    }

    pub fn register_request(&mut self) -> Request {
        let mut from_uri = (*MY_URI).clone();
        from_uri.host_with_port = (*CLIENT_ADDR).into();
        let mut req =
            self.new_request_from_to(Method::Register, from_uri, (*MY_URI).clone(), vec![]);
        req.headers.push(Allow::from(Method::all()).into());
        req
    }

    pub fn invite_request(&mut self, to: Uri) -> Request {
        let sess_id = micros_since_epoch().to_string();
        let body = self.sdp(sess_id).to_string();
        let mut req =
            self.new_request_from_to(Method::Invite, (*MY_URI).clone(), to.clone(), body.into());
        req.uri = to;
        req.headers.push(ContentType(MediaType::Sdp(vec![])).into());
        req
    }

    pub fn ack_request(&mut self, resp: Response) -> Request {
        let mut req = self.new_request(Method::Ack, vec![]);
        for header in resp.headers.clone() {
            match header {
                h @ Header::ContentType(_)
                | h @ Header::ContentLength(_)
                | h @ Header::From(_)
                | h @ Header::To(_) => {
                    req.headers.push(h);
                }
                _ => {}
            }
        }
        if resp.body.len() > 0 {
            req.body = resp.body.clone(); // Copy over SDP
        }
        let cseq = resp.cseq_header().unwrap().seq().unwrap();
        req.cseq_header_mut().unwrap().mut_seq(cseq).unwrap();
        req
    }

    pub fn cancel_request(&mut self, to: Uri) -> Request {
        let req = self.new_request_from_to(Method::Cancel, (*MY_URI).clone(), to, vec![]);
        req
    }

    pub fn bye_request(&mut self, peer: Uri, from: headers::From, to: headers::To) -> Request {
        let mut req = self.new_request(Method::Bye, vec![]);
        req.uri = peer;
        req.headers.push(from.into());
        req.headers.push(to.into());
        req
    }
}

//impl Drop for Txn {
//    fn drop(&mut self) {
//        let call_id = self.call_id.clone();
//        tokio::spawn(async move {
//            let txn_mailboxes = TXN_MAILBOXES.clone();
//            let mut mailboxes = txn_mailboxes.write().await;
//            debug!("dropping mailbox for {}", call_id);
//            mailboxes.remove(&call_id.to_string());
//        });
//    }
//}

pub fn assert_resp_successful(resp: &Response) -> Result<(), Box<dyn Error>> {
    match resp.status_code.kind() {
        StatusCodeKind::Successful => Ok(()),
        _ => Err(format!("unsuccessful resp {}", resp.status_code).into()),
    }
}

pub fn add_auth_to_request(req: &mut Request, opaque: Option<String>, nonce: String) {
    let cnonce = format!("{}/{}", ms_since_epoch(), rand_chars(&mut rng(), 16));
    // TODO(peter): Actually track this?
    let nc = 1;

    let ha1 = md5(format!("{}:{}:{}", *USERNAME, REALM, *PASSWORD));
    let ha2 = md5(format!(
        "{}:{}:{}",
        req.method,
        req.uri.scheme.as_ref().unwrap_or(&Scheme::Sips),
        (*FRANDLINE_PBX_ADDR)
    ));
    let response = md5(format!(
        "{}:{}:{:08x}:{}:auth:{}",
        ha1, nonce, nc, cnonce, ha2
    ));

    req.headers.push(
        Authorization {
            scheme: auth::Scheme::Digest,
            username: (*USERNAME).clone(),
            realm: REALM.into(),
            nonce,
            uri: Uri {
                scheme: Some(Scheme::Sips),
                host_with_port: (*FRANDLINE_PBX_ADDR).clone(),
                ..Default::default()
            },
            response,
            algorithm: Some(Algorithm::Md5),
            opaque,
            qop: Some(AuthQop::Auth { cnonce, nc }),
        }
        .into(),
    );
    req.headers.push(Expires::from(3600).into());
}

#[derive(Clone, Debug)]
pub struct Dialog {
    pub tx_ch: mpsc::Sender<SipMessage>,
    pub rx_ch: broadcast::Sender<SipMessage>,

    ip: Ipv4Addr,

    cseq: u32,
    pub call_id: CallId,

    from_tag: Tag,
    to_tag: Option<Tag>,

    rng: StdRng,
}

impl Dialog {
    pub fn new(
        client_ip: Ipv4Addr,
        tx_ch: mpsc::Sender<SipMessage>,
        rx_ch: broadcast::Sender<SipMessage>,
    ) -> Self {
        let mut rng = StdRng::from_rng(&mut rand::rng());
        let call_id = CallId::from(format!("{}/{}", ms_since_epoch(), rand_chars(&mut rng, 16)));

        let from_tag = rand_chars(&mut rng, 16).into();
        let to_tag = None;

        Dialog {
            tx_ch,
            rx_ch,

            ip: client_ip,

            cseq: 0,
            call_id,

            from_tag,
            to_tag,

            rng,
        }
    }

    pub fn from_request(
        ip: Ipv4Addr,
        tx_ch: mpsc::Sender<SipMessage>,
        rx_ch: broadcast::Sender<SipMessage>,
        msg: &SipMessage,
    ) -> Result<Self, Box<dyn Error>> {
        let call_id = msg.call_id_header()?.clone();
        let cseq = msg.cseq_header()?.seq()?;
        let from_tag = msg.from_header()?.tag()?.ok_or("missing from tag")?;
        let to_tag = msg.to_header()?.tag()?.ok_or("missing to tag")?;

        Ok(Dialog {
            tx_ch,
            rx_ch,

            ip,

            cseq,
            call_id,

            // TODO: Do we need to invert these because the request should be coming from the server?
            from_tag: from_tag,
            to_tag: Some(to_tag),

            rng: StdRng::from_rng(&mut rand::rng()),
        })
    }

    pub fn new_request(&mut self, method: Method, body: Vec<u8>) -> Request {
        self.cseq += 1;
        let branch: String = format!("{}{}", BRANCH_PREFIX, rand_chars(&mut self.rng, 32));

        let mut headers: Headers = Default::default();
        headers.push(
            CSeq {
                seq: self.cseq,
                method,
            }
            .into(),
        );
        headers.push(
            Via {
                version: Version::V2,
                transport: Transport::Tls,
                uri: Uri {
                    host_with_port: (*FRANDLINE_PBX_ADDR).clone(),
                    ..Default::default()
                },
                params: vec![
                    Param::Branch(branch.into()),
                    Param::Other(OtherParam::from("rport"), None),
                ],
            }
            .into(),
        );
        headers.push(UserAgent::from(format!("{}/{}", USER_AGENT, UA_VERSION)).into());
        headers.push(self.call_id.clone().into());
        headers.push(
            Contact {
                display_name: Some((*USERNAME).clone()),
                uri: Uri {
                    scheme: Some(Scheme::Sips),
                    host_with_port: IpAddr::V4(self.ip).into(),
                    auth: Some(Auth {
                        user: (*USERNAME).clone(),
                        password: None,
                    }),
                    ..Default::default()
                },
                params: vec![Param::Q("1".into())],
            }
            .into(),
        );
        headers.push(MaxForwards::from(MAX_FORWARDS).into());
        headers.push(ContentLength::from(body.len() as u32).into());

        // TODO: Move these somewhere else
        headers.push(
            From {
                display_name: Some("1103".to_string()),
                uri: (*SIPS_URI).clone(),
                params: vec![self.from_tag.clone().into()],
            }
            .into(),
        );
        headers.push(
            To {
                display_name: Some("1103".to_string()),
                uri: (*SIPS_URI).clone(),
                // TODO: Use the to_tag here and generate it
                params: vec![self.from_tag.clone().into()],
            }
            .into(),
        );

        Request {
            method,
            uri: Uri {
                scheme: Some(Scheme::Sips),
                host_with_port: (*FRANDLINE_PBX_ADDR).clone(),
                ..Default::default()
            },
            version: Version::V2,
            headers,
            body,
        }
    }

    pub async fn send(
        &self,
        msg: (impl Into<SipMessage> + Clone),
    ) -> Result<(), mpsc::error::SendError<SipMessage>> {
        debug!(call_id=%self.call_id.value().to_string(), msg=%msg.clone().into().to_string().lines().next().unwrap_or("empty"), "SIP Send");
        self.tx_ch.send((msg).into()).await
    }

    pub async fn recv(&self) -> Result<SipMessage, broadcast::error::RecvError> {
        let msg = self.rx_ch.subscribe().recv().await?;
        debug!(call_id=%self.call_id.value().to_string(), msg=%msg.clone().to_string().lines().next().unwrap_or("empty"), "SIP Recv");
        Ok(msg)
    }

    pub async fn register(&mut self) -> Result<(), Box<dyn Error>> {
        let req = self.new_request(rsip::Method::Register, vec![]);
        self.send(req.clone()).await?;

        let resp: Response = self.recv().await?.try_into()?;
        let www_auth = resp
            .www_authenticate_header()
            .ok_or("missing www auth header")?
            .typed()?;

        let mut authed_req = self.new_request(rsip::Method::Register, vec![]);
        add_auth_to_request(&mut authed_req, www_auth.opaque, www_auth.nonce);
        self.send(authed_req.clone()).await?;

        let resp: Response = self.recv().await?.try_into()?;
        assert_resp_successful(&resp)?;

        Ok(())
    }
}
