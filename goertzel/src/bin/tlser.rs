use std::error::Error;
use std::io::Write;
use std::net::TcpStream;
use std::sync::Arc;

use rustls::pki_types::pem::PemObject;
use rustls::pki_types::CertificateDer;
use rustls::{ClientConfig, RootCertStore};

const REGISTER_MSG: &str = "REGISTER sip:18.191.30.101:5060 SIP/2.0
CSeq: 1 REGISTER
Via: SIP/2.0/UDP 192.168.1.151:5060;branch=z9hG4bKXB8gmUTUr0vr6kd8KYwvoZTHNQFDSwRv;rport
User-Agent: Frandline/0.1.0
Call-ID: 1733632458251/7HZ3AhcTdwVXgNz6
Contact: 1103 <sip:1103@{host}:{port}>;q=1
Max-Forwards: 70
Content-Length: 0
From: <sip:1103@{host}:{port}>;tag=lFjYksh8BnQvP2D2
To: <sip:1103@18.191.30.101:5060>
Allow: ACK, BYE, CANCEL, INFO, INVITE, MESSAGE, NOTIFY, OPTIONS, PRACK, PUBLISH, REFER, REGISTER, SUBSCRIBE, UPDATE";


pub const SERVER_NAME: &str = "pbx.frandline.com";
pub const SERVER_PORT: u16 = 5061;


pub fn get_config() -> Result<Arc<ClientConfig>, Box<dyn Error>> {
    let mut root_store = RootCertStore::empty();
    root_store.add_parsable_certificates(
        CertificateDer::pem_file_iter("ssl/ca.crt")?
            .collect::<Result<Vec<_>, _>>()?,
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
    println!("Got sock");
    let mut stream = rustls::Stream::new(&mut conn, &mut sock);
    println!("Got stream");

    let _ = stream.write_all(REGISTER_MSG.as_bytes())?;

    Ok(())
}
