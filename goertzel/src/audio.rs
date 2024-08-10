use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, Stream, SupportedStreamConfig};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

pub struct ItMyMic {
    pub samples_ch: UnboundedReceiver<f32>,
    _stream: Stream,
}

pub fn get_input_samples(sample_rate: u32) -> ItMyMic {
    let (send_ch, rcv_ch) = unbounded_channel();

    let host = cpal::default_host();

    let in_device = host.default_input_device().unwrap();
    let in_config: SupportedStreamConfig = in_device
        .supported_input_configs()
        .unwrap()
        .filter_map(|r| if r.channels() == 2 && r.sample_format() == SampleFormat::F32 {
            r.try_with_sample_rate(cpal::SampleRate(sample_rate))
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

    ItMyMic{
        samples_ch: rcv_ch,
        _stream: in_stream,
    }
}
