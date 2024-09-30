use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, LazyLock};
use std::time::Duration;

use tokio::net::UdpSocket;
use tokio::select;
use tokio::sync::{broadcast, mpsc};
use tokio::task::AbortHandle;
use tokio::time::sleep;
use tracing::{debug, trace, warn};

use crate::asyncutil::and_log_err;


const SAMPLES_PER_BUF: usize = 960;  // 20ms of samples @ 48k
const BUF_SIZE: usize = 2 * SAMPLES_PER_BUF;

static NET_ADDR: LazyLock<Ipv4Addr> = LazyLock::new(|| "192.168.12.0".parse().unwrap());
const NET_MASK: u32 = 0xffffff00;

#[derive(Clone)]
pub struct Socket {
    sock: Arc<UdpSocket>,
    handle: Option<AbortHandle>,

    remote: Option<SocketAddr>
}

impl Socket {
    pub async fn bind() -> Result<Self, Box<dyn Error>> {
        let sock = UdpSocket::bind("0.0.0.0:19512").await?;
        Ok(Self{
            sock: Arc::new(sock),
            handle: None,

            remote: None,
        })
    }

    pub async fn port(&self) -> Result<u16, Box<dyn Error>> {
        Ok(self.sock.local_addr().map(|addr| addr.port())?)
    }

    pub fn is_in_net(&self, ip: IpAddr) -> bool {
        match ip {
            IpAddr::V4(ip) => ip.to_bits() & NET_MASK == (*NET_ADDR).to_bits(),
            IpAddr::V6(_) => false,
        }
    }

    pub async fn connect(&mut self, addr: SocketAddr, mut audio_in: broadcast::Receiver<f32>, audio_out: mpsc::Sender<f32>, n_channels: u16) -> Result<(), Box<dyn Error>> {
        self.remote = Some(addr);
        debug!("rtp: connecting to remote at {}", addr);
        self.sock.connect(addr).await?;
        debug!("rtp: connected to remote");

        let sock = self.sock.clone();
        let handle = tokio::spawn(and_log_err(format!("rtp socket {}", addr), async move {
            let mut got_first_packet = false;

            let mut in_buf = vec![0; BUF_SIZE];

            let mut out_buf = vec![0; BUF_SIZE];
            let mut out_idx = 0;
            debug!("rtp: starting the loop");
            loop {
                select! {
                    n = sock.recv(&mut in_buf) => {
                        if !got_first_packet {
                            debug!("rtp: got my first packet!");
                            sleep(Duration::from_millis(30)).await;
                            got_first_packet = true;
                        }
                        let n = n?;
                        if n != in_buf.len() {
                            warn!("got unexpected packet length: {}", n);
                        }
                        debug!("rtp: processing buffer!");
                        for buf_idx in (0..in_buf.len()).step_by(2) {
                            let sample = i16::from_be_bytes([in_buf[buf_idx], in_buf[buf_idx+1]]);
                            let sample = sample as f32 / 2.0_f32.powi(15);
                            for _ in 0..n_channels {
                                audio_out.send(sample).await?;
                            }
                        }
                    },
                    sample = audio_in.recv() => {
                        let bytes = ((sample? * 2.0_f32.powi(15)) as i16).to_be_bytes();
                        out_buf[out_idx] = bytes[0];
                        out_buf[out_idx+1] = bytes[1];
                        out_idx += 2;
                        if out_idx == BUF_SIZE {
                            debug!("rtp: sent a packet!");
                            sock.send(&out_buf).await?;
                            out_idx = 0;
                        }
                    },
                }
            }
        })).abort_handle();
        self.handle = Some(handle);

        Ok(())
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        debug!("ope i dropped an RTP sock 🧦 connected to {}", self.remote.unwrap_or("0.0.0.0:0".parse().unwrap()));
        match &self.handle {
            Some(handle) => {
                debug!("aborting task");
                handle.abort();
            },
            None => {},
        }
    }
}
