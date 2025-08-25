use std::env;
use std::error::Error;
use std::time::Duration;

use goertzel::sip::{tlssocket, SERVER_NAME, SERVER_PORT};
use rsip::StatusCode;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let ip = public_ip::addr_v4().await.ok_or("no ip")?;

    let mut tls_conn = tlssocket::TlsSipConn::new(ip, SERVER_NAME, SERVER_PORT).await?;

    let password = env::var("SIP_PASSWORD")?;
    let mut dialog = tls_conn.dialog(String::from("1103")).await;
    dialog.register(password.clone()).await?;

    let new_msg = tls_conn
        .new_msg_ch
        .recv()
        .await
        .ok_or("error getting new msg")?;
    let mut new_dialog = tls_conn.dialog_from_req(&new_msg).await?;

    let ringing_resp =
        new_dialog.response_to(new_msg.clone().try_into()?, StatusCode::Ringing, vec![])?;
    new_dialog.send(ringing_resp).await?;

    tokio::time::sleep(Duration::from_secs(3)).await;

    let ok_resp = new_dialog.response_to(new_msg.try_into()?, StatusCode::OK, vec![])?;
    new_dialog.send(ok_resp).await?;

    new_dialog.recv().await?;

    new_dialog.recv().await?;
    Ok(())
}
