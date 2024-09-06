
use std::{panic, process};

use goertzel::{self, hook};
use tokio::process::Command;


const SAMPLE_RATE: u32 = 48000;


#[tokio::main]
async fn main() {
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |v| {
        default_hook(v);
        process::exit(1);
    }));

    let _pin = hook::try_register_shk().unwrap();

    let mic = goertzel::audio::get_input_samples(SAMPLE_RATE);
    dbg!("Got mic, listening...");

    let mut ssid = String::new();
    let mut pass = String::new();
    let mut chars_ch = goertzel::deco::ding(mic.samples_ch);
    while let Some(c) = chars_ch.recv().await {
        if c == '\0' {
            break;
        }
        dbg!(&c);
        ssid.push(c);
    }
    dbg!(&ssid);
    while let Some(c) = chars_ch.recv().await {
        if c == '\0' {
            break;
        }
        dbg!(&c);
        pass.push(c);
    }
    // TODO: Delete debugs
    dbg!(&pass);

    #[cfg(target_os = "none")]
    let status = Command::new("nmcli")
        .args(&["--wait", "20"])
        .args(&["device", "wifi"])
        .arg("connect")
        .arg(&ssid)
        .args(&["password", &pass])
        .spawn()
        .unwrap()
        .wait()
        .await
        .unwrap();
    #[cfg(target_os = "macos")]
    let status = Command::new("networksetup")
        .arg("-setairportnetwork")
        .arg("en0")
        .arg(&ssid)
        .arg(&pass)
        .spawn()
        .unwrap()
        .wait()
        .await
        .unwrap();
    if !status.success() {
        panic!("Failed to connect to Wi-Fi");
    }
}
