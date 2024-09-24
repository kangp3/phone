use std::f32::consts::PI;
use std::time::Duration;

use cpal::Sample;
use tokio::sync::mpsc;
use tokio::task::AbortHandle;


const GAIN: f32 = 0.5;

const OFFHOOK_TONES: (u16, u16) = (350, 440);
const BUSY_TONES: (u16, u16) = (480, 620);
const RING_TONES: (u16, u16) = (440, 480);


pub struct TwoToneGen {
    samples: Vec<f32>,
    sample_idx: usize,
    sample_rate: u32,

    on_count: usize,
    off_count: usize,
    sent_count: usize,
}

impl TwoToneGen {
    pub fn new(rate: u32, f1: u16, f2: u16) -> Self {
        // All frequencies in call progress tones are divisible by 10
        // So generate 1/10s worth of samples
        let bufsize = rate / 10;
        let mut samples = vec![];
        let step = 1. / rate as f32;
        for i in 0..bufsize {
            let samp1 = GAIN * (2. * PI * f1 as f32 * step * i as f32).sin();
            let samp2 = GAIN * (2. * PI * f2 as f32 * step * i as f32).sin();
            samples.push(samp1+samp2);
        }
        Self {
            samples,
            sample_idx: 0,
            sample_rate: rate,

            on_count: bufsize as usize,
            off_count: 0,
            sent_count: 0,
        }
    }

    pub fn off_hook(rate: u32) -> Self {
        Self::new(rate, OFFHOOK_TONES.0, OFFHOOK_TONES.1)
    }

    pub fn busy(rate: u32) -> Self {
        Self::new(rate, BUSY_TONES.0, BUSY_TONES.1)
    }

    pub fn ring(rate: u32) -> Self {
        Self::new(rate, RING_TONES.0, RING_TONES.1)
    }

    pub fn beep(mut self, on_dur: Duration, off_dur: Duration) -> Self {
        self.on_count = (on_dur.as_secs_f32() * self.sample_rate as f32) as usize;
        self.off_count = (off_dur.as_secs_f32() * self.sample_rate as f32) as usize;
        self
    }

    pub fn send_to(mut self, ch: mpsc::Sender<f32>, n_channels: u16) -> AbortHandle {
        tokio::spawn(async move {
            while let Some(s) = self.next() {
                for _ in 0..n_channels {
                    let _ = ch.send(s).await;
                }
            }
        }).abort_handle()
    }
}

impl Iterator for TwoToneGen {
    type Item=f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.off_count > 0 {
            self.sent_count += 1;
            if self.sent_count <= self.on_count {
                let s = self.samples[self.sample_idx];
                self.sample_idx = (self.sample_idx + 1) % self.samples.len();
                Some(s)
            } else {
                if self.sent_count == self.on_count + self.off_count {
                    self.sent_count = 0;
                }
                Some(Sample::EQUILIBRIUM)
            }
        } else {
            let s = self.samples[self.sample_idx];
            self.sample_idx = (self.sample_idx + 1) % self.samples.len();
            Some(s)
        }
    }
}
