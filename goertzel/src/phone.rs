use std::error::Error;

use tokio::select;
use tokio::sync::{broadcast, mpsc};
use tracing::{debug, error};

use crate::{audio, ring};
use crate::hook::{self, SwitchHook};
use crate::nettest::do_i_have_internet;
use crate::tone::TwoToneGen;
use crate::{dtmf, pulse};

pub enum State {
    Connected(Dial),
    Disconnected(WiFi),
    Error(Box<dyn Error>),
}

pub enum WiFi {
    OnHook,  // On hook, standby
    Await,   // Awaiting user input for SSID and pass
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
}

pub struct Phone {
    pub state: State,

    #[cfg(target_os = "macos")]
    _shk_pin: (),
    #[cfg(target_os = "linux")]
    _shk_pin: rppal::gpio::InputPin,
    _shk_ch: broadcast::Sender<SwitchHook>,

    pub audio_in_ch: broadcast::Sender<f32>,
    _audio_in_stream: cpal::Stream,
    _audio_in_cfg: cpal::SupportedStreamConfig,

    pub audio_out_ch: mpsc::Sender<f32>,
    _audio_out_stream: cpal::Stream,
    audio_out_n_channels: u16,
    audio_out_sample_rate: u32,

    pub hook_ch: broadcast::Sender<SwitchHook>,
    pub pulse_ch: broadcast::Sender<u8>,
    pub goertz_ch: broadcast::Sender<u8>,
}

impl Phone {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let (mic_ch, mic_stream, mic_cfg) = audio::get_input_channel()?;
        let (spk_ch, spk_stream, spk_cfg) = audio::get_output_channel()?;

        let (_shk_pin, shk_sender, shk_ch) = hook::try_register_shk()?;
        let (pulse_ch, _, hook_ch, _) = pulse::notgoertzelme(shk_ch);
        let goertz_ch = dtmf::goertzelme(mic_ch.subscribe());

        Ok(Self {
            state: State::Disconnected(WiFi::OnHook),

            _shk_pin,
            _shk_ch: shk_sender,

            audio_in_ch: mic_ch,
            _audio_in_stream: mic_stream,
            _audio_in_cfg: mic_cfg,

            audio_out_ch: spk_ch,
            _audio_out_stream: spk_stream,
            audio_out_sample_rate: spk_cfg.sample_rate().0,
            audio_out_n_channels: spk_cfg.channels(),

            hook_ch,
            pulse_ch,
            goertz_ch,
        })
    }

    pub async fn begin_life(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let mut hook_ch = self.hook_ch.subscribe();
            let audio_out_ch = self.audio_out_ch.clone();

            match &self.state {
                State::Connected(Dial::OnHook) => {
                    debug!("phone on hook");
                    let ring_handle = ring::ring_phone()?;

                    self.state = loop {
                        match hook_ch.recv().await {
                            Ok(SwitchHook::ON) => {},
                            Ok(SwitchHook::OFF) => break State::Connected(Dial::Await),
                            Err(e) => break State::Error(Box::new(e)),
                        }
                    };

                    ring_handle.abort();
                }
                State::Connected(Dial::Ringing) => todo!(),
                State::Connected(Dial::Await) => {
                    debug!("phone picked up");
                    let tone_handle = TwoToneGen::off_hook(self.audio_out_sample_rate)
                        .send_to(audio_out_ch, self.audio_out_n_channels);

                    while let SwitchHook::OFF = hook_ch.recv().await? {}

                    tone_handle.abort();
                    self.state = State::Connected(Dial::OnHook);
                }
                State::Connected(Dial::DialOut) => todo!(),
                State::Connected(Dial::Dialing) => todo!(),
                State::Connected(Dial::Connected) => todo!(),
                State::Connected(Dial::Busy) => todo!(),

                State::Disconnected(WiFi::OnHook) => {
                    debug!("phone on hook ft. no wifi");

                    let new_state = select! {
                        shk_evt = hook_ch.recv() => {
                            match shk_evt {
                                Ok(SwitchHook::ON) => None,
                                Ok(SwitchHook::OFF) => Some(State::Disconnected(WiFi::Await)),
                                Err(e) => Some(State::Error(Box::new(e))),
                            }
                        }
                        has_internet = do_i_have_internet() => {
                            match has_internet {
                                Ok(true) => Some(State::Connected(Dial::OnHook)),
                                Ok(false) => None,
                                Err(e) => Some(State::Error(e)),
                            }
                        }
                    };
                    self.state = if let Some(state) = new_state { state } else {
                        loop {
                            match hook_ch.recv().await {
                                Ok(SwitchHook::ON) => {},
                                Ok(SwitchHook::OFF) => break State::Disconnected(WiFi::Await),
                                Err(e) => break State::Error(Box::new(e)),
                            }
                        }
                    }
                }
                State::Disconnected(WiFi::Await) => {
                    debug!("phone picked up ft. no wifi");
                    let tone_handle = TwoToneGen::no_wifi(self.audio_out_sample_rate)
                        .send_to(audio_out_ch, self.audio_out_n_channels);

                    while let SwitchHook::OFF = hook_ch.recv().await? {}

                    tone_handle.abort();
                    self.state = State::Disconnected(WiFi::OnHook);
                }

                State::Error(e) => {
                    error!(e);
                    loop {}
                }
            }
        }
    }
}
