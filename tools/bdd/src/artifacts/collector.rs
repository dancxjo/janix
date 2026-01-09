use std::path::PathBuf;
use std::fs;
use std::io::Write;
use chrono::{DateTime, Local};
use super::types::*;

/// Manages artifact collection throughout a test run.
#[derive(Debug)]
pub struct ArtifactCollector {
    /// Base directory: docs/behavior/{arch}
    pub base_dir: PathBuf,
    /// Target architecture
    pub arch: String,
    /// Timestamp of test run start
    pub start_time: DateTime<Local>,
    /// Current feature being executed
    current_feature: Option<String>,
    /// Current scenario being executed
    current_scenario: Option<String>,
    /// Step counter within current scenario
    step_counter: usize,
    /// All collected features
    features: Vec<FeatureArtifacts>,
    /// Serial log at start of current step (for diff)
    step_start_serial_len: usize,
    /// Step start time for duration tracking
    step_start_time: Option<std::time::Instant>,
    /// Full serial log pending for scenario end
    pending_scenario_serial: String,
}

impl ArtifactCollector {
    /// Create a new artifact collector for the given architecture.
    pub fn new(arch: &str) -> Self {
        let start_time = Local::now();
        let base_dir = PathBuf::from("docs/behavior").join(arch);

        Self {
            base_dir,
            arch: arch.to_string(),
            start_time,
            current_feature: None,
            current_scenario: None,
            step_counter: 0,
            features: Vec::new(),
            step_start_serial_len: 0,
            step_start_time: None,
            pending_scenario_serial: String::new(),
        }
    }

