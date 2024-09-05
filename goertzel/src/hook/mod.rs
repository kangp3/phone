#[cfg(target_os = "none")]
mod rpi;
#[cfg(target_os = "none")]
pub use rpi::*;

#[cfg(not(target_os = "none"))]
mod notpi;
#[cfg(not(target_os = "none"))]
pub use notpi::*;
