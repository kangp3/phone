use std::error::Error;
use std::{panic, process};

use goertzel::hook::SwitchHook;
use goertzel::{self, hook, pulse};
#[cfg(feature = "wifi")]
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
    let outfile: Option<String> = Arguments::from_env().opt_value_from_str("-o")?;

    // Set up panic hook to exit program
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |v| {
        default_hook(v);
        process::exit(1);
    }));

    // Set up the SHK GPIO pin (or ctrlc on non-Raspberry Pi)
    let (_shk_pin, _shk_send_ch, shk_recv_ch) = hook::try_register_shk()?;
    let (notgoertzel_ch, mut hangup_ch) = pulse::notgoertzelme(shk_recv_ch);

    tokio::spawn(async move {
        while let Ok(shk_evt) = hangup_ch.recv().await {
            if shk_evt == SwitchHook::ON {
                info!("PHONE SLAM");
            }
        }
    });

    // Get the audio source (WAV file or mic)
    #[cfg(feature = "wav")]
    let mic = {
        if let Some(fname) = outfile {
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
    let mut chars_ch = goertzel::deco::ding(mic.samples_ch, notgoertzel_ch);
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

    #[cfg(all(target_os = "linux", feature = "wifi"))]
    let status = Command::new("nmcli")
        .args(&["--wait", "20"])
        .args(&["device", "wifi"])
        .arg("connect")
        .arg(&ssid)
        .args(&["password", &pass])
        .spawn()?
        .wait()
        .await?;
    #[cfg(all(target_os = "macos", feature = "wifi"))]
    let status = Command::new("networksetup")
        .arg("-setairportnetwork")
        .arg("en0")
        .arg(&ssid)
        .arg(&pass)
        .spawn()?
        .wait()
        .await?;
    #[cfg(feature = "wifi")]
    if !status.success() {
        panic!("Failed to connect to Wi-Fi");
    }
    Ok(())
}
