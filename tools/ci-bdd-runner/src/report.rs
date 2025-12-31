use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::BTreeMap;

#[derive(Deserialize, Debug, Clone)]
struct ArtifactMeta {
    version: u32,
    arch: String,
    feature: String,
    scenario: String,
    step: StepMeta,
    artifacts: Artifacts,
}

#[derive(Deserialize, Debug, Clone)]
struct StepMeta {
    index: usize,
    text: String,
    status: String,
    timestamp: String,
}

#[derive(Deserialize, Debug, Clone)]
struct Artifacts {
    screenshot: Option<String>,
    serial_tail: Option<String>,
}

pub fn generate_report(out_dir: &Path) -> Result<()> {
    // We walk the structure: Arch -> Feature -> Scenario -> Steps
    // But since directories are slugified, we rely on reading meta.json to get nice names.
    // We can traverse recursively and collect all meta.json files.

    let mut metas = Vec::new();
    visit_dirs(out_dir, &mut metas)?;

    // Group by Arch -> Feature -> Scenario
    // Map<Arch, Map<Feature, Map<Scenario, Vec<Meta>>>>
    let mut tree: BTreeMap<String, BTreeMap<String, BTreeMap<String, Vec<(PathBuf, ArtifactMeta)>>>> = BTreeMap::new();

    for (path, meta) in metas {
        tree.entry(meta.arch.clone())
            .or_default()
            .entry(meta.feature.clone())
            .or_default()
            .entry(meta.scenario.clone())
            .or_default()
            .push((path, meta));
    }

    // Generate Markdown
    let encoded = generate_markdown(out_dir, &tree);
    fs::write(out_dir.join("index.md"), encoded).context("Failed to write index.md")?;
    
    Ok(())
}

fn visit_dirs(dir: &Path, cb: &mut Vec<(PathBuf, ArtifactMeta)>) -> Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else if path.file_name().unwrap_or_default() == "meta.json" {
                let content = fs::read_to_string(&path)?;
                if let Ok(meta) = serde_json::from_str::<ArtifactMeta>(&content) {
                    // Start relative path from step dir
                    let step_dir = path.parent().unwrap().to_path_buf();
                    cb.push((step_dir, meta));
                }
            }
        }
    }
    Ok(())
}

fn generate_markdown(
    out_dir: &Path,
    tree: &BTreeMap<String, BTreeMap<String, BTreeMap<String, Vec<(PathBuf, ArtifactMeta)>>>>
) -> String {
    let mut md = String::new();
    md.push_str("# BDD Test Report\n\n");

    for (arch, features) in tree {
        md.push_str(&format!("## Architecture: {}\n\n", arch));
        
        for (feature, scenarios) in features {
            md.push_str(&format!("### Feature: {}\n\n", feature));
            
            for (scenario, steps) in scenarios {
                md.push_str(&format!("#### Scenario: {}\n\n", scenario));
                
                md.push_str("| Index | Step | Status | Artifacts |\n");
                md.push_str("| :---: | --- | :---: | --- |\n");
                
                // Sort steps by index
                let mut steps = steps.clone();
                steps.sort_by_key(|(_, m)| m.step.index);

                for (step_path, meta) in steps {
                    let status_emoji = match meta.step.status.as_str() {
                        "passed" => "✅",
                        "failed" => "❌",
                        "skipped" => "⏭️",
                        _ => "❓",
                    };

                    let rel_step_dir = step_path.strip_prefix(out_dir).unwrap_or(&step_path);

                    let mut links = Vec::new();
                    if meta.artifacts.screenshot.is_some() {
                        // Assuming screen.ppm
                         let link = rel_step_dir.join("screen.png");
                         links.push(format!("[Screen]({})", link.display()));
                    }
                    if meta.artifacts.serial_tail.is_some() {
                        let link = rel_step_dir.join("serial_tail.txt");
                         links.push(format!("[Log]({})", link.display()));
                    }
                    
                    let link_str = links.join(" ");

                    md.push_str(&format!("| {} | {} | {} | {} |\n", 
                        meta.step.index, 
                        meta.step.text, 
                        status_emoji, 
                        link_str
                    ));
                }
                md.push_str("\n");
            }
        }
    }
    md
}
