use std::thread::sleep;
use std::time::Duration;
#[cfg(feature = "wav")]
use std::ops::Range;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, Stream, SupportedStreamConfig};
use tokio::sync::mpsc::{channel, Receiver};
use tracing::info;


const SAMPLE_BUF_SIZE: usize = 65536;


pub struct ItMyMic {
    pub samples_ch: Receiver<f32>,
    _stream: Option<Stream>,
}

pub fn get_mic_samples(sample_rate: u32) -> ItMyMic {
    let (send_ch, rcv_ch) = channel(SAMPLE_BUF_SIZE);

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
                info!("Failed to get input device configs, retrying...");
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
                    send_ch.try_send(*sample * 2.0_f32.powf(15.0)).unwrap();
                }
                playback_idx += 1;
            }
        },
        move |_| { panic!("Fuck error handling 😮"); },
        None,
    ).unwrap();

    in_stream.play().unwrap();

    ItMyMic{
        samples_ch: rcv_ch,
        _stream: Some(in_stream),
    }
}

#[cfg(feature = "wav")]
pub fn get_mic_samples_with_outfile(sample_rate: u32, fname: String) -> ItMyMic {
    use crate::asyncutil::and_log_err;

    let mut mic = get_mic_samples(sample_rate);
    let (send_ch, recv_ch) = channel(SAMPLE_BUF_SIZE);

    let mut writer = hound::WavWriter::create(fname, hound::WavSpec{
        channels: 1,
        sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    }).unwrap();

    tokio::spawn(async move {
        and_log_err(async {
            while let Some(sample) = mic.samples_ch.recv().await {
                writer.write_sample(sample / 2.0_f32.powf(16.0))?;
                send_ch.try_send(sample)?;
            }
            Ok(())
        }).await;
        writer.finalize().unwrap();
    });

    ItMyMic{
        samples_ch: recv_ch,
        _stream: mic._stream,
    }
}

#[cfg(feature = "wav")]
pub fn get_wav_samples(fname: String, sample_range: Option<Range<u32>>) -> ItMyMic {
    use crate::asyncutil::and_log_err;

    let (send_ch, rcv_ch) = channel(SAMPLE_BUF_SIZE);

    let mut reader = hound::WavReader::open(fname).unwrap();
    let n_samples = (&reader).len();
    if let Some(ref sample_range) = sample_range {
        reader.seek(sample_range.start).unwrap();
    };
    let sample_format = reader.spec().sample_format;
    let reader_bits = reader.spec().bits_per_sample;
    let n_channels = reader.spec().channels;

    let samples: Box<dyn Iterator<Item=Result<f32, hound::Error>> + Send> = {
        match (sample_format, reader_bits) {
            (hound::SampleFormat::Float, _) => Box::new(reader.into_samples::<f32>().map(|s| Ok(s? * 2.0_f32.powf(15.0)))),
            (hound::SampleFormat::Int, 8) => Box::new(reader.into_samples::<i8>().map(|s| Ok(((s? as i16) << 8) as f32))),
            (hound::SampleFormat::Int, 16) => Box::new(reader.into_samples::<i16>().map(|s| Ok(s? as f32))),
            (hound::SampleFormat::Int, 32) => Box::new(reader.into_samples::<i32>().map(|s| Ok((s? >> 16) as f32))),
            (hound::SampleFormat::Int, n) => panic!("stinky sample format: {}", n),
        }
    };

    tokio::spawn(and_log_err(async move {
        for s in samples.step_by(n_channels.into()).take(sample_range.unwrap_or(0..n_samples).len()) {
            send_ch.send(s?).await?;
        }
        Ok(())
    }));

    ItMyMic{
        samples_ch: rcv_ch,
        _stream: None,
    }
}
