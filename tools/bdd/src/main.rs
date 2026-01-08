//! BDD Test Runner for Thing-OS
//!
//! Runs cucumber-rs tests against the OS in QEMU.
//! 
//! Configuration via environment variables:
//! - BDD_ARCH: Target architecture (default: x86_64)
//! - BDD_FEATURE: Specific feature file to run (optional)

mod artifacts;
mod reporter;
mod steps;
mod world;

use cucumber::World;
use std::path::PathBuf;
use std::fs;
use world::ThingOsWorld;
use reporter::ThingOsReporter;

fn main() {
    // Get configuration from environment
    let arch = std::env::var("BDD_ARCH").unwrap_or_else(|_| "x86_64".to_string());
    let feature = std::env::var("BDD_FEATURE").ok();

    eprintln!("[bdd] Running tests for architecture: {}", arch);

    // Build features path
    let features_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("features");
    let features_path = if let Some(f) = feature {
        features_dir.join(format!("{}.feature", f))
    } else {
        features_dir
    };

    // Create custom reporter with artifact collection
    let reporter = ThingOsReporter::new(&arch);

    // Create output directory and JSON file
    let output_dir = PathBuf::from("docs/behavior").join(&arch);
    let _ = fs::create_dir_all(&output_dir);
    let json_file = fs::File::create(output_dir.join("results.json"))
        .expect("Failed to create results.json");

    // Create JSON writer for structured output
    let json_writer = cucumber::writer::Json::new(json_file);

    // Run cucumber with tokio runtime and both reporters
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(
            ThingOsWorld::cucumber()
                .with_writer(
                    cucumber::writer::Tee::new(reporter, json_writer)
                )
                .run(features_path)
        );
}
