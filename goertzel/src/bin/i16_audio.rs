use std::error::Error;

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::SampleFormat;
use goertzel::audio;

pub fn main() -> Result<(), Box<dyn Error>> {
    // Why do I need this??????
    let _ = audio::get_input_channel();

    let host = cpal::default_host();
    let device = host.default_input_device().ok_or("missing input device")?;
    let _config = device
        .supported_input_configs()?
        .filter_map(|r| (r.sample_format() == SampleFormat::I16).then_some(r))
        .filter_map(|r| r.try_with_sample_rate(cpal::SampleRate(48000)))
        .map(|r| dbg!(r))
        .next()
        .ok_or("could not get supported input config")?;

    Ok(())
}
