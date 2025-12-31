use crate::steps::BootWorld;
use crate::writer::ArtifactWriter;
use cucumber::{Cucumber, World};
use std::env;
use std::path::PathBuf;

mod qemu;
mod writer;
mod shared;
mod steps;
mod report;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let arch = env::var("ARCH").unwrap_or_else(|_| "x86_64".to_string());
    
    // Default artifacts dir
    let out_str = env::var("ARTIFACTS").unwrap_or_else(|_| "artifacts".to_string());
    let out = PathBuf::from(out_str);

    // Default feature path
    let feature_str = env::var("FEATURE_PATH").unwrap_or_else(|_| "tools/bdd/features".to_string());
    let feature_path = PathBuf::from(feature_str);

    // Install panic hook
    std::panic::set_hook(Box::new(|info| {
        eprintln!("Panic occurred: {:?}", info);
        if let Some(s) = info.payload().downcast_ref::<&str>() {
            eprintln!("Panic payload: {}", s);
        }
    }));

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
        run_id: uuid::Uuid::new_v4().to_string(),
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
    match report::generate_report(&out) {
        Ok(_) => println!("Report generated at {}/index.md", out.display()),
        Err(e) => eprintln!("Failed to generate report: {:?}", e),
    }
    println!("Done.");
    
    Ok(())
}
