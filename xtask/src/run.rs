use crate::iso;
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct RunArgs {
    pub env: String,
    pub gdb: bool,
    pub gdb_port: Option<u16>,
    pub timeout_secs: Option<u64>,
    pub interactive: bool,
    pub frozen: bool,
    pub cmdline: Option<String>,
}

pub fn run(args: RunArgs) -> Result<()> {
    match args.env.as_str() {
        "x86_64" => run_qemu_x86_64(args),
        "aarch64" => run_qemu_aarch64(args),
        "riscv64" => run_qemu_riscv64(args),
        "loongarch64" => run_qemu_loongarch64(args),
        _ => anyhow::bail!("Unsupported env for run: {}", args.env),
    }
}

fn run_with_timeout(mut cmd: Command, timeout: Option<u64>) -> Result<()> {
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
            if !status.success() {
                anyhow::bail!("Command exited early with failure: {}", status);
            }
            return Ok(());
        }

        if start.elapsed() >= timeout {
            println!("Timeout reached, killing process...");
            child.kill().context("Failed to kill process")?;
            child.wait().context("Failed to wait after kill")?;
            return Ok(());
        }

        std::thread::sleep(Duration::from_millis(100));
    }
}

fn append_common_args(cmd: &mut Command, args: &RunArgs, default_gdb_port: u16) {
    if !args.interactive {
        cmd.arg("-nographic");
    } else {
        cmd.arg("-serial").arg("stdio");
    }
    cmd.arg("-no-reboot");

    // GDB setup
    let gdb_port = match args.gdb_port {
        Some(p) if p > 0 => p,
        _ => default_gdb_port,
    };

    if args.gdb || args.frozen || args.gdb_port.is_some() {
        cmd.arg("-gdb").arg(format!("tcp::{}", gdb_port));
        println!("    GDB stub enabled on port {}...", gdb_port);
    }

    if args.frozen {
        println!("    Waiting for GDB connection (frozen)...");
        cmd.arg("-S");
    }
    
    // Memory default
    cmd.arg("-m").arg("2G");
}

fn run_qemu_x86_64(args: RunArgs) -> Result<()> {
    iso::run(args.env.clone(), args.cmdline.clone(), None)?;

    let root = project_root();
    let iso_path = root.join(format!("target/iso/thingos-{}.iso", args.env));
    let ovmf_dir = root.join("vendor/ovmf");
    let ovmf_code = ovmf_dir.join("ovmf-code-x86_64.fd");
    let ovmf_vars = ovmf_dir.join("ovmf-vars-x86_64.fd");

    let mut use_uefi = false;
    if ovmf_code.exists() && ovmf_vars.exists() {
        use_uefi = true;
    }

    println!("==> Running QEMU x86_64...");
    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.arg("-M").arg("q35");

    append_common_args(&mut cmd, &args, 1234);

    if use_uefi {
        cmd.arg("-drive").arg(format!("if=pflash,format=raw,readonly=on,file={}", ovmf_code.display()));
        cmd.arg("-drive").arg(format!("if=pflash,format=raw,readonly=on,file={}", ovmf_vars.display()));
    }
    
    cmd.arg("-cdrom").arg(&iso_path);
    run_with_timeout(cmd, args.timeout_secs)
}

fn run_qemu_aarch64(args: RunArgs) -> Result<()> {
    iso::run(args.env.clone(), args.cmdline.clone(), None)?;
    let root = project_root();
    let iso_path = root.join(format!("target/iso/thingos-{}.iso", args.env));
    let ovmf_dir = root.join("vendor/ovmf");
    let ovmf_code = ovmf_dir.join("ovmf-code-aarch64.fd");
    let ovmf_vars = ovmf_dir.join("ovmf-vars-aarch64.fd");

    println!("==> Running QEMU aarch64...");
    let mut cmd = Command::new("qemu-system-aarch64");
    cmd.arg("-M").arg("virt");
    cmd.arg("-cpu").arg("cortex-a72");
    cmd.arg("-device").arg("ramfb");
    cmd.arg("-device").arg("qemu-xhci");
    cmd.arg("-device").arg("usb-kbd");
    cmd.arg("-device").arg("usb-mouse");

    append_common_args(&mut cmd, &args, 2234);

    cmd.arg("-bios").arg(&ovmf_code);
    cmd.arg("-cdrom").arg(&iso_path);

    run_with_timeout(cmd, args.timeout_secs)
}

fn run_qemu_riscv64(args: RunArgs) -> Result<()> {
    iso::run(args.env.clone(), args.cmdline.clone(), None)?;
    let root = project_root();
    let iso_path = root.join(format!("target/iso/thingos-{}.iso", args.env));
    let ovmf_dir = root.join("vendor/ovmf");
    let ovmf_code = ovmf_dir.join("ovmf-code-riscv64.fd");
    let ovmf_vars = ovmf_dir.join("ovmf-vars-riscv64.fd");

    println!("==> Running QEMU riscv64...");
    let mut cmd = Command::new("qemu-system-riscv64");
    cmd.arg("-M").arg("virt");
    cmd.arg("-cpu").arg("rv64");
    cmd.arg("-device").arg("ramfb");
    cmd.arg("-device").arg("qemu-xhci");
    cmd.arg("-device").arg("usb-kbd");
    cmd.arg("-device").arg("usb-mouse");

    append_common_args(&mut cmd, &args, 3234);

    cmd.arg("-drive").arg(format!("if=pflash,unit=0,format=raw,readonly=on,file={}", ovmf_code.display()));
    cmd.arg("-drive").arg(format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars.display()));
    cmd.arg("-cdrom").arg(&iso_path);

    run_with_timeout(cmd, args.timeout_secs)
}

fn run_qemu_loongarch64(args: RunArgs) -> Result<()> {
    iso::run(args.env.clone(), args.cmdline.clone(), None)?;
    let root = project_root();
    let iso_path = root.join(format!("target/iso/thingos-{}.iso", args.env));
    let ovmf_dir = root.join("vendor/ovmf");
    let ovmf_code = ovmf_dir.join("ovmf-code-loongarch64.fd");
    let ovmf_vars = ovmf_dir.join("ovmf-vars-loongarch64.fd");

    println!("==> Running QEMU loongarch64...");
    let mut cmd = Command::new("qemu-system-loongarch64");
    cmd.arg("-M").arg("virt");
    cmd.arg("-cpu").arg("la464");
    cmd.arg("-device").arg("ramfb");
    cmd.arg("-device").arg("qemu-xhci");
    cmd.arg("-device").arg("usb-kbd");
    cmd.arg("-device").arg("usb-mouse");

    append_common_args(&mut cmd, &args, 4234);

    cmd.arg("-drive").arg(format!("if=pflash,unit=0,format=raw,readonly=on,file={}", ovmf_code.display()));
    cmd.arg("-drive").arg(format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars.display()));
    cmd.arg("-cdrom").arg(&iso_path);

    run_with_timeout(cmd, args.timeout_secs)
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}
