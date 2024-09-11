use ctrlc;

pub fn try_register_shk() -> Result<(), ctrlc::Error> {
    dbg!("Registering SHK handler...");
    ctrlc::set_handler(move || {
        panic!("PHONE SLAM");
    })?;
    dbg!("Registered SHK handler");

    Ok(())
}
