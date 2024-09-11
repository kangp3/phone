use std::thread::sleep;
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, Stream, SupportedStreamConfig};
use tokio::sync::broadcast::{channel, Receiver, Sender};

const SAMPLE_BUF_SIZE: usize = 65536;

pub struct ItMyMic {
    pub samples_tx: Sender<f32>,
    pub samples_rx: Receiver<f32>,
    _stream: Option<Stream>,
}

pub fn get_mic_samples(sample_rate: u32) -> ItMyMic {
    let (send_ch, rcv_ch) = channel(SAMPLE_BUF_SIZE);
    let send_ch2 = send_ch.clone();

    let host = cpal::default_host();

    let in_device = host.default_input_device().unwrap();
    let in_config: SupportedStreamConfig = {
        loop {
            if let Ok(configs) = in_device.supported_input_configs() {
                break configs
                    .filter_map(|r| if r.channels() == 2 && r.sample_format() == SampleFormat::F32 {
                        r.try_with_sample_rate(cpal::SampleRate(sample_rate))
                    } else {
                        None
                    }).next().unwrap();
            } else {
                dbg!("Failed to get input device configs, retrying...");
                sleep(Duration::from_secs(1));
            }
        }
    };

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
        move |_| { panic!("Fuck error handling 😮"); },
        None,
    ).unwrap();

    in_stream.play().unwrap();

    ItMyMic{
        samples_tx: send_ch2,
        samples_rx: rcv_ch,
        _stream: Some(in_stream),
    }
}

#[cfg(feature = "wav")]
pub fn get_mic_samples_with_outfile(sample_rate: u32, fname: String) -> ItMyMic {
    let mic = get_mic_samples(sample_rate);
    let (send_ch, recv_ch) = channel(SAMPLE_BUF_SIZE);
    let send_ch2 = send_ch.clone();

    let mut writer = hound::WavWriter::create(fname, hound::WavSpec{
        channels: 2,
        sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    }).unwrap();
    let mut samples_rx = mic.samples_tx.subscribe();
    tokio::spawn(async move {
        loop {
            match samples_rx.recv().await {
                Ok(sample) => {
                    writer.write_sample(sample).unwrap();
                    send_ch.send(sample).unwrap();
                },
                Err(e) => {
                    dbg!(e);
                    break;
                }
            }
        }
        writer.finalize().unwrap();
    });

    ItMyMic{
        samples_tx: send_ch2,
        samples_rx: recv_ch,
        _stream: mic._stream,
    }
}

#[cfg(feature = "wav")]
pub fn get_wav_samples(fname: String) -> ItMyMic {
    let (send_ch, rcv_ch) = channel(SAMPLE_BUF_SIZE);
    let send_ch2 = send_ch.clone();

    let reader = hound::WavReader::open(fname).unwrap();
    let reader_bits = reader.spec().bits_per_sample;
    let samples = reader.into_samples::<i32>();
    let mut is_l_channel = false;
    tokio::spawn(async move {
        for s in samples {
            if is_l_channel {
                match s {
                    Ok(s) => {
                        send_ch.send(s as f32/2.0f32.powi(reader_bits.into())).unwrap();
                    },
                    Err(_) => break,
                }
            }
            is_l_channel = !is_l_channel;
        }
    });

    ItMyMic{
        samples_tx: send_ch2,
        samples_rx: rcv_ch,
        _stream: None,
    }
}
