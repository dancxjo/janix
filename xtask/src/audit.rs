use anyhow::{Context, Result};
use cargo_metadata::MetadataCommand;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

const ALLOWED_STD_CRATES: &[&str] = &[
    "xtask",
    "pciids",
    "bdd",
    "unifont-gen",
    "display_proto_tests",
    "abi-macros",
    "stem-macros",
];

const REQUIRED_NOSTD_CRATES: &[&str] =
    &["kernel", "stem", "stem-macros", "abi", "abi-macros", "bran"];

pub fn audit() -> Result<()> {
    println!("🔍 Platform Boundary Audit");
    println!("============================================================");

    let metadata = MetadataCommand::new()
        .exec()
        .context("Failed to run cargo metadata")?;

    let mut errors = Vec::new();
    let mut count = 0;

    let allowed_std: HashSet<&str> = ALLOWED_STD_CRATES.iter().copied().collect();
    let required_nostd: HashSet<&str> = REQUIRED_NOSTD_CRATES.iter().copied().collect();

    for package in &metadata.packages {
        // Only check workspace members
        if !metadata.workspace_members.contains(&package.id) {
            continue;
        }

        let name = package.name.as_str();

        if allowed_std.contains(name) {
            println!("✓ {:30} [std allowed - build tool]", name);
            continue;
        }

        let manifest_path = package.manifest_path.as_std_path();
        // Check if it's in userspace (heuristic: path contains "userspace")
        let is_userspace = manifest_path
            .components()
            .any(|c| c.as_os_str() == "userspace");
        let is_kernel_or_core = required_nostd.contains(name);

        if is_userspace || is_kernel_or_core {
            count += 1;
            let crate_root = manifest_path.parent().unwrap();
            if is_nostd_crate(crate_root) {
                println!("✓ {:30} [no_std compliant]", name);
            } else {
                let msg = format!("{} is missing #![no_std] declaration", name);
                println!("✗ {:30} [MISSING #![no_std]]", name);
                errors.push(msg);
            }
        } else {
            println!("- {:30} [skipped]", name);
        }
    }

    println!("============================================================");
    println!("Checked {} crates for no_std compliance", count);

    if !errors.is_empty() {
        println!("\n❌ ERRORS:");
        for error in &errors {
            println!("  - {}", error);
        }
        anyhow::bail!("Audit failed with {} errors", errors.len());
    }

    println!("\n✅ Platform boundary audit passed!");
    println!("   All kernel/userspace crates are no_std compliant.");

    Ok(())
}

fn is_nostd_crate(crate_path: &Path) -> bool {
    let src_dir = crate_path.join("src");
    for file_name in ["lib.rs", "main.rs"] {
        let file_path = src_dir.join(file_name);
        if file_path.exists() {
            if let Ok(content) = fs::read_to_string(&file_path) {
                // Check first 20 lines
                for line in content.lines().take(20) {
                    if line.contains("#![no_std]")
                        || line.contains("#![cfg_attr(not(test), no_std)]")
                    {
                        return true;
                    }
                }
            }
        }
    }
    false
}
