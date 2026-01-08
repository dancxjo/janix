//! BDD Test Runner for Thing-OS
//!
//! Runs cucumber-rs tests against the OS in QEMU.
//! 
//! Configuration via environment variables:
//! - BDD_ARCH: Target architecture (default: x86_64)
//! - BDD_FEATURE: Specific feature file to run (optional)

mod steps;
mod world;

use cucumber::World;
use std::path::PathBuf;
use world::ThingOsWorld;

fn main() {
    // Get configuration from environment
    let arch = std::env::var("BDD_ARCH").unwrap_or_else(|_| "x86_64".to_string());
    let feature = std::env::var("BDD_FEATURE").ok();

    println!("\n=== Running BDD tests for {} ===\n", arch);

    // Build features path
    let features_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("features");
    let features_path = if let Some(f) = feature {
        features_dir.join(format!("{}.feature", f))
    } else {
        features_dir
    };

    // Run cucumber with tokio runtime
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(
            ThingOsWorld::cucumber()
                .run(features_path)
        );
}
