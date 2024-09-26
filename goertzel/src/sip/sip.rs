use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, LazyLock};
use std::time::{SystemTime, UNIX_EPOCH};

use local_ip_address::local_ip;
use md5::{Md5, Digest};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rsip::headers::auth::{Algorithm, AuthQop};
use rsip::headers::{auth, CallId, ContentLength, Expires, MaxForwards, UserAgent};
use rsip::param::OtherParam;
use rsip::prelude::ToTypedHeader;
use rsip::typed::{Allow, Authorization, CSeq, Contact, From, To, Via};
use rsip::{Auth, Header, Headers, Method, Param, Request, Response, Scheme, SipMessage, StatusCode, Transport, Uri, Version};
use tokio::sync::{mpsc, RwLock, RwLockWriteGuard};


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
pub static SERVER_ADDR: LazyLock<SocketAddr> = LazyLock::new(|| {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 12, 182)), 5060)
});
pub static CLIENT_ADDR: LazyLock<SocketAddr> = LazyLock::new(|| {
    SocketAddr::new(local_ip().unwrap(), 5060)
});

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
    let msg = SipMessage::Request(txn.register_request()?);
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
            let mut req = txn.register_request()?;
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

    pub fn response_to(&self, req: Request, status_code: StatusCode, body: Vec<u8>) -> Result<Response, Box<dyn Error>> {
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

        Ok(Response{
            status_code,
            version: Version::V2,
            headers,
            body,
        })
    }

    fn new_request(&mut self, method: Method, body: Vec<u8>) -> Result<Request, Box<dyn Error>> {
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
        headers.push(From{
            display_name: Some(USERNAME.into()),
            uri: Uri {
                scheme: Some(Scheme::Sip),
                host_with_port: (*SERVER_ADDR).into(),
                auth: Some(Auth{
                    user: USERNAME.into(),
                    password: None,
                }),
                ..Default::default()
            },
            params: vec![self.from_tag.clone()],
        }.into());
        headers.push(self.call_id.clone().into());
        headers.push(To{
            display_name: None,
            uri: Uri {
                scheme: Some(Scheme::Sip),
                host_with_port: (*SERVER_ADDR).into(),
                auth: Some(Auth{
                    user: USERNAME.into(),
                    password: None,
                }),
                ..Default::default()
            },
            params: vec![self.to_tag.clone()].into_iter().filter_map(|t| t).collect(),
        }.into());
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

        Ok(Request {
            method,
            uri: Uri {
                scheme: Some(Scheme::Sip),
                host_with_port: (*SERVER_ADDR).into(),
                ..Default::default()
            },
            version: Version::V2,
            headers,
            body,
        })
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

    pub fn register_request(&mut self) -> Result<Request, Box<dyn Error>> {
        let mut req = self.new_request(Method::Register, vec![])?;
        req.headers.push(Allow::from(Method::all()).into());
        Ok(req)
    }
}
