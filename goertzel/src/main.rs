use std::f64::consts;
use std::sync::mpsc::channel;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound;


const FNAME: &str = "/Users/kangp3/Documents/projects/phone/audio_samples/cortelco_48k.wav";
const WINDOW_INTERVAL: u32 = 3000;
const CHUNK_SIZE: u32 = 3000;
const SAMPLE_FREQ: u32 = 48000;
const FREQS: [u32;7] = [697, 770, 852, 941, 1209, 1336, 1477];


fn goertzel_coeff(target_freq: u32, sample_freq: u32) -> f64 {
    2.0 * (
        2.0 * consts::PI / CHUNK_SIZE as f64 * (
            0.5 + CHUNK_SIZE as f64 * target_freq as f64 / sample_freq as f64
        )
    ).cos()
}


fn goertzel_me(samples: &Vec<i32>, mut q1: f64, mut q2: f64, coeff: f64) -> (f64, f64, f64, f64) {
    let mut q0: f64 = 0.0;
    for sample in samples {
        let sample: f64 = (*sample).try_into().unwrap();
        q0 = coeff * q1 - q2 + sample;
        q2 = q1;
        q1 = q0;
    }
    return (q1*q1 + q2*q2 - q1*q2*coeff, q0, q1, q2);
}


fn main() {
    let mag_threshold = 42.5_f64.exp();
    let coeffs = FREQS.map(|f| goertzel_coeff(f, SAMPLE_FREQ));

    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let supported_config = device
        .supported_output_configs()
        .unwrap()
        .filter_map(|r| r.try_with_sample_rate(cpal::SampleRate(SAMPLE_FREQ)))
        .next()
        .unwrap();

    let mut reader = hound::WavReader::open(FNAME).unwrap();
    let samples = reader.samples::<i32>();

    let mut playback_reader = hound::WavReader::open(FNAME).unwrap();
    let playback_samples = playback_reader.samples::<i32>().collect::<Result<Vec<_>, _>>().unwrap();
    let mut idx = 0;

    let (send_ch, rcv_ch) = channel::<i32>();
    let playback_stream = device.build_output_stream(
        &supported_config.into(),
        move |data: &mut[f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                let next_sample = playback_samples[idx];

                // Only send the left channel into the send channel
                if idx % 2 == 0 {
                    send_ch.send(next_sample).unwrap();
                }
                *sample = next_sample as f32;

                idx += 1;
            }
        },
        move |_| { dbg!("Fuck error handling"); },
        None
    ).unwrap();

    playback_stream.play().unwrap();

    let mut chunk = vec![];
    while let Ok(sample) = rcv_ch.recv() {
        chunk.push(sample);
        if chunk.len() == WINDOW_INTERVAL as usize {
            let active_freqs: Vec<_> = coeffs
                .iter()
                .map(|c| goertzel_me(&chunk, 0.0, 0.0, *c).0)
                .enumerate()
                .filter_map(|(idx, mag)| (mag > mag_threshold).then_some(idx))
                .collect();
            let digit = match active_freqs[..] {
                [0, 4] => 1,
                [0, 5] => 2,
                [0, 6] => 3,
                [1, 4] => 4,
                [1, 5] => 5,
                [1, 6] => 6,
                [2, 4] => 7,
                [2, 5] => 8,
                [2, 6] => 9,
                [3, 4] => 10,
                [3, 5] => 11,
                [3, 6] => 12,
                _ => 0,
            };
            if digit != 0 {
                dbg!(digit);
            }
            chunk = vec![];
        }
    }
}
