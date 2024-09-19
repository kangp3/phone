use std::error::Error;
use std::ops::Range;
use std::{panic, process};

use goertzel::{self, hook};
use tokio::process::Command;
#[cfg(feature = "wav")]
use pico_args::Arguments;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};


const SAMPLE_RATE: u32 = 48000;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    // Grab file names from cmd line args
    #[cfg(feature = "wav")]
    let (
        infile,
        outfile,
        sample_range,
    ): (Option<String>, Option<String>, Option<Range<u32>>) = {
        let mut args = Arguments::from_env();
        (
            args.opt_value_from_str("-f")?,
            args.opt_value_from_str("-o")?,
            {
                if let (Some(start_sample), Some(end_sample)) = (args.opt_value_from_str("-s")?, args.opt_value_from_str("-e")?) {
                    Some(start_sample..end_sample)
                } else {
                    None
                }
            }
        )
    };

    // Set up panic hook to exit program
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |v| {
        default_hook(v);
        process::exit(1);
    }));

    // Set up the SHK GPIO pin (or ctrlc on non-Rasbperry Pi)
    let _pin = hook::try_register_shk()?;

    // Get the audio source (WAV file or mic)
    #[cfg(feature = "wav")]
    let mic = {
        if let Some(fname) = infile {
            goertzel::audio::get_wav_samples(fname, sample_range)
        } else if let Some(fname) = outfile {
            goertzel::audio::get_mic_samples_with_outfile(SAMPLE_RATE, fname)
        } else {
            goertzel::audio::get_mic_samples(SAMPLE_RATE)
        }
    };
    #[cfg(not(feature = "wav"))]
    let mic = goertzel::audio::get_mic_samples(SAMPLE_RATE);
    info!("Got mic, listening...");

    let mut ssid = String::new();
    let mut pass = String::new();
    let mut chars_ch = goertzel::deco::ding(mic.samples_ch);
    while let Some(c) = chars_ch.recv().await {
        if c == '\0' {
            break;
        }
        info!("{}", &c);
        ssid.push(c);
    }
    info!("{}", &ssid);
    while let Some(c) = chars_ch.recv().await {
        if c == '\0' {
            break;
        }
        info!("{}", &c);
        pass.push(c);
    }
    // TODO: Delete debugs
    info!("{}", &pass);

    #[cfg(target_os = "linux")]
    let status = Command::new("nmcli")
        .args(&["--wait", "20"])
        .args(&["device", "wifi"])
        .arg("connect")
        .arg(&ssid)
        .args(&["password", &pass])
        .spawn()?
        .wait()
        .await?;
    #[cfg(target_os = "macos")]
    let status = Command::new("networksetup")
        .arg("-setairportnetwork")
        .arg("en0")
        .arg(&ssid)
        .arg(&pass)
        .spawn()?
        .wait()
        .await?;
    if !status.success() {
        panic!("Failed to connect to Wi-Fi");
    }
    Ok(())
}
