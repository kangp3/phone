use std::collections::HashMap;
use std::error::Error;
use std::net::Ipv4Addr;
use std::sync::Arc;

use rsip::prelude::{HeadersExt, UntypedHeader};
use rsip::{HostWithPort, SipMessage};
use rustls::pki_types::ServerName;
use rustls::RootCertStore;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, RwLock};
use tokio_rustls::TlsConnector;
use tracing::warn;
use uuid::Uuid;

use crate::asyncutil::and_log_err;

use super::Dialog;

const MESSAGE_CHANNEL_SIZE: usize = 64;

pub struct TlsSipConn {
    client_ip: Ipv4Addr,
    sip_instance_uuid: Uuid,

    pub host: String,
    pub port: u16,

    pub tx_ch: mpsc::Sender<SipMessage>,
    pub new_msg_ch: mpsc::Receiver<SipMessage>,

    dialogs: Arc<RwLock<HashMap<String, mpsc::Sender<SipMessage>>>>,
}

impl TlsSipConn {
    pub async fn new(client_ip: Ipv4Addr, host: &str, port: u16) -> Result<Self, Box<dyn Error>> {
        let sip_instance_uuid = Uuid::new_v4();

        let dialogs = Arc::new(RwLock::new(
            HashMap::<String, mpsc::Sender<SipMessage>>::new(),
        ));

        let (send_send_ch, mut send_recv_ch) = mpsc::channel(MESSAGE_CHANNEL_SIZE);
        let (new_msg_send_ch, new_msg_recv_ch) = mpsc::channel(MESSAGE_CHANNEL_SIZE);

        let conn = TlsSipConn {
            client_ip,
            sip_instance_uuid: sip_instance_uuid.clone(),

            host: String::from(host),
            port,

            tx_ch: send_send_ch.clone(),
            new_msg_ch: new_msg_recv_ch,

            dialogs: dialogs.clone(),
        };

        let connector = get_tls_connector();
        let sock = TcpStream::connect((host, port)).await?;
        let stream = connector
            .connect(ServerName::try_from(host)?.to_owned(), sock)
            .await?;
        let (recv_stream, mut send_stream) = tokio::io::split(stream);

        // TODO: Drop handlers for these coroutines
        let dialogs_ref = dialogs.clone();
        let new_msg_send_ch = new_msg_send_ch.clone();
        tokio::spawn(and_log_err("tls sip recv", async move {
            let mut lines = BufReader::new(recv_stream).lines();
            let mut msg_str = String::new();
            while let Some(line) = lines.next_line().await? {
                msg_str.push_str(&line);
                msg_str.push_str("\r\n");
                if line.is_empty() {
                    // TODO: Just emit the message and build a layer that
                    // consumes SIP messages and routes them to dialogs
                    let msg = SipMessage::try_from(msg_str.clone())?;
                    let call_id = msg.call_id_header()?.value().to_string();

                    let rx_send_ch = {
                        let dialogs_handle = dialogs_ref.read().await;
                        if let Some(dialog) = dialogs_handle.get(&call_id) {
                            (*dialog).clone()
                        } else {
                            new_msg_send_ch.clone()
                        }
                    };

                    rx_send_ch.send(msg).await?;
                    msg_str.clear();
                }
            }
            Err("broke out of the lines loop".into()) // TODO: Maybe retry on this error
        }));

        tokio::spawn(and_log_err("tls sip send", async move {
            loop {
                match send_recv_ch.recv().await {
                    Some(msg) => {
                        let call_id = msg.call_id_header()?.value().to_string();

                        {
                            let dialogs_handle = dialogs.read().await;
                            if !dialogs_handle.contains_key(&call_id) {
                                warn!("Message sent on unknown dialog {}", &call_id);
                            }
                        }

                        send_stream.write_all(msg.to_string().as_bytes()).await?;
                    }
                    None => return Err("got a none on the send".into()), // TODO: Maybe retry on this error
                }
            }
        }));

        Ok(conn)
    }

    pub async fn dialog(&self, username: String) -> Dialog {
        let host_with_port = HostWithPort::from((self.host.clone(), self.port));

        let (rx_send_ch, rx_recv_ch) = mpsc::channel(MESSAGE_CHANNEL_SIZE);
        let dialog = Dialog::new(
            host_with_port,
            self.client_ip,
            self.sip_instance_uuid,
            username,
            self.tx_ch.clone(),
            rx_recv_ch,
        );
        {
            let mut dialogs_handle = self.dialogs.write().await;
            dialogs_handle.insert(dialog.call_id.value().to_string(), rx_send_ch);
        }
        dialog
    }

    pub async fn dialog_from_req(&self, msg: &SipMessage) -> Result<Dialog, Box<dyn Error>> {
        let (rx_send_ch, rx_recv_ch) = mpsc::channel(MESSAGE_CHANNEL_SIZE);
        let dialog = Dialog::from_request(
            (self.host.clone(), self.port).into(),
            self.client_ip,
            self.sip_instance_uuid,
            self.tx_ch.clone(),
            rx_recv_ch,
            &msg,
        )?;
        rx_send_ch.send(msg.clone()).await?;
        {
            let mut dialogs_handle = self.dialogs.write().await;
            dialogs_handle.insert(dialog.call_id.value().to_string(), rx_send_ch);
        }
        Ok(dialog)
    }
}

fn get_tls_connector() -> TlsConnector {
    let root_store = RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let tls_config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    TlsConnector::from(Arc::new(tls_config))
}
