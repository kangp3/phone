use std::time::Duration;
use std::error::Error;

use rppal::gpio::{Gpio, Trigger, InputPin};
use rppal::system::DeviceInfo;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tracing::{debug, warn};

use crate::hook::SwitchHook;


pub fn try_register_shk() -> Result<(InputPin, Sender<SwitchHook>, Receiver<SwitchHook>), Box<dyn Error>> {
    DeviceInfo::new()?;

    debug!("Registering SHK handler...");
    let (shk_send_ch, shk_recv_ch) = channel(1);
    let shk_send_ch2 = shk_send_ch.clone();
    let shk_send_ch3 = shk_send_ch.clone();

    let gpio = Gpio::new()?;
    let mut shk = gpio.get(15)?.into_input();
    shk.set_async_interrupt(
        Trigger::FallingEdge,
        Some(Duration::from_millis(10)),
        move |_| {
            if let Err(e) = shk_send_ch2.send(SwitchHook::ON) {
                warn!("{}", e);
            }
        }
    )?;
    shk.set_async_interrupt(
        Trigger::RisingEdge,
        Some(Duration::from_millis(10)),
        move |_| {
            if let Err(e) = shk_send_ch3.send(SwitchHook::OFF) {
                warn!("{}", e);
            }
        }
    )?;
    debug!("Registered SHK handler");

    Ok((shk, shk_send_ch, shk_recv_ch))
}
