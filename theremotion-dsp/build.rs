use faust_build::build_dsp;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=dsp");
    if std::env::var("THEREMOTION_REGEN_DSP").is_ok() {
        build_dsp("dsp/instrument.dsp");

        let mut generated = PathBuf::new();
        generated.push(std::env::var("OUT_DIR").unwrap());
        generated.push("dsp.rs");

        let mut dst = PathBuf::new();
        dst.push("src");
        dst.push("dsp.rs");

        std::fs::copy(generated, dst).unwrap();
    }
}
