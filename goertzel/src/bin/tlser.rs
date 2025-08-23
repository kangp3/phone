use std::error::Error;

use goertzel::get_header;
use goertzel::sip::{add_auth_to_request, tlssocket, Dialog, SERVER_NAME, SERVER_PORT};
use rsip::prelude::*;
use rsip::{Header, Response};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let ip = public_ip::addr_v4().await.ok_or("no ip")?;

    let mut tls_conn = tlssocket::bind(SERVER_NAME, SERVER_PORT).await?;

    let mut dialog = Dialog::new(ip);
    let register_req = dialog.new_request(rsip::Method::Register, vec![]);
    tls_conn.tx_ch.send(register_req.clone().into()).await?;
    println!("Wrote to stream: {:?}", register_req);

    let response_msg = tls_conn.rx_ch.recv().await.ok_or("uh oh bad")?;
    println!("Response msg is: {:?}", response_msg);

    let resp = Response::try_from(response_msg)?;

    let www_auth = get_header!(resp.headers, Header::WwwAuthenticate);
    let mut authed_register_req = dialog.new_request(rsip::Method::Register, vec![]);
    add_auth_to_request(&mut authed_register_req, www_auth.opaque, www_auth.nonce);
    tls_conn
        .tx_ch
        .send(authed_register_req.clone().into())
        .await?;
    println!("Wrote to stream: {:?}", authed_register_req);

    let response_msg = tls_conn.rx_ch.recv().await.ok_or("uh oh bad")?;
    println!("Response msg is: {:?}", response_msg);

    Ok(())
}
