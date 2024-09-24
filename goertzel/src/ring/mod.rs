#[cfg(target_os = "linux")]
#[path = "rpi.rs"]
mod rings;

#[cfg(target_os = "macos")]
#[path = "notpi.rs"]
mod rings;

pub use rings::*;
