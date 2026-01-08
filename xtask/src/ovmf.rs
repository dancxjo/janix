//! OVMF firmware download and setup.

use xshell::{Shell, cmd};
use crate::common::Result;

/// Download and install OVMF firmware for the target architecture.
pub fn ovmf(sh: &Shell, _arch: &str) -> Result<()> {
    sh.create_dir("ovmf")?;

    // Use pinned OVMF release from rust-osdev/ovmf-prebuilt (same as trunk branch)
    let release = std::env::var("THINGOS_OVMF_RELEASE")
        .unwrap_or_else(|_| "edk2-stable202508-r1".to_string());
    let archive_name = format!("{}-bin.tar.xz", release);
    let archive_url = format!(
        "https://github.com/rust-osdev/ovmf-prebuilt/releases/download/{}/{}",
        release, archive_name
    );
    let archive_path = format!("ovmf/{}", archive_name);

    // Check if we already have the required files
    if sh.path_exists("ovmf/ovmf-code-x86_64.fd") && sh.path_exists("ovmf/ovmf-vars-x86_64.fd") {
        println!("OVMF already present, skipping download.");
        return Ok(());
    }

    println!("Using OVMF release: {}", release);

    if !sh.path_exists(&archive_path) {
        println!("Downloading {}...", archive_name);
        cmd!(sh, "curl -fLo {archive_path} {archive_url}").run()?;
    } else {
        println!("Reusing cached {}", archive_name);
    }

    let extract_dir = format!("ovmf/extract-{}", release);
    sh.remove_path(&extract_dir)?;
    sh.create_dir(&extract_dir)?;

    println!("Extracting OVMF archive...");
    cmd!(sh, "tar -xJf {archive_path} -C {extract_dir}").run()?;

    // Map extracted files to expected locations
    let base = format!("{}/{}-bin", extract_dir, release);
    let mappings = [
        ("x64/code.fd", "ovmf/ovmf-code-x86_64.fd"),
        ("x64/vars.fd", "ovmf/ovmf-vars-x86_64.fd"),
        ("aarch64/code.fd", "ovmf/ovmf-code-aarch64.fd"),
        ("aarch64/vars.fd", "ovmf/ovmf-vars-aarch64.fd"),
        ("riscv64/code.fd", "ovmf/ovmf-code-riscv64.fd"),
        ("riscv64/vars.fd", "ovmf/ovmf-vars-riscv64.fd"),
    ];

    for (src_rel, dest) in mappings {
        let src = format!("{}/{}", base, src_rel);
        if sh.path_exists(&src) {
            println!("Installing {} -> {}", src_rel, dest);
            sh.copy_file(&src, dest)?;
        }
    }

    sh.remove_path(&extract_dir)?;
    println!("OVMF installed successfully.");

    Ok(())
}
