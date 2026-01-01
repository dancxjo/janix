use anyhow::{Context, Result};
use std::env;
use std::process::Command;

pub fn run(arch: Option<String>) -> Result<()> {
    let arch = arch.unwrap_or_else(|| "all".to_string());
    let root = project_root();

    println!("==> Running BDD tests for arch: {}...", arch);

    // Ensure we have a fresh build first?
    // Usually BDD runner might expect built artifacts.
    // The previous `make bdd` didn't explicitly depend on build, but `ci-bdd-runner` probably just runs QEMU which uses the ISO.
    // Let's protect the user by ensuring build/iso exists or just rely on them running `just build` first.
    // Ideally `xtask test` should do it all, but re-building every time is slow.
    // Let's assume user flow is `just build` then `just test`.
    // OR we can check if artifacts exist.
    
    // For now, let's just invoke the runner.
    
    let status = Command::new("cargo")
        .arg("run")
        .arg("-p")
        .arg("bdd")
        .env("ARCH", &arch)
        .current_dir(&root)
        .status()
        .context("Failed to run ci-bdd-runner")?;

    if !status.success() {
        anyhow::bail!("BDD tests failed");
    }

    Ok(())
}

fn project_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}
