use std::error::Error;
use std::time::Duration;
use std::{panic, process};

use goertzel::phone::Phone;
use goertzel::ring;
use tokio::time::sleep;
use tracing::{error, info};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    // Set up panic hook to exit program
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |v| {
        default_hook(v);
        process::exit(1);
    }));

    let phone = Phone::new().await?;
    info!("Got mic, listening...");

    {
        let _ring = ring::ring_phone()?;
        sleep(Duration::from_secs(1)).await;
    }

    if let Err(e) = phone.begin_life().await {
        error!(e);
    }
    Ok(())
}
