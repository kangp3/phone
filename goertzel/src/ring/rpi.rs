use std::error::Error;
use std::time::Duration;

use rppal::gpio::Gpio;
use tokio::task::AbortHandle;
use tokio::time::sleep;
use tracing::debug;

use crate::asyncutil::and_log_err;


const RM_PIN: u8 = 17;
const FR_PIN: u8 = 12;
const RING_FREQ: f64 = 20.;
const RING_DUTY: f64 = 0.5;


pub struct RingHandle {
    handle: AbortHandle,
}

pub fn ring_phone() -> Result<RingHandle, Box<dyn Error>> {
    let gpio = Gpio::new()?;
    let mut rm = gpio.get(RM_PIN)?.into_output_low();
    let mut fr = gpio.get(FR_PIN)?.into_output_low();

    let handle = tokio::spawn(and_log_err("ringing", async move {
        loop {
            rm.set_high();
            fr.set_pwm_frequency(RING_FREQ, RING_DUTY)?;
            sleep(Duration::from_secs(1)).await;

            rm.set_low();
            fr.clear_pwm()?;
            sleep(Duration::from_secs(1)).await;
        }
    })).abort_handle();
    Ok(RingHandle{ handle })
}

impl Drop for RingHandle {
    fn drop(&mut self) {
        debug!("dropping ring");
        self.handle.abort();
    }
}
