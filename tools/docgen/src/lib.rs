use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RunMeta {
    pub git_sha: Option<String>,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ResultsStore {
    #[serde(default)]
    pub run: RunMeta,
    #[serde(default)]
    pub features: BTreeMap<String, FeatureResult>,
    #[serde(default)]
    pub scenarios: Vec<LegacyScenarioResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FeatureResult {
    #[serde(default)]
    pub scenarios: BTreeMap<String, ScenarioEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ScenarioEntry {
    #[serde(default)]
    pub arches: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LegacyScenarioResult {
    #[serde(default)]
    pub feature: String,
    pub scenario: String,
    pub results: HashMap<String, String>, // arch -> status
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScenarioRow {
    pub feature: String,
    pub scenario: String,
    pub results: HashMap<String, String>, // arch -> status
}

pub fn update_readme_from_results(store: &ResultsStore, root: &Path) -> Result<()> {
    let mut rows: Vec<ScenarioRow> = Vec::new();
    if !store.features.is_empty() {
        for (feature_name, feature) in &store.features {
            for (scenario_name, scenario) in &feature.scenarios {
                rows.push(ScenarioRow {
                    feature: feature_name.clone(),
                    scenario: scenario_name.clone(),
                    results: scenario.arches.clone().into_iter().collect(),
                });
            }
        }
    } else {
        for scen in &store.scenarios {
            rows.push(ScenarioRow {
                feature: scen.feature.clone(),
                scenario: scen.scenario.clone(),
                results: scen.results.clone(),
            });
        }
    }

    let mut features: HashMap<String, Vec<&ScenarioRow>> = HashMap::new();
    for scen in &rows {
        features.entry(scen.feature.clone()).or_default().push(scen);
    }

    let mut sorted_features: Vec<_> = features.keys().collect();
    sorted_features.sort();

    let mut md = String::new();
    md.push_str("## Test Status\n\n");
    md.push_str(
        "> _This section is auto-generated from BDD test results. Do not edit by hand._\n\n",
    );

    let arches = vec!["x86_64", "aarch64", "riscv64", "loongarch64"];

    for feat_name in sorted_features {
        md.push_str(&format!("### {}\n\n", feat_name));
        md.push_str("| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 |\n");
        md.push_str("|----------|--------|---------|---------|-------------|\n");

        let scenarios = &features[feat_name];
        let mut sorted_scenarios = scenarios.clone();
        sorted_scenarios.sort_by_key(|s| &s.scenario);

        for scen in sorted_scenarios {
            md.push_str(&format!("| {} |", scen.scenario));
            for arch in &arches {
                let status = scen.results.get(*arch).map(|s| s.as_str()).unwrap_or("-");
                let icon = match status {
                    "pass" | "passed" => "✅",
                    "failed" => "❌",
                    _ => "⚪",
                };
                md.push_str(&format!(" {} |", icon));
            }
            md.push_str("\n");
        }
        md.push_str("\n");
    }

    update_readme(&md, root)?;
    Ok(())
}

fn update_readme(status_md: &str, root: &Path) -> Result<()> {
    let readme_path = root.join("README.md");
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
    }
    Ok(())
}
