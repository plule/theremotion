fn main() {
    println!("cargo:rerun-if-changed=ui");
    println!("cargo:rerun-if-changed=../assets/icon.svg");
    slint_build::compile("ui/main.slint").unwrap();
}
