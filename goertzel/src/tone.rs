use std::f32::consts::PI;
use std::time::Duration;

use cpal::Sample;
use tokio::sync::mpsc;
use tokio::task::AbortHandle;
use tracing::debug;


const GAIN: f32 = 0.5;

const OFFHOOK_TONES: (u16, u16) = (350, 440);
const BUSY_TONES: (u16, u16) = (480, 620);
const RING_TONES: (u16, u16) = (440, 480);


pub struct TwoToneGen {
    samples: Vec<f32>,
    sample_rate: u32,

    on_count: usize,
    off_count: usize,

    handle: Option<AbortHandle>,
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
            sample_rate: rate,

            on_count: bufsize as usize,
            off_count: 0,

            handle: None,
        }
    }

    pub fn off_hook(rate: u32) -> Self {
        Self::new(rate, OFFHOOK_TONES.0, OFFHOOK_TONES.1)
    }

    pub fn no_wifi(rate: u32) -> Self {
        Self::new(rate, OFFHOOK_TONES.0, OFFHOOK_TONES.1)
            .beep(Duration::from_millis(500), Duration::from_millis(500))
    }

    pub fn busy(rate: u32) -> Self {
        Self::new(rate, BUSY_TONES.0, BUSY_TONES.1)
            .beep(Duration::from_millis(500), Duration::from_millis(500))
    }

    pub fn ring(rate: u32) -> Self {
        Self::new(rate, RING_TONES.0, RING_TONES.1)
            .beep(Duration::from_secs(2), Duration::from_secs(4))
    }

    pub fn beep(mut self, on_dur: Duration, off_dur: Duration) -> Self {
        self.on_count = (on_dur.as_secs_f32() * self.sample_rate as f32) as usize;
        self.off_count = (off_dur.as_secs_f32() * self.sample_rate as f32) as usize;
        self
    }

    pub fn play(&mut self, ch: mpsc::Sender<f32>, n_channels: u16) {
        let on_count = self.on_count;
        let off_count = self.off_count;

        let samples = self.samples.clone();
        let handle = tokio::spawn(async move {
            let mut sample_idx = 0;
            let mut sent_count = 0;
            loop {
                let sample = if off_count > 0 {
                    sent_count += 1;
                    if sent_count <= on_count {
                        let s = samples[sample_idx];
                        sample_idx = (sample_idx + 1) % samples.len();
                        s
                    } else {
                        if sent_count == on_count + off_count {
                            sent_count = 0;
                        }
                        Sample::EQUILIBRIUM
                    }
                } else {
                    let s = samples[sample_idx];
                    sample_idx = (sample_idx + 1) % samples.len();
                    s
                };
                for _ in 0..n_channels {
                    let _ = ch.send(sample).await;
                }
            }
        });
        self.handle = Some(handle.abort_handle());
    }
}

impl Drop for TwoToneGen {
    fn drop(&mut self) {
        debug!("dropping tone");
        match &self.handle {
            Some(handle) => handle.abort(),
            None => {},
        }
    }
}
