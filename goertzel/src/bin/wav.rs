use std::error::Error;

use goertzel::dtmf::{goertzeliter, CHUNK_SIZE};
use pico_args::Arguments;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};


const CORRECT_DIGS: [u8; 67] = [1,7,7,7,3,3,2,2,2,8,8,7,7,7,12,7,7,7,7,3,3,1,1,0,1,2,2,2,3,3,6,6,8,3,3,7,7,7,0,6,6,3,3,8,8,8,3,3,7,7,7,4,7,7,7,2,3,8,8,2,8,3,3,1,1,2,0];


fn main() -> Result<(), Box<dyn Error>>{
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let mut args = Arguments::from_env();
    let infile = args.value_from_str("-f")?;
    let start_idx = args.opt_value_from_str("-s")?.map(|i: u32| i / CHUNK_SIZE as u32 * CHUNK_SIZE as u32);
    let end_idx = args.opt_value_from_str("-e")?.map(|i: u32| (i / CHUNK_SIZE as u32 + 1) * CHUNK_SIZE as u32);

    let samples = goertzel::audio::get_wav_samples(infile, start_idx, end_idx);
    let digs = goertzeliter(samples)?;

    info!("{}", digs.into_iter().map(|d| format!("{}", d)).collect::<String>());
    info!("{}", CORRECT_DIGS.into_iter().map(|d| format!("{}", d)).collect::<String>());

    Ok(())
}
