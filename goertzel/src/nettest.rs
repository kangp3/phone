use std::net::Ipv4Addr;
use std::process::Stdio;

use anyhow::{anyhow, Result};
use tokio::process::Command;

pub async fn do_i_have_internet() -> Result<bool> {
    Ok(Command::new("ping")
        .args(&["-c", "1"]) // try sending 1 packet
        .args(&["-W", "1"]) // 1s timeout
        .arg("8.8.8.8") // Big G tell me what it is
        .stdout(Stdio::null())
        .spawn()?
        .wait()
        .await?
        .success())
}

pub async fn can_i_has_ip() -> Result<Ipv4Addr> {
    public_ip::addr_v4().await.ok_or(anyhow!("no IP 4 me :("))
}
