use core::str;
use std::error::Error;
use std::net::SocketAddr;

use rsip::{Method, SipMessage};
use tokio::net::UdpSocket;
use tokio::select;
use tokio::sync::mpsc;
use tracing::debug;

use crate::asyncutil::and_log_err;
use crate::sip::Txn;

use super::TXN_MAILBOXES;


const BUF_SIZE: usize = 4096;
const MESSAGE_CHANNEL_SIZE: usize = 64;

pub async fn bind() -> Result<(mpsc::Sender<(SocketAddr, SipMessage)>, mpsc::Receiver<Txn>), Box<dyn Error>> {
    let socket = UdpSocket::bind("0.0.0.0:5060").await?;

    let (inbound_trx_send_ch, inbound_trx_recv_ch) = mpsc::channel(MESSAGE_CHANNEL_SIZE);
    let (outbound_ch, mut outbound_recv) = mpsc::channel(MESSAGE_CHANNEL_SIZE);

    let outbound_ch2 = outbound_ch.clone();
    let txn_mailboxes = TXN_MAILBOXES.clone();
    tokio::spawn(and_log_err("sip inbound", async move {
        loop {
            let mut buf = vec![0u8; BUF_SIZE];
            select! {
                recv = socket.recv_from(&mut buf) => {
                    let (len, _) = recv?;
                    buf.truncate(len);
                    // TODO(peter): Throw away messages if they don't try_from instead of crashing
                    let msg = SipMessage::try_from(str::from_utf8(&buf)?)?;
                    debug!("GOT MESSAGE: {}", msg);
                    let headers = match msg {
                        SipMessage::Request(ref req) => &req.headers,
                        SipMessage::Response(ref resp) => &resp.headers,
                    };
                    let mut call_id = None;
                    for header in headers.iter() {
                        match &header {
                            rsip::Header::CallId(h) => {
                                call_id = Some((*h).to_string());
                                break;
                            }
                            _ => {},
                        }
                    }
                    // TODO(peter): Throw away messages if they don't try_from instead of crashing
                    let call_id = call_id.ok_or("missing call id")?;
                    let mut should_create_txn = false;
                    {
                        let mailboxes = txn_mailboxes.read().await;
                        match mailboxes.get(&call_id) {
                            Some(mailbox) => { mailbox.try_send(msg.clone())?; }
                            None => match msg {
                                SipMessage::Request(ref req) => match req.method() {
                                    Method::Invite => should_create_txn = true,
                                    _ => Err(format!("got non-invite request with no active txn: {}", req))?,
                                },
                                SipMessage::Response(_) => Err("got a response with no active txn")?,
                            }
                        }
                    }
                    if should_create_txn {
                        let txn = if let SipMessage::Request(ref req) = msg {
                            let mailboxes = txn_mailboxes.write().await;
                            match Txn::from_req(req.clone(), outbound_ch2.clone(), mailboxes) {
                                Ok(txn) => Some(txn),
                                Err(e) => {
                                    if e.to_string() == "mailbox already exists in map" {
                                        None
                                    } else {
                                        Err(e)?
                                    }
                                }
                            }
                        } else { None };
                        {
                            let mailboxes = txn_mailboxes.read().await;
                            match mailboxes.get(&call_id) {
                                Some(mailbox) => mailbox.send(msg).await?,
                                None => Err("should have a mailbox by now")?,
                            };
                        }
                        if let Some(txn) = txn {
                            // TODO(peter): Handle sending back busy tones if the channel is not
                            // listening
                            inbound_trx_send_ch.send(txn).await?;
                        }
                    }
                },
                send = outbound_recv.recv() => {
                    let (addr, msg) = send.ok_or("socket send channel closed")?;
                    debug!("SENT MESSAGE: {}", msg);
                    let msg_bytes: Vec<u8> = msg.into();
                    let len = socket.send_to(&msg_bytes, addr).await?;
                    (len == msg_bytes.len()).then_some(()).ok_or("byte len does not match")?;
                },
            }
        }
    }));

    Ok((outbound_ch, inbound_trx_recv_ch))
}
