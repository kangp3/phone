use goertzel;


const SAMPLE_RATE: u32 = 48000;


#[tokio::main]
async fn main() {
    let mic = goertzel::audio::get_input_samples(SAMPLE_RATE);

    let mut digs_ch = goertzel::dtmf::goertzelme(mic.samples_ch);
    while let Some(dig) = digs_ch.recv().await {
        dbg!(dig);
    }
}
