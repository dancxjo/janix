use anyhow::Result;
use docgen::{ResultsStore, update_readme_from_results};
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    let results_path = PathBuf::from("artifacts/bdd/results.json");
    if !results_path.exists() {
        eprintln!(
            "Error: canonical results.json not found at {}",
            results_path.display()
        );
        std::process::exit(1);
    }

    println!("Loading BDD results from {}", results_path.display());
    let content = fs::read_to_string(&results_path)?;
    let store: ResultsStore = serde_json::from_str(&content)?;

    println!("Found {} features.", store.features.len());

    update_readme_from_results(&store, Path::new("."))?;

    Ok(())
}
