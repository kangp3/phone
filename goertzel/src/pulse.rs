use std::time::Duration;

use tokio::sync::broadcast;
use tokio::time::sleep;
use tracing::{trace, warn};

use crate::asyncutil::and_log_err;
use crate::hook::SwitchHook;

const PULSE_TIMEOUT_MS: u64 = 150;

pub fn notgoertzelme(
    mut shk_ch: broadcast::Receiver<SwitchHook>,
) -> (
    broadcast::Sender<u8>,
    broadcast::Receiver<u8>,
    broadcast::Sender<SwitchHook>,
    broadcast::Receiver<SwitchHook>,
) {
    let (digit_send_ch, digit_recv_ch) = broadcast::channel(1);
    let digit_send_ch2 = digit_send_ch.clone();
    let (onhook_send_ch, onhook_recv_ch) = broadcast::channel(1);
    let onhook_send_ch2 = onhook_send_ch.clone();

    tokio::spawn(and_log_err("pulse_detect", async move {
        loop {
            let hook_event = shk_ch.recv().await?;
            if hook_event == SwitchHook::OFF {
                let _ = onhook_send_ch.send(SwitchHook::OFF);
                continue;
            }

            let mut digit = 0;
            loop {
                tokio::select! {
                    _ = sleep(Duration::from_millis(PULSE_TIMEOUT_MS)) => {
                        trace!("I hang up");
                        let _ = onhook_send_ch.send(SwitchHook::ON);
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
                            let _ = digit_send_ch.send(digit % 10);
                        }
                        break;
                    }
                    _ = shk_ch.recv() => trace!("I pulse odd"),
                };
            }
        }
    }));

    (
        digit_send_ch2,
        digit_recv_ch,
        onhook_send_ch2,
        onhook_recv_ch,
    )
}
