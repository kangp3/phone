use std::error::Error;
use std::time::Duration;
use std::{panic, process};

#[cfg(feature = "wav")]
use goertzel::asyncutil::and_log_err;
use goertzel::phone::Phone;
use goertzel::ring;
#[cfg(feature = "wav")]
use hound;
#[cfg(feature = "wav")]
use pico_args::Arguments;
use tokio::time::sleep;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    // Grab file names from cmd line args
    #[cfg(feature = "wav")]
    let outfile: Option<String> = Arguments::from_env().opt_value_from_str("-o")?;

    // Set up panic hook to exit program
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |v| {
        default_hook(v);
        process::exit(1);
    }));

    let phone = Phone::new().await?;
    info!("Got mic, listening...");

    // TODO(peter): Re-think the WAV file writing. Doesn't work if we're hijacking SIGINT
    #[cfg(feature = "wav")]
    if let Some(fname) = outfile {
        let mut writer = hound::WavWriter::create(fname, hound::WavSpec{
            channels: 1,
            sample_rate: 48000,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        })?;

        let mut samples_ch = phone.audio_in_ch.subscribe();

        tokio::spawn(async move {
            and_log_err("wav_write", async {
                loop {
                    let sample = samples_ch.recv().await?;
                    writer.write_sample(sample)?;
                }
            }).await;
            writer.finalize().unwrap();
            info!("Finalized WAV writer");
        });
        info!("Started up WAV writer");
    }

    {
        let _ring = ring::ring_phone()?;
        sleep(Duration::from_secs(1)).await;
    }

    if let Err(e) = phone.begin_life().await {
        error!(e);
    }
    Ok(())
}
