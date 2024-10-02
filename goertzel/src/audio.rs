use std::error::Error;
use std::thread;
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{InputCallbackInfo, OutputCallbackInfo, Sample, SampleFormat, Stream, StreamConfig, SupportedStreamConfig};
use tokio::sync::{broadcast, mpsc};
use tracing::info;


const INPUT_SAMPLE_RATE: u32 = 48000;
const INPUT_BUF_SIZE: usize = 1<<16;

const OUTPUT_SAMPLE_RATE: u32 = 48000;
const OUTPUT_BUF_SIZE: usize = 1<<12;


pub fn get_input_channel() -> Result<(broadcast::Sender<i16>, Stream, SupportedStreamConfig), Box<dyn Error>> {
    let (send_ch, _rcv_ch) = broadcast::channel(INPUT_BUF_SIZE);
    let send_ch_i16 = send_ch.clone();
    let send_ch_f32 = send_ch.clone();

    let host = cpal::default_host();
    let device = loop {
        if let Some(device) = host.default_input_device() {
            break device;
        } else {
            info!("Failed to get input device, retrying...");
            thread::sleep(Duration::from_secs(1));
        }
    };
    let mut supported_config = None;
    for cfg in device.supported_input_configs()? {
        if cfg.sample_format() == SampleFormat::I16 {
            let candidate = cfg.try_with_sample_rate(cpal::SampleRate(INPUT_SAMPLE_RATE));
            if candidate.is_some() {
                supported_config = candidate;
                break;
            }
        }
        if cfg.sample_format() == SampleFormat::F32 {
            let candidate = cfg.try_with_sample_rate(cpal::SampleRate(INPUT_SAMPLE_RATE));
            if candidate.is_some() {
                supported_config = candidate;
            }
        }
    }
    let supported_config = supported_config.ok_or("could not get supported input config")?;
    let config: StreamConfig = supported_config.clone().into();

    let n_channels = supported_config.channels();
    let handle_i16 = move |data: &[i16], _: &InputCallbackInfo| {
        for sample in data.iter().step_by(n_channels.into()) {
            let _ = send_ch_i16.send(*sample);
        }
    };
    let handle_f32 = move |data: &[f32], _: &InputCallbackInfo| {
        for sample in data.iter().step_by(n_channels.into()) {
            let _ = send_ch_f32.send((*sample * 2.0_f32.powi(15)) as i16);
        }
    };
    let handle_err = move |_| { panic!("Fuck error handling 😮") };

    let stream = match supported_config.sample_format() {
        SampleFormat::I16 => device.build_input_stream(&config, handle_i16, handle_err, None)?,
        SampleFormat::F32 => device.build_input_stream(&config, handle_f32, handle_err, None)?,
        _ => Err("invalid sample format")?,
    };

    stream.play().unwrap();

    Ok((send_ch, stream, supported_config))
}

#[cfg(feature = "wav")]
pub fn get_wav_samples(fname: String, start_idx: Option<u32>, end_idx: Option<u32>) -> Box<dyn Iterator<Item=i16>> {
    let mut reader = hound::WavReader::open(fname).unwrap();
    let total_samples = (&reader).len();
    let start_idx = start_idx.unwrap_or(0);
    let end_idx = end_idx.unwrap_or(total_samples);
    reader.seek(start_idx).unwrap();

    let sample_format = reader.spec().sample_format;
    let reader_bits = reader.spec().bits_per_sample;
    let n_channels = reader.spec().channels;

    let samples: Box<dyn Iterator<Item=Result<i16, hound::Error>> + Send> = {
        match (sample_format, reader_bits) {
            (hound::SampleFormat::Float, _) => Box::new(reader.into_samples::<f32>().map(|s| Ok((s? * 2.0_f32.powi(15)) as i16))),
            (hound::SampleFormat::Int, 8) => Box::new(reader.into_samples::<i8>().map(|s| Ok((s? as i16) << 8))),
            (hound::SampleFormat::Int, 16) => Box::new(reader.into_samples::<i16>().map(|s| Ok(s?))),
            (hound::SampleFormat::Int, 32) => Box::new(reader.into_samples::<i32>().map(|s| Ok((s? >> 16) as i16))),
            (hound::SampleFormat::Int, n) => panic!("stinky sample format: {}", n),
        }
    };

    Box::new(samples.step_by(n_channels.into()).take((end_idx - start_idx) as usize).map(|s| s.unwrap()))
}

pub fn get_output_channel() -> Result<(mpsc::Sender<i16>, Stream, SupportedStreamConfig), Box<dyn Error>> {
    let host = cpal::default_host();
    let device = loop {
        if let Some(device) = host.default_output_device() {
            break device;
        } else {
            info!("Failed to get output device, retrying...");
            thread::sleep(Duration::from_secs(1));
        }
    };
    let mut supported_config = None;
    for cfg in device.supported_output_configs()? {
        if cfg.sample_format() == SampleFormat::I16 {
            let candidate = cfg.try_with_sample_rate(cpal::SampleRate(OUTPUT_SAMPLE_RATE));
            if candidate.is_some() {
                supported_config = candidate;
                break;
            }
        }
        if cfg.sample_format() == SampleFormat::F32 {
            let candidate = cfg.try_with_sample_rate(cpal::SampleRate(OUTPUT_SAMPLE_RATE));
            if candidate.is_some() {
                supported_config = candidate;
            }
        }
    }
    let supported_config = supported_config.ok_or("could not get supported output config")?;
    let config: StreamConfig = supported_config.clone().into();

    let n_channels = supported_config.channels();
    let (send_ch, mut rcv_ch) = mpsc::channel(OUTPUT_BUF_SIZE);
    let stream = match supported_config.sample_format() {
        SampleFormat::I16 => device.build_output_stream(
            &config,
            move |data: &mut [i16], _: &OutputCallbackInfo| {
                let mut sample_idx = 0;
                let mut curr_sample = Sample::EQUILIBRIUM;
                for sample in data.iter_mut() {
                    curr_sample = if sample_idx % n_channels == 0 {
                        rcv_ch.try_recv().unwrap_or(Sample::EQUILIBRIUM)
                    } else {
                        curr_sample
                    };
                    *sample = curr_sample;
                    sample_idx += 1;
                }
            },
            move |_| { panic!("Fuck error handling (output) 😮") },
            None,
        )?,
        SampleFormat::F32 => device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &OutputCallbackInfo| {
                let mut sample_idx = 0;
                let mut curr_sample = Sample::EQUILIBRIUM;
                for sample in data.iter_mut() {
                    curr_sample = if sample_idx % n_channels == 0 {
                        (rcv_ch.try_recv().unwrap_or(Sample::EQUILIBRIUM) as f32) / 2.0_f32.powi(15)
                    } else {
                        curr_sample
                    };
                    *sample = curr_sample;
                    sample_idx += 1;
                }
            },
            move |_| { panic!("Fuck error handling (output) 😮") },
            None,
        )?,
        _ => Err("invalid sample format")?,
    };

    stream.play()?;

    Ok((send_ch, stream, supported_config))
}
