use std::error::Error;

use goertzel::sip::{add_auth_to_request, tlssocket, SERVER_NAME, SERVER_PORT};
use rsip::prelude::*;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let ip = public_ip::addr_v4().await.ok_or("no ip")?;

    let tls_conn = tlssocket::TlsSipConn::new(ip, SERVER_NAME, SERVER_PORT).await?;

    let mut dialog = tls_conn.dialog().await;
    let register_req = dialog.new_request(rsip::Method::Register, vec![]);
    dialog.send(register_req.clone()).await?;

    let response_msg = dialog.recv().await?;

    let www_auth = response_msg
        .www_authenticate_header()
        .ok_or("missing www auth header")?
        .typed()?;
    let mut authed_register_req = dialog.new_request(rsip::Method::Register, vec![]);
    add_auth_to_request(&mut authed_register_req, www_auth.opaque, www_auth.nonce);
    dialog.send(authed_register_req.clone()).await?;

    dialog.recv().await?;

    Ok(())
}
