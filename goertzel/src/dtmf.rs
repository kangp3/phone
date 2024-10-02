use std::array::from_fn;
use std::error::Error;
use std::f64::consts::{self, PI};
use std::slice::Iter;
use std::sync::LazyLock;

use itertools::Itertools;
use ringbuf::storage::Heap;
use ringbuf::SharedRb;
use ringbuf::traits::{RingBuffer, Consumer};
use tokio::sync::{broadcast, mpsc};
use tracing::{debug, trace};

use crate::asyncutil::and_log_err;


pub const NULL: u8 = u8::MAX;
pub const SEXTILE: u8 = 10;
pub const OCTOTHORPE: u8 = 12;

const DIGIT_CHANNEL_SIZE: usize = 3;

// TODO(peter): Make this a runtime input
const SAMPLE_FREQ: u32 = 48000;

pub const WINDOW_INTERVAL: usize = 1200;
pub const CHUNK_SIZE: usize = 1200;  // 12.75ms of sample

const THRESH_REL_PEAKS: [f64; 7] = [1.35, 1.35, 1.15, 1.35, 300., 2., 2.];
const THRESH_REL_ENERGY: f64 = 42.;
const THRESH_MAG: f64 = 2e9;

const HITS_TO_BEGIN: usize = 2;
const MISSES_TO_END: usize = 1;

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
        let sample = sample * self.ham_c_iter.next().unwrap();
        for (coeff, ring) in &mut self.ariana_goertzde {
            let mut riter = ring.iter();
            let q2 = *riter.next().unwrap_or(&0.0);
            let q1 = *riter.next().unwrap_or(&0.0);
            ring.push_overwrite(*coeff * q1 - q2 + sample);
        }
        self.total_energy += sample*sample;
    }

    fn get_digit(self: &Self) -> u8 {
        let mut nrgs = self.ariana_goertzde.iter().map(|(coeff, ring)| {
            let mut riter = ring.iter();
            let q2 = *riter.next().unwrap_or(&0.0);
            let q1 = *riter.next().unwrap_or(&0.0);
            q1*q1 + q2*q2 - q1*q2*coeff
        }).enumerate();
        let row_nrgs: Vec<_> = nrgs.by_ref().take(N_ROW_FREQS)
            .sorted_by(|a, b| b.1.partial_cmp(&a.1).unwrap())
            .collect();
        let col_nrgs: Vec<_> = nrgs.take(N_COL_FREQS)
            .sorted_by(|a, b| b.1.partial_cmp(&a.1).unwrap())
            .collect();

        let digit = 'dig: {
            let (row_idx, row_nrg) = row_nrgs[0];
            let (col_idx, col_nrg) = col_nrgs[0];
            if row_nrg < THRESH_MAG || col_nrg < THRESH_MAG
                || row_nrg < row_nrgs[1].1 * THRESH_REL_PEAKS[row_idx]
                || col_nrg < col_nrgs[1].1 * THRESH_REL_PEAKS[col_idx]
                || row_nrg + col_nrg < THRESH_REL_ENERGY * self.total_energy
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
        let (row_idx, row_nrg) = row_nrgs[0];
        let (col_idx, col_nrg) = col_nrgs[0];
        trace!("");
        trace!("{}", digit);
        trace!("{}: row_nrg ({:.5}) >= THRESH_MAG ({})", row_nrg >= THRESH_MAG, row_nrg.log10(), THRESH_MAG.log10());
        trace!("{}: col_nrg ({:.5}) >= THRESH_MAG ({})", col_nrg >= THRESH_MAG, col_nrg.log10(), THRESH_MAG.log10());
        trace!("{}: row_nrg ({:.5}) >= row_nrgs[1].1 ({:.5}) * THRESH_REL_PEAKS ({:.5}) (R: {:.5})", row_nrg >= row_nrgs[1].1 * THRESH_REL_PEAKS[row_idx], row_nrg.log10(), row_nrgs[1].1.log10(), (row_nrgs[1].1 * THRESH_REL_PEAKS[row_idx]).log10(), row_nrg/row_nrgs[1].1);
        trace!("{}: col_nrg ({:.5}) >= col_nrgs[1].1 ({:.5}) * THRESH_REL_PEAKS ({:.5}) (R: {:.5})", col_nrg >= col_nrgs[1].1 * THRESH_REL_PEAKS[col_idx], col_nrg.log10(), col_nrgs[1].1.log10(), (col_nrgs[1].1 * THRESH_REL_PEAKS[col_idx]).log10(), col_nrg/col_nrgs[1].1);
        trace!("{}: row_nrg + col_nrg ({:.5}) >= THRESH_REL_ENERGY * self.total_energy ({:.5}) (R: {:.5})", row_nrg + col_nrg >= THRESH_REL_ENERGY * self.total_energy, (row_nrg + col_nrg).log10(), (THRESH_REL_ENERGY * self.total_energy).log10(), (row_nrg + col_nrg) / self.total_energy);
        trace!("{} {}", col_nrgs[0].1.log10(), row_nrgs[0].1.log10());
        trace!(pretty_row_nrgs);
        trace!(pretty_col_nrgs);

        digit
    }
}


struct DigState {
    sent_dig: u8,
    curr_dig: u8,
    n_hits: usize,
    n_misses: usize,
}

