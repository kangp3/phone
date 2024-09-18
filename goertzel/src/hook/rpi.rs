use std::time::Duration;
use std::error::Error;

use rppal::gpio::{Gpio, Trigger, InputPin};
use rppal::system::DeviceInfo;

pub fn try_register_shk() -> Result<InputPin, Box<dyn Error>> {
    DeviceInfo::new()?;

    dbg!("Registering SHK handler...");
    let gpio = Gpio::new()?;
    let mut shk = gpio.get(15)?.into_input();
    shk.set_async_interrupt(
        Trigger::FallingEdge,
        Some(Duration::from_millis(10)),
        |_| {
            panic!("PHONE SLAM");
        }
    )?;
    dbg!("Registered SHK handler");

    Ok(shk)
}
