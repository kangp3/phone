#[cfg(target_arch = "arm")]
#[path = "rpi.rs"]
mod hooks;

#[cfg(not(target_arch = "arm"))]
#[path = "notpi.rs"]
mod hooks;

pub use hooks::*;

mod shk;
pub use shk::*;