impl Default for DigState {
    fn default() -> Self {
        Self {
            sent_dig: NULL,
            curr_dig: NULL,
            n_hits: 0,
            n_misses: 0,
        }
    }
}

impl DigState {
    fn poosh(&mut self, dig: u8) -> Option<u8> {
        match (self.sent_dig, self.curr_dig, self.n_hits, self.n_misses) {
            (sent, _, _, _) if dig == sent => {
                self.n_hits = 0;
                self.n_misses = 0;
            },
            (NULL, cur, _, _) if dig != cur => {
                self.curr_dig = dig;
                self.n_hits = 1;
            },
            (NULL, cur, n_hits, _) if dig == cur && n_hits < HITS_TO_BEGIN - 1 => {
                self.n_hits += 1;
            },
            (NULL, cur, n_hits, _) if dig == cur && n_hits == HITS_TO_BEGIN - 1 => {
                self.sent_dig = cur;
                self.n_hits = 0;
                return Some(cur);
            },
            (_, _, _, n_misses) if dig == NULL && n_misses < MISSES_TO_END - 1 => {
                self.n_misses += 1;
            }
            (_, _, _, n_misses) if dig == NULL && n_misses == MISSES_TO_END - 1 => {
                self.sent_dig = NULL;
                self.n_hits = 0;
                self.n_misses = 0;
            }
            (_, cur, _, n_misses) if dig != cur && n_misses < MISSES_TO_END - 1 => {
                self.curr_dig = dig;
                self.n_hits = 0;
                self.n_misses += 1;
                if HITS_TO_BEGIN > 1 {
                    self.n_hits += 1;
                }
            }
            (_, cur, _, n_misses) if dig != cur && n_misses == MISSES_TO_END - 1 => {
                self.sent_dig = NULL;
                self.curr_dig = dig;
                self.n_hits = 0;
                self.n_misses = 0;
                if HITS_TO_BEGIN == 1 {
                    self.sent_dig = cur;
                    self.n_misses = 0;
                    return Some(cur);
                } else {
                    self.n_hits += 1;
                }
            }
            (_, cur, _, n_misses) if dig == cur && n_misses < MISSES_TO_END - 1 => {
                self.n_misses += 1;
                self.n_hits += 1;
            }
            (_, cur, n_hits, n_misses) if dig == cur && n_misses == MISSES_TO_END - 1 => {
                self.n_misses = 0;
                if n_hits < HITS_TO_BEGIN - 1 {
                    self.n_hits += 1;
                } else {
                    self.sent_dig = cur;
                    self.n_hits = 0;
                    return Some(cur);
                }
            }
            _ => panic!("stinky dig state")
        }
        None
    }
}


pub fn goertzelme(mut sample_channel: broadcast::Receiver<i16>) -> mpsc::Receiver<u8> {
    let mut sample_idx = 0;
    let mut goertzel_idx = 0;
    let mut goertzelers: Vec<_> = (0..(CHUNK_SIZE / WINDOW_INTERVAL))
        .into_iter()
        .map(|_| Goertzeler::new())
        .collect();

    let (send_ch, rcv_ch) = mpsc::channel(DIGIT_CHANNEL_SIZE);
    tokio::spawn(and_log_err("goertzeling", async move {
        let mut dig_state = DigState::default();
        'main: loop {
            while sample_idx < WINDOW_INTERVAL {
                let sample = tokio::select! {
                    sample = sample_channel.recv() => sample,
                    _ = send_ch.closed() => {
                        debug!("goertzel dig ch closed");
                        break 'main;
                    },
                };
                if let Err(_) = sample {
                    continue;
                }
                let sample = sample? as f64;
                for goertzeler in goertzelers.iter_mut() {
                    goertzeler.push(sample);
                }
                sample_idx += 1;
            }

            let detected_dig = goertzelers[goertzel_idx].get_digit();

            if let Some(dig) = dig_state.poosh(detected_dig) {
                if let Err(_) = send_ch.send(dig).await { break }
            }

            goertzelers[goertzel_idx] = Goertzeler::new();

            goertzel_idx += 1;
            if goertzel_idx == goertzelers.len() {
                goertzel_idx = 0;
            }

            sample_idx = 0;
        }
        Ok(())
    }));

    rcv_ch
}


pub fn goertzeliter(mut samples: Box<dyn Iterator<Item=i16>>) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut sample_idx = 0;
    let mut goertzel_idx = 0;
    let mut goertzelers: Vec<_> = (0..(CHUNK_SIZE / WINDOW_INTERVAL))
        .into_iter()
        .map(|_| Goertzeler::new())
        .collect();

    let mut digs = vec![];
    let mut dig_state = DigState::default();
    loop {
        while sample_idx < WINDOW_INTERVAL {
            if let Some(sample) = samples.next() {
                for goertzeler in goertzelers.iter_mut() {
                    goertzeler.push(sample as f64);
                }
            } else {
                return Ok(digs);
            }
            sample_idx += 1;
        }

        let detected_dig = goertzelers[goertzel_idx].get_digit();

        if let Some(dig) = dig_state.poosh(detected_dig) {
            digs.push(dig);
        }

        goertzelers[goertzel_idx] = Goertzeler::new();

        goertzel_idx += 1;
        if goertzel_idx == goertzelers.len() {
            goertzel_idx = 0;
        }

        sample_idx = 0;
    }
}
