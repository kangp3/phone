use std::env;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let target = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target == "linux" {
        println!("cargo::rustc-link-search={}", manifest_dir);
        println!("cargo::rustc-link-lib=asound");
    }
}
