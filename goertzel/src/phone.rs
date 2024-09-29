use std::error::Error;
use std::net::SocketAddr;
use std::time::Duration;

use rsip::prelude::{HeadersExt, ToTypedHeader};
use rsip::SipMessage;
use tokio::process::Command;
use tokio::select;
use tokio::sync::{broadcast, mpsc};
use tokio::time::sleep;
use tracing::{debug, error, info};

use crate::contacts::CONTACTS;
use crate::sip::{Txn, SERVER_ADDR, TXN_MAILBOXES};
use crate::{audio, deco, ring, sip};
use crate::hook::{self, SwitchHook};
use crate::nettest::do_i_have_internet;
use crate::tone::TwoToneGen;
use crate::{dtmf, pulse};

pub enum State {
    Connected(Dial),
    Disconnected(WiFi),
}

pub enum WiFi {
    OnHook,  // On hook, standby
    Await,   // Awaiting user input for SSID and pass
    Error(Box<dyn Error>),
}

// TODO(peter): SIP registration steps
pub enum Dial {
    OnHook,
    Ringing,
    Await,
    DialOut(u32, rsip::headers::From),
    Dialing(u32, rsip::headers::From, rsip::headers::To),
    Connected(u32, rsip::Uri, rsip::headers::From, rsip::headers::To),
    Busy,
    Error(Box<dyn Error>),
}

pub struct Phone {
    pub state: State,

    #[cfg(target_os = "macos")]
    _shk_pin: (),
    #[cfg(target_os = "linux")]
    _shk_pin: rppal::gpio::InputPin,

    pub audio_in_ch: broadcast::Sender<f32>,
    _audio_in_stream: cpal::Stream,
    _audio_in_cfg: cpal::SupportedStreamConfig,

    pub audio_out_ch: mpsc::Sender<f32>,
    _audio_out_stream: cpal::Stream,
    audio_out_n_channels: u16,
    audio_out_sample_rate: u32,

    pub hook_ch: broadcast::Sender<SwitchHook>,
    pub pulse_ch: broadcast::Sender<u8>,

    sip_send_ch: mpsc::Sender<(SocketAddr, SipMessage)>,
    sip_txn_ch: mpsc::Receiver<sip::Txn>,

    sip_txn: Option<sip::Txn>,

    to_uri: Option<rsip::Uri>,
}

impl Phone {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let (mic_ch, mic_stream, mic_cfg) = audio::get_input_channel()?;
        let (spk_ch, spk_stream, spk_cfg) = audio::get_output_channel()?;

        let (shk_pin, _, shk_ch) = hook::try_register_shk()?;
        let (pulse_ch, _, hook_ch, _) = pulse::notgoertzelme(shk_ch);

        let has_internet = do_i_have_internet().await?;
        #[cfg(target_os = "macos")]
        let is_on_hook = true;
        #[cfg(target_os = "linux")]
        let is_on_hook = shk_pin.is_low();
        let state = match (has_internet, is_on_hook) {
            (true, true) => State::Connected(Dial::OnHook),
            (true, false) => State::Connected(Dial::Await),
            (false, true) => State::Disconnected(WiFi::OnHook),
            (false, false) => State::Disconnected(WiFi::Await),
        };

        let (sip_send_ch, sip_txn_ch) = sip::socket::bind().await?;
        if has_internet {
            debug!("Registering to SIP server");
            sip::register(sip_send_ch.clone()).await?;
        };

