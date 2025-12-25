use anyhow::{Context, Result};
use std::process::Command;
use std::path::{Path, PathBuf};
use crate::iso;

#[derive(Debug, Clone)]
pub struct RunArgs {
    pub env: String,
    pub gdb: bool,
}

pub fn run(args: RunArgs) -> Result<()> {
    match args.env.as_str() {
        "hosted" => run_hosted(),
        "x86_64" => run_qemu_x86_64(args.gdb),
        "aarch64" => run_qemu_aarch64(args.gdb),
        _ => anyhow::bail!("Unsupported env for run: {}. Use hosted, x86_64 or aarch64", args.env),
    }
}

fn run_hosted() -> Result<()> {
    println!("==> Running hosted kernel...");
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    
    let status = Command::new(cargo)
        .arg("run")
        .arg("-p")
        .arg("kernel_hosted")
        .status()
        .context("Failed to run hosted kernel")?;

    if !status.success() {
        anyhow::bail!("Hosted kernel run failed");
    }
    Ok(())
}

fn run_qemu_x86_64(gdb: bool) -> Result<()> {
    // Ensure ISO exists (rebuilds kernel too)
    iso::run("x86_64".to_string())?;

    let root = project_root();
    let iso_path = root.join("target/iso/thingos-x86_64.iso");
    let ovmf_dir = root.join("vendor/ovmf");
    
    let ovmf_code = ovmf_dir.join("ovmf-code-x86_64.fd");
    let ovmf_vars = ovmf_dir.join("ovmf-vars-x86_64.fd");

    if !ovmf_code.exists() || !ovmf_vars.exists() {
        eprintln!("[WARNING] OVMF files missing in vendor/ovmf/. QEMU might fail if they were placeholders.");
        // We continue anyway, as per "just run ... streams serial logs" - checking failure happens at runtime.
    }

    println!("==> Running QEMU x86_64...");
    
    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.arg("-M").arg("q35");
    cmd.arg("-serial").arg("stdio");
    cmd.arg("-no-reboot");
    
    // Firmware
    // -drive if=pflash,format=raw,readonly=on,file=...
    cmd.arg("-drive").arg(format!("if=pflash,format=raw,readonly=on,file={}", ovmf_code.display()));
    cmd.arg("-drive").arg(format!("if=pflash,format=raw,readonly=on,file={}", ovmf_vars.display()));
    
    // CDROM
    cmd.arg("-cdrom").arg(&iso_path);

    if gdb {
        println!("    Waiting for GDB connection on port 1234...");
        cmd.arg("-s").arg("-S");
    }

    let status = cmd.status().context("Failed to run QEMU")?;
    if !status.success() {
        anyhow::bail!("QEMU exited with error");
    }

    Ok(())
}

fn run_qemu_aarch64(gdb: bool) -> Result<()> {
    // Ensure ISO exists
    iso::run("aarch64".to_string())?;

    let root = project_root();
    let iso_path = root.join("target/iso/thingos-aarch64.iso");
    let ovmf_code = root.join("vendor/ovmf/ovmf-code-aarch64.fd");

    println!("==> Running QEMU aarch64...");

    let mut cmd = Command::new("qemu-system-aarch64");
    cmd.arg("-M").arg("virt");
    cmd.arg("-cpu").arg("cortex-a72");
    cmd.arg("-serial").arg("stdio");
    cmd.arg("-no-reboot");
    
    // Firmware
    cmd.arg("-bios").arg(&ovmf_code);

    // CDROM
    cmd.arg("-cdrom").arg(&iso_path);
    
    // Graphics (optional, but good for future)
    // cmd.arg("-device").arg("ramfb"); 

    if gdb {
        println!("    Waiting for GDB connection on port 1234...");
        cmd.arg("-s").arg("-S");
    }

    let status = cmd.status().context("Failed to run QEMU")?;
    if !status.success() {
        anyhow::bail!("QEMU exited with error");
    }

    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()
}
