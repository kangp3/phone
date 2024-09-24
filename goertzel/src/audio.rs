use std::error::Error;
use std::thread;
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat, Stream, SupportedStreamConfig};
use tokio::sync::{broadcast, mpsc};
use tracing::info;


const INPUT_SAMPLE_RATE: u32 = 48000;
const INPUT_BUF_SIZE: usize = 1<<16;
const IN_CHANNELS: u16 = 2;

const OUTPUT_SAMPLE_RATE: u32 = 48000;
const OUTPUT_BUF_SIZE: usize = 1<<12;


pub fn get_input_channel() -> Result<(broadcast::Sender<f32>, Stream, SupportedStreamConfig), Box<dyn Error>> {
    let (send_ch, _rcv_ch) = broadcast::channel(INPUT_BUF_SIZE);

    let host = cpal::default_host();
    let device = loop {
        if let Some(device) = host.default_input_device() {
            break device;
        } else {
            info!("Failed to get input device, retrying...");
            thread::sleep(Duration::from_secs(1));
        }
    };
    let config = device.supported_input_configs()?
        .filter_map(|r| (r.channels() == IN_CHANNELS).then_some(r))
        .filter_map(|r| (r.sample_format() == SampleFormat::F32).then_some(r))
        .filter_map(|r| r.try_with_sample_rate(cpal::SampleRate(INPUT_SAMPLE_RATE)))
        .next().ok_or("could not get supported input config")?;

    let send_ch2 = send_ch.clone();
    let stream = device.build_input_stream(
        &config.clone().into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            for sample in data.iter().step_by(IN_CHANNELS.into()) {
                let _ = send_ch2.send(*sample);
            }
        },
        move |_| { panic!("Fuck error handling 😮"); },
        None,
    ).unwrap();

    stream.play().unwrap();

    Ok((send_ch, stream, config))
}

#[cfg(feature = "wav")]
pub fn get_wav_samples(fname: String, start_idx: Option<u32>, end_idx: Option<u32>) -> Box<dyn Iterator<Item=f32>> {
    let mut reader = hound::WavReader::open(fname).unwrap();
    let total_samples = (&reader).len();
    let start_idx = start_idx.unwrap_or(0);
    let end_idx = end_idx.unwrap_or(total_samples);
    reader.seek(start_idx).unwrap();

    let sample_format = reader.spec().sample_format;
    let reader_bits = reader.spec().bits_per_sample;
    let n_channels = reader.spec().channels;

    let samples: Box<dyn Iterator<Item=Result<f32, hound::Error>> + Send> = {
        match (sample_format, reader_bits) {
            (hound::SampleFormat::Float, _) => Box::new(reader.into_samples::<f32>().map(|s| Ok(s?))),
            (hound::SampleFormat::Int, 8) => Box::new(reader.into_samples::<i8>().map(|s| Ok((s? as f32) / 2.0_f32.powi(7)))),
            (hound::SampleFormat::Int, 16) => Box::new(reader.into_samples::<i16>().map(|s| Ok((s? as f32) / 2.0_f32.powi(15)))),
            (hound::SampleFormat::Int, 32) => Box::new(reader.into_samples::<i32>().map(|s| Ok((s?as f32) / 2.0_f32.powi(31)))),
            (hound::SampleFormat::Int, n) => panic!("stinky sample format: {}", n),
        }
    };

    Box::new(samples.step_by(n_channels.into()).take((end_idx - start_idx) as usize).map(|s| s.unwrap()))
}

pub fn get_output_channel() -> Result<(mpsc::Sender<f32>, Stream, SupportedStreamConfig), Box<dyn Error>> {
    let host = cpal::default_host();
    let device = loop {
        if let Some(device) = host.default_output_device() {
            break device;
        } else {
            info!("Failed to get output device, retrying...");
            thread::sleep(Duration::from_secs(1));
        }
    };
    let config = device.supported_output_configs()?
        .filter_map(|r| (r.sample_format() == SampleFormat::F32).then_some(r))
        .filter_map(|r| r.try_with_sample_rate(cpal::SampleRate(OUTPUT_SAMPLE_RATE)))
        .next().ok_or("could not get supported output config")?;

    let (send_ch, mut rcv_ch) = mpsc::channel(OUTPUT_BUF_SIZE);
    let stream = device.build_output_stream(
        &config.clone().into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                *sample = rcv_ch.try_recv().unwrap_or(Sample::EQUILIBRIUM)
            }
        },
        move |_| { panic!("Fuck error handling (output) 😮"); },
        None,
    )?;

    stream.play()?;

    Ok((send_ch, stream, config))
}
