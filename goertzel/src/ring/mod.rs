#[cfg(target_arch = "arm")]
#[path = "rpi.rs"]
mod rings;

#[cfg(not(target_arch = "arm"))]
#[path = "notpi.rs"]
mod rings;

pub use rings::*;
