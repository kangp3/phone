use std::env;
use std::error::Error;

use goertzel::contacts::CONTACTS;
use goertzel::sip::{tlssocket, SERVER_NAME, SERVER_PORT};
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

    let password = env::var("SIP_PASSWORD")?;
    let mut dialog = tls_conn.dialog(String::from("1102")).await;
    dialog.register(password.clone()).await?;

    let mut dialog = tls_conn.dialog(String::from("1102")).await;
    let to = (*CONTACTS)
        .get("1103")
        .ok_or("contact is missing after I EXPLICITLY checked it")?;
    dialog.invite(password.clone(), to.clone()).await?;

    dialog.recv().await?;

    let msg = dialog.recv().await?;
    dialog.ack(msg.try_into()?).await?;

    dialog.recv().await?;

    Ok(())
}
