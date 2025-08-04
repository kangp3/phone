use core::str;
use std::error::Error;
use std::io::{BufRead, Write};
use std::net::TcpStream;
use std::sync::Arc;

use goertzel::sip::Dialog;
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


pub fn main() -> Result<(), Box<dyn Error>> {
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

    for res_line in stream.lines() {
        if let Ok(line) = res_line {
            if line.len() == 0 {
                break
            }
            println!("Read from stream: {}", line);
        } else {
            res_line?;
        }
    }

    Ok(())
}
