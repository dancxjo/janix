//! BDD test runner task.

use xshell::{Shell, cmd};
use crate::common::{rust_target, Result};
use crate::build::build;
use crate::image::build_iso;
use crate::limine::limine;
use crate::ovmf::ovmf;

/// Run BDD tests for a single architecture.
pub fn bdd(
    sh: &Shell,
    feature: Option<String>,
    _tags: Option<String>,
    arch: Option<String>,
) -> Result<()> {
    // Default to x86_64 if no architecture specified
    let architecture = arch.unwrap_or_else(|| "x86_64".to_string());

    // Install rustup target
    let target = rust_target(&architecture);
    println!("Ensuring rustup target {} is installed...", target);
    // Ignore errors - target may already be installed or use build-std
    let _ = cmd!(sh, "rustup target add {target}").run();

    // Build ISO
    println!("\n=== Building for {} ===\n", architecture);
    ovmf(sh, &architecture)?;
    limine(sh)?;
    build(sh, &architecture, "dev")?;
    build_iso(sh, &architecture)?;

    // Run tests
    println!("\n=== Running BDD tests for {} ===\n", architecture);

    let mut cmd = std::process::Command::new("cargo");
    cmd.args(["run", "-p", "bdd"]);
    cmd.env("BDD_ARCH", &architecture);

    if let Some(ref f) = feature {
        cmd.env("BDD_FEATURE", f);
    }

    let status = cmd.spawn()?.wait()?;
    if !status.success() {
        return Err(format!("Tests failed for: {}", architecture).into());
    }

    println!("\n=== All tests passed ===\n");
    Ok(())
}
