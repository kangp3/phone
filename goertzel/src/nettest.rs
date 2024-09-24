use std::error::Error;
use std::process::Stdio;

use tokio::process::Command;


pub async fn do_i_have_internet() -> Result<bool, Box<dyn Error>> {
    Ok(Command::new("ping")
        .args(&["-c", "1"])  // try sending 1 packet
        .args(&["-W", "1"])  // 1s timeout
        .arg("8.8.8.8")      // Big G tell me what it is
        .stdout(Stdio::null())
        .spawn()?
        .wait()
        .await?
        .success())
}
