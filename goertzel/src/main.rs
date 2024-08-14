
use goertzel;
use tokio::process::Command;


const SAMPLE_RATE: u32 = 48000;


#[tokio::main]
async fn main() {
    let mic = goertzel::audio::get_input_samples(SAMPLE_RATE);

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

    Command::new("nmcli")
        .args(&["--wait", "5"])
        .args(&["device", "wifi"])
        .arg("connect")
        .arg(&ssid)
        .args(&["password", &pass])
        .spawn()
        .unwrap()
        .wait()
        .await
        .unwrap();
}
