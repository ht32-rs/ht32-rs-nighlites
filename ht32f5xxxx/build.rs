use std::env;
use std::fs;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());
        let device_file = if env::var_os("CARGO_FEATURE_HT32F50220_30").is_some() {
                    "src/ht32f50220_30/device.x"
                } else if env::var_os("CARGO_FEATURE_HT32F50231_41").is_some() {
                    "src/ht32f50231_41/device.x"
                } else if env::var_os("CARGO_FEATURE_HT32F50343").is_some() {
                    "src/ht32f50343/device.x"
                } else if env::var_os("CARGO_FEATURE_HT32F52142").is_some() {
                    "src/ht32f52142/device.x"
                } else if env::var_os("CARGO_FEATURE_HT32F52220_30").is_some() {
                    "src/ht32f52220_30/device.x"
                } else if env::var_os("CARGO_FEATURE_HT32F52231_41").is_some() {
                    "src/ht32f52231_41/device.x"
                } else if env::var_os("CARGO_FEATURE_HT32F52243_53").is_some() {
                    "src/ht32f52243_53/device.x"
                } else if env::var_os("CARGO_FEATURE_HT32F52331_41").is_some() {
                    "src/ht32f52331_41/device.x"
                } else if env::var_os("CARGO_FEATURE_HT32F52342_52").is_some() {
                    "src/ht32f52342_52/device.x"
                } else if env::var_os("CARGO_FEATURE_HT32F52344_54").is_some() {
                    "src/ht32f52344_54/device.x"
                } else if env::var_os("CARGO_FEATURE_HT32F52357_67").is_some() {
                    "src/ht32f52357_67/device.x"
                } else if env::var_os("CARGO_FEATURE_HT32F57331_41").is_some() {
                    "src/ht32f57331_41/device.x"
                } else if env::var_os("CARGO_FEATURE_HT32F57342_52").is_some() {
                    "src/ht32f57342_52/device.x"
                } else if env::var_os("CARGO_FEATURE_HT32F59041").is_some() {
                    "src/ht32f59041/device.x"
                } else if env::var_os("CARGO_FEATURE_HT32F59741").is_some() {
                    "src/ht32f59741/device.x"
                } else { panic!("No device features selected"); };
        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }
    println!("cargo:rerun-if-changed=build.rs");
}