use std::f64::consts;
use std::iter::Chain;
use std::slice::Iter;
use std::sync::mpsc::channel;
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Sample;
use hound;
use ringbuf::storage::Heap;
use ringbuf::SharedRb;
use ringbuf::traits::{RingBuffer, Consumer};


const FNAME: &str = "cortelco_48k.wav";
const WINDOW_INTERVAL: u32 = 1000;
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


// TODO: Struct this with ring buffers?
fn goertzel_me(samples: Chain<Iter<i32>, Iter<i32>>, mut q1: f64, mut q2: f64, coeff: f64) -> (f64, f64, f64, f64) {
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
    // TODO: Use this as an iterator not a collected vec
    let samples = reader.samples::<i32>().collect::<Result<Vec<_>, _>>().unwrap();

    let (send_ch, rcv_ch) = channel::<i32>();
    let mut playback_idx = 0;
    let out_stream = device.build_output_stream(
        &supported_config.into(),
        move |data: &mut[f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                // Only send the left channel into the send channel
                if playback_idx < samples.len() && playback_idx % 2 == 0 {
                    let next_sample = samples[playback_idx];
                    send_ch.send(next_sample).unwrap();
                    *sample = next_sample as f32;
                } else {
                    *sample = Sample::EQUILIBRIUM;
                }

                playback_idx += 1;
            }
        },
        move |_| { dbg!("Fuck error handling"); },
        None
    ).unwrap();

    out_stream.play().unwrap();

    let mut sample_idx = 0;
    let mut chunk = SharedRb::<Heap<i32>>::new(CHUNK_SIZE as usize);
    let mut last_digit = 0;
    while let Ok(sample) = rcv_ch.recv_timeout(Duration::from_millis(100)) {
        chunk.push_overwrite(sample);
        if sample_idx >= CHUNK_SIZE && sample_idx % WINDOW_INTERVAL == 0 {
            // TODO: Move this code into an "identify digit" function
            let active_freqs: Vec<_> = coeffs
                .iter()
                .map(|c| goertzel_me(chunk.iter(), 0.0, 0.0, *c).0)
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
            if digit != 0 && digit != last_digit {
                dbg!(digit);
            }
            last_digit = digit;
        }

        sample_idx += 1;
    }
}
