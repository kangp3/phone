use std::error::Error;
use std::net::SocketAddr;

use rsip::SipMessage;
use tokio::process::Command;
use tokio::select;
use tokio::sync::{broadcast, mpsc};
use tracing::{debug, error, info};

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
    OnHook,     // On hook, standby
    Ringing,    // Receiving call
    Await,      // Awaiting user input for dialing (playing dial tone)
    DialOut,    // Dial request sent, awaiting ACK
    Dialing,    // Dialing (playing ringing tone)
    Connected,  // Voice connected
    Busy,       // Line is busy (playing busy tone)
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

    sip_send_ch: Option<mpsc::Sender<(SocketAddr, SipMessage)>>,
    sip_recv_ch: Option<broadcast::Sender<(SocketAddr, SipMessage)>>,
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

        let (sip_send_ch, sip_recv_ch) = if !has_internet { (None, None) } else {
            let (sip_send_ch, sip_recv_ch) = sip::socket::bind().await?;
            let sip_ch = sip_recv_ch.subscribe();
            sip::register(sip_send_ch.clone(), sip_ch).await?;
            (Some(sip_send_ch), Some(sip_recv_ch))
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
            sip_recv_ch,
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
                    let mut hook_ch = self.hook_ch.subscribe();

                    self.state = loop {
                        match hook_ch.recv().await {
                            Ok(SwitchHook::ON) => {},
                            Ok(SwitchHook::OFF) => break State::Connected(Dial::Await),
                            Err(e) => break State::Connected(Dial::Error(Box::new(e))),
                        }
                    };
                }
                State::Connected(Dial::Ringing) => {
                    debug!("ringing");
                    let mut hook_ch = self.hook_ch.subscribe();
                    let ring_handle = ring::ring_phone()?;

                    self.state = loop {
                        match hook_ch.recv().await {
                            Ok(SwitchHook::ON) => {},
                            Ok(SwitchHook::OFF) => break State::Connected(Dial::Connected),
                            Err(e) => break State::Connected(Dial::Error(Box::new(e))),
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

                    self.state = loop {
                        select! {
                            dig = dig_ch.recv() => match dig {
                                Some(dig) => debug!("GOT DIG: {}", dig),
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
                State::Connected(Dial::DialOut) => {
                    debug!("dialed out");
                },
                State::Connected(Dial::Dialing) => {
                    debug!("dialing");
                    let audio_out_ch = self.audio_out_ch.clone();
                    let mut hook_ch = self.hook_ch.subscribe();
                    let tone_handle = TwoToneGen::ring(self.audio_out_sample_rate)
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
                State::Connected(Dial::Connected) => todo!(),
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
                                Ok(true) => Some(State::Connected(Dial::OnHook)),
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
                            Ok(_) => State::Connected(Dial::Await),
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
