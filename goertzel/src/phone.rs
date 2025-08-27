use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use rsip::SipMessage;
use sdp_rs::lines::media::MediaType;
use sdp_rs::SessionDescription;
use tokio::process::Command;
use tokio::select;
use tokio::sync::{broadcast, mpsc};
use tokio::time::sleep;
use tracing::{debug, error, info};

use crate::contacts::CONTACTS;
use crate::hook::{self, SwitchHook};
use crate::nettest::do_i_have_internet;
use crate::sip::tlssocket::TlsSipConn;
use crate::tone::TwoToneGen;
use crate::{audio, deco, ring, rtp, sip};
use crate::{dtmf, pulse};
use anyhow::{anyhow, Result};

pub enum State {
    Connected(TlsSipConn, Dial),
    Disconnected(WiFi),
}

pub enum WiFi {
    OnHook, // On hook, standby
    Await,  // Awaiting user input for SSID and pass
    Error(anyhow::Error),
}

// TODO(peter): SIP registration steps
pub enum Dial {
    OnHook,
    Ringing(sip::Dialog, rtp::socket::Socket, SipMessage),
    Await,
    DialOut(sip::Dialog, rtp::socket::Socket),
    Dialing(sip::Dialog, rtp::socket::Socket),
    Connected(sip::Dialog, rtp::socket::Socket),
    Busy,
    Error(anyhow::Error),
}

pub struct Phone {
    pub state: State,

    #[cfg(target_os = "macos")]
    _shk_pin: (),
    #[cfg(target_os = "linux")]
    _shk_pin: rppal::gpio::InputPin,

    pub audio_in_ch: broadcast::Sender<i16>,
    _audio_in_stream: cpal::Stream,
    _audio_in_cfg: cpal::SupportedStreamConfig,

    pub audio_out_ch: mpsc::Sender<i16>,
    _audio_out_stream: cpal::Stream,
    audio_out_sample_rate: u32,

    pub hook_ch: broadcast::Sender<SwitchHook>,
    pub pulse_ch: broadcast::Sender<u8>,

    username: String,
    password: String,
}

