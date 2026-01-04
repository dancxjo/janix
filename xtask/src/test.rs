use anyhow::{Context, Result};
use std::process::Command;

pub fn run(
    arch: Option<String>,
    smoke: bool,
    feature: Option<String>,
    tag: Option<String>,
    force_all: bool,
) -> Result<()> {
    let arch = arch.unwrap_or_else(|| "all".to_string());
    let feature = feature.unwrap_or_else(|| "all".to_string());

    // Safety check: Prevent accidental massive test runs
    if arch == "all" && feature == "all" && tag.is_none() && !smoke && !force_all {
        anyhow::bail!(
            "SAFETY CHECK FAILED: You are trying to run ALL tests on ALL architectures.\n\
             This takes a long time and is usually not what you want.\n\
             \n\
             Please narrow your scope:\n\
             1. Use --tag to select specific scenarios (e.g. --tag '@mouse')\n\
             2. Use --feature to select a specific feature file\n\
             3. Use --arch to select a specific architecture\n\
             4. Use --smoke for a quick sanity check\n\
             \n\
             If you REALLY want to run everything, use --force-all."
        );
    }

    let archs = if arch == "all" {
        vec!["x86_64", "aarch64", "riscv64", "loongarch64"]
    } else {
        vec![arch.as_str()]
    };

    for a in archs {
        println!("\n==> Running BDD tests for {}...", a);
        let mut cmd = Command::new("cargo");
        cmd.args(["run", "-p", "bdd"]);

        cmd.env("BDD_ARCH", a);

        if smoke {
            cmd.env("BDD_SMOKE", "true");
        }

        // Always pass BDD_FEATURE so the bdd tool knows (it defaults to "all" if missing, but let's be explicit)
        cmd.env("BDD_FEATURE", &feature);

        if let Some(t) = &tag {
            cmd.env("BDD_TAG", t);
        }

        let status = cmd.status().context("Failed to run BDD tests")?;

        if !status.success() {
            anyhow::bail!("BDD tests failed for {}", a);
        }
    }

    Ok(())
}
