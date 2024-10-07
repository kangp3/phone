use std::error::Error;

use goertzel::ring;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let _ring = ring::ring_phone()?;
    loop {}
}

