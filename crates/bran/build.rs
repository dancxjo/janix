fn main() {
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    // Tell cargo to pass the linker script to the linker..
    println!("cargo:rustc-link-arg=-T{manifest_dir}/linker-{arch}.ld");
    // ..and to re-run if it changes.
    println!("cargo:rerun-if-changed=linker-{arch}.ld");
}
