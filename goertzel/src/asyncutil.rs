use std::error::Error;
use std::future::Future;

pub async fn and_log_err(fut: impl Future<Output=Result<(), Box<dyn Error>>>) {
    if let Err(e) = fut.await {
        dbg!(e);
    }
}
