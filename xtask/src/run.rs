use crate::iso;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct RunArgs {
    pub env: String,
    pub gdb: bool,
    pub gdb_port: Option<u16>,
    pub timeout_secs: Option<u64>,
    pub interactive: bool,
    pub cmdline: Option<String>,
}

pub fn run(args: RunArgs) -> Result<()> {
    match args.env.as_str() {
        "x86_64" => run_qemu_x86_64(
            args.gdb,
            args.gdb_port,
            args.timeout_secs,
            args.interactive,
            args.cmdline,
        ),
        "aarch64" => run_qemu_aarch64(
            args.gdb,
            args.gdb_port,
            args.timeout_secs,
            args.interactive,
            args.cmdline,
        ),
        _ => anyhow::bail!(
            "Unsupported env for run: {}. Use x86_64 or aarch64",
            args.env
        ),
    }
}

fn run_with_timeout(mut cmd: Command, timeout: Option<u64>) -> Result<()> {
    // If no timeout, just run and wait
    if timeout.is_none() {
        let status = cmd.status().context("Failed to run command")?;
        if !status.success() {
            anyhow::bail!("Command failed with status: {}", status);
        }
        return Ok(());
    }

    let timeout = Duration::from_secs(timeout.unwrap());
    let mut child = cmd.spawn().context("Failed to spawn command")?;
    let start = Instant::now();

    loop {
        if let Some(status) = child.try_wait().context("Failed to check status")? {
            // Exited early?
            if !status.success() {
                anyhow::bail!("Command exited early with failure: {}", status);
            }
            return Ok(());
        }

        if start.elapsed() >= timeout {
            println!("Timeout reached, killing process...");
            child.kill().context("Failed to kill process")?;
            child.wait().context("Failed to wait after kill")?;
            // We consider timeout kill 'success' for typical test usage logic where we just wanted to run for N seconds.
            // But usually tests want to Assert stdout.
            // Child stdout is printed to parent stdout by default. Capture is done by caller of xtask.
            return Ok(());
        }

        std::thread::sleep(Duration::from_millis(100));
    }
}

fn run_qemu_x86_64(
    gdb: bool,
    gdb_port: Option<u16>,
    timeout: Option<u64>,
    interactive: bool,
    cmdline: Option<String>,
) -> Result<()> {
    // Ensure ISO exists (rebuilds kernel too)
    iso::run("x86_64".to_string(), cmdline)?;

    let root = project_root();
    let iso_path = root.join("target/iso/thingos-x86_64.iso");
    let ovmf_dir = root.join("vendor/ovmf");

    let ovmf_code = ovmf_dir.join("ovmf-code-x86_64.fd");
    let ovmf_vars = ovmf_dir.join("ovmf-vars-x86_64.fd");

    let mut use_uefi = false;
    if ovmf_code.exists() && ovmf_vars.exists() {
        // limit read to header
        if let Ok(data) = std::fs::read(&ovmf_code) {
            if data.starts_with(b"PLACEHOLDER") {
                eprintln!("[WARNING] OVMF is a placeholder. Falling back to BIOS.");
            } else {
                use_uefi = true;
            }
        }
    } else {
        eprintln!("[WARNING] OVMF files missing. QEMU will use default BIOS.");
    }

    println!("==> Running QEMU x86_64...");

    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.arg("-M").arg("q35");
    cmd.arg("-m").arg("512");
    if !interactive {
        cmd.arg("-nographic");
    }
    cmd.arg("-serial").arg("stdio");
    cmd.arg("-no-reboot");

    if use_uefi {
        cmd.arg("-drive").arg(format!(
            "if=pflash,format=raw,readonly=on,file={}",
            ovmf_code.display()
        ));
        cmd.arg("-drive").arg(format!(
            "if=pflash,format=raw,readonly=on,file={}",
            ovmf_vars.display()
        ));
    }
    // else default BIOS
    // else default BIOS
    cmd.arg("-cdrom").arg(&iso_path);

    // Create a dummy disk for AHCI testing
    let disk_path = root.join("disk.img");
    if !disk_path.exists() {
        println!("    Creating 64MB disk.img...");
        let f = std::fs::File::create(&disk_path).expect("create disk.img");
        f.set_len(64 * 1024 * 1024).expect("resize disk.img");
    }

    // Attach AHCI Controller and Disk
    cmd.arg("-device").arg("ahci,id=ahci");
    cmd.arg("-drive").arg(format!(
        "id=disk,file={},if=none,format=raw",
        disk_path.display()
    ));
    cmd.arg("-device").arg("ide-hd,drive=disk,bus=ahci.0");

    // GDB setup
    if let Some(port) = gdb_port {
        cmd.arg("-gdb").arg(format!("tcp::{}", port));
        println!("    GDB stub enabled on custom port {}...", port);
    } else {
        cmd.arg("-s");
        println!("    GDB stub enabled on default port 1234...");
    }

    if gdb {
        println!("    Waiting for GDB connection (frozen)...");
        cmd.arg("-S");
    }

    run_with_timeout(cmd, timeout)
}

fn run_qemu_aarch64(
    gdb: bool,
    gdb_port: Option<u16>,
    timeout: Option<u64>,
    interactive: bool,
    cmdline: Option<String>,
) -> Result<()> {
    // Ensure ISO exists
    iso::run("aarch64".to_string(), cmdline)?;

    let root = project_root();
    let iso_path = root.join("target/iso/thingos-aarch64.iso");
    let ovmf_code = root.join("vendor/ovmf/ovmf-code-aarch64.fd");

    let mut use_uefi = false;
    if ovmf_code.exists() {
        if let Ok(data) = std::fs::read(&ovmf_code) {
            if data.starts_with(b"PLACEHOLDER") {
                eprintln!(
                    "[WARNING] OVMF is a placeholder. AArch64 boot may fail (requires firmware)."
                );
            } else {
                use_uefi = true;
            }
        }
    }

    println!("==> Running QEMU aarch64...");

    let mut cmd = Command::new("qemu-system-aarch64");
    cmd.arg("-M").arg("virt");
    cmd.arg("-cpu").arg("cortex-a72");
    cmd.arg("-m").arg("2048"); // match trunk defaults (2G)
    if !interactive {
        cmd.arg("-nographic");
    } else {
        cmd.arg("-serial").arg("stdio");
    }
    cmd.arg("-no-reboot");
    cmd.arg("-d").arg("int,cpu_reset");
    cmd.arg("-D").arg("qemu.log");

    if use_uefi {
        cmd.arg("-bios").arg(&ovmf_code);
    }

    cmd.arg("-device").arg("ramfb");
    cmd.arg("-device").arg("qemu-xhci");
    cmd.arg("-device").arg("usb-kbd");
    cmd.arg("-device").arg("usb-mouse");
    cmd.arg("-cdrom").arg(&iso_path);

    // GDB setup
    if let Some(port) = gdb_port {
        cmd.arg("-gdb").arg(format!("tcp::{}", port));
        println!("    GDB stub enabled on custom port {}...", port);
    } else {
        cmd.arg("-s");
        println!("    GDB stub enabled on default port 1234...");
    }

    if gdb {
        println!("    Waiting for GDB connection (frozen)...");
        cmd.arg("-S");
    }

    run_with_timeout(cmd, timeout)
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}
