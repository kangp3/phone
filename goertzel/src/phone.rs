use std::error::Error;
use std::time::Duration;

use tokio::sync::{broadcast, mpsc};
use tracing::debug;

use crate::audio;
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
    Error,   // Error occurred
}

// TODO(peter): SIP registration steps
pub enum Dial {
    OnHook,     // On hook, standby
    Ringing,    // Receiving call
    Await,      // Awaiting user input for dialing (playing dial tone)
    DialOut,    // Dial request sent, awaiting ACK
    Dialing,    // Dialing (playing ringing tone)
    Connected,  // Voice connected
    BusyErr,    // Busy/error occurred (playing busy tone)
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
    pub goertz_ch: broadcast::Sender<u8>,
}

impl Phone {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let state = if do_i_have_internet().await? {
            State::Connected(Dial::OnHook)
        } else {
            State::Disconnected(WiFi::OnHook)
        };

        let (mic_ch, mic_stream, mic_cfg) = audio::get_input_channel()?;
        let (spk_ch, spk_stream, spk_cfg) = audio::get_output_channel()?;

        let (_shk_pin, _, shk_ch) = hook::try_register_shk()?;
        let (pulse_ch, _, hook_ch, _) = pulse::notgoertzelme(shk_ch);
        let goertz_ch = dtmf::goertzelme(mic_ch.subscribe());

        Ok(Self {
            state,

            _shk_pin,

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
            while let SwitchHook::ON = hook_ch.recv().await? {}

            debug!("PHONE PICKED UP");
            let audio_out_ch = self.audio_out_ch.clone();
            let tone_handle = TwoToneGen::off_hook(self.audio_out_sample_rate)
                .beep(Duration::from_millis(500), Duration::from_millis(500))
                .send_to(audio_out_ch, self.audio_out_n_channels);

            while let SwitchHook::OFF = hook_ch.recv().await? {}
            tone_handle.abort();
        }
    }
}
