use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

pub fn run() -> Result<()> {
    let root = project_root();
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    // 1. Build Bran (Kernel)
    println!("==> Building bran (x86_64)...");
    let status = Command::new(&cargo)
        .arg("build")
        .arg("--manifest-path")
        .arg("crates/bran/Cargo.toml")
        .arg("--target")
        .arg("x86_64-unknown-none") // Bran makefile uses this by default
        .env("RUSTFLAGS", "-C relocation-model=static")
        .current_dir(&root)
        .status()
        .context("Failed to build bran")?;

    if !status.success() {
        anyhow::bail!("Bran build failed");
    }

    // 2. Build Sprout (Userland)
    println!("==> Building sprout (x86_64)...");
    let status = Command::new(&cargo)
        .arg("build")
        .arg("--manifest-path")
        .arg("crates/sprout/Cargo.toml")
        .arg("--target")
        .arg("x86_64-unknown-none")
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
    println!("==> Building bloom (x86_64)...");
    let status = Command::new(&cargo)
        .arg("build")
        .arg("--manifest-path")
        .arg("crates/bloom/Cargo.toml")
        .arg("--target")
        .arg("x86_64-unknown-none")
        .arg("-Z")
        .arg("build-std=core,alloc,compiler_builtins")
        .env("RUSTFLAGS", "-C relocation-model=pic -C link-arg=-pie") // Assumed same as Sprout
        .env("RUSTC_BOOTSTRAP", "1")
        .current_dir(&root)
        .status()
        .context("Failed to build bloom")?;

    if !status.success() {
        anyhow::bail!("Bloom build failed");
    }

    Ok(())
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}