    /// Ensure the base directory exists.
    pub fn init(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.base_dir)
    }

    /// Get directory for current feature.
    pub fn feature_dir(&self) -> PathBuf {
        let mut path = self.base_dir.clone();
        if let Some(ref feature) = self.current_feature {
            path = path.join(Self::slugify(feature));
        }
        path
    }

    /// Get directory for current scenario.
    pub fn scenario_dir(&self) -> PathBuf {
        let mut path = self.feature_dir();
        if let Some(ref scenario) = self.current_scenario {
            path = path.join(Self::slugify(scenario));
        }
        path
    }

    /// Get directory for current step.
    pub fn step_dir(&self) -> PathBuf {
        self.scenario_dir().join(format!("{:02}", self.step_counter))
    }

    /// Called when a feature starts.
    pub fn on_feature_start(&mut self, name: &str) {
        self.current_feature = Some(name.to_string());
        let dir = self.feature_dir();
        let _ = fs::create_dir_all(&dir);

        self.features.push(FeatureArtifacts {
            name: name.to_string(),
            dir,
            scenarios: Vec::new(),
        });
    }

    /// Called when a feature ends.
    pub fn on_feature_end(&mut self) {
        if let Some(feature) = self.features.last() {
            let _ = self.write_feature_readme(feature);
        }
        self.current_feature = None;
    }

    /// Called when a scenario starts.
    pub fn on_scenario_start(&mut self, name: &str) {
        self.current_scenario = Some(name.to_string());
        self.step_counter = 0;

        let dir = self.scenario_dir();
        let _ = fs::create_dir_all(&dir);

        if let Some(feature) = self.features.last_mut() {
            feature.scenarios.push(ScenarioArtifacts {
                name: name.to_string(),
                dir,
                steps: Vec::new(),
                passed: true,
            });
        }
    }

    /// Called when a scenario ends.
    pub fn on_scenario_end(&mut self, passed: bool, _full_serial: &str) {
        let serial_to_write = std::mem::take(&mut self.pending_scenario_serial);
        
        let scenario_to_write = if let Some(feature) = self.features.last_mut() {
            if let Some(scenario) = feature.scenarios.last_mut() {
                scenario.passed = passed;

                let log_path = scenario.dir.join("serial.log");
                if !serial_to_write.is_empty() {
                    let _ = fs::write(&log_path, &serial_to_write);
                }

                Some(scenario.clone())
            } else {
                None
            }
        } else {
            None
        };

        if let Some(ref scenario) = scenario_to_write {
            let _ = self.write_scenario_readme(scenario);
        }
        self.current_scenario = None;
    }

    /// Set the full serial log for the current scenario (called from steps).
    pub fn set_scenario_serial(&mut self, serial: &str) {
        self.pending_scenario_serial = serial.to_string();
    }

    /// Called when a step starts.
    pub fn on_step_start(&mut self, keyword: &str, name: &str, serial_len: usize) {
        self.step_counter += 1;
        self.step_start_serial_len = serial_len;
        self.step_start_time = Some(std::time::Instant::now());

        let dir = self.step_dir();
        let _ = fs::create_dir_all(&dir);

        if let Some(feature) = self.features.last_mut() {
            if let Some(scenario) = feature.scenarios.last_mut() {
                scenario.steps.push(StepArtifacts {
                    name: name.to_string(),
                    keyword: keyword.to_string(),
                    result: StepResult::Skipped,
                    dir,
                    screenshot_before: None,
                    screenshot_after: None,
                    registers: None,
                    serial_log: None,
                    serial_excerpt: String::new(),
                    duration_ms: 0,
                });
            }
        }
    }

    /// Get the path for a step screenshot.
    pub fn screenshot_path(&self, phase: &str) -> PathBuf {
        self.step_dir().join(format!("{}.png", phase))
    }

    /// Get the path for step registers.
    pub fn register_path(&self) -> PathBuf {
        self.step_dir().join("registers.txt")
    }

    /// Called when a step ends.
    pub fn on_step_end(
        &mut self,
        result: StepResult,
        screenshot_before: Option<PathBuf>,
        screenshot_after: Option<PathBuf>,
        registers: Option<PathBuf>,
        full_serial: &str,
    ) {
        let duration_ms = self.step_start_time
            .map(|t| t.elapsed().as_millis() as u64)
            .unwrap_or(0);

        let step_serial = if self.step_start_serial_len < full_serial.len() {
            full_serial[self.step_start_serial_len..].to_string()
        } else {
            String::new()
        };

        let step_dir = self.step_dir();
        let log_path = step_dir.join("serial.log");
        if !step_serial.is_empty() {
            let _ = fs::write(&log_path, &step_serial);
        }

        let step_to_write = if let Some(feature) = self.features.last_mut() {
            if let Some(scenario) = feature.scenarios.last_mut() {
                if let Some(step) = scenario.steps.last_mut() {
                    step.result = result;
                    step.screenshot_before = screenshot_before;
                    step.screenshot_after = screenshot_after;
                    step.registers = registers;
                    step.serial_log = if log_path.exists() { Some(log_path.clone()) } else { None };
                    step.serial_excerpt = step_serial;
                    step.duration_ms = duration_ms;
                    Some(step.clone())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        if let Some(ref step) = step_to_write {
            let _ = self.write_step_readme(step);
        }
    }

    /// Generate the top-level architecture README.
    pub fn generate_arch_readme(&self) -> std::io::Result<PathBuf> {
        let readme_path = self.base_dir.join("README.md");
        let mut file = fs::File::create(&readme_path)?;

        writeln!(file, "# BDD Test Results - {}", self.arch)?;
        writeln!(file)?;
        writeln!(file, "> Last run: {}", self.start_time.format("%Y-%m-%d %H:%M:%S"))?;
        writeln!(file)?;

        let (features_passed, features_failed) = self.count_features();
        let (scenarios_passed, scenarios_failed) = self.count_scenarios();
        let (steps_passed, steps_failed, steps_skipped) = self.count_steps();

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

        for feature in &self.features {
            let all_passed = feature.scenarios.iter().all(|s| s.passed);
            let icon = if all_passed { "✅" } else { "❌" };
            let slug = Self::slugify(&feature.name);
            writeln!(file, "- {} [{}](./{}/)", icon, feature.name, slug)?;
        }

        Ok(readme_path)
    }

    fn write_feature_readme(&self, feature: &FeatureArtifacts) -> std::io::Result<()> {
        let readme_path = feature.dir.join("README.md");
        let mut file = fs::File::create(&readme_path)?;

        let all_passed = feature.scenarios.iter().all(|s| s.passed);
        let icon = if all_passed { "✅" } else { "❌" };

        writeln!(file, "# {} Feature: {}", icon, feature.name)?;
        writeln!(file)?;
        writeln!(file, "> Last run: {}", self.start_time.format("%Y-%m-%d %H:%M:%S"))?;
        writeln!(file)?;

        writeln!(file, "## Scenarios")?;
        writeln!(file)?;

        for scenario in &feature.scenarios {
            let icon = if scenario.passed { "✅" } else { "❌" };
            let slug = Self::slugify(&scenario.name);
            let step_count = scenario.steps.len();
            let passed_count = scenario.steps.iter().filter(|s| s.result == StepResult::Passed).count();
            writeln!(file, "- {} [{}](./{}) ({}/{})", icon, scenario.name, slug, passed_count, step_count)?;
        }

        Ok(())
    }

    fn write_scenario_readme(&self, scenario: &ScenarioArtifacts) -> std::io::Result<()> {
        let readme_path = scenario.dir.join("README.md");
        let mut file = fs::File::create(&readme_path)?;

        let icon = if scenario.passed { "✅" } else { "❌" };

        writeln!(file, "# {} Scenario: {}", icon, scenario.name)?;
        writeln!(file)?;
        writeln!(file, "> Last run: {}", self.start_time.format("%Y-%m-%d %H:%M:%S"))?;
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

    fn write_step_readme(&self, step: &StepArtifacts) -> std::io::Result<()> {
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

    pub fn count_features(&self) -> (usize, usize) {
        let passed = self.features.iter()
            .filter(|f| f.scenarios.iter().all(|s| s.passed))
            .count();
        (passed, self.features.len() - passed)
    }

    fn count_scenarios(&self) -> (usize, usize) {
        let total: Vec<_> = self.features.iter()
            .flat_map(|f| &f.scenarios)
            .collect();
        let passed = total.iter().filter(|s| s.passed).count();
        (passed, total.len() - passed)
    }

    fn count_steps(&self) -> (usize, usize, usize) {
        let total: Vec<_> = self.features.iter()
            .flat_map(|f| &f.scenarios)
            .flat_map(|s| &s.steps)
            .collect();
        let passed = total.iter().filter(|s| s.result == StepResult::Passed).count();
        let failed = total.iter().filter(|s| s.result == StepResult::Failed).count();
        let skipped = total.iter().filter(|s| s.result == StepResult::Skipped).count();
        (passed, failed, skipped)
    }

    pub fn slugify(name: &str) -> String {
        name.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }
}
