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
        _ => anyhow::bail!(
            "Unsupported env for run: {}. Currently only x86_64 is supported in this xtask port.",
            args.env
        ),
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
        use_uefi = true;
    } else {
        eprintln!("[WARNING] OVMF files missing ({:?}). QEMU will use default BIOS.", ovmf_code);
    }

    println!("==> Running QEMU x86_64...");

    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.arg("-M").arg("q35");
    cmd.arg("-m").arg("2G"); // Match GNUmakefile default
    if !interactive {
        cmd.arg("-nographic");
    } else {
        cmd.arg("-serial").arg("stdio");
    }
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
    
    cmd.arg("-cdrom").arg(&iso_path);

    // GDB setup
    if let Some(port) = gdb_port {
        cmd.arg("-gdb").arg(format!("tcp::{}", port));
        println!("    GDB stub enabled on custom port {}...", port);
    } else {
        // GNUmakefile: run-gdb sets WITH_GDB=1 -> "-gdb tcp::1234"
        if gdb {
             cmd.arg("-s"); // Shorthand for -gdb tcp::1234
             println!("    GDB stub enabled on default port 1234...");
        }
    }

    if gdb {
        println!("    Waiting for GDB connection (frozen)...");
        cmd.arg("-S");
    }

    run_with_timeout(cmd, timeout)
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}
