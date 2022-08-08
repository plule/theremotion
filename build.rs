use std::path::PathBuf;

use faust_build::build_dsp;

fn main() {
    println!("cargo:rerun-if-changed=dsp");
    println!(r"cargo:rerun-if-env-changed=LEAPSDK_LIB_PATH");

    let leapsdk_path = std::env::var("LEAPSDK_LIB_PATH")
        .unwrap_or_else(|_| r"C:\Program Files\Ultraleap\LeapSDK\lib\x64".to_string());

    let leapsdk_path = PathBuf::from(leapsdk_path);

    if !leapsdk_path.is_dir() {
        println!("cargo:warning=Could not find LeapSDK at the location {}. Install it from https://developer.leapmotion.com/tracking-software-download or set its location with the environment variable LEAPSDK_LIB_PATH.", leapsdk_path.display());
    } else {
        let path_str = leapsdk_path
            .to_str()
            .unwrap_or_else(|| panic!("{} is not a valid path.", leapsdk_path.display()));

        // Link to LeapC.lib
        println!(r"cargo:rustc-link-search={}", path_str);
        println!(r"cargo:rustc-link-lib=static=LeapC");
    }

    if std::env::var("LEAPOTRON_REGEN_DSP").is_ok() {
        build_dsp("dsp/instrument.dsp");

        let mut generated = PathBuf::new();
        generated.push(std::env::var("OUT_DIR").unwrap());
        generated.push("dsp.rs");

        let mut dst = PathBuf::new();
        dst.push("src");
        dst.push("faust.rs");

        std::fs::copy(generated, dst).unwrap();
    }
}
