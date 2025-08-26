use std::env;

use anyhow::{anyhow, Result};
use goertzel::contacts::CONTACTS;
use goertzel::sip::{tlssocket, SERVER_NAME, SERVER_PORT};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
pub async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let ip = public_ip::addr_v4().await.ok_or(anyhow!("no ip"))?;

    let tls_conn = tlssocket::TlsSipConn::new(ip, SERVER_NAME, SERVER_PORT).await?;

    let password = env::var("SIP_PASSWORD")?;
    let mut dialog = tls_conn.dialog(String::from("1103")).await;
    dialog.register(password.clone()).await?;

    let mut dialog = tls_conn.dialog(String::from("1103")).await;
    let to = (*CONTACTS)
        .get("1102")
        .ok_or(anyhow!("contact is missing after I EXPLICITLY checked it"))?;
    dialog.invite(password.clone(), to.clone()).await?;

    dialog.recv().await?;

    let msg = dialog.recv().await?;
    dialog.ack(msg.try_into()?).await?;

    dialog.recv().await?;
    dialog.recv().await?;

    Ok(())
}