impl Phone {
    pub async fn new(username: String, password: String) -> Result<Self> {
        let (mic_ch, mic_stream, mic_cfg) = audio::get_input_channel()?;
        let (spk_ch, spk_stream, spk_cfg) = audio::get_output_channel()?;

        let (shk_pin, _, shk_ch) = hook::try_register_shk()?;
        let (pulse_ch, _, hook_ch, _) = pulse::notgoertzelme(shk_ch);

        let tls_conn = if do_i_have_internet().await? {
            debug!("Registering to SIP server");
            let ip = public_ip::addr_v4().await.ok_or(anyhow!("no ip"))?;
            let username = username.clone();
            let password = password.clone();
            let tls_conn =
                sip::tlssocket::TlsSipConn::new(ip, sip::SERVER_NAME, sip::SERVER_PORT).await?;
            tls_conn.dialog(username).await.register(password).await?;
            Some(tls_conn)
        } else {
            None
        };

        #[cfg(target_os = "macos")]
        let is_on_hook = true;
        #[cfg(target_os = "linux")]
        let is_on_hook = shk_pin.is_low();

        let state = match (tls_conn, is_on_hook) {
            (Some(tls_conn), true) => State::Connected(tls_conn, Dial::OnHook),
            (Some(tls_conn), false) => State::Connected(tls_conn, Dial::Await),
            (None, true) => State::Disconnected(WiFi::OnHook),
            (None, false) => State::Disconnected(WiFi::Await),
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

            hook_ch,
            pulse_ch,

            username,
            password,
        })
    }

    async fn get_wifi_creds(&self) -> Result<()> {
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
            Err(anyhow!("no Wi-Fi 4 me :("))
        } else {
            Ok(())
        }
    }

    pub async fn begin_life(mut self) -> Result<()> {
        loop {
            self.state = match self.state {
                State::Connected(mut tls_conn, Dial::OnHook) => {
                    debug!("phone on hook");
                    let mut hook_ch = self.hook_ch.subscribe();

                    loop {
                        select! {
                            recvd = tls_conn.new_msg_ch.recv() => {
                                let invite = recvd.ok_or(anyhow!("new msg chan closed"))?;
                                let dialog = tls_conn.dialog_from_req(&invite).await?;
                                let rtp_sock = rtp::socket::Socket::bind().await?;
                                break State::Connected(tls_conn, Dial::Ringing(dialog, rtp_sock, invite));
                            },
                            hook_evt = hook_ch.recv() => match hook_evt {
                                Ok(SwitchHook::ON) => {},
                                Ok(SwitchHook::OFF) => break State::Connected(tls_conn, Dial::Await),
                                Err(e) => break State::Connected(tls_conn, Dial::Error(e.into())),
                            },
                        }
                    }
                }
                State::Connected(tls_conn, Dial::Ringing(mut dialog, rtp_sock, invite)) => {
                    debug!("ringing");
                    let mut hook_ch = self.hook_ch.subscribe();
                    let _ring = ring::ring_phone()?;

                    let resp = dialog.response_to(
                        invite.clone().try_into()?,
                        rsip::StatusCode::Ringing,
                        vec![],
                    )?;
                    dialog.send(resp).await?;

                    loop {
                        select! {
                            msg = dialog.recv() => match msg? {
                                SipMessage::Request(req) => match req.method() {
                                    rsip::Method::Cancel => {
                                        let resp = dialog.response_to(req, rsip::StatusCode::RequestTerminated, vec![])?;
                                        dialog.send(resp).await?;
                                        break State::Connected(tls_conn, Dial::OnHook);
                                    },
                                    _ => Err(anyhow!("got non-cancel request during ringing: {}", req))?,
                                },
                                SipMessage::Response(_) => Err(anyhow!("got unexpected response during ringing"))?,
                            },
                            hook_evt = hook_ch.recv() => match hook_evt {
                                Ok(SwitchHook::ON) => {},
                                Ok(SwitchHook::OFF) => {
                                    // TODO(peter): Include appropriate SDP params in OK
                                    let sdp = dialog.sdp_from(invite.clone().try_into()?)?;
                                    let ref resp = dialog.sdp_response_to(invite.clone().try_into()?, rsip::StatusCode::OK, sdp)?;
                                    dialog.send(resp.clone()).await?;
                                    match dialog.recv().await? {
                                        SipMessage::Request(req) => match req.method() {
                                            rsip::Method::Ack => break State::Connected(tls_conn, Dial::Connected(dialog, rtp_sock)),
                                            _ => Err(anyhow!("got non-ack request during ringing: {}", req))?,
                                        }
                                        _ => Err(anyhow!("got unexpected response during ringing"))?,
                                    }
                                },
                                Err(e) => break State::Connected(tls_conn, Dial::Error(e.into())),
                            },
                        }
                    }
                }
                State::Connected(tls_conn, Dial::Await) => {
                    debug!("phone picked up");
                    let audio_out_ch = self.audio_out_ch.clone();
                    let pulse_ch = self.pulse_ch.subscribe();
                    let goertzel_ch = dtmf::goertzelme(self.audio_in_ch.subscribe());
                    let mut hook_ch = self.hook_ch.subscribe();

                    let mut tone = TwoToneGen::off_hook(self.audio_out_sample_rate);
                    tone.play(audio_out_ch);

                    let mut dig_ch = deco::de_digs(goertzel_ch, pulse_ch);
                    let mut dialog = tls_conn.dialog(self.username.clone()).await;

                    let mut number = String::new();
                    loop {
                        select! {
                            _ = sleep(Duration::from_secs(1)), if (*CONTACTS).contains_key(&number) && number != self.username => {
                                let to = (*CONTACTS).get(&number).ok_or(anyhow!("contact is missing after I EXPLICITLY checked it"))?;
                                dialog.invite(self.password.clone(), to.clone()).await?;
                                let rtp_sock = rtp::socket::Socket::bind().await?;
                                break State::Connected(tls_conn, Dial::DialOut(dialog, rtp_sock));
                            },
                            dig = dig_ch.recv() => match dig {
                                Some(dig) => {
                                    debug!("GOT DIG: {}", dig);
                                    number.push((dig + b'0').into());
                                },
                                None => break State::Connected(tls_conn, Dial::Error(anyhow!("dig channel died :("))),
                            },
                            hook_evt = hook_ch.recv() => match hook_evt {
                                Ok(SwitchHook::ON) => break State::Connected(tls_conn, Dial::OnHook),
                                Ok(SwitchHook::OFF) => {},
                                Err(e) => break State::Connected(tls_conn, Dial::Error(e.into())),
                            },
                        }
                    }
                }
                State::Connected(mut tls_conn, Dial::DialOut(mut dialog, rtp_sock)) => {
                    debug!("dialed out");
                    let mut hook_ch = self.hook_ch.subscribe();

                    loop {
                        select! {
                            _ = sleep(Duration::from_secs(5)) => {
                                dialog.cancel().await?;
                                // TODO(peter): Assert what this should be
                                dialog.recv().await?;
                                break State::Connected(tls_conn, Dial::Busy)
                            },
                            msg = tls_conn.new_msg_ch.recv() => match msg {
                                Some(SipMessage::Request(_)) => Err(anyhow!("unexpected request during dial out"))?,
                                Some(SipMessage::Response(resp)) => match resp.status_code {
                                    rsip::StatusCode::BusyHere |
                                    rsip::StatusCode::Decline => {
                                        dialog.ack(resp).await?;
                                        break State::Connected(tls_conn, Dial::Busy)
                                    },
                                    rsip::StatusCode::Ringing => {
                                        break State::Connected(tls_conn, Dial::Dialing(dialog, rtp_sock))
                                    },
                                    _ => {},
                                },
                                None => Err(anyhow!("tls conn closed"))?,
                            },
                            hook_evt = hook_ch.recv() => match hook_evt {
                                Ok(SwitchHook::ON) => break State::Connected(tls_conn, Dial::OnHook),
                                Ok(SwitchHook::OFF) => break State::Connected(tls_conn, Dial::Error(anyhow!("got off hook during dial out"))),
                                Err(e) => break State::Connected(tls_conn, Dial::Error(e.into())),
                            },
                        }
                    }
                }
                State::Connected(mut tls_conn, Dial::Dialing(mut dialog, rtp_sock)) => {
                    debug!("dialing");
                    let audio_out_ch = self.audio_out_ch.clone();
                    let mut hook_ch = self.hook_ch.subscribe();

                    let mut tone = TwoToneGen::ring(self.audio_out_sample_rate);
                    tone.play(audio_out_ch);

                    loop {
                        select! {
                            msg = tls_conn.new_msg_ch.recv() => match msg {
                                Some(SipMessage::Request(_)) => Err(anyhow!("unexpected request during dial out"))?,
                                Some(SipMessage::Response(resp)) => match resp.status_code {
                                    rsip::StatusCode::OK => {
                                        dialog.ack(resp.clone()).await?;
                                        break State::Connected(tls_conn, Dial::Connected(dialog, rtp_sock));
                                    },
                                    _ => {},
                                },
                                None => Err(anyhow!("tls conn closed"))?,
                            },
                            hook_evt = hook_ch.recv() => match hook_evt {
                                Ok(SwitchHook::ON) => break State::Connected(tls_conn, Dial::OnHook),
                                Ok(SwitchHook::OFF) => {},
                                Err(e) => break State::Connected(tls_conn, Dial::Error(e.into())),
                            },
                        }
                    }
                }
                State::Connected(mut tls_conn, Dial::Connected(mut dialog, mut rtp_sock)) => {
                    debug!("connected yay");
                    let mut hook_ch = self.hook_ch.subscribe();

                    loop {
                        select! {
                            msg = tls_conn.new_msg_ch.recv() => match msg {
                                Some(SipMessage::Request(req)) => match req.method {
                                    rsip::Method::Invite => {
                                        let req_sdp = SessionDescription::from_str(std::str::from_utf8(&req.body)?)?;
                                        let sdp_ip = {
                                            let connection = req_sdp.connection.clone().ok_or(anyhow!("connection line doesn't exist"))?;
                                            connection.connection_address.base
                                        };
                                        let mut sdp_port = None;
                                        for desc in &req_sdp.media_descriptions {
                                            if desc.media.media == MediaType::Audio {
                                                sdp_port = Some(desc.media.port);
                                            }
                                        }
                                        let sdp_port = sdp_port.ok_or(anyhow!("missing audio SDP port"))?;
                                        let sdp_addr = SocketAddr::new(sdp_ip, sdp_port);

                                        let audio_in_ch = self.audio_in_ch.subscribe();
                                        let audio_out_ch = self.audio_out_ch.clone();
                                        rtp_sock.connect(sdp_addr, audio_in_ch, audio_out_ch).await?;

                                        let sdp = dialog.sdp_from(req.clone())?;
                                        let resp = dialog.sdp_response_to(req.clone(), rsip::StatusCode::OK, sdp)?;
                                        dialog.send(resp.clone()).await?;

                                        match dialog.recv().await? {
                                            SipMessage::Request(req) => match req.method() {
                                                rsip::Method::Ack => continue,
                                                _ => Err(anyhow!("got non-ack request during connected: {}", req))?,
                                            }
                                            _ => Err(anyhow!("got unexpected response during connected"))?,
                                        }
                                    },
                                    _ => Err(anyhow!("got unexpected request method during connected"))?,
                                },
                                Some(SipMessage::Response(_)) => Err(anyhow!("unexpected response during connected"))?,
                                None => Err(anyhow!("tls conn closed"))?,
                            },
                            msg = dialog.recv() => match msg {
                                Ok(SipMessage::Request(req)) => match req.method {
                                    rsip::Method::Invite => {
                                        let req_sdp = SessionDescription::from_str(std::str::from_utf8(&req.body)?)?;
                                        let sdp_ip = {
                                            let connection = req_sdp.connection.clone().ok_or(anyhow!("connection line doesn't exist"))?;
                                            connection.connection_address.base
                                        };
                                        let mut sdp_port = None;
                                        for desc in &req_sdp.media_descriptions {
                                            if desc.media.media == MediaType::Audio {
                                                sdp_port = Some(desc.media.port);
                                            }
                                        }
                                        let sdp_port = sdp_port.ok_or(anyhow!("missing audio SDP port"))?;
                                        let sdp_addr = SocketAddr::new(sdp_ip, sdp_port);

                                        let audio_in_ch = self.audio_in_ch.subscribe();
                                        let audio_out_ch = self.audio_out_ch.clone();
                                        rtp_sock.connect(sdp_addr, audio_in_ch, audio_out_ch).await?;

                                        let sdp = dialog.sdp_from(req.clone())?;
                                        let resp = dialog.sdp_response_to(req.clone(), rsip::StatusCode::OK, sdp)?;
                                        dialog.send(resp.clone()).await?;

                                        match dialog.recv().await? {
                                            SipMessage::Request(req) => match req.method() {
                                                rsip::Method::Ack => continue,
                                                _ => Err(anyhow!("got non-ack request during connected: {}", req))?,
                                            }
                                            _ => Err(anyhow!("got unexpected response during connected"))?,
                                        }
                                    },
                                    rsip::Method::Bye => {
                                        let resp = dialog.response_to(req.clone(), rsip::StatusCode::OK, vec![])?;
                                        dialog.send(resp).await?;
                                        break State::Connected(tls_conn, Dial::Error(anyhow!("other party hung up")));
                                    },
                                    _ => Err(anyhow!("got unexpected request method during connected"))?,
                                },
                                Ok(SipMessage::Response(_)) => Err(anyhow!("unexpected response during connected"))?,
                                Err(e) => Err(e)?,
                            },
                            hook_evt = hook_ch.recv() => match hook_evt {
                                Ok(SwitchHook::ON) => {
                                    dialog.bye().await?;
                                    sip::assert_status(&dialog.recv().await?.try_into()?)?;
                                    break State::Connected(tls_conn, Dial::OnHook);
                                },
                                Ok(SwitchHook::OFF) => {},
                                Err(e) => break State::Connected(tls_conn, Dial::Error(e.into())),
                            },
                        }
                    }
                }
                State::Connected(tls_conn, Dial::Busy) => {
                    debug!("busy");
                    let audio_out_ch = self.audio_out_ch.clone();
                    let mut hook_ch = self.hook_ch.subscribe();

                    let mut tone = TwoToneGen::busy(self.audio_out_sample_rate);
                    tone.play(audio_out_ch);

                    loop {
                        match hook_ch.recv().await {
                            Ok(SwitchHook::ON) => break State::Connected(tls_conn, Dial::OnHook),
                            Ok(SwitchHook::OFF) => {}
                            Err(e) => break State::Connected(tls_conn, Dial::Error(e.into())),
                        }
                    }
                }
                State::Connected(tls_conn, Dial::Error(e)) => {
                    error!("{:?}", e);
                    let mut hook_ch = self.hook_ch.subscribe();

                    loop {
                        match hook_ch.recv().await {
                            Ok(SwitchHook::ON) => break State::Connected(tls_conn, Dial::OnHook),
                            Ok(SwitchHook::OFF) => {}
                            Err(e) => break State::Disconnected(WiFi::Error(e.into())),
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
                                Err(e) => Some(State::Disconnected(WiFi::Error(e.into()))),
                            }
                        }
                        has_internet = do_i_have_internet() => {
                            match has_internet {
                                Ok(true) => {
                                    let ip = public_ip::addr_v4().await.ok_or(anyhow!("no ip"))?;
                                    let tls_conn =
                                        sip::tlssocket::TlsSipConn::new(ip, sip::SERVER_NAME, sip::SERVER_PORT).await?;
                                    tls_conn.dialog(self.username.clone()).await.register(self.password.clone()).await?;
                                    Some(State::Connected(tls_conn, Dial::OnHook))
                                },
                                Ok(false) => None,
                                Err(e) => Some(State::Disconnected(WiFi::Error(e))),
                            }
                        }
                    };
                    if let Some(state) = new_state {
                        state
                    } else {
                        loop {
                            match hook_ch.recv().await {
                                Ok(SwitchHook::ON) => {}
                                Ok(SwitchHook::OFF) => break State::Disconnected(WiFi::Await),
                                Err(e) => break State::Disconnected(WiFi::Error(e.into())),
                            }
                        }
                    }
                }
                State::Disconnected(WiFi::Await) => {
                    debug!("phone picked up ft. no wifi");
                    let audio_out_ch = self.audio_out_ch.clone();
                    let mut hook_ch = self.hook_ch.subscribe();

                    let mut tone = TwoToneGen::no_wifi(self.audio_out_sample_rate);
                    tone.play(audio_out_ch);

                    select! {
                        wifi_evt = self.get_wifi_creds() => match wifi_evt {
                            Ok(_) => {
                                let ip = public_ip::addr_v4().await.ok_or(anyhow!("no ip"))?;
                                let tls_conn =
                                    sip::tlssocket::TlsSipConn::new(ip, sip::SERVER_NAME, sip::SERVER_PORT).await?;
                                tls_conn.dialog(self.username.clone()).await.register(self.password.clone()).await?;
                                State::Connected(tls_conn, Dial::Await)
                            },
                            Err(e) => State::Disconnected(WiFi::Error(e)),
                        },
                        shk_evt = hook_ch.recv() => match shk_evt {
                            Ok(SwitchHook::ON) => State::Disconnected(WiFi::OnHook),
                            Ok(SwitchHook::OFF) => State::Disconnected(WiFi::Await),
                            Err(e) => State::Disconnected(WiFi::Error(e.into())),
                        },
                    }
                }
                State::Disconnected(WiFi::Error(e)) => {
                    error!("{:?}", e);
                    let mut hook_ch = self.hook_ch.subscribe();

                    loop {
                        match hook_ch.recv().await {
                            Ok(SwitchHook::ON) => break State::Disconnected(WiFi::OnHook),
                            Ok(SwitchHook::OFF) => {}
                            Err(e) => break State::Disconnected(WiFi::Error(e.into())),
                        }
                    }
                }
            }
        }
    }
}
