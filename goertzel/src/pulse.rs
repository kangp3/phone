use std::time::Duration;

use tokio::sync::broadcast::{channel, Receiver};
use tokio::time::sleep;
use tracing::{trace, warn};

use crate::asyncutil::and_log_err;
use crate::hook::SwitchHook;


const PULSE_TIMEOUT_MS: u64 = 150;


pub fn notgoertzelme(mut shk_ch: Receiver<SwitchHook>) -> (Receiver<u8>, Receiver<SwitchHook>) {
    let (digit_send_ch, digit_recv_ch) = channel(1);
    let (hangup_send_ch, hangup_recv_ch) = channel(1);

    tokio::spawn(and_log_err(async move {
        loop {
            let hook_event = shk_ch.recv().await?;
            if hook_event == SwitchHook::OFF { continue }

            let mut digit = 0;
            loop {
                tokio::select! {
                    _ = sleep(Duration::from_millis(PULSE_TIMEOUT_MS)) => {
                        trace!("I hang up");
                        hangup_send_ch.send(SwitchHook::ON)?;
                        break;
                    }
                    _ = shk_ch.recv() => {
                        digit += 1;
                        trace!("I pulse even");
                    }
                };
                tokio::select! {
                    _ = sleep(Duration::from_millis(PULSE_TIMEOUT_MS)) => {
                        if digit > 10 {
                            warn!("Something bad, digit is {}", digit);
                        } else {
                            trace!("I send {}", digit);
                            digit_send_ch.send(digit % 10)?;
                        }
                        break;
                    }
                    _ = shk_ch.recv() => trace!("I pulse odd"),
                };
            }
        }
    }));

    return (digit_recv_ch, hangup_recv_ch);
}
