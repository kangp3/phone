use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use md5::{Digest, Md5};
use rand::distr::Alphanumeric;
use rand::prelude::*;
use rand::rngs::StdRng;
use rand::{rng, Rng};
use rsip::headers::auth::{Algorithm, AuthQop};
use rsip::headers::{auth, CallId, ContentLength, Expires, MaxForwards, UserAgent};
use rsip::param::OtherParam;
use rsip::typed::{Authorization, CSeq, Contact, ContentType, From, MediaType, To, Via};
use rsip::{prelude::*, StatusCodeKind};
use rsip::{
    Auth, Header, Headers, HostWithPort, Method, Param, Request, Response, Scheme, SipMessage,
    StatusCode, Transport, Uri, Version,
};
use sdp_rs::{MediaDescription, SessionDescription};
use tokio::sync::mpsc;
use tracing::{debug, trace};
use uuid::Uuid;
use vec1::Vec1;

const REALM: &str = "asterisk";
const USER_AGENT: &str = "Frandline";
const UA_VERSION: &str = "0.1.0";
// Branch should always be prefixed with magic string z9hG4bK
// https://www.ietf.org/rfc/rfc3261.txt (8.1.1.7)
const BRANCH_PREFIX: &str = "z9hG4bK";
const MAX_FORWARDS: u32 = 70;

pub const SERVER_NAME: &str = "pbx.frandline.com";
pub const SERVER_PORT: u16 = 5061;

