use std::f64::consts;

use hound;
use itertools::Itertools;


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

    let mut reader = hound::WavReader::open(FNAME).unwrap();
    dbg!(reader.spec());
    let samples = reader.samples::<i32>();

    let mut found_digits = vec![];
    for (idx, chunk) in samples.step_by(2).chunks(WINDOW_INTERVAL as usize).into_iter().enumerate() {
        let idx = idx * WINDOW_INTERVAL as usize;
        let chunk = chunk.collect::<Result<Vec<i32>, _>>().unwrap();

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
            found_digits.push(digit);
        }
    };
    dbg!(found_digits);
}
