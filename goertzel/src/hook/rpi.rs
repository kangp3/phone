use std::time::Duration;
use std::error::Error;

use rppal::gpio::{Gpio, Trigger, InputPin};
use rppal::system::DeviceInfo;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tracing::{debug, warn};

use crate::hook::SwitchHook;


const SHK_PIN: u8 = 15;


pub fn try_register_shk() -> Result<(InputPin, Sender<SwitchHook>, Receiver<SwitchHook>), Box<dyn Error>> {
    DeviceInfo::new()?;

    debug!("Registering SHK handler...");
    // TODO(peter): Maybe migrate this to MPSC
    let (shk_send_ch, shk_recv_ch) = channel(1);
    let shk_send_ch2 = shk_send_ch.clone();

    let gpio = Gpio::new()?;
    let mut shk = gpio.get(SHK_PIN)?.into_input();
    shk.set_async_interrupt(
        Trigger::Both,
        Some(Duration::from_millis(10)),
        move |evt| {
            let state = match evt.trigger {
                Trigger::RisingEdge => SwitchHook::OFF,
                Trigger::FallingEdge => SwitchHook::ON,
                e => panic!("what i got edge {}", e),
            };
            if let Err(e) = shk_send_ch2.send(state) {
                warn!("{}", e);
            }
        }
    )?;
    debug!("Registered SHK handler");

    Ok((shk, shk_send_ch, shk_recv_ch))
}
