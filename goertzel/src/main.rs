use std::error::Error;
use std::{panic, process};

#[cfg(feature = "wav")]
use goertzel::asyncutil::and_log_err;
use goertzel::deco;
use goertzel::hook::SwitchHook;
use goertzel::phone::Phone;
#[cfg(feature = "wav")]
use hound;
#[cfg(feature = "wifi")]
use tokio::process::Command;
#[cfg(feature = "wav")]
use pico_args::Arguments;
use tracing::{debug, info};
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

    let mut phone = Phone::new().await?;
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

    let pulse_ch = phone.pulse_ch.subscribe();
    let goertzel_ch = phone.goertz_ch.subscribe();
    let mut chars_ch = deco::ding(goertzel_ch, pulse_ch);

    phone.begin_life().await?;

    let mut ssid = String::new();
    let mut pass = String::new();
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
