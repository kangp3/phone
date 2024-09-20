#[cfg(target_os = "linux")]
#[path = "rpi.rs"]
mod hooks;

#[cfg(target_os = "macos")]
#[path = "notpi.rs"]
mod hooks;

pub use hooks::*;

mod shk;
pub use shk::*;
