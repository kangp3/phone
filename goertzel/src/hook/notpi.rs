use ctrlc;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tracing::{debug, warn};

use crate::hook::SwitchHook;


pub fn try_register_shk() -> Result<((), Sender<SwitchHook>, Receiver<SwitchHook>), ctrlc::Error> {
    debug!("Registering SHK handler...");

    let (shk_send_ch, shk_recv_ch) = channel(1);
    let shk_send_ch2 = shk_send_ch.clone();
    let mut on_hook = false;

    // TODO(peter): Use SIGUSR1 (10) for this
    ctrlc::set_handler(move || {
        if let Err(e) = shk_send_ch2.send(if on_hook { SwitchHook::OFF } else { SwitchHook::ON }) {
            warn!("{}", e);
        }
        on_hook = !on_hook;
    })?;
    debug!("Registered SHK handler");

    Ok(((), shk_send_ch, shk_recv_ch))
}
