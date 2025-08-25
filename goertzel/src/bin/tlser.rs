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

    let mut tls_conn = tlssocket::TlsSipConn::new(ip, SERVER_NAME, SERVER_PORT).await?;

    let password = env::var("SIP_PASSWORD")?;
    let mut dialog_1103 = tls_conn.dialog(String::from("1103")).await;
    dialog_1103.register(password.clone()).await?;

    let mut dialog_1102 = tls_conn.dialog(String::from("1102")).await;
    dialog_1102.register(password.clone()).await?;

    let mut dialog_1102 = tls_conn.dialog(String::from("1102")).await;
    let to = (*CONTACTS)
        .get("1103")
        .ok_or("contact is missing after I EXPLICITLY checked it")?;
    dialog_1102.invite(password.clone(), to.clone()).await?;

    println!("Waiting for new dialog");
    let new_msg = tls_conn
        .new_msg_ch
        .recv()
        .await
        .ok_or("error getting new msg")?;
    println!("Got new dialog");
    let mut new_dialog = tls_conn.dialog_from_req(&new_msg).await?;
    new_dialog.recv().await?;

    Ok(())
}
