use std::array::from_fn;
use std::f64::consts::{self, PI};
use std::slice::Iter;
use std::sync::LazyLock;

use itertools::Itertools;
use ringbuf::storage::Heap;
use ringbuf::SharedRb;
use ringbuf::traits::{RingBuffer, Consumer};
use tokio::sync::mpsc::{UnboundedReceiver, unbounded_channel};


pub const NULL: u8 = u8::MAX;
pub const STAR: u8 = 10;
pub const OCTOTHORPE: u8 = 12;

const WINDOW_INTERVAL: usize = 1000;
const CHUNK_SIZE: usize = 2000;
// TODO(peter): Make this a runtime input
const SAMPLE_FREQ: u32 = 48000;
const THRESHOLD_MAG: f64 = 50.0;


const FREQS: [u32;7] = [697, 770, 852, 941, 1209, 1336, 1477];
static GZ_COEFFS: LazyLock<[f64;7]> = LazyLock::new(|| {
    FREQS.map(|f| {
        2.0 * (2.0 * consts::PI / CHUNK_SIZE as f64 * (
            0.5 + CHUNK_SIZE as f64 * f as f64 / SAMPLE_FREQ as f64
        )).cos()
    })
});
static HAM_COEFFS: LazyLock<[f64;CHUNK_SIZE]> = LazyLock::new(|| {
    from_fn(|n| 0.54 - 0.46* (2.0*PI*(n as f64)/((CHUNK_SIZE-1) as f64)).cos())
});


struct Goertzeler<'a> {
    ham_c_iter: Iter<'a, f64>,
    ariana_goertzde: [(f64, SharedRb::<Heap<f64>>);7],
}

impl<'a> Goertzeler<'a> {
    fn new() -> Self {
        Self {
            ham_c_iter: (*HAM_COEFFS).iter(),
            ariana_goertzde: (*GZ_COEFFS).map(|c| (c, SharedRb::<Heap<f64>>::new(2))),
        }
    }

    fn push(self: &mut Self, sample: f64) {
        let ham_c = self.ham_c_iter.next().unwrap();
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


pub fn goertzelme(mut sample_channel: UnboundedReceiver<f32>) -> UnboundedReceiver<u8> {
    let mut sample_idx = 0;
    let mut goertzel_idx = 0;
    let mut goertzelers: Vec<_> = (0..(CHUNK_SIZE / WINDOW_INTERVAL))
        .into_iter()
        .map(|_| Goertzeler::new())
        .collect();
    let mut last_digit = NULL;

    let (send_ch, rcv_ch) = unbounded_channel();
    tokio::spawn(async move {
        while let Some(sample) = sample_channel.recv().await {
            if sample_idx == WINDOW_INTERVAL {
                let goertzeler = &goertzelers[goertzel_idx];
                let sorted_mags: Vec<_> = goertzeler.goertzel_me()
                    .into_iter()
                    .enumerate()
                    .sorted_by(|a, b| b.1.partial_cmp(&a.1).unwrap())
                    .collect();
                let bg_sum = sorted_mags[2..].iter().map(|(_, mag)| mag).sum::<f64>();
                let digit = match sorted_mags[0..2] {
                    _ if sorted_mags[1].1 < bg_sum * THRESHOLD_MAG => NULL,
                    [(3, _), (5, _)] |
                    [(5, _), (3, _)] => 0,
                    [(f1, _), (f2, _)] if f2 > 3 && f1 < 4 => (f1*3 + f2-3).try_into().unwrap(),
                    [(f1, _), (f2, _)] if f1 > 3 && f2 < 4 => (f2*3 + f1-3).try_into().unwrap(),
                    _ => NULL,
                };
                if digit != NULL && digit != last_digit {
                    send_ch.send(digit).unwrap();
                }
                last_digit = digit;

                goertzelers[goertzel_idx] = Goertzeler::new();

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
    });

    rcv_ch
}
