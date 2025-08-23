use core::str;
use std::error::Error;
use std::net::SocketAddr;

use rsip::prelude::HeadersExt;
use rsip::{Method, SipMessage};
use tokio::net::UdpSocket;
use tokio::select;
use tokio::sync::{broadcast, mpsc};
use tracing::{debug, trace};

use crate::asyncutil::and_log_err;
use crate::sip::{ack_to, response_to, Txn, SERVER_ADDR};

use super::TXN_MAILBOXES;

const BUF_SIZE: usize = 4096;
const MESSAGE_CHANNEL_SIZE: usize = 64;

pub async fn bind() -> Result<
    (
        mpsc::Sender<(SocketAddr, SipMessage)>,
        broadcast::Sender<(Txn, SipMessage)>,
    ),
    Box<dyn Error>,
> {
    let socket = UdpSocket::bind("0.0.0.0:5060").await?;

    let (inbound_trx_send_ch, _) = broadcast::channel(MESSAGE_CHANNEL_SIZE);
    let (outbound_ch, mut outbound_recv) = mpsc::channel(MESSAGE_CHANNEL_SIZE);

    let outbound_ch2 = outbound_ch.clone();
    let inbound_trx_send_ch2 = inbound_trx_send_ch.clone();
    let txn_mailboxes = TXN_MAILBOXES.clone();
    tokio::spawn(and_log_err("sip inbound", async move {
        loop {
            let mut buf = vec![0u8; BUF_SIZE];
            select! {
                recv = socket.recv_from(&mut buf) => {
                    let (len, _) = recv.map_err(|e| format!("socket recv: {}", e))?;
                    buf.truncate(len);
                    // TODO(peter): Throw away messages if they don't try_from instead of crashing
                    let msg = SipMessage::try_from(str::from_utf8(&buf)?)?;
                    trace!("got message:\n{}", msg);
                    let call_id: String = msg.call_id_header()?.to_string();
                    let mut has_mailbox = false;
                    let mut txn_req = None;
                    {
                        let mailboxes = txn_mailboxes.read().await;
                        match mailboxes.get(&call_id) {
                            Some(mailbox) => {
                                trace!("has mailbox");
                                has_mailbox = true;
                                mailbox.send(msg.clone()).map_err(|e| format!("mailbox send: {}", e))?;
                            },
                            None => match msg {
                                SipMessage::Request(ref req) => match req.method() {
                                    Method::Invite => {
                                        trace!("make mailbox");
                                        txn_req = Some(req);
                                    },
                                    _ => trace!("no mailbox"),
                                },
                                SipMessage::Response(_) => trace!("no mailbox"),
                            },
                        }
                    }
                    if !has_mailbox {
                        match txn_req {
                            Some(req) => {
                                if inbound_trx_send_ch2.receiver_count() == 0 {
                                    let resp = response_to(req, rsip::StatusCode::BusyHere);
                                    trace!("sending busy resp");
                                    outbound_ch2.send(((*SERVER_ADDR).clone(), resp)).await.map_err(|e| format!("busy send: {}", e))?;
                                } else {
                                    let txn = {
                                        let mailboxes = txn_mailboxes.write().await;
                                        match Txn::from_req(req.clone(), outbound_ch2.clone(), mailboxes) {
                                            Ok(txn) => Some(txn),
                                            Err(e) if e.to_string() == "mailbox already exists in map" => None,
                                            Err(e) => Err(e)?,
                                        }
                                    };
                                    if let Some(txn) = txn {
                                        if let Err(_) = inbound_trx_send_ch2.send((txn, msg.clone())) {
                                            let resp = response_to(req, rsip::StatusCode::BusyHere);
                                            trace!("sending busy resp");
                                            outbound_ch2.send(((*SERVER_ADDR).clone(), resp)).await.map_err(|e| format!("busy send 2: {}", e))?;
                                        }
                                    } else {
                                        let mailboxes = txn_mailboxes.read().await;
                                        match mailboxes.get(&call_id) {
                                            Some(mailbox) => {
                                                mailbox.send(msg.clone()).map_err(|e| format!("send to mailbox: {}", e))?;
                                                debug!("SENT INVITE");
                                            },
                                            None => Err("should have a mailbox by now")?,
                                        };
                                    }
                                }
                            },
                            None => match msg {
                                SipMessage::Request(ref req) => match req.method {
                                    Method::Ack => {},
                                    Method::Bye |
                                    Method::Options => {
                                        trace!("sending ok resp");
                                        let resp = response_to(req, rsip::StatusCode::OK);
                                        outbound_ch2.send(((*SERVER_ADDR).clone(), resp)).await.map_err(|e| format!("ok send: {}", e))?;
                                    },
                                    Method::Cancel => {
                                        trace!("sending ok resp");
                                        let resp = response_to(req, rsip::StatusCode::OK);
                                        outbound_ch2.send(((*SERVER_ADDR).clone(), resp)).await.map_err(|e| format!("ok send 2: {}", e))?;
                                        trace!("sending terminated resp");
                                        let resp = response_to(req, rsip::StatusCode::RequestTerminated);
                                        outbound_ch2.send(((*SERVER_ADDR).clone(), resp)).await.map_err(|e| format!("terminated send: {}", e))?;
                                    },
                                    method => Err(format!("unexpected req with method {}", method))?,
                                },
                                SipMessage::Response(ref resp) => match resp.cseq_header()?.method()? {
                                    Method::Invite if resp.status_code().code() >= 200 => {
                                        trace!("sending ack req");
                                        let ack = ack_to(resp);
                                        outbound_ch2.send(((*SERVER_ADDR).clone(), ack)).await.map_err(|e| format!("ack send: {}", e))?;
                                    },
                                    _ => {},
                                },
                            },
                        }
                    }
                },
                send = outbound_recv.recv() => {
                    let (addr, msg) = send.ok_or("socket send channel closed")?;
                    trace!("sent message:\n{}", msg);
                    let msg_bytes: Vec<u8> = msg.into();
                    let len = socket.send_to(&msg_bytes, addr).await.map_err(|e| format!("socket send: {}", e))?;
                    (len == msg_bytes.len()).then_some(()).ok_or("byte len does not match")?;
                },
            }
        }
    }));

    Ok((outbound_ch, inbound_trx_send_ch))
}
