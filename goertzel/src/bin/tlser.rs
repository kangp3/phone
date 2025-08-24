use std::error::Error;

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

    let mut dialog = tls_conn.dialog().await;
    dialog.register().await?;

    Ok(())
}
