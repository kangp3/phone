use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, SupportedStreamConfig};
use tokio::sync::mpsc::unbounded_channel;

use goertzel;


#[tokio::main]
async fn main() {
    let (send_ch, rcv_ch) = unbounded_channel();

    let host = cpal::default_host();
    // Get input from mic
    let in_device = host.default_input_device().unwrap();
    let in_config: SupportedStreamConfig = in_device
        .supported_input_configs()
        .unwrap()
        .map(|r| dbg!(r))
        .filter_map(|r| if r.channels() == 2 && r.sample_format() == SampleFormat::F32 {
            // TODO(peter): Make sample rate an input into the goertzels
            r.try_with_sample_rate(cpal::SampleRate(48000))
        } else {
            None
        }).next().unwrap();
    let mut playback_idx = 0;
    let in_stream = in_device.build_input_stream(
        &in_config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            for sample in data.iter() {
                if playback_idx % 2 == 0 {
                    send_ch.send(*sample * 2.0_f32.powf(16.0)).unwrap();
                }
                playback_idx += 1;
            }
        },
        move |_| { dbg!("Fuck error handling 😮"); },
        None,
    ).unwrap();
    in_stream.play().unwrap();

    let mut digs_ch = goertzel::goertzelme(rcv_ch);
    while let Some(dig) = digs_ch.recv().await {
        dbg!(dig);
    }
}
