use std::error::Error;
use std::time::Duration;

use goertzel::audio;
use tokio::net::UdpSocket;
use tokio::select;
use tokio::time::sleep;
use tracing::{info, warn};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};


const SAMPLES_PER_BUF: usize = 960;  // 20ms of samples @ 48k
const BUF_SIZE: usize = 2 * SAMPLES_PER_BUF;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    info!("BINDING SOCK");
    let sock = UdpSocket::bind("0.0.0.0:19514").await?;
    sock.connect("127.0.0.1:19513").await?;

    info!("SENDING ACK");
    let ack_buf = vec![0; BUF_SIZE];
    sock.send(&ack_buf).await?;

    info!("SET UP AUDIO CH");
    let (spk_ch, _spk_stream, spk_cfg) = audio::get_output_channel()?;
    info!("SPK CFG: {}", spk_cfg.channels());

    let mut got_first_packet = false;
    let mut in_buf = vec![0; BUF_SIZE];
    info!("BEGINNING LOOP");
    loop {
        select! {
            n = sock.recv(&mut in_buf) => {
                if !got_first_packet {
                    sleep(Duration::from_millis(30)).await;
                    got_first_packet = true;
                }
                let n = n?;
                if n != in_buf.len() {
                    warn!("got unexpected packet length: {}", n);
                }
                for buf_idx in (0..in_buf.len()).step_by(2) {
                    let sample = i16::from_be_bytes([in_buf[buf_idx], in_buf[buf_idx+1]]);
                    let sample = sample as f32 / 2.0_f32.powi(15);
                    spk_ch.send(sample).await?;
                    spk_ch.send(sample).await?;
                }
            },
        }
    }
}
