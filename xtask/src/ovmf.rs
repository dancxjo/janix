//! OVMF firmware download and setup.

use xshell::{Shell, cmd};
use crate::common::Result;

/// All supported architectures for OVMF download.
pub const ALL_ARCHES: &[&str] = &["x86_64", "aarch64", "riscv64", "loongarch64"];

/// Download and install OVMF firmware for all architectures.
pub fn ovmf_all(sh: &Shell) -> Result<()> {
    // Check if all files already exist
    let all_present = ALL_ARCHES.iter().all(|arch| {
        let code_file = format!("ovmf/ovmf-code-{}.fd", arch);
        let vars_file = format!("ovmf/ovmf-vars-{}.fd", arch);
        sh.path_exists(&code_file) && sh.path_exists(&vars_file)
    });

    if all_present {
        println!("OVMF already present for all architectures, skipping download.");
        return Ok(());
    }

    // Download and extract once, install all
    download_and_install_ovmf(sh)?;
    Ok(())
}

/// Download and install OVMF firmware for a specific architecture.
pub fn ovmf(sh: &Shell, arch: &str) -> Result<()> {
    let code_file = format!("ovmf/ovmf-code-{}.fd", arch);
    let vars_file = format!("ovmf/ovmf-vars-{}.fd", arch);

    // Check if we already have the required files for THIS architecture
    if sh.path_exists(&code_file) && sh.path_exists(&vars_file) {
        println!("OVMF already present for {}, skipping download.", arch);
        return Ok(());
    }

    // Download and extract (installs all, but we only needed this one)
    download_and_install_ovmf(sh)?;
    
    // Verify we got what we needed
    if !sh.path_exists(&code_file) || !sh.path_exists(&vars_file) {
        println!("WARNING: OVMF firmware for {} not available.", arch);
        println!("  Tests for {} will fail at QEMU startup.", arch);
    }

    Ok(())
}

/// Internal: Download and install all OVMF files.
fn download_and_install_ovmf(sh: &Shell) -> Result<()> {
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
    // Mappings match trunk branch xtask/src/fetch.rs
    let base = format!("{}/{}-bin", extract_dir, release);
    let mappings = [
        ("x64/code.fd", "ovmf/ovmf-code-x86_64.fd"),
        ("x64/vars.fd", "ovmf/ovmf-vars-x86_64.fd"),
        ("aarch64/code.fd", "ovmf/ovmf-code-aarch64.fd"),
        ("aarch64/vars.fd", "ovmf/ovmf-vars-aarch64.fd"),
        ("riscv64/code.fd", "ovmf/ovmf-code-riscv64.fd"),
        ("riscv64/vars.fd", "ovmf/ovmf-vars-riscv64.fd"),
        ("loongarch64/code.fd", "ovmf/ovmf-code-loongarch64.fd"),
        ("loongarch64/vars.fd", "ovmf/ovmf-vars-loongarch64.fd"),
    ];

    for (src_rel, dest) in mappings {
        let src = format!("{}/{}", base, src_rel);
        if sh.path_exists(&src) {
            println!("Installing {} -> {}", src_rel, dest);
            sh.copy_file(&src, dest)?;
        } else {
            eprintln!("[WARNING] Missing OVMF artifact: {}", src_rel);
        }
    }

    sh.remove_path(&extract_dir)?;
    println!("OVMF installed successfully.");

    Ok(())
}
