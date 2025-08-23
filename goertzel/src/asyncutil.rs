use std::error::Error;
use std::future::Future;

use tokio::select;
use tracing::error;

pub async fn and_log_err(
    tag: impl AsRef<str> + tracing::Value,
    fut: impl Future<Output = Result<(), Box<dyn Error>>>,
) {
    if let Err(e) = fut.await {
        error!(tag, e);
    }
}

pub async fn race<T>(fut1: impl Future<Output = T>, fut2: impl Future<Output = T>) -> T {
    select! {
        r = fut1 => r,
        r = fut2 => r,
    }
}
