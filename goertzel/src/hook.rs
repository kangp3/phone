use std::time::Duration;

use rppal::gpio::{Gpio, Trigger, InputPin};
use rppal::system::DeviceInfo;

pub fn try_register_shk() -> Result<InputPin, ()> {
    if let Err(_) = DeviceInfo::new() {
        return Err(());
    }

    dbg!("Registering SHK handler...");
    let gpio = Gpio::new().unwrap();
    let mut shk = gpio.get(15).unwrap().into_input();
    shk.set_async_interrupt(
        Trigger::FallingEdge,
        Some(Duration::from_millis(10)),
        |_| {
            panic!("PHONE SLAM");
        }
    ).unwrap();
    dbg!("Registered SHK handler");

    Ok(shk)
}
