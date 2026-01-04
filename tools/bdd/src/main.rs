use anyhow::Result;
use cucumber::World;
use std::path::PathBuf;

mod extra_steps;
mod keyboard_steps;
mod qemu;
mod report;
mod shared;
mod steps;
mod store;
mod writer;

#[tokio::main]
async fn main() -> Result<()> {
    // Read configuration from environment variables passed by xtask
    let arch = std::env::var("BDD_ARCH").unwrap_or_else(|_| "x86_64".to_string());
    
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_dir.parent().unwrap().parent().unwrap().to_path_buf();
    let feature_dir = manifest_dir.join("features");

    let writer = writer::ArtifactWriter {
        out_dir: root.join("artifacts"),
        arch: arch.clone(),
        _run_id: uuid::Uuid::new_v4().to_string(),
        current_feature: String::new(),
        current_scenario: String::new(),
        step_index: 0,
        scenario_failed: false,
        current_steps: Vec::new(),
    };
    
    println!("Running BDD suite from: {}", feature_dir.display());
    println!("Architecture: {}", arch);
    
    use crate::steps::BootWorld;

    let feature_filter = std::env::var("BDD_FEATURE").unwrap_or_else(|_| "all".to_string());
    
    if feature_filter == "all" {
        BootWorld::cucumber()
            .max_concurrent_scenarios(1)
            .with_writer(writer)
            .run(feature_dir)
            .await;
    } else {
        // Simple filter on the feature file name
        let filter = format!("{}.feature", feature_filter);
        BootWorld::cucumber()
            .max_concurrent_scenarios(1)
            .with_writer(writer)
            .filter_run(feature_dir, move |f, _r, _s| {
                f.path.as_ref()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    .map(|n| n.contains(&filter))
                    .unwrap_or(false)
            })
            .await;
    }

    if shared::ANY_FAILURE.load(std::sync::atomic::Ordering::SeqCst) {
        std::process::exit(101);
    }

    Ok(())
}
