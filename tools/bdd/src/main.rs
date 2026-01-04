use crate::steps::BootWorld;
use crate::writer::ArtifactWriter;
use cucumber::World;
use std::env;
use std::path::PathBuf;

mod qemu;
mod report;
mod shared;
mod steps;
mod store;
mod writer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let arch = env::var("ARCH").unwrap_or_else(|_| "all".to_string());

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
        scenario_failed: false,
    };

    println!("Starting Cucumber...");
    let runner = BootWorld::cucumber()
        .max_concurrent_scenarios(1)
        .with_writer(writer);

    let is_smoke = env::var("SMOKE_TEST").map(|v| v == "1").unwrap_or(false);

    if arch != "all" || is_smoke {
        // Create list of architectures to match
        let archs: Vec<String> = if arch.contains(',') {
            arch.split(',').map(|s| s.to_string()).collect()
        } else {
            vec![arch.clone()]
        };

        if is_smoke {
            println!("Filtering for SMOKE tests only.");
        }

        println!("Filtering scenarios for architectures: {:?}", archs);

        runner
            .filter_run(feature_path, move |_, _, scenario| {
                // If smoke test is requested, strictly require @smoke tag
                if is_smoke {
                    if !scenario.tags.iter().any(|t| t == "smoke") {
                        return false;
                    }
                }

                // If arch is all (and we are here only because of smoke), we don't filter by arch
                if arch == "all" {
                    return true;
                }

                // Check if arch name appears in scenario name
                for a in &archs {
                    if scenario.name.contains(a) || scenario.name.contains(&a.to_uppercase()) {
                        return true;
                    }
                }

                // Check if any step value mentions the arch
                for step in &scenario.steps {
                    for a in &archs {
                        // Match "arch", "\"arch\"", etc.
                        if step.value.contains(a) || step.value.contains(&format!("\"{}\"", a)) {
                            return true;
                        }
                    }
                }

                false
            })
            .await;
    } else {
        println!("Running all architectures.");
        runner.run(feature_path).await;
    }

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
