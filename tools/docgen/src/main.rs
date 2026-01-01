use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RunReport {
    feature: String,
    scenario: String,
    arch: String,
    status: String,
    artifacts_dir: String,
}

#[derive(Serialize)]
struct ScenarioSummary {
    name: String,
    results: HashMap<String, String>, // arch -> status
    artifacts: HashMap<String, String>, // arch -> dir
}

fn main() -> Result<()> {
    let artifacts_dir = PathBuf::from("artifacts/bdd");
    if !artifacts_dir.exists() {
        println!("No BDD artifacts found at {}", artifacts_dir.display());
        return Ok(());
    }

    println!("Scanning artifacts...");
    let mut reports = Vec::new();

    for entry in WalkDir::new(&artifacts_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().map_or(false, |e| e == "json") {
            let content = fs::read_to_string(entry.path())?;
            if let Ok(report) = serde_json::from_str::<RunReport>(&content) {
                reports.push(report);
            }
        }
    }

    println!("Found {} reports.", reports.len());

    let mut features: HashMap<String, HashMap<String, ScenarioSummary>> = HashMap::new();

    for report in reports {
        let feat = features.entry(report.feature.clone()).or_default();
        let scen = feat.entry(report.scenario.clone()).or_insert_with(|| ScenarioSummary {
            name: report.scenario.clone(),
            results: HashMap::new(),
            artifacts: HashMap::new(),
        });
        
        scen.results.insert(report.arch.clone(), report.status.clone());
        scen.artifacts.insert(report.arch.clone(), report.artifacts_dir.clone());
    }

    // Generate Markdown
    let mut md = String::new();
    md.push_str("## Test Status\n\n");
    
    let arches = vec!["x86_64", "aarch64", "riscv64", "loongarch64"];
    
    for (feat_name, scenarios) in features {
        md.push_str(&format!("### {}\n\n", feat_name));
        md.push_str("| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 |\n");
        md.push_str("|----------|--------|---------|---------|-------------|\n");
        
        for (_, scen) in scenarios {
            md.push_str(&format!("| {} |", scen.name));
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
