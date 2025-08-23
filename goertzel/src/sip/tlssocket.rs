use std::error::Error;
use std::sync::Arc;

use rsip::SipMessage;
use rustls::pki_types::ServerName;
use rustls::RootCertStore;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_rustls::TlsConnector;

use crate::asyncutil::and_log_err;

const MESSAGE_CHANNEL_SIZE: usize = 64;

pub struct TLSConn {
    pub host: String,
    pub port: u16,

    pub rx_ch: mpsc::Receiver<SipMessage>,
    pub tx_ch: mpsc::Sender<SipMessage>,
}

impl TLSConn {
    pub async fn new(host: &str, port: u16) -> Result<Self, Box<dyn Error>> {
        let connector = get_tls_connector();
        let sock = TcpStream::connect((host, port)).await?;
        let stream = connector
            .connect(ServerName::try_from(host)?.to_owned(), sock)
            .await?;
        let (recv_stream, mut send_stream) = tokio::io::split(stream);

        let (recv_send_ch, recv_recv_ch) = mpsc::channel(MESSAGE_CHANNEL_SIZE);
        tokio::spawn(and_log_err("tls sip recv", async move {
            let mut lines = BufReader::new(recv_stream).lines();
            let mut msg_str = String::new();
            while let Some(line) = lines.next_line().await? {
                msg_str.push_str(&line);
                msg_str.push_str("\r\n");
                if line.is_empty() {
                    recv_send_ch
                        .send(SipMessage::try_from(msg_str.clone())?)
                        .await?;
                    msg_str.clear();
                }
            }
            Ok(()) // TODO: This is an error, we shouldn't ever be here
        }));

        let (send_send_ch, mut send_recv_ch) = mpsc::channel(MESSAGE_CHANNEL_SIZE);
        tokio::spawn(and_log_err("tls sip send", async move {
            loop {
                match send_recv_ch.recv().await {
                    Some(sip_msg) => {
                        let msg_str = format!("{}", sip_msg);
                        send_stream.write_all(msg_str.as_bytes()).await?;
                    }
                    None => return Ok(()),
                }
            }
        }));

        Ok(TLSConn {
            host: String::from(host),
            port,

            rx_ch: recv_recv_ch,
            tx_ch: send_send_ch,
        })
    }
}

fn get_tls_connector() -> TlsConnector {
    let root_store = RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let tls_config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    TlsConnector::from(Arc::new(tls_config))
}
