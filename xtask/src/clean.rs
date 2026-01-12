//! Clean tasks.

use crate::common::Result;
use xshell::{Shell, cmd};

/// Clean build artifacts.
pub fn clean(sh: &Shell) -> Result<()> {
    println!("Cleaning build artifacts...");
    cmd!(sh, "cargo clean").run()?;
    sh.remove_path("iso_root")?;
    sh.remove_path("thing-os-x86_64.iso")?;
    sh.remove_path("thing-os-x86_64.hdd")?;
    sh.remove_path("thing-os-aarch64.iso")?;
    sh.remove_path("thing-os-aarch64.hdd")?;
    sh.remove_path("thing-os-riscv64.iso")?;
    sh.remove_path("thing-os-riscv64.hdd")?;
    sh.remove_path("thing-os-loongarch64.iso")?;
    sh.remove_path("thing-os-loongarch64.hdd")?;
    sh.remove_path("bran/bin-x86_64")?;
    sh.remove_path("bran/bin-aarch64")?;
    sh.remove_path("bran/bin-riscv64")?;
    sh.remove_path("bran/bin-loongarch64")?;
    // Clean OVMF firmware files (but keep cached archive)
    sh.remove_path("ovmf/ovmf-code-x86_64.fd")?;
    sh.remove_path("ovmf/ovmf-vars-x86_64.fd")?;
    sh.remove_path("ovmf/ovmf-code-aarch64.fd")?;
    sh.remove_path("ovmf/ovmf-vars-aarch64.fd")?;
    sh.remove_path("ovmf/ovmf-code-riscv64.fd")?;
    sh.remove_path("ovmf/ovmf-vars-riscv64.fd")?;
    sh.remove_path("ovmf/ovmf-code-loongarch64.fd")?;
    sh.remove_path("ovmf/ovmf-vars-loongarch64.fd")?;
    Ok(())
}

/// Clean everything including downloaded dependencies.
pub fn distclean(sh: &Shell) -> Result<()> {
    clean(sh)?;
    println!("Removing downloaded dependencies...");
    sh.remove_path("limine")?;
    sh.remove_path("ovmf")?;
    Ok(())
}
