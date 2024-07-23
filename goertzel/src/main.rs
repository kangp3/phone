use std::fs::File;
use std::io::BufReader;

use hound;
use hound::WavSamples;
use itertools::Itertools;
use itertools::Chunk;


const WINDOW_INTERVAL: usize = 3000;
const CHUNK_SIZE: usize = 3000;
const COEFFS: [f64;7] = [
    1.9915137618771384,
    1.9899020339626003,
    1.9876909939485932,
    1.9847500449657665,
    1.9747170780302008,
    1.9691286690584107,
    1.9629874688668658,
];


fn goertzel_me(samples: &Vec<i32>, mut q1: f64, mut q2: f64, coeff: f64) -> (f64, f64, f64, f64){
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
    let MAG_THRESHOLD: f64 = 23.5_f64.exp();

    let mut reader = hound::WavReader::open("/Users/kangp3/Documents/projects/phone/audio_samples/cortelco_48k.wav").unwrap();
    let samples = reader.samples::<i32>();

    let mut found_digits = vec![];
    for chunk in samples.chunks(WINDOW_INTERVAL).into_iter() {
        let chunk = chunk.collect::<Result<Vec<i32>, _>>().unwrap();
        let (mag_0, _, _, _) = goertzel_me(&chunk, 0.0, 0.0, COEFFS[0]);
        let (mag_4, _, _, _) = goertzel_me(&chunk, 0.0, 0.0, COEFFS[4]);
        if mag_0 > MAG_THRESHOLD && mag_4 > MAG_THRESHOLD {
            found_digits.push(1);
        }
    };
    dbg!(found_digits);
}
