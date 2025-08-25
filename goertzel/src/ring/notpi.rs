use std::time::Duration;

use anyhow::Result;
use tokio::task::AbortHandle;
use tokio::time::sleep;
use tracing::{debug, info};

use crate::asyncutil::and_log_err;

pub struct RingHandle {
    handle: AbortHandle,
}

pub fn ring_phone() -> Result<RingHandle> {
    let handle = tokio::spawn(and_log_err("ringing", async move {
        loop {
            info!("Ring ring\x07");
            sleep(Duration::from_secs(1)).await;

            info!("No ring ring");
            sleep(Duration::from_secs(1)).await;
        }
    }))
    .abort_handle();
    Ok(RingHandle { handle })
}

impl Drop for RingHandle {
    fn drop(&mut self) {
        debug!("dropping ring");
        self.handle.abort();
    }
}
