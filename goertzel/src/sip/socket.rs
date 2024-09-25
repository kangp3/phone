use core::str;
use std::error::Error;
use std::net::SocketAddr;

use rsip::SipMessage;
use tokio::net::UdpSocket;
use tokio::select;
use tokio::sync::{broadcast, mpsc};

use crate::asyncutil::and_log_err;


const BUF_SIZE: usize = 4096;
const MESSAGE_CHANNEL_SIZE: usize = 64;

pub async fn bind() -> Result<(mpsc::Sender<(SocketAddr, SipMessage)>, broadcast::Sender<(SocketAddr, SipMessage)>), Box<dyn Error>> {
    let socket = UdpSocket::bind("0.0.0.0:5060").await?;

    let (inbound_ch, _) = broadcast::channel(MESSAGE_CHANNEL_SIZE);
    let (outbound_ch, mut outbound_recv) = mpsc::channel(MESSAGE_CHANNEL_SIZE);

    let inbound_ch2 = inbound_ch.clone();
    tokio::spawn(and_log_err("sip inbound", async move {
        loop {
            let mut buf = vec![0u8; BUF_SIZE];
            select! {
                recv = socket.recv_from(&mut buf) => {
                    let (len, addr) = recv?;
                    buf.truncate(len);
                    let msg = SipMessage::try_from(str::from_utf8(&buf)?)?;
                    let _ = inbound_ch2.send((addr, msg));
                },
                send = outbound_recv.recv() => {
                    let (addr, msg): (SocketAddr, SipMessage) = send.ok_or("socket send channel closed")?;
                    let msg_bytes: Vec<u8> = msg.into();
                    let len = socket.send_to(&msg_bytes, addr).await?;
                    (len == msg_bytes.len()).then_some(()).ok_or("byte len does not match")?;
                },
            }
        }
    }));

    Ok((outbound_ch, inbound_ch))
}
