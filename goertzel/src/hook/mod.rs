#[cfg(target_os = "none")]
#[path = "rpi.rs"]
mod hooks;

#[cfg(not(target_os = "none"))]
#[path = "notpi.rs"]
mod hooks;

pub use hooks::*;
