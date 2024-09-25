use std::error::Error;
use std::time::Duration;

use tokio::task::AbortHandle;
use tokio::time::sleep;
use tracing::info;

use crate::asyncutil::and_log_err;


pub fn ring_phone() -> Result<AbortHandle, Box<dyn Error>> {
    Ok(tokio::spawn(and_log_err("ringing", async move {
        loop {
            info!("Ring ring\x07");
            sleep(Duration::from_secs(1)).await;

            info!("No ring ring");
            sleep(Duration::from_secs(1)).await;
        }
    })).abort_handle())
}
