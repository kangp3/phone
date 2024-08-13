use goertzel;


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
        ssid.push(c);
    }
    while let Some(c) = chars_ch.recv().await {
        if c == '\0' {
            break;
        }
        pass.push(c);
    }
    dbg!(ssid);
    dbg!(pass);
}
