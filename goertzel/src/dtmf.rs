use std::array::from_fn;
use std::f64::consts::{self, PI};
use std::slice::Iter;
use std::sync::LazyLock;

use itertools::Itertools;
use ringbuf::storage::Heap;
use ringbuf::SharedRb;
use ringbuf::traits::{RingBuffer, Consumer};
use tokio::sync::mpsc::{Receiver, channel};
use tracing::trace;

use crate::asyncutil::and_log_err;


pub const NULL: u8 = u8::MAX;
pub const SEXTILE: u8 = 10;
pub const OCTOTHORPE: u8 = 12;

const DIGIT_CHANNEL_SIZE: usize = 64;

// TODO(peter): Make this a runtime input
const SAMPLE_FREQ: u32 = 48000;
const SAMPLE_SCALE_FACTOR: f64 = 32768.0; // 2^15

const WINDOW_INTERVAL: usize = 1200;
const CHUNK_SIZE: usize = 1200;  // 12.75ms of sample

const THRESH_REL_PEAK_ROW: f64 = 6.0;
const THRESH_REL_PEAK_COL: f64 = 6.3;
const THRESH_REL_ENERGY: f64 = 42.;
const THRESH_MAG: f64 = 1e9;

const HITS_TO_BEGIN: usize = 2;
const MISSES_TO_END: usize = 2;

const N_ROW_FREQS: usize = 4;
const N_COL_FREQS: usize = 3;
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
    total_energy: f64,
}

impl<'a> Goertzeler<'a> {
    fn new() -> Self {
        Self {
            ham_c_iter: (*HAM_COEFFS).iter(),
            ariana_goertzde: (*GZ_COEFFS).map(|c| (c, SharedRb::<Heap<f64>>::new(2))),
            total_energy: 0.,
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


pub fn goertzelme(mut sample_channel: Receiver<f32>) -> Receiver<u8> {
    let mut sample_idx = 0;
    let mut goertzel_idx = 0;
    let mut goertzelers: Vec<_> = (0..(CHUNK_SIZE / WINDOW_INTERVAL))
        .into_iter()
        .map(|_| Goertzeler::new())
        .collect();

    let (send_ch, rcv_ch) = channel(DIGIT_CHANNEL_SIZE);
    tokio::spawn(and_log_err(async move {
        let mut curr_digit = NULL;
        let mut n_hit = 0;
        let mut n_miss = 0;
        loop {
            while sample_idx < WINDOW_INTERVAL {
                let sample = sample_channel.recv().await
                    .ok_or("goertzel hungers for audio samples")? as f64 * SAMPLE_SCALE_FACTOR;
                for goertzeler in goertzelers.iter_mut() {
                    goertzeler.push(sample);
                }
                sample_idx += 1;
            }

            let goertzeler = &goertzelers[goertzel_idx];
            let mut goertzel_nrgs = goertzeler.goertzel_me().into_iter().enumerate();
            let row_nrgs: Vec<_> = goertzel_nrgs.by_ref().take(N_ROW_FREQS)
                .sorted_by(|a, b| b.1.partial_cmp(&a.1).unwrap())
                .collect();
            let col_nrgs: Vec<_> = goertzel_nrgs.take(N_COL_FREQS)
                .sorted_by(|a, b| b.1.partial_cmp(&a.1).unwrap())
                .collect();

            let digit = 'dig: {
                let (row_idx, row_nrg) = row_nrgs[0];
                let (col_idx, col_nrg) = col_nrgs[0];
                if row_nrg < THRESH_MAG || col_nrg < THRESH_MAG
                    || row_nrg < row_nrgs[1].1 * THRESH_REL_PEAK_ROW
                    || col_nrg < col_nrgs[1].1 * THRESH_REL_PEAK_COL
                    || row_nrg + col_nrg < THRESH_REL_ENERGY * goertzeler.total_energy
                {
                    break 'dig NULL;
                }
                match (row_idx, col_idx) {
                    (3, 5) => 0,
                    (f1, f2) => (f1*3 + f2-3).try_into().unwrap(),
                }
            };

            let pretty_row_nrgs = row_nrgs.clone().into_iter().map(|(idx, nrg)| format!("{}:{:.5} ", idx, nrg.log10())).collect::<String>();
            let pretty_col_nrgs = col_nrgs.clone().into_iter().map(|(idx, nrg)| format!("{}:{:.5} ", idx, nrg.log10())).collect::<String>();
            trace!(digit);
            trace!("{} {}", col_nrgs[0].1.log10(), row_nrgs[0].1.log10());
            trace!(pretty_row_nrgs);
            trace!(pretty_col_nrgs);

            if digit == NULL {
                n_miss += 1;
            } else if digit == curr_digit {
                n_hit += 1;
                n_miss = 0;
            } else {
                curr_digit = digit;
                n_hit = 1;
                n_miss = 0;
            }
            if n_hit == HITS_TO_BEGIN {
                send_ch.try_send(curr_digit)?;
            }
            // TODO(peter): Clean up this logic for when misses to end > hits to begin
            if n_miss == MISSES_TO_END {
                curr_digit = NULL;
            }

            goertzelers[goertzel_idx] = Goertzeler::new();

            goertzel_idx += 1;
            if goertzel_idx == goertzelers.len() {
                goertzel_idx = 0;
            }

            sample_idx = 0;
        }
    }));

    rcv_ch
}
