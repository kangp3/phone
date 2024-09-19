use ctrlc;
use tracing::debug;

pub fn try_register_shk() -> Result<(), ctrlc::Error> {
    debug!("Registering SHK handler...");
    ctrlc::set_handler(move || {
        panic!("PHONE SLAM");
    })?;
    debug!("Registered SHK handler");

    Ok(())
}
