use std::error::Error;

use goertzel::audio;
use tokio::net::UdpSocket;
use tokio::select;
use tokio::sync::broadcast;
use tracing::info;
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
    let sock = UdpSocket::bind("0.0.0.0:19513").await?;
    sock.connect("127.0.0.1:19514").await?;

    info!("GETTING MIC");
    let (mic_ch, _mic_stream, _) = audio::get_input_channel()?;

    info!("WAITING FOR ACK");
    let mut ack_buf = vec![0; BUF_SIZE];
    _ = sock.recv(&mut ack_buf).await;

    info!("SET UP AUDIO CH");
    let mut audio_in_ch = mic_ch.subscribe();

    let mut out_buf = vec![0; BUF_SIZE];
    let mut out_idx = 0;
    info!("BEGINNING LOOP");
    loop {
        select! {
            sample = audio_in_ch.recv() => {
                let bytes = ((sample? * 2.0_f32.powi(15)) as i16).to_be_bytes();
                out_buf[out_idx] = bytes[0];
                out_buf[out_idx+1] = bytes[1];
                out_idx += 2;
                if out_idx == BUF_SIZE {
                    sock.send(&out_buf).await?;
                    out_idx = 0;
                }
            },
        }
    }
}
