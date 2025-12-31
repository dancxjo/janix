use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ArtifactMeta {
    version: u32,
    arch: String,
    feature: String,
    scenario: String,
    step: StepMeta,
    artifacts: Artifacts,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct StepMeta {
    index: usize,
    text: String,
    status: String,
    timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Artifacts {
    screenshot: Option<String>,
    serial_tail: Option<String>,
}

pub fn generate_report(out_dir: &Path) -> Result<()> {
    // We walk the structure: Arch -> Feature -> Scenario -> Steps
    let mut metas = Vec::new();
    visit_dirs(out_dir, &mut metas)?;

    // Group by Arch -> Feature -> Scenario
    // Map<Arch, Map<Feature, Map<Scenario, Vec<(PathBuf, ArtifactMeta)>>>>
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

    // Generate Per-Scenario Reports
    for (arch, features) in &tree {
        for (feature, scenarios) in features {
            for (scenario, steps) in scenarios {
                generate_scenario_report(out_dir, arch, feature, scenario, steps)?;
            }
        }
    }

    // Generate Index
    let encoded = generate_index(out_dir, &tree);
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

fn generate_scenario_report(
    _base_dir: &Path,
    arch: &str,
    feature: &str,
    scenario: &str,
    steps: &Vec<(PathBuf, ArtifactMeta)>
) -> Result<()> {
    if steps.is_empty() {
        return Ok(());
    }

    // Prepare content
    let mut md = String::new();
    md.push_str(&format!("# Scenario: {}\n\n", scenario));
    md.push_str(&format!("**Architecture**: `{}`  \n", arch));
    md.push_str(&format!("**Feature**: `{}`\n\n", feature));
    
    md.push_str("[Back to Index](../../../../index.md)\n\n");

    let mut sorted_steps = steps.clone();
    sorted_steps.sort_by_key(|(_, m)| m.step.index);

    // Steps Table
    md.push_str("## Steps Summary\n\n");
    md.push_str("| Index | Step | Status | Artifacts |\n");
    md.push_str("| :---: | --- | :---: | --- |\n");

    for (_, meta) in &sorted_steps {
        let status_emoji = match meta.step.status.as_str() {
            "passed" => "✅",
            "failed" => "❌",
            "skipped" => "⏭️",
            _ => "❓",
        };
        
        let mut artifacts_str = String::new();
        if meta.artifacts.screenshot.is_some() {
            artifacts_str.push_str("📸 ");
        }
        if meta.artifacts.serial_tail.is_some() {
            artifacts_str.push_str("📝 ");
        }

        md.push_str(&format!("| {} | {} | {} | {} |\n", 
            meta.step.index, 
            meta.step.text, 
            status_emoji,
            artifacts_str
        ));
    }
    md.push_str("\n");

    // Detailed Steps
    md.push_str("## Execution Details\n\n");
    
    for (step_path, meta) in &sorted_steps {
        md.push_str(&format!("### {}. {} {}\n\n", meta.step.index + 1, meta.step.text, 
            match meta.step.status.as_str() {
                "passed" => "✅",
                "failed" => "❌",
                 _ => ""
            }
        ));

        // Images
        if let Some(screen_file) = &meta.artifacts.screenshot {
            // Determine relative path to image. step_path is absolute.
            // We want path relative to THIS report file.
            // THIS report file is at: 
            // base_dir/arch/feature/scenario/report.md
            // Image is at:
            // step_path/screen.png
            
            // Actually, step_path IS the step directory.
            // So relative path is just "steps/<step_slug>/screen.png"
            // Let's verify directory structure from writer.rs:
            // out_dir/arch/feature/scenario/steps/step_slug
            
            // step_path is .../steps/step_slug
            let step_slug = step_path.file_name().unwrap().to_string_lossy();
            let rel_img_path = format!("steps/{}/{}", step_slug, screen_file);
            
            md.push_str(&format!("![Screenshot]({})\n\n", rel_img_path));
        }

        // Serial Log
         if let Some(log_file) = &meta.artifacts.serial_tail {
            let step_slug = step_path.file_name().unwrap().to_string_lossy();
            let rel_log_path = step_path.join(log_file);
            
            // Read content to embed or link? Let's link for now, maybe embed if short.
            // User asked for "with pictures embedded".
            // Let's embed the log in a block code for ease of reading.
            if let Ok(content) = fs::read_to_string(&rel_log_path) {
                 md.push_str("```\n");
                 md.push_str(&content);
                 md.push_str("\n```\n\n");
            } else {
                 let rel_log_link = format!("steps/{}/{}", step_slug, log_file);
                 md.push_str(&format!("[Serial Log]({})\n\n", rel_log_link));
            }
        }
        
        md.push_str("---\n\n");
    }

    // Save report
    // Structure: out_dir/arch/feature/scenario/report.md
    // We can pick any path from the steps to find the scenario dir.
    // steps[0].0 is .../features/scenario/steps/step001
    // Parent is .../features/scenario/steps
    // Grandparent is .../features/scenario
    
    let first_step_path = &steps[0].0;
    let scenario_dir = first_step_path.parent().unwrap().parent().unwrap();
    
    fs::write(scenario_dir.join("report.md"), md).context("Failed to write scenario report")?;

    Ok(())
}

fn generate_index(
    out_dir: &Path,
    tree: &BTreeMap<String, BTreeMap<String, BTreeMap<String, Vec<(PathBuf, ArtifactMeta)>>>>
) -> String {
    let mut md = String::new();
    md.push_str("# BDD Test Report Index\n\n");

    for (arch, features) in tree {
        md.push_str(&format!("## Architecture: {}\n\n", arch));
        
        for (feature, scenarios) in features {
            md.push_str(&format!("### Feature: {}\n\n", feature));
            
            md.push_str("| Scenario | Status | Steps | Report |\n");
            md.push_str("| --- | :---: | :---: | :---: |\n");
            
            for (scenario, steps) in scenarios {
                let count = steps.len();
                // Determine overall status
                let all_passed = steps.iter().all(|(_, m)| m.step.status == "passed");
                let any_failed = steps.iter().any(|(_, m)| m.step.status == "failed");
                
                let emoji = if any_failed { "❌" } else if all_passed { "✅" } else { "⚠️" };
                
                // Link to report.md in the scenario directory
                // We need to reconstruct the path: arch/feature/scenario/report.md
                // But wait, feature and scenario are slugified in directories but plain text here.
                // We need the SLUG for the link.
                // Fortunately, we have the path in the steps!
                let first_step_path = &steps[0].0;
                let scenario_dir = first_step_path.parent().unwrap().parent().unwrap();
                let rel_scenario_dir = scenario_dir.strip_prefix(out_dir).unwrap_or(scenario_dir);
                let report_link = rel_scenario_dir.join("report.md");

                md.push_str(&format!("| {} | {} | {} | [View Report]({}) |\n", 
                    scenario, 
                    emoji, 
                    count,
                    report_link.display()
                ));
            }
            md.push_str("\n");
        }
    }
    md
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_generate_report() -> Result<()> {
        let dir = tempdir()?;
        let root = dir.path();
        
        // Setup mock structure
        // x86_64/feature_a/scenario_b/steps/001_step_slug/meta.json
        let step_dir = root.join("x86_64/feature_a/scenario_b/steps/001_step_slug");
        fs::create_dir_all(&step_dir)?;
        
        // Mock meta.json
        let meta = ArtifactMeta {
            version: 1,
            arch: "x86_64".to_string(),
            feature: "Feature A".to_string(),
            scenario: "Scenario B".to_string(),
            step: StepMeta {
                index: 0,
                text: "I do something".to_string(),
                status: "passed".to_string(),
                timestamp: "2023-01-01T00:00:00Z".to_string(),
            },
            artifacts: Artifacts {
                screenshot: Some("screen.png".to_string()),
                serial_tail: Some("serial_tail.txt".to_string()),
            },
        };
        
        let meta_json = serde_json::to_string(&meta)?;
        fs::write(step_dir.join("meta.json"), meta_json)?;
        fs::write(step_dir.join("screen.png"), "fake image")?;
        fs::write(step_dir.join("serial_tail.txt"), "fake log")?;

        // Run generator
        generate_report(root)?;

        // Check index.md
        let index_path = root.join("index.md");
        assert!(index_path.exists());
        let index_content = fs::read_to_string(&index_path)?;
        assert!(index_content.contains("# BDD Test Report Index"));
        assert!(index_content.contains("Scenario B"));
        assert!(index_content.contains("[View Report]"));

        // Check scenario report
        let report_path = root.join("x86_64/feature_a/scenario_b/report.md");
        assert!(report_path.exists());
        let report_content = fs::read_to_string(&report_path)?;
        assert!(report_content.contains("# Scenario: Scenario B"));
        assert!(report_content.contains("![Screenshot](steps/001_step_slug/screen.png)"));
        assert!(report_content.contains("fake log"));

        Ok(())
    }
}
