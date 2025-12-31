use crate::steps::BootWorld;
use crate::writer::ArtifactWriter;
use cucumber::World;
use std::env;
use std::path::PathBuf;

mod qemu;
mod report;
mod shared;
mod steps;
mod writer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let arch = env::var("ARCH").unwrap_or_else(|_| "x86_64".to_string());

    // Default artifacts dir
    let out_str = env::var("ARTIFACTS").unwrap_or_else(|_| "artifacts".to_string());
    let out = PathBuf::from(out_str);

    // Default feature path
    let feature_str = env::var("FEATURE_PATH").unwrap_or_else(|_| "tools/bdd/features".to_string());
    let feature_path = PathBuf::from(feature_str);

    println!("Starting CI BDD Runner for arch: {}", arch);
    println!("Artifacts will be saved to: {}", out.display());
    println!("Running features from: {}", feature_path.display());

    // Clean output dir
    if out.exists() {
        if let Err(e) = std::fs::remove_dir_all(&out) {
            eprintln!("Warning: Failed to clear output dir: {}", e);
        }
    }
    std::fs::create_dir_all(&out)?;

    let writer = ArtifactWriter {
        out_dir: out.clone(),
        arch: arch.clone(),
        _run_id: uuid::Uuid::new_v4().to_string(),
        current_feature: String::new(),
        current_scenario: String::new(),
        step_index: 0,
    };

    println!("Starting Cucumber...");
    BootWorld::cucumber()
        .max_concurrent_scenarios(1)
        .with_writer(writer)
        .run(feature_path)
        .await;
    println!("Cucumber finished.");

    println!("Generating report...");
    // Generate report
    if let Err(e) = report::generate_report(&out) {
        eprintln!("Failed to generate report: {:?}", e);
    } else {
        println!("Report generated at {}/index.md", out.display());
    }
    println!("Done.");

    // Check for soft failures
    if crate::shared::ANY_FAILURE.load(std::sync::atomic::Ordering::SeqCst) {
        eprintln!("CI Failure: One or more steps failed.");
        std::process::exit(1);
    }

    Ok(())
}
