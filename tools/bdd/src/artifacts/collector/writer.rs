use std::path::PathBuf;
use std::fs;
use std::io::Write;
use super::ArtifactCollector;
use super::super::types::*;

pub fn generate_arch_readme(collector: &ArtifactCollector) -> std::io::Result<PathBuf> {
    let readme_path = collector.base_dir.join("README.md");
    let mut file = fs::File::create(&readme_path)?;

    writeln!(file, "# BDD Test Results - {}", collector.arch)?;
    writeln!(file)?;
    writeln!(file, "> Last run: {}", collector.start_time.format("%Y-%m-%d %H:%M:%S"))?;
    writeln!(file)?;

    let (features_passed, features_failed) = collector.count_features();
    let (scenarios_passed, scenarios_failed) = collector.count_scenarios();
    let (steps_passed, steps_failed, steps_skipped) = collector.count_steps();

    writeln!(file, "## Summary")?;
    writeln!(file)?;
    writeln!(file, "| Metric | Passed | Failed |")?;
    writeln!(file, "|--------|--------|--------|")?;
    writeln!(file, "| Features | {} | {} |", features_passed, features_failed)?;
    writeln!(file, "| Scenarios | {} | {} |", scenarios_passed, scenarios_failed)?;
    writeln!(file, "| Steps | {} | {} ({} skipped) |", steps_passed, steps_failed, steps_skipped)?;
    writeln!(file)?;

    writeln!(file, "## Features")?;
    writeln!(file)?;

    for feature in &collector.features {
        let all_passed = feature.scenarios.iter().all(|s| s.passed);
        let icon = if all_passed { "✅" } else { "❌" };
        let slug = ArtifactCollector::slugify(&feature.name);
        writeln!(file, "- {} [{}](./{}/)", icon, feature.name, slug)?;
    }

    Ok(readme_path)
}

pub fn write_feature_readme(collector: &ArtifactCollector, feature: &FeatureArtifacts) -> std::io::Result<()> {
    let readme_path = feature.dir.join("README.md");
    let mut file = fs::File::create(&readme_path)?;

    let all_passed = feature.scenarios.iter().all(|s| s.passed);
    let icon = if all_passed { "✅" } else { "❌" };

    writeln!(file, "# {} Feature: {}", icon, feature.name)?;
    writeln!(file)?;
    writeln!(file, "> Last run: {}", collector.start_time.format("%Y-%m-%d %H:%M:%S"))?;
    writeln!(file)?;

    writeln!(file, "## Scenarios")?;
    writeln!(file)?;

    for scenario in &feature.scenarios {
        let icon = if scenario.passed { "✅" } else { "❌" };
        let slug = ArtifactCollector::slugify(&scenario.name);
        let step_count = scenario.steps.len();
        let passed_count = scenario.steps.iter().filter(|s| s.result == StepResult::Passed).count();
        writeln!(file, "- {} [{}](./{}) ({}/{})", icon, scenario.name, slug, passed_count, step_count)?;
    }

    Ok(())
}

pub fn write_scenario_readme(collector: &ArtifactCollector, scenario: &ScenarioArtifacts) -> std::io::Result<()> {
    let readme_path = scenario.dir.join("README.md");
    let mut file = fs::File::create(&readme_path)?;

    let icon = if scenario.passed { "✅" } else { "❌" };

    writeln!(file, "# {} Scenario: {}", icon, scenario.name)?;
    writeln!(file)?;
    writeln!(file, "> Last run: {}", collector.start_time.format("%Y-%m-%d %H:%M:%S"))?;
    writeln!(file)?;

    writeln!(file, "## Steps")?;
    writeln!(file)?;
    writeln!(file, "| # | Step | Result | Duration | Artifacts |")?;
    writeln!(file, "|---|------|--------|----------|-----------|")?;

    for (i, step) in scenario.steps.iter().enumerate() {
        let step_dir = format!("{:02}", i + 1);
        let screenshot_link = if step.screenshot_after.is_some() {
            format!("<a href=\"./{}/after.png\"><img src=\"./{}/after.png\" width=\"150\" /></a>", step_dir, step_dir)
        } else {
            "-".to_string()
        };
        let log_link = if step.serial_log.is_some() {
            format!("[📜](./{}/serial.log)", step_dir)
        } else {
            "-".to_string()
        };
        let reg_link = if step.registers.is_some() {
                format!("[💾](./{}/registers.txt)", step_dir)
        } else {
                "-".to_string()
        };

        writeln!(
            file,
            "| {} | {} {} | {} | {}ms | {} {} {} |",
            i + 1,
            step.keyword,
            step.name,
            step.result.emoji(),
            step.duration_ms,
            screenshot_link,
            log_link,
            reg_link
        )?;
    }
    writeln!(file)?;

    let log_path = scenario.dir.join("serial.log");
    if log_path.exists() {
            if let Ok(content) = fs::read_to_string(log_path) {
            writeln!(file, "<details>")?;
            writeln!(file, "<summary>📜 Full Serial Log</summary>")?;
            writeln!(file)?;
            writeln!(file, "```")?;
            writeln!(file, "{}", content)?;
            writeln!(file, "```")?;
            writeln!(file, "</details>")?;
            }
    }

    Ok(())
}

pub fn write_step_readme(step: &StepArtifacts) -> std::io::Result<()> {
    let readme_path = step.dir.join("README.md");
    let mut file = fs::File::create(&readme_path)?;

    writeln!(file, "# {} {} {}", step.result.emoji(), step.keyword, step.name)?;
    writeln!(file)?;
    writeln!(file, "**Result:** {} | **Duration:** {}ms", step.result.name(), step.duration_ms)?;
    writeln!(file)?;

    if step.screenshot_before.is_some() || step.screenshot_after.is_some() {
        writeln!(file, "## Screenshots")?;
        writeln!(file)?;
        if step.screenshot_before.is_some() {
            writeln!(file, "### Before")?;
            writeln!(file, "![Before](./before.png)")?;
            writeln!(file)?;
        }
        if step.screenshot_after.is_some() {
            writeln!(file, "### After")?;
            writeln!(file, "![After](./after.png)")?;
            writeln!(file)?;
        }
    }

    if let Some(ref reg_path) = step.registers {
            if let Ok(content) = fs::read_to_string(reg_path) {
            writeln!(file, "## Registers")?;
            writeln!(file)?;
            writeln!(file, "```")?;
            writeln!(file, "{}", content)?;
            writeln!(file, "```")?;
            writeln!(file)?;
            }
    }

    if !step.serial_excerpt.is_empty() {
        writeln!(file, "<details open>")?;
        writeln!(file, "<summary>Serial Output</summary>")?;
        writeln!(file)?;
        writeln!(file, "```")?;
        writeln!(file, "{}", step.serial_excerpt)?;
        writeln!(file, "```")?;
        writeln!(file, "</details>")?;
    }

    Ok(())
}
