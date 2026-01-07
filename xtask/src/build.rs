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
        .env("RUSTFLAGS", "-C relocation-model=static")
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
        .arg("apps/bloom/Cargo.toml")
        .arg("--target")
        .arg(target)
        .arg("-Z")
        .arg("build-std=core,alloc,compiler_builtins")
        .env("RUSTFLAGS", "-C relocation-model=static")
        .env("RUSTC_BOOTSTRAP", "1")
        .current_dir(&root)
        .status()
        .context("Failed to build bloom")?;

    if !status.success() {
        anyhow::bail!("Bloom build failed");
    }

    // 3.5 Build Clock (Demo App)
    println!("    Building clock...");
    let status = Command::new(&cargo)
        .arg("build")
        .arg("--manifest-path")
        .arg("apps/clock/Cargo.toml")
        .arg("--target")
        .arg(target)
        .arg("-Z")
        .arg("build-std=core,alloc,compiler_builtins")
        .env("RUSTFLAGS", "-C relocation-model=static")
        .env("RUSTC_BOOTSTRAP", "1")
        .current_dir(&root)
        .status()
        .context("Failed to build clock")?;

    if !status.success() {
        anyhow::bail!("Clock build failed");
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
        .env("RUSTFLAGS", "-C relocation-model=static")
        .env("RUSTC_BOOTSTRAP", "1")
        .current_dir(&root)
        .status()
        .context("Failed to build sprout_exit")?;

    if !status.success() {
        anyhow::bail!("Sprout Exit build failed");
    }

    // 5. Build Heap Smoke (Test Module)
    println!("    Building heap_smoke...");
    let status = Command::new(&cargo)
        .arg("build")
        .arg("--manifest-path")
        .arg("crates/heap_smoke/Cargo.toml")
        .arg("--target")
        .arg(target)
        .arg("-Z")
        .arg("build-std=core,alloc,compiler_builtins")
        .env("RUSTFLAGS", "-C relocation-model=static")
        .env("RUSTC_BOOTSTRAP", "1")
        .current_dir(&root)
        .status()
        .context("Failed to build heap_smoke")?;

    if !status.success() {
        anyhow::bail!("Heap Smoke build failed");
    }

    // 6. Build Apps
    let apps = [
        "apps/graph_smoke",
        "apps/log_smoke",
        "apps/cap_fail",
        "apps/inputd",
        "apps/echo",
        "apps/inspector",
        "apps/logview",
        "apps/simd_check",
        "apps/thingcheck",
        "apps/timed",
        "drivers/rtc_cmos",
        "drivers/rtc_pl031",
        "apps/pcid",
        "apps/hello_window",
        "apps/thread_test",
        "apps/textd",
    ];
    for app in apps {
        println!("    Building {}...", app);
        let status = Command::new(&cargo)
            .arg("build")
            .arg("--manifest-path")
            .arg(format!("{}/Cargo.toml", app))
            .arg("--target")
            .arg(target)
            .arg("-Z")
            .arg("build-std=core,alloc,compiler_builtins")
            .env("RUSTFLAGS", "-C relocation-model=static")
            .env("RUSTC_BOOTSTRAP", "1")
            .current_dir(&root)
            .status()
            .context(format!("Failed to build {}", app))?;

        if !status.success() {
            anyhow::bail!("{} build failed", app);
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
