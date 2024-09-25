use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::LazyLock;

use local_ip_address::local_ip;
use md5::{Md5, Digest};
use rsip::headers::auth::{self, Algorithm, AuthQop};
use rsip::headers::{CallId, ContentLength, Expires, MaxForwards, UserAgent};
use rsip::param::OtherParam;
use rsip::prelude::ToTypedHeader;
use rsip::typed::{Allow, Authorization, CSeq, Contact, From, To, Via};
use rsip::{Auth, Header, Headers, Method, Param, Request, Scheme, SipMessage, Transport, Uri, Version};
use tokio::sync::{broadcast, mpsc};
use tracing::debug;


const USER_AGENT: &str = "Frandline";
const UA_VERSION: &str = "0.1.0";

const BRANCH: &str = "branch-jiogadrbocaw";
const TAG: &str = "tag-rjiowuqyropdrbocjwort";
const CNONCE: &str = "cnonce-ruitodabicawerioajwefiojodsfd";
const CALL_ID: &str = "call-id-gjiorbjcohquwrtioquweoruieo";

// TODO(peter): Make these configurable
pub static SERVER_ADDR: LazyLock<SocketAddr> = LazyLock::new(|| {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 12, 182)), 5060)
});
const USERNAME: &str = "1100";
const PASSWORD: &str = "SW2fur7facrarac";

pub fn register_request() -> SipMessage {
    let server_ip = (*SERVER_ADDR).ip();
    let my_ip = local_ip().unwrap();
    let socket_addr = SocketAddr::new(my_ip, 5060);

    let mut headers: Headers = Default::default();
    headers.push(CSeq{ seq: 1, method: Method::Register }.into());
    headers.push(Via{
        version: Version::V2,
        transport: Transport::Udp,
        uri: Uri {
            host_with_port: socket_addr.into(),
            ..Default::default()
        },
        params: vec![
            Param::Branch(BRANCH.into()),
            Param::Other(OtherParam::from("rport"), None)
        ],
    }.into());
    headers.push(UserAgent::from(format!("{}/{}", USER_AGENT, UA_VERSION)).into());
    headers.push(From{
        display_name: Some(USERNAME.into()),
        uri: Uri {
            scheme: Some(Scheme::Sip),
            host_with_port: server_ip.into(),
            auth: Some(Auth{
                user: USERNAME.into(),
                password: None,
            }),
            ..Default::default()
        },
        params: vec![Param::Tag(TAG.into())],
    }.into());
    headers.push(CallId::from(CALL_ID).into());
    headers.push(To{
        display_name: None,
        uri: Uri {
            scheme: Some(Scheme::Sip),
            host_with_port: server_ip.into(),
            auth: Some(Auth{
                user: USERNAME.into(),
                password: None,
            }),
            ..Default::default()
        },
        params: vec![],
    }.into());
    headers.push(Contact{
        display_name: Some(USERNAME.into()),
        uri: Uri {
            scheme: Some(Scheme::Sip),
            host_with_port: server_ip.into(),
            auth: Some(Auth{
                user: USERNAME.into(),
                password: None,
            }),
            ..Default::default()
        },
        params: vec![Param::Q("1".into())],
    }.into());
    headers.push(Allow::from(Method::all()).into());
    headers.push(Expires::from(3600).into());
    headers.push(ContentLength::from(0).into());
    headers.push(MaxForwards::from(70).into());

    SipMessage::Request(Request {
        method: Method::Register,
        uri: Uri {
            scheme: Some(Scheme::Sip),
            host_with_port: server_ip.into(),
            ..Default::default()
        },
        headers,
        version: Version::V2,
        body: vec![],
    })
}

fn md5(s: String) -> String {
    let mut hasher = Md5::new();
    hasher.update(s);
    format!("{:x}", hasher.finalize())
}

pub fn authed_register_request(opaque: Option<String>, nonce: String) -> SipMessage {
    let server_ip = (*SERVER_ADDR).ip();
    let my_ip = local_ip().unwrap();
    let socket_addr = SocketAddr::new(my_ip, 5060);

    let cnonce = String::from(CNONCE);
    let nc = 1;

    let ha1 = md5(format!("{}:{}:{}", USERNAME, "asterisk", PASSWORD));
    let ha2 = md5(format!("REGISTER:sip:{}", (*SERVER_ADDR).ip()));
    let response = md5(format!("{}:{}:{:08x}:{}:auth:{}", ha1, nonce, nc, cnonce, ha2));

    let mut headers: Headers = Default::default();
    headers.push(CSeq{ seq: 2, method: Method::Register }.into());
    headers.push(Via{
        version: Version::V2,
        transport: Transport::Udp,
        uri: Uri {
            host_with_port: socket_addr.into(),
            ..Default::default()
        },
        params: vec![
            Param::Branch(BRANCH.into()),
            Param::Other(OtherParam::from("rport"), None)
        ],
    }.into());
    headers.push(UserAgent::from("Frandline/0.1.0").into());
    headers.push(Authorization{
        scheme: auth::Scheme::Digest,
        username: USERNAME.into(),
        realm: "asterisk".to_string(),
        nonce,
        uri: Uri {
            scheme: Some(Scheme::Sip),
            host_with_port: server_ip.into(),
            auth: None,
            ..Default::default()
        },
        response,
        algorithm: Some(Algorithm::Md5),
        opaque,
        qop: Some(AuthQop::Auth { cnonce, nc }),
    }.into());
    headers.push(From{
        display_name: None,
        uri: Uri {
            scheme: Some(Scheme::Sip),
            host_with_port: server_ip.into(),
            auth: Some(Auth{
                user: USERNAME.into(),
                password: None,
            }),
            ..Default::default()
        },
        params: vec![Param::Tag(TAG.into())],
    }.into());
    headers.push(CallId::from(CALL_ID).into());
    headers.push(To{
        display_name: None,
        uri: Uri {
            scheme: Some(Scheme::Sip),
            host_with_port: server_ip.into(),
            auth: Some(Auth{
                user: USERNAME.into(),
                password: None,
            }),
            ..Default::default()
        },
        params: vec![],
    }.into());
    headers.push(Contact{
        display_name: None,
        uri: Uri {
            scheme: Some(Scheme::Sip),
            host_with_port: my_ip.into(),
            auth: Some(Auth{
                user: USERNAME.into(),
                password: None,
            }),
            ..Default::default()
        },
        params: vec![Param::Q("1".into())],
    }.into());
    headers.push(Allow::from(Method::all()).into());
    headers.push(Expires::from(3600).into());
    headers.push(ContentLength::from(0).into());
    headers.push(MaxForwards::from(70).into());

    SipMessage::Request(Request {
        method: Method::Register,
        uri: Uri {
            scheme: Some(Scheme::Sip),
            host_with_port: server_ip.into(),
            ..Default::default()
        },
        headers,
        version: Version::V2,
        body: vec![],
    })
}

pub async fn register(out_ch: mpsc::Sender<(SocketAddr, SipMessage)>, mut in_ch: broadcast::Receiver<(SocketAddr, SipMessage)>) -> Result<(), Box<dyn Error>> {
    let msg = register_request();
    debug!("{}", msg);
    out_ch.send((*SERVER_ADDR, msg)).await?;
    let (_, msg) = in_ch.recv().await?;
    debug!("{}", msg);
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
        let req = authed_register_request(opaque, nonce);
        debug!("AUTH REQ: {}", req);
        out_ch.send((*SERVER_ADDR, req)).await?;
    }
    Ok(())
}
