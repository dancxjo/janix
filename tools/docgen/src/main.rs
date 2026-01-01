use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct RunMeta {
    pub git_sha: Option<String>,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ResultsStore {
    #[serde(default)]
    pub run: RunMeta,
    #[serde(default)]
    pub scenarios: Vec<ScenarioResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ScenarioResult {
    pub feature: String,
    pub scenario: String,
    pub results: HashMap<String, String>, // arch -> status
}

fn main() -> Result<()> {
    let results_path = PathBuf::from("artifacts/bdd/results.json");
    if !results_path.exists() {
        eprintln!("Error: canonical results.json not found at {}", results_path.display());
        std::process::exit(1);
    }

    println!("Loading BDD results from {}", results_path.display());
    let content = fs::read_to_string(&results_path)?;
    let store: ResultsStore = serde_json::from_str(&content)?;

    println!("Found {} scenarios.", store.scenarios.len());

    // Group by feature
    let mut features: HashMap<String, Vec<&ScenarioResult>> = HashMap::new();
    for scen in &store.scenarios {
        features.entry(scen.feature.clone()).or_default().push(scen);
    }
    
    // Sort features for stable output
    let mut sorted_features: Vec<_> = features.keys().collect();
    sorted_features.sort();

    // Generate Markdown
    let mut md = String::new();
    md.push_str("## Test Status\n\n");
    md.push_str("> _This section is auto-generated from BDD test results. Do not edit by hand._\n\n");
    
    let arches = vec!["x86_64", "aarch64", "riscv64", "loongarch64"];
    
    for feat_name in sorted_features {
        md.push_str(&format!("### {}\n\n", feat_name));
        md.push_str("| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 |\n");
        md.push_str("|----------|--------|---------|---------|-------------|\n");
        
        let scenarios = &features[feat_name];
        // Sort scenarios by name
        let mut sorted_scenarios = scenarios.clone();
        sorted_scenarios.sort_by_key(|s| &s.scenario);

        for scen in sorted_scenarios {
            md.push_str(&format!("| {} |", scen.scenario));
            for arch in &arches {
                let status = scen.results.get(*arch).map(|s| s.as_str()).unwrap_or("-");
                let icon = match status {
                    "pass" => "✅",
                    "failed" => "❌",
                    _ => "⚪"
                };
                md.push_str(&format!(" {} |", icon));
            }
            md.push_str("\n");
        }
        md.push_str("\n");
    }
    
    update_readme(&md)?;
    
    Ok(())
}

fn update_readme(status_md: &str) -> Result<()> {
    let readme_path = PathBuf::from("README.md");
    if !readme_path.exists() {
        return Ok(());
    }
    
    let content = fs::read_to_string(&readme_path)?;
    let start_marker = "<!-- DOCGEN:STATUS:BEGIN -->";
    let end_marker = "<!-- DOCGEN:STATUS:END -->";
    
    if let (Some(start), Some(end)) = (content.find(start_marker), content.find(end_marker)) {
        let mut new_content = String::with_capacity(content.len());
        new_content.push_str(&content[..start + start_marker.len()]);
        new_content.push_str("\n");
        new_content.push_str(status_md);
        new_content.push_str(&content[end..]);
        
        fs::write(readme_path, new_content)?;
        println!("README.md updated.");
    } else {
        println!("Markdown markers not found in README.md");
    }
    
    Ok(())
}
