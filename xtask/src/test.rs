use anyhow::{Context, Result};
use std::process::Command;

pub fn run(arch: Option<String>, smoke: bool, feature: Option<String>) -> Result<()> {
    let arch = arch.unwrap_or_else(|| "all".to_string());
    
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
        
        if let Some(f) = &feature {
            cmd.env("BDD_FEATURE", f);
        }

        let status = cmd.status().context("Failed to run BDD tests")?;

        if !status.success() {
            anyhow::bail!("BDD tests failed for {}", a);
        }
    }

    Ok(())
}
