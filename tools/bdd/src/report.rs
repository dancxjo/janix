use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArtifactMeta {
    pub version: u32,
    pub arch: String,
    pub feature: String,
    pub scenario: String,
    pub step: StepMeta,
    pub artifacts: Artifacts,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StepMeta {
    pub index: usize,
    pub text: String,
    pub status: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Artifacts {
    pub screenshot: Option<String>,
    pub serial_tail: Option<String>,
}

pub fn generate_report(out_dir: &Path) -> Result<()> {
    let store_path = out_dir.join("bdd/results.json");
    if store_path.exists() {
        let store = crate::store::ResultsStore::load(&store_path)?;
        generate_index_from_store(out_dir, &store)?;
    }
    Ok(())
}

pub fn generate_index_from_store(out_dir: &Path, store: &crate::store::ResultsStore) -> Result<()> {
    let mut md = String::new();
    md.push_str("# BDD Test Report Index\n\n");

    for (feature, f_res) in &store.features {
        md.push_str(&format!("## Feature: {}\n\n", feature));
        md.push_str("| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 | Report |\n");
        md.push_str("| --- | :---: | :---: | :---: | :---: | :---: |\n");

        for (scenario, s_res) in &f_res.scenarios {
            md.push_str(&format!("| {} |", scenario));
            let arches = vec!["x86_64", "aarch64", "riscv64", "loongarch64"];
            for arch in &arches {
                let status = s_res.arches.get(*arch).map(|s| s.as_str()).unwrap_or("-");
                let icon = match status {
                    "pass" | "passed" => "✅",
                    "failed" => "❌",
                    _ => "⚪",
                };
                md.push_str(&format!(" {} |", icon));
            }

            let mut report_link = String::from("-");
            for arch in &arches {
                if s_res.arches.contains_key(*arch) {
                    let f_slug = slugify(feature);
                    let s_slug = slugify(scenario);
                    report_link = format!("[View Report]({}/{}/{}/report.md)", arch, f_slug, s_slug);
                    break;
                }
            }
            md.push_str(&format!(" {} |\n", report_link));
        }
        md.push_str("\n");
    }
    fs::write(out_dir.join("index.md"), md).context("Failed to write index.md")?;
    Ok(())
}

fn slugify(s: &str) -> String {
    s.chars().map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { '_' }).collect()
}

pub fn generate_scenario_report_from_mem(
    out_dir: &Path,
    arch: &str,
    feature: &str,
    scenario: &str,
    steps: &[ArtifactMeta],
) -> Result<()> {
    if steps.is_empty() { return Ok(()); }
    let mut md = String::new();
    md.push_str(&format!("# Scenario: {}\n\n", scenario));
    md.push_str(&format!("**Architecture**: `{}`  \n", arch));
    md.push_str(&format!("**Feature**: `{}`\n\n", feature));
    md.push_str("[Back to Index](../../../../index.md)\n\n");

    md.push_str("## Steps Summary\n\n| Index | Step | Status | Artifacts |\n| :---: | --- | :---: | --- |\n");
    for meta in steps {
        let status_emoji = match meta.step.status.as_str() {
            "passed" => "✅",
            "failed" => "❌",
            "skipped" => "⏭️",
            _ => "❓",
        };
        let mut art = String::new();
        if meta.artifacts.screenshot.is_some() { art.push_str("�� "); }
        if meta.artifacts.serial_tail.is_some() { art.push_str("📝 "); }
        md.push_str(&format!("| {} | {} | {} | {} |\n", meta.step.index, meta.step.text, status_emoji, art));
    }
    md.push_str("\n## Execution Details\n\n");
    for meta in steps {
        md.push_str(&format!("### {}. {} {}\n\n", meta.step.index + 1, meta.step.text, if meta.step.status == "passed" { "✅" } else { "❌" }));
        if let Some(screen_file) = &meta.artifacts.screenshot {
            let step_slug = format!("{:03}_{}", meta.step.index, slugify(&meta.step.text));
            md.push_str(&format!("![Screenshot](steps/{}/{})\n\n", step_slug, screen_file));
        }
        if let Some(log_file) = &meta.artifacts.serial_tail {
             let step_slug = format!("{:03}_{}", meta.step.index, slugify(&meta.step.text));
             let f_slug = slugify(feature);
             let s_slug = slugify(scenario);
             let log_path = out_dir.join(arch).join(f_slug).join(s_slug).join("steps").join(&step_slug).join(log_file);
             if let Ok(content) = fs::read_to_string(&log_path) {
                md.push_str("```\n");
                md.push_str(&content);
                md.push_str("\n```\n\n");
            } else {
                md.push_str(&format!("[Serial Log](steps/{}/{})\n\n", step_slug, log_file));
            }
        }
        md.push_str("---\n\n");
    }
    let f_slug = slugify(feature);
    let s_slug = slugify(scenario);
    let scenario_dir = out_dir.join(arch).join(f_slug).join(s_slug);
    fs::create_dir_all(&scenario_dir)?;
    fs::write(scenario_dir.join("report.md"), md).context("Failed to write scenario report")?;
    Ok(())
}