        Ok(Self {
            state,

            _shk_pin: shk_pin,

            audio_in_ch: mic_ch,
            _audio_in_stream: mic_stream,
            _audio_in_cfg: mic_cfg,

            audio_out_ch: spk_ch,
            _audio_out_stream: spk_stream,
            audio_out_sample_rate: spk_cfg.sample_rate().0,
            audio_out_n_channels: spk_cfg.channels(),

            hook_ch,
            pulse_ch,

            sip_send_ch,
            sip_txn_ch,

            sip_txn: None,

            to_uri: None,
        })
    }

    async fn get_wifi_creds(&self) -> Result<(), Box<dyn Error>> {
        let pulse_ch = self.pulse_ch.subscribe();
        let goertzel_ch = dtmf::goertzelme(self.audio_in_ch.subscribe());
        let mut chars_ch = deco::ding(goertzel_ch, pulse_ch);

        let mut ssid = String::new();
        let mut pass = String::new();
        while let Some(c) = chars_ch.recv().await {
            if c == '\0' {
                break;
            }
            debug!("{}", &c);
            ssid.push(c);
        }
        info!("{}", &ssid);
        while let Some(c) = chars_ch.recv().await {
            if c == '\0' {
                break;
            }
            debug!("{}", &c);
            pass.push(c);
        }
        info!("{}", &pass);

        #[cfg(target_os = "linux")]
        let status = Command::new("nmcli")
            .args(&["--wait", "20"])
            .args(&["device", "wifi"])
            .arg("connect")
            .arg(&ssid)
            .args(&["password", &pass])
            .spawn()?
            .wait()
            .await?;
        #[cfg(target_os = "macos")]
        let status = Command::new("networksetup")
            .arg("-setairportnetwork")
            .arg("en0")
            .arg(&ssid)
            .arg(&pass)
            .spawn()?
            .wait()
            .await?;

        if !status.success() {
            Err("no Wi-Fi 4 me :(".into())
        } else {
            Ok(())
        }
    }

    pub async fn begin_life(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            match &self.state {
                State::Connected(Dial::OnHook) => {
                    debug!("phone on hook");
                    self.sip_txn = None;
                    let mut hook_ch = self.hook_ch.subscribe();

                    self.state = loop {
                        select! {
                            txn = self.sip_txn_ch.recv() => {
                                let txn = txn.ok_or("txn ch closed")?;
                                self.sip_txn = Some(txn);
                                break State::Connected(Dial::Ringing);
                            },
                            hook_evt = hook_ch.recv() => match hook_evt {
                                Ok(SwitchHook::ON) => {},
                                Ok(SwitchHook::OFF) => break State::Connected(Dial::Await),
                                Err(e) => break State::Connected(Dial::Error(Box::new(e))),
                            },
                        }
                    };
                }
                State::Connected(Dial::Ringing) => {
                    debug!("ringing");
                    let mut hook_ch = self.hook_ch.subscribe();
                    let ring_handle = ring::ring_phone()?;

                    let txn = self.sip_txn.as_mut().ok_or("missing txn in ringing state")?;
                    let msg = txn.rx_ch.recv().await.ok_or("closed rx channel in ringing")?;
                    let invite = match msg {
                        SipMessage::Request(req) => req,
                        SipMessage::Response(_) => Err("unexpected response")?,
                    };
                    let peer = invite.contact_header()?.uri()?;
                    let remote = {
                        let from = invite.from_header()?.typed()?;
                        rsip::typed::To {
                            display_name: from.display_name,
                            uri: from.uri,
                            params: from.params,
                        }
                    };
                    let (addr, resp) = txn.response_to(invite.clone(), rsip::StatusCode::Ringing, vec![])?;
                    txn.tx_ch.send((addr, resp)).await?;

                    self.state = loop {
                        select! {
                            msg = txn.rx_ch.recv() => match msg.ok_or("closed rx channel in ringing")? {
                                SipMessage::Request(req) => match req.method() {
                                    rsip::Method::Cancel => {
                                        let (addr, resp) = txn.response_to(req, rsip::StatusCode::RequestTerminated, vec![])?;
                                        txn.tx_ch.send((addr, resp)).await?;
                                        break State::Connected(Dial::OnHook)
                                    },
                                    _ => Err(format!("got non-cancel request during ringing: {}", req))?,
                                },
                                SipMessage::Response(_) => Err("got unexpected response during ringing")?,
                            },
                            hook_evt = hook_ch.recv() => match hook_evt {
                                Ok(SwitchHook::ON) => {},
                                Ok(SwitchHook::OFF) => {
                                    // TODO(peter): Include appropriate SDP params in OK
                                    let sdp = txn.sdp_from(invite.clone())?;
                                    let (addr, ref resp) = txn.sdp_response_to(invite.clone(), rsip::StatusCode::OK, sdp)?;
                                    let cseq = invite.cseq_header()?.seq()?;
                                    let local = {
                                        let to = resp.to_header()?.typed()?;
                                        rsip::typed::From {
                                            display_name: to.display_name,
                                            uri: to.uri,
                                            params: to.params,
                                        }
                                    };
                                    txn.tx_ch.send((addr, resp.clone())).await?;
                                    match txn.rx_ch.recv().await.ok_or("closed rx channel in ringing")? {
                                        SipMessage::Request(req) => match req.method() {
                                            rsip::Method::Ack => break State::Connected(Dial::Connected(cseq, peer.clone(), local.into(), remote.into())),
                                            _ => Err(format!("got non-ack request during ringing: {}", req))?,
                                        }
                                        _ => Err("got unexpected response during ringing")?,
                                    }
                                },
                                Err(e) => break State::Connected(Dial::Error(Box::new(e))),
                            },
                        }
                    };

                    ring_handle.abort();
                },
                State::Connected(Dial::Await) => {
                    debug!("phone picked up");
                    let audio_out_ch = self.audio_out_ch.clone();
                    let pulse_ch = self.pulse_ch.subscribe();
                    let goertzel_ch = dtmf::goertzelme(self.audio_in_ch.subscribe());
                    let mut hook_ch = self.hook_ch.subscribe();
                    let tone_handle = TwoToneGen::off_hook(self.audio_out_sample_rate)
                        .send_to(audio_out_ch, self.audio_out_n_channels);
                    let mut dig_ch = deco::de_digs(goertzel_ch, pulse_ch);
                    self.sip_txn = {
                        let txn_mailboxes = TXN_MAILBOXES.clone();
                        let mailboxes = txn_mailboxes.write().await;
                        Some(Txn::new(self.sip_send_ch.clone(), mailboxes))
                    };
                    let txn = self.sip_txn.as_mut().ok_or("how is txn missing here")?;

                    let mut number = String::new();
                    self.state = loop {
                        select! {
                            _ = sleep(Duration::from_secs(4)), if (*CONTACTS).contains_key(&number) => {
                                let to = (*CONTACTS).get(&number).ok_or("contact is missing after I EXPLICITLY checked it")?;
                                self.to_uri = Some(to.clone());
                                let msg = SipMessage::Request(txn.invite_request(to.clone()));
                                txn.tx_ch.send(((*SERVER_ADDR).clone(), msg)).await?;
                                let msg = txn.rx_ch.recv().await.ok_or("closed rx channel in ringing")?;
                                match msg {
                                    SipMessage::Request(_) => Err("unexpected request")?,
                                    SipMessage::Response(resp) => {
                                        let mut opaque = None;
                                        let mut nonce = String::new();
                                        for header in resp.headers {
                                            match header {
                                                rsip::Header::WwwAuthenticate(h) => {
                                                    let h = h.typed()?;
                                                    opaque = h.opaque;
                                                    nonce = h.nonce;
                                                    break;
                                                },
                                                _ => {},
                                            }
                                        }
                                        let msg = SipMessage::Request({
                                            let mut req = txn.invite_request(to.clone());
                                            txn.add_auth_to_request(&mut req, opaque, nonce);
                                            req
                                        });
                                        txn.tx_ch.send(((*SERVER_ADDR).clone(), msg)).await?;
                                        let msg = txn.rx_ch.recv().await.ok_or("closed rx channel")?;
                                        match msg {
                                            SipMessage::Request(_) => Err("expected 200 response to authed invite, got request")?,
                                            SipMessage::Response(ref r) => {
                                                (r.status_code == rsip::StatusCode::Trying).then_some(()).ok_or("response status not Trying")?;
                                                let cseq = msg.cseq_header()?.seq()?;
                                                let local = msg.from_header()?;
                                                break State::Connected(Dial::DialOut(cseq, local.clone()));
                                            },
                                        }
                                    },
                                };
                            },
                            dig = dig_ch.recv() => match dig {
                                Some(dig) => {
                                    debug!("GOT DIG: {}", dig);
                                    number.push((dig + b'0').into());
                                },
                                None => break State::Connected(Dial::Error("dig channel died :(".into())),
                            },
                            hook_evt = hook_ch.recv() => match hook_evt {
                                Ok(SwitchHook::ON) => break State::Connected(Dial::OnHook),
                                Ok(SwitchHook::OFF) => {},
                                Err(e) => break State::Connected(Dial::Error(Box::new(e))),
                            },
                        }
                    };

                    tone_handle.abort();
                }
                State::Connected(Dial::DialOut(cseq, local)) => {
                    debug!("dialed out");
                    let mut hook_ch = self.hook_ch.subscribe();
                    let txn = self.sip_txn.as_mut().ok_or("how is txn missing here")?;

                    self.state = loop {
                        select! {
                            _ = sleep(Duration::from_secs(5)) => {
                                let to = self.to_uri.clone().ok_or("missing to uri in dial out")?;
                                let req = txn.cancel_request(to);
                                txn.tx_ch.send(((*SERVER_ADDR).clone(), rsip::SipMessage::Request(req))).await?;
                                txn.rx_ch.recv().await;
                                break State::Connected(Dial::Busy)
                            },
                            msg = txn.rx_ch.recv() => match msg.ok_or("closed rx channel in dialout")? {
                                SipMessage::Request(_) => Err("unexpected request during dial out")?,
                                SipMessage::Response(resp) => match resp.status_code {
                                    // TOOD(peter): Handle a busy
                                    rsip::StatusCode::Decline => {
                                        let req = txn.ack_request(resp);
                                        txn.tx_ch.send(((*SERVER_ADDR).clone(), rsip::SipMessage::Request(req))).await?;
                                        break State::Connected(Dial::Busy)
                                    },
                                    rsip::StatusCode::Ringing => {
                                        let remote = resp.to_header()?;
                                        break State::Connected(Dial::Dialing(*cseq, local.clone(), remote.clone()))
                                    },
                                    _ => {},
                                },
                            },
                            hook_evt = hook_ch.recv() => match hook_evt {
                                Ok(SwitchHook::ON) => break State::Connected(Dial::OnHook),
                                Ok(SwitchHook::OFF) => break State::Connected(Dial::Error("got off hook during dial out".into())),
                                Err(e) => break State::Connected(Dial::Error(Box::new(e))),
                            },
                        }
                    }
                },
                State::Connected(Dial::Dialing(cseq, local, remote)) => {
                    debug!("dialing");
                    let audio_out_ch = self.audio_out_ch.clone();
                    let mut hook_ch = self.hook_ch.subscribe();
                    let txn = self.sip_txn.as_mut().ok_or("how is txn missing here")?;

                    let tone_handle = TwoToneGen::ring(self.audio_out_sample_rate)
                        .send_to(audio_out_ch, self.audio_out_n_channels);

                    self.state = loop {
                        select! {
                            msg = txn.rx_ch.recv() => match msg.ok_or("closed rx channel in dialing")? {
                                SipMessage::Request(_) => Err("unexpected request during dial out")?,
                                SipMessage::Response(resp) => match resp.status_code {
                                    rsip::StatusCode::OK => {
                                        let req = txn.ack_request(resp.clone());
                                        txn.tx_ch.send(((*SERVER_ADDR).clone(), rsip::SipMessage::Request(req))).await?;
                                        let peer = resp.contact_header()?.uri()?;
                                        break State::Connected(Dial::Connected(*cseq, peer, local.clone(), remote.clone()));
                                    },
                                    _ => {},
                                },
                            },
                            hook_evt = hook_ch.recv() => match hook_evt {
                                Ok(SwitchHook::ON) => break State::Connected(Dial::OnHook),
                                Ok(SwitchHook::OFF) => {},
                                Err(e) => break State::Connected(Dial::Error(Box::new(e))),
                            },
                        }
                    };

                    tone_handle.abort();
                },
                State::Connected(Dial::Connected(cseq, peer, local, remote)) => {
                    debug!("connected yay");
                    // TODO(peter): Wire up RTP
                    let mut hook_ch = self.hook_ch.subscribe();
                    let txn = self.sip_txn.as_mut().ok_or("how is txn missing here")?;

                    self.state = loop {
                        select! {
                            msg = txn.rx_ch.recv() => match msg.ok_or("closed rx channel in connected")? {
                                SipMessage::Request(req) => match req.method {
                                    rsip::Method::Invite => {
                                        let sdp = txn.sdp_from(req.clone())?;
                                        let (addr, resp) = txn.sdp_response_to(req, rsip::StatusCode::OK, sdp)?;
                                        txn.tx_ch.send((addr, resp)).await?;

                                        match txn.rx_ch.recv().await.ok_or("closed rx channel in connected")? {
                                            SipMessage::Request(req) => match req.method() {
                                                rsip::Method::Ack => break State::Connected(Dial::Connected(*cseq, peer.clone(), local.clone(), remote.clone())),
                                                _ => Err(format!("got non-ack request during connected: {}", req))?,
                                            }
                                            _ => Err("got unexpected response during connected")?,
                                        }
                                    },
                                    rsip::Method::Bye => {
                                        let (addr, resp) = txn.response_to(req.clone(), rsip::StatusCode::OK, vec![])?;
                                        txn.tx_ch.send((addr, resp)).await?;
                                        break State::Connected(Dial::Error("other party hung up".into()));
                                    },
                                    _ => Err(format!("got unexpected request method during connected"))?,
                                },
                                SipMessage::Response(_) => Err("unexpected request during connected")?,
                            },
                            hook_evt = hook_ch.recv() => match hook_evt {
                                Ok(SwitchHook::ON) => {
                                    let msg = SipMessage::Request(txn.bye_request(peer.clone(), local.clone(), remote.clone()));
                                    txn.tx_ch.send(((*SERVER_ADDR).clone(), msg)).await?;
                                    match txn.rx_ch.recv().await.ok_or("closed rx channel in ringing")? {
                                        SipMessage::Request(_) => Err("got unexpected request after bye")?,
                                        SipMessage::Response(resp) => {
                                            (resp.status_code == rsip::StatusCode::OK).then_some(()).ok_or("bye response status not 200")?;
                                            break State::Connected(Dial::OnHook);
                                        },
                                    }
                                },
                                Ok(SwitchHook::OFF) => {},
                                Err(e) => break State::Connected(Dial::Error(Box::new(e))),
                            },
                        }
                    };
                },
                State::Connected(Dial::Busy) => {
                    debug!("busy");
                    let audio_out_ch = self.audio_out_ch.clone();
                    let mut hook_ch = self.hook_ch.subscribe();
                    let tone_handle = TwoToneGen::busy(self.audio_out_sample_rate)
                        .send_to(audio_out_ch, self.audio_out_n_channels);

                    self.state = loop {
                        match hook_ch.recv().await {
                            Ok(SwitchHook::ON) => break State::Connected(Dial::OnHook),
                            Ok(SwitchHook::OFF) => {},
                            Err(e) => break State::Connected(Dial::Error(Box::new(e))),
                        }
                    };

                    tone_handle.abort();
                },
                State::Connected(Dial::Error(e)) => {
                    error!(e);
                    self.sip_txn = None;
                    let mut hook_ch = self.hook_ch.subscribe();

                    self.state = loop {
                        match hook_ch.recv().await {
                            Ok(SwitchHook::ON) => break State::Connected(Dial::OnHook),
                            Ok(SwitchHook::OFF) => {},
                            Err(e) => break State::Disconnected(WiFi::Error(Box::new(e))),
                        }
                    }
                }

                State::Disconnected(WiFi::OnHook) => {
                    debug!("phone on hook ft. no wifi");
                    let mut hook_ch = self.hook_ch.subscribe();

                    let new_state = select! {
                        shk_evt = hook_ch.recv() => {
                            match shk_evt {
                                Ok(SwitchHook::ON) => None,
                                Ok(SwitchHook::OFF) => Some(State::Disconnected(WiFi::Await)),
                                Err(e) => Some(State::Disconnected(WiFi::Error(Box::new(e)))),
                            }
                        }
                        has_internet = do_i_have_internet() => {
                            match has_internet {
                                Ok(true) => match sip::register(self.sip_send_ch.clone()).await {
                                    Ok(_) => Some(State::Connected(Dial::OnHook)),
                                    Err(e) => Some(State::Disconnected(WiFi::Error(e))),
                                },
                                Ok(false) => None,
                                Err(e) => Some(State::Disconnected(WiFi::Error(e))),
                            }
                        }
                    };
                    self.state = if let Some(state) = new_state { state } else {
                        loop {
                            match hook_ch.recv().await {
                                Ok(SwitchHook::ON) => {},
                                Ok(SwitchHook::OFF) => break State::Disconnected(WiFi::Await),
                                Err(e) => break State::Disconnected(WiFi::Error(Box::new(e))),
                            }
                        }
                    }
                }
                State::Disconnected(WiFi::Await) => {
                    debug!("phone picked up ft. no wifi");
                    let audio_out_ch = self.audio_out_ch.clone();
                    let mut hook_ch = self.hook_ch.subscribe();
                    let tone_handle = TwoToneGen::no_wifi(self.audio_out_sample_rate)
                        .send_to(audio_out_ch, self.audio_out_n_channels);

                    self.state = select! {
                        wifi_evt = self.get_wifi_creds() => match wifi_evt {
                            Ok(_) => match sip::register(self.sip_send_ch.clone()).await {
                                Ok(_) => State::Connected(Dial::Await),
                                Err(e) => State::Disconnected(WiFi::Error(e)),
                            },
                            Err(e) => State::Disconnected(WiFi::Error(e)),
                        },
                        shk_evt = hook_ch.recv() => match shk_evt {
                            Ok(SwitchHook::ON) => State::Disconnected(WiFi::OnHook),
                            Ok(SwitchHook::OFF) => State::Disconnected(WiFi::Await),
                            Err(e) => State::Disconnected(WiFi::Error(Box::new(e))),
                        },
                    };

                    tone_handle.abort();
                }
                State::Disconnected(WiFi::Error(e)) => {
                    error!(e);
                    let mut hook_ch = self.hook_ch.subscribe();

                    self.state = loop {
                        match hook_ch.recv().await {
                            Ok(SwitchHook::ON) => break State::Disconnected(WiFi::OnHook),
                            Ok(SwitchHook::OFF) => {},
                            Err(e) => break State::Disconnected(WiFi::Error(Box::new(e))),
                        }
                    }
                }
            }
        }
    }
}
