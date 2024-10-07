use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, LazyLock};
use std::time::Duration;

use tokio::net::UdpSocket;
use tokio::sync::{broadcast, mpsc};
use tokio::task::AbortHandle;
use tokio::time::sleep;
use tracing::{debug, warn};

use crate::asyncutil::and_log_err;


const SAMPLES_PER_BUF: usize = 960;  // 20ms of samples @ 48k
const BUF_SIZE: usize = 2 * SAMPLES_PER_BUF;

static NET_ADDR: LazyLock<Ipv4Addr> = LazyLock::new(|| "10.100.0.0".parse().unwrap());
const NET_MASK: u32 = 0xffff0000;

#[derive(Clone)]
pub struct Socket {
    sock: Arc<UdpSocket>,
    in_handle: Option<AbortHandle>,
    out_handle: Option<AbortHandle>,

    remote: Option<SocketAddr>
}

impl Socket {
    pub async fn bind() -> Result<Self, Box<dyn Error>> {
        let sock = UdpSocket::bind("0.0.0.0:19512").await?;
        Ok(Self{
            sock: Arc::new(sock),
            in_handle: None,
            out_handle: None,

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

    pub async fn connect(&mut self, addr: SocketAddr, mut audio_in: broadcast::Receiver<i16>, audio_out: mpsc::Sender<i16>) -> Result<(), Box<dyn Error>> {
        self.remote = Some(addr);
        debug!("rtp: connecting to remote at {}", addr);
        self.sock.connect(addr).await?;
        debug!("rtp: connected to remote");

        let sock = self.sock.clone();
        let in_handle = tokio::spawn(and_log_err(format!("rtp recv socket {}", addr), async move {
            let mut got_first_packet = false;

            let mut buf = vec![0; BUF_SIZE];
            loop {
                let n = sock.recv(&mut buf).await?;
                if !got_first_packet {
                    sleep(Duration::from_millis(30)).await;
                    got_first_packet = true;
                }
                if n != buf.len() {
                    warn!("got unexpected packet length: {}", n);
                }
                for buf_idx in (0..buf.len()).step_by(2) {
                    let sample = i16::from_be_bytes([buf[buf_idx], buf[buf_idx+1]]);
                    audio_out.send(sample).await?;
                }
            }
        })).abort_handle();
        self.in_handle = Some(in_handle);

        let sock = self.sock.clone();
        let out_handle = tokio::spawn(and_log_err(format!("rtp send socket {}", addr), async move {
            let mut buf = vec![0; BUF_SIZE];
            let mut buf_idx = 0;
            loop {
                let sample = audio_in.recv().await?;
                let bytes = sample.to_be_bytes();
                buf[buf_idx] = bytes[0];
                buf[buf_idx+1] = bytes[1];
                buf_idx += 2;
                if buf_idx == BUF_SIZE {
                    sock.send(&buf).await?;
                    buf_idx = 0;
                }
            }
        })).abort_handle();
        self.out_handle = Some(out_handle);

        Ok(())
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        debug!("ope i dropped an RTP sock 🧦 connected to {}", self.remote.unwrap_or("0.0.0.0:0".parse().unwrap()));
        match &self.in_handle {
            Some(handle) => {
                debug!("aborting in task");
                handle.abort();
            },
            None => {},
        }
        match &self.out_handle {
            Some(handle) => {
                debug!("aborting out task");
                handle.abort();
            },
            None => {},
        }
    }
}
