use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

pub fn run(env: &str) -> Result<()> {
    let root = project_root();
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    let target = match env {
        "x86_64" => "x86_64-unknown-none",
        "aarch64" => "aarch64-unknown-none",
        "riscv64" => "riscv64gc-unknown-none-elf",
        "loongarch64" => "loongarch64-unknown-none",
        _ => anyhow::bail!("Unsupported architecture for build: {}", env),
    };

    println!("==> Building for {} (target: {})...", env, target);

    // 1. Build Bran (Kernel)
    println!("    Building bran...");
    let status = Command::new(&cargo)
        .arg("build")
        .arg("--manifest-path")
        .arg("crates/bran/Cargo.toml")
        .arg("--target")
        .arg(target)
        .env("RUSTFLAGS", "-C relocation-model=static")
        .env("RUSTC_BOOTSTRAP", "1")
        .current_dir(&root)
        .status()
        .context("Failed to build bran")?;

    if !status.success() {
        anyhow::bail!("Bran build failed");
    }

    // 2. Build Sprout (Userland)
    println!("    Building sprout...");
    let status = Command::new(&cargo)
        .arg("build")
        .arg("--manifest-path")
        .arg("crates/sprout/Cargo.toml")
        .arg("--target")
        .arg(target)
        .arg("-Z")
        .arg("build-std=core,alloc,compiler_builtins")
        .env("RUSTFLAGS", "-C relocation-model=pic -C link-arg=-pie")
        .env("RUSTC_BOOTSTRAP", "1")
        .current_dir(&root)
        .status()
        .context("Failed to build sprout")?;

    if !status.success() {
        anyhow::bail!("Sprout build failed");
    }

    // 3. Build Bloom (Desktop)
    println!("    Building bloom...");
    let status = Command::new(&cargo)
        .arg("build")
        .arg("--manifest-path")
        .arg("crates/bloom/Cargo.toml")
        .arg("--target")
        .arg(target)
        .arg("-Z")
        .arg("build-std=core,alloc,compiler_builtins")
        .env("RUSTFLAGS", "-C relocation-model=pic -C link-arg=-pie")
        .env("RUSTC_BOOTSTRAP", "1")
        .current_dir(&root)
        .status()
        .context("Failed to build bloom")?;


    if !status.success() {
        anyhow::bail!("Bloom build failed");
    }

    // 4. Build Sprout Exit (Test Module)
    println!("    Building sprout_exit...");
    let status = Command::new(&cargo)
        .arg("build")
        .arg("--manifest-path")
        .arg("crates/sprout_exit/Cargo.toml")
        .arg("--target")
        .arg(target)
        .arg("-Z")
        .arg("build-std=core,alloc,compiler_builtins")
        .env("RUSTFLAGS", "-C relocation-model=pic -C link-arg=-pie")
        .env("RUSTC_BOOTSTRAP", "1")
        .current_dir(&root)
        .status()
        .context("Failed to build sprout_exit")?;

    if !status.success() {
        anyhow::bail!("Sprout Exit build failed");
    }

    Ok(())
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}
