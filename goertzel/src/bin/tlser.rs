use std::env;
use std::time::Duration;

use anyhow::{anyhow, Result};
use goertzel::contacts::CONTACTS;
use goertzel::sip::{tlssocket, SERVER_NAME, SERVER_PORT};
use rsip::StatusCode;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
pub async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let ip = public_ip::addr_v4().await.ok_or(anyhow!("no ip"))?;

    let mut tls_conn = tlssocket::TlsSipConn::new(ip, SERVER_NAME, SERVER_PORT).await?;

    let password = env::var("SIP_PASSWORD")?;
    let mut dialog_1103 = tls_conn.dialog(String::from("1103")).await;
    dialog_1103.register(password.clone()).await?;

    let mut dialog_1102 = tls_conn.dialog(String::from("1102")).await;
    dialog_1102.register(password.clone()).await?;

    let mut dialog_1102 = tls_conn.dialog(String::from("1102")).await;
    let to = (*CONTACTS)
        .get("1103")
        .ok_or(anyhow!("contact is missing after I EXPLICITLY checked it"))?;
    dialog_1102.invite(password.clone(), to.clone()).await?;

    let new_msg = tls_conn
        .new_msg_ch
        .recv()
        .await
        .ok_or(anyhow!("error getting new msg"))?;
    let mut new_dialog = tls_conn.dialog_from_req(&new_msg).await?;

    let ringing_resp =
        new_dialog.response_to(new_msg.clone().try_into()?, StatusCode::Ringing, vec![])?;
    new_dialog.send(ringing_resp).await?;

    dialog_1102.recv().await?;

    tokio::time::sleep(Duration::from_secs(3)).await;

    let ok_resp = new_dialog.response_to(new_msg.try_into()?, StatusCode::OK, vec![])?;
    new_dialog.send(ok_resp).await?;

    let msg = dialog_1102.recv().await?;
    dialog_1102.ack(msg.try_into()?).await?;

    new_dialog.recv().await?;
    dialog_1102.recv().await?;

    Ok(())
}
