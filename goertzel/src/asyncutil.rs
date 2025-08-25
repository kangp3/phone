use std::fmt::Display;
use std::future::Future;

use anyhow::Result;
use tokio::select;
use tracing::error;

pub async fn and_log_err(
    tag: impl AsRef<str> + tracing::Value + Display,
    fut: impl Future<Output = anyhow::Result<()>>,
) {
    if let Result::Err(e) = fut.await {
        error!("{}\n{:?}", tag, &e);
    }
}

pub async fn race<T>(fut1: impl Future<Output = T>, fut2: impl Future<Output = T>) -> T {
    select! {
        r = fut1 => r,
        r = fut2 => r,
    }
}
