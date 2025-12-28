use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

pub fn run() -> Result<()> {
    let root = project_root();
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    // 1. Build Kernel
    println!("==> Building kernel (x86_64)...");
    let status = Command::new(&cargo)
        .arg("build")
        .arg("-p")
        .arg("kernel_x86_64")
        .arg("--target")
        .arg("targets/x86_64-thingos.json")
        .arg("-Z")
        .arg("build-std=core,alloc,compiler_builtins")
        .arg("-Z")
        .arg("build-std-features=compiler-builtins-mem")
        .current_dir(&root)
        .status()
        .context("Failed to build kernel")?;

    if !status.success() {
        anyhow::bail!("Kernel build failed");
    }

    // 2. Build User Apps
    println!("==> Building user apps...");
    let user_apps = [
        "graph_dump",
        "ps2_keyboard",
        "keylog",
        "syscall_crud_smoke",
        "clock",
        "sleep_smoke",
        "rtc_x86",
        "rtc_aarch64",
        "sleep_accuracy_smoke",
        "ls_boot",
        "cat_boot",
        "fb_smoke",
    ];

    for app in user_apps {
        let status = Command::new(&cargo)
            .arg("build")
            .arg("--manifest-path")
            .arg("user/Cargo.toml")
            .arg("-p")
            .arg(app)
            .arg("--target")
            .arg("x86_64-unknown-none")
            .current_dir(&root)
            .status()
            .context(format!("Failed to build app {}", app))?;

        if !status.success() {
            anyhow::bail!("App {} build failed", app);
        }
    }

    Ok(())
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}
