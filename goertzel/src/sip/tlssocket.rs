use std::collections::HashMap;
use std::error::Error;
use std::net::Ipv4Addr;
use std::sync::Arc;

use rsip::prelude::{HeadersExt, UntypedHeader};
use rsip::SipMessage;
use rustls::pki_types::ServerName;
use rustls::RootCertStore;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::{broadcast, mpsc, RwLock};
use tokio_rustls::TlsConnector;

use crate::asyncutil::and_log_err;

use super::Dialog;

const MESSAGE_CHANNEL_SIZE: usize = 64;

pub struct TlsSipConn {
    client_ip: Ipv4Addr,
    pub host: String,
    pub port: u16,

    pub tx_ch: mpsc::Sender<SipMessage>,
    pub dialog_ch: broadcast::Receiver<Dialog>,

    dialogs: Arc<RwLock<HashMap<String, Dialog>>>,
}

impl TlsSipConn {
    pub async fn new(client_ip: Ipv4Addr, host: &str, port: u16) -> Result<Self, Box<dyn Error>> {
        let dialogs = Arc::new(RwLock::new(HashMap::<String, Dialog>::new()));

        let (send_send_ch, mut send_recv_ch) = mpsc::channel(MESSAGE_CHANNEL_SIZE);
        let (dialog_send_ch, dialog_recv_ch) = broadcast::channel(MESSAGE_CHANNEL_SIZE);

        let conn = TlsSipConn {
            client_ip,
            host: String::from(host),
            port,

            tx_ch: send_send_ch.clone(),
            dialog_ch: dialog_recv_ch,

            dialogs: dialogs.clone(),
        };

        let connector = get_tls_connector();
        let sock = TcpStream::connect((host, port)).await?;
        let stream = connector
            .connect(ServerName::try_from(host)?.to_owned(), sock)
            .await?;
        let (recv_stream, mut send_stream) = tokio::io::split(stream);

        let dialogs_ref = dialogs.clone();
        let tx_ch = send_send_ch.clone();
        let dialog_send_ch = dialog_send_ch.clone();
        tokio::spawn(and_log_err("tls sip recv", async move {
            let mut lines = BufReader::new(recv_stream).lines();
            let mut msg_str = String::new();
            while let Some(line) = lines.next_line().await? {
                msg_str.push_str(&line);
                msg_str.push_str("\r\n");
                if line.is_empty() {
                    let msg = SipMessage::try_from(msg_str.clone())?;
                    let call_id = msg.call_id_header()?.value().to_string();

                    let rx_send_ch = {
                        let mut dialogs_handle = dialogs_ref.write().await;
                        if let Some(dialog) = dialogs_handle.get(&call_id) {
                            (*dialog).rx_ch.clone()
                        } else {
                            let (rx_send_ch, _) = broadcast::channel(MESSAGE_CHANNEL_SIZE);
                            let new_dialog = Dialog::from_request(
                                client_ip.clone(),
                                tx_ch.clone(),
                                rx_send_ch.clone(),
                                &msg,
                            )?;
                            dialogs_handle.insert(call_id, new_dialog.to_owned());
                            dialog_send_ch.send(new_dialog)?;
                            rx_send_ch
                        }
                    };

                    rx_send_ch.send(msg)?;
                    msg_str.clear();
                }
            }
            Err("broke out of the lines loop".into()) // TODO: Maybe retry on this error
        }));

        let tx_ch = send_send_ch.clone();
        tokio::spawn(and_log_err("tls sip send", async move {
            loop {
                match send_recv_ch.recv().await {
                    Some(msg) => {
                        let call_id = msg.call_id_header()?.value().to_string();

                        {
                            let mut dialogs_handle = dialogs.write().await;
                            if !dialogs_handle.contains_key(&call_id) {
                                let (rx_send_ch, _) = broadcast::channel(MESSAGE_CHANNEL_SIZE);
                                let new_dialog =
                                    Dialog::new(client_ip.clone(), tx_ch.clone(), rx_send_ch);
                                dialogs_handle.insert(call_id, new_dialog.to_owned());
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

    pub async fn dialog(&self) -> Dialog {
        let (rx_send_ch, _) = broadcast::channel(MESSAGE_CHANNEL_SIZE);
        let dialog = Dialog::new(self.client_ip, self.tx_ch.clone(), rx_send_ch);
        {
            let mut dialogs_handle = self.dialogs.write().await;
            dialogs_handle.insert(dialog.call_id.value().to_string(), dialog.to_owned());
        }
        dialog
    }
}

fn get_tls_connector() -> TlsConnector {
    let root_store = RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let tls_config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    TlsConnector::from(Arc::new(tls_config))
}
