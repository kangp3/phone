use ctrlc;

pub fn try_register_shk() -> Result<(), ()> {
    dbg!("Registering SHK handler...");
    ctrlc::set_handler(move || {
        panic!("PHONE SLAM");
    }).unwrap();
    dbg!("Registered SHK handler");

    Ok(())
}
