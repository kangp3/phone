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

    let mut tls_conn_1103 = tlssocket::TlsSipConn::new(ip, SERVER_NAME, SERVER_PORT).await?;
    let tls_conn_1102 = tlssocket::TlsSipConn::new(ip, SERVER_NAME, SERVER_PORT).await?;

    let password = env::var("SIP_PASSWORD")?;
    let mut dialog_1103 = tls_conn_1103.dialog(String::from("1103")).await;
    dialog_1103.register(password.clone()).await?;

    let mut dialog_1102 = tls_conn_1102.dialog(String::from("1102")).await;
    dialog_1102.register(password.clone()).await?;

    let mut dialog_1102 = tls_conn_1102.dialog(String::from("1102")).await;
    let to = (*CONTACTS)
        .get("1103")
        .ok_or("contact is missing after I EXPLICITLY checked it")?;
    dialog_1102.invite(password.clone(), to.clone()).await?;

    println!("Waiting for new dialog");
    let new_dialog = tls_conn_1103.dialog_ch.recv().await?;
    println!("Got new dialog");
    new_dialog.recv().await?;

    Ok(())
}
