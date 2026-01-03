use anyhow::{Context, Result};
use std::env;
use std::process::Command;

pub fn run(arch: Option<String>, smoke: bool, feature: Option<String>) -> Result<()> {
    let arch = arch.unwrap_or_else(|| "all".to_string());
    let feature = feature.unwrap_or_else(|| "all".to_string());
    let root = project_root();

    println!("==> Running BDD tests for arch: {}, feature: {}...", arch, feature);

    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("-p")
        .arg("bdd")
        .env("ARCH", &arch)
        .env("SMOKE_TEST", if smoke { "1" } else { "0" })
        .current_dir(&root);

    if feature != "all" {
        let feature_filename = if feature.ends_with(".feature") {
            feature.clone()
        } else {
            format!("{}.feature", feature)
        };
        let feature_path = root.join("tools/bdd/features").join(feature_filename);
        if !feature_path.exists() {
            anyhow::bail!("Feature file not found: {}", feature_path.display());
        }
        cmd.env("FEATURE_PATH", feature_path);
    }

    let status = cmd.status()
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
