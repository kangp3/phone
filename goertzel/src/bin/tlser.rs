use core::str;
use std::error::Error;
use std::io::{BufRead, Write};
use std::net::TcpStream;
use std::sync::Arc;

use goertzel::sip::Dialog;
use itertools::Itertools;
use rsip::SipMessage;
use rustls::{ClientConfig, RootCertStore};


pub const LOCAL_HOST: &str = "172.56.162.25";
pub const SERVER_NAME: &str = "pbx.frandline.com";
pub const SERVER_PORT: u16 = 5061;


pub fn get_config() -> Result<Arc<ClientConfig>, Box<dyn Error>> {
    let root_store = RootCertStore::from_iter(
        webpki_roots::TLS_SERVER_ROOTS.iter().cloned(),
    );
    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    Ok(Arc::new(config))
}


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    if let Some(ip) = public_ip::addr_v4().await {
        println!("public ip address: {:?}", ip);
    } else {
        println!("couldn't get an IP address");
    }
    let tls_config = get_config()?;
    println!("Got config");
    let mut conn = rustls::ClientConnection::new(tls_config, SERVER_NAME.try_into()?)?;
    println!("Got connection");
    let mut sock = TcpStream::connect((SERVER_NAME, SERVER_PORT))?;
    let local_port = sock.local_addr()?.port();
    println!("Got sock at: {}", local_port);
    let mut stream = rustls::Stream::new(&mut conn, &mut sock);
    println!("Got stream");

    let mut dialog = Dialog::new();
    let register_msg = dialog.new_request(rsip::Method::Register, vec![]);
    let register_bytes = format!("{}", rsip::SipMessage::from(register_msg.clone()));
    let _ = stream.write_all(register_bytes.as_bytes())?;
    println!("Wrote to stream: {:?}", register_bytes);

    let response_str = stream.lines()
        .into_iter()
        .map_while(|line| match line {
            Ok(s) => if s.is_empty() { None } else { Some(s) },
            Err(_) => None, // TODO: Handle these errors?
        })
        .collect_vec()
        .iter()
        .join("\r\n");
    let response_msg = SipMessage::try_from(response_str)?;
    println!("Response msg is: {:?}", response_msg);

    Ok(())
}
