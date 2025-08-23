use std::error::Error;
use std::sync::Arc;

use rsip::SipMessage;
use rustls::RootCertStore;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_rustls::TlsConnector;

use crate::asyncutil::and_log_err;

use super::{SERVER_NAME, SERVER_PORT};

const MESSAGE_CHANNEL_SIZE: usize = 64;

pub async fn bind() -> Result<(mpsc::Receiver<SipMessage>, mpsc::Sender<SipMessage>), Box<dyn Error>>
{
    let connector = get_tls_connector();
    let sock = TcpStream::connect((SERVER_NAME, SERVER_PORT)).await?;
    let stream = connector.connect(SERVER_NAME.try_into()?, sock).await?;
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

    Ok((recv_recv_ch, send_send_ch))
}

fn get_tls_connector() -> TlsConnector {
    let root_store = RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let tls_config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    TlsConnector::from(Arc::new(tls_config))
}
