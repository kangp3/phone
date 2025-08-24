use std::error::Error;

use goertzel::sip::{add_auth_to_request, tlssocket, SERVER_NAME, SERVER_PORT};
use rsip::prelude::*;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let ip = public_ip::addr_v4().await.ok_or("no ip")?;

    let tls_conn = tlssocket::TlsSipConn::new(ip, SERVER_NAME, SERVER_PORT).await?;

    let mut dialog = tls_conn.dialog().await;
    let register_req = dialog.new_request(rsip::Method::Register, vec![]);
    dialog.send(register_req.clone()).await?;
    println!("Wrote to stream: {:?}", register_req);

    let response_msg = dialog.recv().await?;
    println!("Response msg is: {:?}", response_msg);

    let www_auth = response_msg
        .www_authenticate_header()
        .ok_or("missing www auth header")?
        .typed()?;
    let mut authed_register_req = dialog.new_request(rsip::Method::Register, vec![]);
    add_auth_to_request(&mut authed_register_req, www_auth.opaque, www_auth.nonce);
    dialog.send(authed_register_req.clone()).await?;
    println!("Wrote to stream: {:?}", authed_register_req);

    let response_msg = dialog.recv().await?;
    println!("Response msg is: {:?}", response_msg);

    Ok(())
}
