use std::env;
use std::fs;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());
        let device_file = if env::var_os("CARGO_FEATURE_HT32F125X").is_some() {
                    "src/ht32f125x/device.x"
                } else if env::var_os("CARGO_FEATURE_HT32F1653_54").is_some() {
                    "src/ht32f1653_54/device.x"
                } else if env::var_os("CARGO_FEATURE_HT32F1655_56").is_some() {
                    "src/ht32f1655_56/device.x"
                } else if env::var_os("CARGO_FEATURE_HT32F175X").is_some() {
                    "src/ht32f175x/device.x"
                } else { panic!("No device features selected"); };
        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }
    println!("cargo:rerun-if-changed=build.rs");
}