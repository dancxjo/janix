use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

pub fn run(env: String, port: Option<u16>) -> Result<()> {
    let root = project_root();
    
    let (target_triple, gdb_bin, gdb_arch, default_port) = match env.as_str() {
        "x86_64" => ("x86_64-unknown-none", "gdb", "i386:x86-64", 1234),
        "aarch64" => ("aarch64-unknown-none", "gdb-multiarch", "aarch64", 2234),
        "riscv64" => ("riscv64gc-unknown-none-elf", "gdb-multiarch", "riscv:rv64", 3234),
        "loongarch64" => ("loongarch64-unknown-none", "gdb-multiarch", "loongarch64", 4234),
        _ => anyhow::bail!("Unsupported env for inspect: {}", env),
    };

    let port = port.unwrap_or(default_port);

    let kernel_path = root.join("target").join(target_triple).join("debug/bran");
    if !kernel_path.exists() {
        anyhow::bail!("Kernel binary not found at {}. Run `just build` first.", kernel_path.display());
    }

    println!("==> Starting GDB for {}...", env);
    println!("    Binary: {}", kernel_path.display());
    println!("    Target: localhost:{}", port);

    // Prefer GDB from env if set
    let gdb_bin = std::env::var("GDB_BIN").unwrap_or_else(|_| gdb_bin.to_string());

    let mut cmd = Command::new(&gdb_bin);
    // Batch mode or interactive? User request: "get the right gdb session... right decompiled code".
    // Usually interactive gdb session.
    // GNUmakefile `inspect` used `-batch ... -ex "quit"`. This implies it just prints info and exits.
    // User said "get the right gdb session".
    // If they want interactive, we shouldn't use -batch unless they asked for the makefile's behavior which WAS batch.
    // The makefile `inspect` target prints registers, backtrace, instructions, source, then quits.
    // "Look at the way make inspect-* was running." -> It was batch.
    
    cmd.arg("-batch");
    if !gdb_arch.is_empty() {
        cmd.arg("-ex").arg(format!("set architecture {}", gdb_arch));
    }
    cmd.arg("-ex").arg(format!("file {}", kernel_path.display()));
    cmd.arg("-ex").arg(format!("target remote :{}", port));
    cmd.arg("-ex").arg("set pagination off");
    
    // Commands from makefile
    cmd.arg("-ex").arg("echo \\n--- REGISTERS ---\\n");
    cmd.arg("-ex").arg("info registers");
    cmd.arg("-ex").arg("echo \\n--- BACKTRACE ---\\n");
    cmd.arg("-ex").arg("bt");
    cmd.arg("-ex").arg("echo \\n--- INSTRUCTIONS ---\\n");
    cmd.arg("-ex").arg("x/10i $pc");
    cmd.arg("-ex").arg("echo \\n--- SOURCE ---\\n");
    cmd.arg("-ex").arg("list *$pc");
    cmd.arg("-ex").arg("quit");

    let status = cmd.status().context("Failed to run GDB")?;
    if !status.success() {
        anyhow::bail!("GDB exited with failure");
    }

    Ok(())
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}
