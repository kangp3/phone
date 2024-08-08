use std::f64::consts::{self, PI};
use std::slice::Iter;
use std::sync::mpsc::channel;
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, SupportedStreamConfig};
use itertools::Itertools;
use ringbuf::storage::Heap;
use ringbuf::SharedRb;
use ringbuf::traits::{RingBuffer, Consumer};


const WINDOW_INTERVAL: u32 = 1000;
const CHUNK_SIZE: u32 = 2000;
const SAMPLE_FREQ: u32 = 48000;
const FREQS: [u32;7] = [697, 770, 852, 941, 1209, 1336, 1477];
const THRESHOLD_MAG: f64 = 50.0;


fn goertzel_coeff(target_freq: u32, sample_freq: u32) -> f64 {
    2.0 * (
        2.0 * consts::PI / CHUNK_SIZE as f64 * (
            0.5 + CHUNK_SIZE as f64 * target_freq as f64 / sample_freq as f64
        )
    ).cos()
}


struct Goertzeler<'a> {
    ham_coeffs: Iter<'a, f64>,
    ariana_goertzde: [(f64, SharedRb::<Heap<f64>>);7],
}

impl<'a> Goertzeler<'a> {
    fn new(coeffs: [f64;7], ham_coeffs: Iter<'a, f64>) -> Self {
        Self {
            ham_coeffs,
            ariana_goertzde: coeffs.map(|c| (c, SharedRb::<Heap<f64>>::new(2))),
        }
    }

    fn push(self: &mut Self, sample: f64) {
        let ham_c = self.ham_coeffs.next().unwrap();
        for (coeff, ring) in &mut self.ariana_goertzde {
            let mut riter = ring.iter();
            let q2 = *riter.next().unwrap_or(&0.0);
            let q1 = *riter.next().unwrap_or(&0.0);
            ring.push_overwrite(*coeff * q1 - q2 + sample*ham_c);
        }
    }

    fn goertzel_me(self: &Self) -> Vec<f64> {
        self.ariana_goertzde.iter().map(|(coeff, ring)| {
            let mut riter = ring.iter();
            let q2 = *riter.next().unwrap_or(&0.0);
            let q1 = *riter.next().unwrap_or(&0.0);
            q1*q1 + q2*q2 - q1*q2*coeff
        }).collect()
    }
}


#[tokio::main]
async fn main() {
    let gz_coeffs = FREQS.map(|f| goertzel_coeff(f, SAMPLE_FREQ));
    let ham_coeffs: Vec<_> = (0..CHUNK_SIZE)
        .map(|n| 0.54 - 0.46* (2.0*PI*(n as f64)/((CHUNK_SIZE-1) as f64)).cos())
        .collect();

    let (send_sample_ch, rcv_sample_ch) = channel::<f32>();

    let host = cpal::default_host();
    // Get input from mic
    let in_device = host.default_input_device().unwrap();
    let in_config: SupportedStreamConfig = in_device
        .supported_input_configs()
        .unwrap()
        .map(|r| dbg!(r))
        .filter_map(|r| if r.channels() == 2 && r.sample_format() == SampleFormat::F32 {
            r.try_with_sample_rate(cpal::SampleRate(SAMPLE_FREQ))
        } else {
            None
        }).next().unwrap();
    let mut playback_idx = 0;
    let in_stream = in_device.build_input_stream(
        &in_config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            for sample in data.iter() {
                if playback_idx % 2 == 0 {
                    send_sample_ch.send(*sample * 2.0_f32.powf(16.0)).unwrap();
                }
                playback_idx += 1;
            }
        },
        move |_| { dbg!("Fuck error handling 😮"); },
        None,
    ).unwrap();
    in_stream.play().unwrap();

    let mut sample_idx = 0;
    let mut goertzel_idx = 0;
    let mut goertzelers: Vec<_> = (0..(CHUNK_SIZE / WINDOW_INTERVAL))
        .into_iter()
        .map(|_| Goertzeler::new(gz_coeffs, ham_coeffs.iter()))
        .collect();
    let mut last_digit = 0;
    while let Ok(sample) = rcv_sample_ch.recv_timeout(Duration::from_millis(100)) {
        if sample_idx == WINDOW_INTERVAL {
            let goertzeler = &goertzelers[goertzel_idx];
            let sorted_mags: Vec<_> = goertzeler.goertzel_me()
                .into_iter()
                .enumerate()
                .sorted_by(|a, b| b.1.partial_cmp(&a.1).unwrap())
                .collect();
            let bg_sum = sorted_mags[2..].iter().map(|(_, mag)| mag).sum::<f64>();
            if goertzel_idx % 10 == 0 {
                //dbg!(sorted_mags[1].1 / bg_sum);
            }
            let digit = match sorted_mags[0..2] {
                _ if sorted_mags[1].1 < bg_sum * THRESHOLD_MAG => 0,
                [(f1, _), (f2, _)] if f2 > 3 && f1 < 4 => f1*3 + f2-3,
                [(f1, _), (f2, _)] if f1 > 3 && f2 < 4 => f2*3 + f1-3,
                _ => 0,
            };
            if digit != 0 && digit != last_digit {
                dbg!(digit);
            }
            last_digit = digit;

            goertzelers[goertzel_idx] = Goertzeler::new(gz_coeffs, ham_coeffs.iter());

            goertzel_idx += 1;
            if goertzel_idx == goertzelers.len() {
                goertzel_idx = 0;
            }
            sample_idx = 0;
        }
        for goertzeler in goertzelers.iter_mut() {
            goertzeler.push(sample as f64);
        }

        sample_idx += 1;
    }
    // Why doesn't this exit?
    // dbg!("DONE?");
}
