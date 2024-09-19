use std::error::Error;
use std::future::Future;

use tracing::error;


pub async fn and_log_err(fut: impl Future<Output=Result<(), Box<dyn Error>>>) {
    if let Err(e) = fut.await {
        error!(e);
    }
}