fn md5(s: String) -> String {
    let mut hasher = Md5::new();
    hasher.update(s);
    format!("{:x}", hasher.finalize())
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

pub fn assert_status(resp: &Response) -> Result<()> {
    match resp.status_code.kind() {
        StatusCodeKind::Successful | StatusCodeKind::Provisional => Ok(()),
        _ => Err(anyhow!("unsuccessful resp {}", resp.status_code)),
    }
}

fn uri(user: String, host_with_port: HostWithPort) -> Uri {
    Uri {
        scheme: Some(Scheme::Sips),
        auth: Some(Auth {
            user,
            password: None,
        }),
        host_with_port,
        ..Default::default()
    }
}

fn flip_from(from: From) -> To {
    To {
        display_name: from.display_name,
        uri: from.uri,
        params: from.params,
    }
}

fn flip_to(to: To) -> From {
    From {
        display_name: to.display_name,
        uri: to.uri,
        params: to.params,
    }
}

#[derive(Debug)]
pub struct Dialog {
    pub tx_ch: mpsc::Sender<SipMessage>,
    pub rx_ch: mpsc::Receiver<SipMessage>,

    server_host: HostWithPort,
    client_ip: Ipv4Addr,
    sip_instance_uuid: Uuid,

    username: String,

    cseq: u32,

    pub call_id: CallId,
    from: From,
    to: Option<To>,

    rng: StdRng,
}

impl Dialog {
    pub fn new(
        server_host: HostWithPort,
        client_ip: Ipv4Addr,
        sip_instance_uuid: Uuid,
        username: String,
        tx_ch: mpsc::Sender<SipMessage>,
        rx_ch: mpsc::Receiver<SipMessage>,
    ) -> Self {
        let mut rng = StdRng::from_rng(&mut rand::rng());
        let call_id = CallId::from(format!("{}/{}", ms_since_epoch(), rand_chars(&mut rng, 16)));

        let from = From {
            display_name: Some(username.clone()),
            uri: uri(username.clone(), server_host.clone()),
            params: vec![],
        }
        .with_tag(rand_chars(&mut rng, 16).into());

        Dialog {
            tx_ch,
            rx_ch,

            server_host,
            client_ip,
            sip_instance_uuid,

            username,

            cseq: 0,
            call_id,

            from,
            to: None,

            rng,
        }
    }

    pub fn from_request(
        server_host: HostWithPort,
        client_ip: Ipv4Addr,
        sip_instance_uuid: Uuid,
        tx_ch: mpsc::Sender<SipMessage>,
        rx_ch: mpsc::Receiver<SipMessage>,
        msg: &SipMessage,
    ) -> Result<Self> {
        let mut rng = StdRng::from_rng(&mut rand::rng());

        let call_id = msg.call_id_header()?.clone();
        let cseq = msg.cseq_header()?.seq()?;
        let from = msg.from_header()?.typed()?;
        let to = msg
            .to_header()?
            .typed()?
            .with_tag(rand_chars(&mut rng, 16).into());
        let username = to.uri.user().ok_or(anyhow!("missing to user"))?.to_string();

        Ok(Dialog {
            tx_ch,
            rx_ch,

            server_host,
            client_ip,
            sip_instance_uuid,

            username,

            cseq,
            call_id,

            from: flip_to(to),
            to: Some(flip_from(from)),

            rng,
        })
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
                unicast_address: self.client_ip.into(),
            },
            session_name: sdp_rs::lines::SessionName::from(USER_AGENT.to_string()),
            session_info: None,
            uri: None,
            emails: vec![],
            phones: vec![],
            connection: Some(sdp_rs::lines::Connection {
                nettype: sdp_rs::lines::common::Nettype::In,
                addrtype: sdp_rs::lines::common::Addrtype::Ip4,
                connection_address: IpAddr::V4(self.client_ip).into(),
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

    pub fn sdp_from(&self, req: Request) -> Result<SessionDescription> {
        let sdp = SessionDescription::from_str(std::str::from_utf8(&req.body)?)?;
        let sess_id = sdp.origin.sess_id;
        Ok(self.sdp(sess_id))
    }

    pub fn sdp_response_to(
        &mut self,
        req: Request,
        status_code: StatusCode,
        sdp: SessionDescription,
    ) -> Result<SipMessage> {
        let mut resp = self.response_to(req, status_code, sdp.to_string().into_bytes())?;
        resp.headers_mut()
            .push(ContentType(MediaType::Sdp(vec![])).into());
        Ok(resp)
    }

    pub async fn send(&self, msg: impl Into<SipMessage> + Clone) -> Result<()> {
        trace!(
            user=%self.username,
            call_id=%self.call_id.value().to_string(),
            msg=%msg.clone().into().to_string(),
            "SIP Send lines",
        );
        debug!(
            user=%self.username,
            call_id=%self.call_id.value().to_string(),
            msg=%msg.clone().into().to_string().lines().next().unwrap_or("empty"),
            "SIP Send",
        );
        self.tx_ch.send((msg).into()).await?;
        Ok(())
    }

    pub async fn recv(&mut self) -> Result<SipMessage> {
        let msg = self
            .rx_ch
            .recv()
            .await
            .ok_or(anyhow!("failed recv dialog rx"))?;
        trace!(
            user=%self.username,
            call_id=%self.call_id.value().to_string(),
            msg=%msg.clone().to_string(),
            "SIP Recv lines",
        );
        debug!(
            user=%self.username,
            call_id=%self.call_id.value().to_string(),
            msg=%msg.clone().to_string().lines().next().unwrap_or("empty"),
            "SIP Recv",
        );
        Ok(msg)
    }

    fn contact(&self) -> Contact {
        Contact {
            display_name: Some(self.username.clone()),
            uri: Uri {
                scheme: Some(Scheme::Sips),
                host_with_port: self.server_host.clone(),
                auth: Some(Auth {
                    user: self.username.clone(),
                    password: None,
                }),
                params: vec![
                    Param::Transport(Transport::Tls),
                    Param::Other("ob".into(), None),
                ],
                ..Default::default()
            },
            params: vec![
                Param::Q("1".into()),
                Param::Other(
                    "+sip.instance".into(),
                    Some(format!("\"<urn:uuid:{}>\"", self.sip_instance_uuid).into()),
                ),
                Param::Other("reg-id".into(), Some("1".into())),
            ],
        }
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
                    host_with_port: self.server_host.clone(),
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
        headers.push(self.contact().into());
        headers.push(self.from.clone().into());
        if let Some(to) = &self.to {
            headers.push(to.clone().into());
        }
        headers.push(MaxForwards::from(MAX_FORWARDS).into());
        headers.push(ContentLength::from(body.len() as u32).into());

        Request {
            method,
            uri: Uri {
                scheme: Some(Scheme::Sips),
                host_with_port: self.server_host.clone(),
                ..Default::default()
            },
            version: Version::V2,
            headers,
            body,
        }
    }

    fn new_register_request(&mut self) -> Result<Request> {
        let mut req = self.new_request(rsip::Method::Register, vec![]);
        let from_header = req.from_header()?.typed()?;
        // Register should only run on the start of a dialog, so To hasn't been set.
        // It should be set with the same URI as the From, but without a tag.
        let to_header = To {
            display_name: from_header.display_name,
            uri: from_header.uri,
            params: vec![],
        };
        req.headers_mut().push(to_header.into());
        Ok(req)
    }

    pub fn response_to(
        &mut self,
        req: Request,
        status_code: StatusCode,
        body: Vec<u8>,
    ) -> Result<SipMessage> {
        self.cseq = req.cseq_header()?.seq()?;

        let mut headers: Headers = Default::default();
        for header in req.headers().clone() {
            match header {
                h @ Header::CallId(_)
                | h @ Header::CSeq(_)
                | h @ Header::From(_)
                | h @ Header::Via(_) => headers.push(h),
                ref h @ Header::To(ref to) => match (to.tag()?, self.to.clone()) {
                    (Some(_), _) => headers.push(h.clone()),
                    (None, Some(to)) => headers.push(to.into()),
                    (None, None) => {
                        let to = to.typed()?.with_tag(rand_chars(&mut self.rng, 16).into());
                        self.to = Some(to.clone());
                        headers.push(to.into());
                    }
                },
                _ => {}
            }
        }
        headers.push(self.contact().into());
        headers.push(ContentLength::from(body.len() as u32).into());

        Ok(SipMessage::Response(Response {
            status_code,
            version: Version::V2,
            headers,
            body,
        }))
    }

    pub fn add_auth_to_request(
        &self,
        req: &mut Request,
        password: String,
        opaque: Option<String>,
        nonce: String,
    ) {
        let cnonce = format!("{}/{}", ms_since_epoch(), rand_chars(&mut rng(), 16));
        // TODO(peter): Actually track this?
        let nc = 1;

        let ha1 = md5(format!("{}:{}:{}", self.username, REALM, password));
        let ha2 = md5(format!(
            "{}:{}:{}",
            req.method,
            req.uri.scheme.as_ref().unwrap_or(&Scheme::Sips),
            self.server_host,
        ));
        let response = md5(format!(
            "{}:{}:{:08x}:{}:auth:{}",
            ha1, nonce, nc, cnonce, ha2
        ));

        req.headers.push(
            Authorization {
                scheme: auth::Scheme::Digest,
                username: self.username.clone(),
                realm: REALM.into(),
                nonce,
                uri: Uri {
                    scheme: Some(Scheme::Sips),
                    host_with_port: self.server_host.clone(),
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

    pub fn set_to(&mut self, to: To) {
        self.to = Some(to);
    }

    pub async fn register(&mut self, password: String) -> Result<()> {
        let req = self.new_register_request()?;
        self.send(req).await?;

        let resp: Response = self.recv().await?.try_into()?;
        let www_auth = resp
            .www_authenticate_header()
            .ok_or(anyhow!("missing www auth header"))?
            .typed()?;

        let mut authed_req = self.new_register_request()?;
        self.add_auth_to_request(&mut authed_req, password, www_auth.opaque, www_auth.nonce);
        self.send(authed_req.clone()).await?;

        let resp: Response = self.recv().await?.try_into()?;
        assert_status(&resp)?;

        Ok(())
    }

    pub async fn invite(&mut self, password: String, to: To) -> Result<()> {
        let sess_id = micros_since_epoch().to_string();
        let body = self.sdp(sess_id).to_string();
        let mut req = self.new_request(Method::Invite, body.clone().into());
        req.uri = to.clone().uri;
        req.headers.push(to.clone().into());
        req.headers.push(ContentType(MediaType::Sdp(vec![])).into());
        self.send(req).await?;

        let resp: Response = self.recv().await?.try_into()?;
        let www_auth = resp
            .www_authenticate_header()
            .ok_or(anyhow!("missing www auth header"))?
            .typed()?;

        let mut req = self.new_request(Method::Invite, body.into());
        req.uri = to.clone().uri;
        req.headers.push(to.into());
        req.headers.push(ContentType(MediaType::Sdp(vec![])).into());
        self.add_auth_to_request(&mut req, password, www_auth.opaque, www_auth.nonce);
        self.send(req).await?;

        let resp = self.recv().await?;
        assert_status(&resp.clone().try_into()?)?;

        Ok(())
    }

    pub async fn ack(&mut self, resp: Response) -> Result<()> {
        let mut req = self.new_request(Method::Ack, vec![]);

        let cseq = resp.cseq_header()?.seq()?;
        req.cseq_header_mut()?.mut_seq(cseq)?;
        self.send(req).await?;

        Ok(())
    }

    pub async fn cancel(&mut self) -> Result<()> {
        let req = self.new_request(Method::Cancel, vec![]);
        self.send(req).await?;

        Ok(())
    }

    pub async fn bye(&mut self) -> Result<()> {
        let req = self.new_request(Method::Bye, vec![]);
        self.send(req).await?;

        Ok(())
    }
}
