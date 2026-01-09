//! Artifact collection for BDD test runs.
//!
//! Manages screenshots, serial logs, and generates markdown reports.
//! Outputs to `/docs/behavior/${ARCH}/${FEATURE}/${SCENARIO}/${STEP}/`
//! with markdown summaries at each level.

use std::path::PathBuf;
use std::fs;
use std::io::Write;
use std::sync::OnceLock;
use chrono::{DateTime, Local};
use tokio::sync::Mutex;

/// Global artifact collector instance.
static COLLECTOR: OnceLock<Mutex<ArtifactCollector>> = OnceLock::new();

/// Global serial log cache (updated by world, read by reporter).
static SERIAL_LOG: OnceLock<Mutex<String>> = OnceLock::new();

/// Initialize the global artifact collector for the given architecture.
pub fn init_global(arch: &str) {
    let collector = ArtifactCollector::new(arch);
    let _ = collector.init();
    let _ = COLLECTOR.set(Mutex::new(collector));
    let _ = SERIAL_LOG.set(Mutex::new(String::new()));
    let _ = QMP_SOCKET.set(Mutex::new(None));
}

/// Get the global artifact collector.
pub fn global() -> &'static Mutex<ArtifactCollector> {
    COLLECTOR.get().expect("ArtifactCollector not initialized - call init_global first")
}

/// Update the global serial log cache (called from world).
pub async fn set_latest_serial(log: &str) {
    if let Some(cache) = SERIAL_LOG.get() {
        let mut serial = cache.lock().await;
        *serial = log.to_string();
    }
}

/// Get the latest serial log (for reporter to use).
pub async fn get_latest_serial() -> String {
    if let Some(cache) = SERIAL_LOG.get() {
        cache.lock().await.clone()
    } else {
        String::new()
    }
}

/// Global QMP socket path (for reporter access to screenshots).
static QMP_SOCKET: OnceLock<Mutex<Option<PathBuf>>> = OnceLock::new();

/// Set the global QMP socket path (called from world).
pub fn set_qmp_socket(path: Option<PathBuf>) {
    if let Some(cache) = QMP_SOCKET.get() {
        if let Ok(mut socket) = cache.try_lock() {
            *socket = path;
        }
    }
}

/// Take a screenshot using the global QMP socket (for reporter).
pub async fn take_screenshot_global(output_path: &std::path::Path) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    use tokio::io::AsyncWriteExt;
    use tokio::net::UnixStream;

    let socket_path = QMP_SOCKET.get()
        .and_then(|cache| cache.try_lock().ok())
        .and_then(|guard| guard.clone())
        .ok_or("No QMP socket available")?;

    let mut stream = UnixStream::connect(&socket_path).await?;

    // Read QMP greeting
    let mut buf = vec![0u8; 4096];
    let _ = tokio::time::timeout(
        std::time::Duration::from_millis(200),
        tokio::io::AsyncReadExt::read(&mut stream, &mut buf)
    ).await;

    // Send qmp_capabilities to enter command mode
    let caps_cmd = r#"{"execute": "qmp_capabilities"}"#;
    stream.write_all(caps_cmd.as_bytes()).await?;
    stream.write_all(b"\n").await?;

    // Read capabilities response
    let _ = tokio::time::timeout(
        std::time::Duration::from_millis(200),
        tokio::io::AsyncReadExt::read(&mut stream, &mut buf)
    ).await;

    // Ensure output directory exists
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Use temp file for PPM, convert to PNG
    let ppm_path = output_path.with_extension("ppm");

    // Send screendump command
    let screendump_cmd = format!(
        r#"{{"execute": "screendump", "arguments": {{"filename": "{}"}}}}"#,
        ppm_path.display()
    );
    stream.write_all(screendump_cmd.as_bytes()).await?;
    stream.write_all(b"\n").await?;

    // Wait for file to be written
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Read response
    let mut response = vec![0u8; 4096];
    let _ = tokio::time::timeout(
        std::time::Duration::from_millis(500),
        tokio::io::AsyncReadExt::read(&mut stream, &mut response)
    ).await;

    if !ppm_path.exists() {
        return Err(format!("Screenshot not saved to {}", ppm_path.display()).into());
    }

    // Convert PPM to PNG
    let png_path = output_path.with_extension("png");
    let img = image::open(&ppm_path)?;
    img.save(&png_path)?;
    let _ = std::fs::remove_file(&ppm_path);

    Ok(png_path)
}

/// Dump CPU registers using the global QMP socket (for reporter).
pub async fn dump_registers_global(output_path: &std::path::Path) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    use tokio::io::AsyncWriteExt;
    use tokio::net::UnixStream;

    let socket_path = QMP_SOCKET.get()
        .and_then(|cache| cache.try_lock().ok())
        .and_then(|guard| guard.clone())
        .ok_or("No QMP socket available")?;

    let mut stream = UnixStream::connect(&socket_path).await?; // Corrected to use socket_path

    // Read QMP greeting
    let mut buf = vec![0u8; 4096];
    let _ = tokio::time::timeout(
        std::time::Duration::from_millis(200),
        tokio::io::AsyncReadExt::read(&mut stream, &mut buf)
    ).await;

    // Send qmp_capabilities to enter command mode
    let caps_cmd = r#"{"execute": "qmp_capabilities"}"#;
    stream.write_all(caps_cmd.as_bytes()).await?;
    stream.write_all(b"\n").await?;

    // Read capabilities response
    let _ = tokio::time::timeout(
        std::time::Duration::from_millis(200),
        tokio::io::AsyncReadExt::read(&mut stream, &mut buf)
    ).await;

    // Ensure output directory exists
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Send human-monitor-command for info registers
    // We use human-monitor-command because qmp doesn't have a direct 'query-registers' command stable across all archs
    let info_regs_cmd = r#"{"execute": "human-monitor-command", "arguments": {"command-line": "info registers"}}"#;
    stream.write_all(info_regs_cmd.as_bytes()).await?;
    stream.write_all(b"\n").await?;

    // Read response
    let mut response = vec![0u8; 16384]; // larger buffer for registers
    let n = tokio::time::timeout(
        std::time::Duration::from_millis(500),
        tokio::io::AsyncReadExt::read(&mut stream, &mut response)
    ).await??;

    // Parse JSON response to extract the actual output
    let response_str = String::from_utf8_lossy(&response[..n]);
    
    // Simple JSON parsing to avoid pulling in serde_json dependency just for this
    // We expect: {"return": "..."}
    let content = if let Some(start) = response_str.find("\"return\": \"") {
        let remainder = &response_str[start + 11..];
        if let Some(end) = remainder.rfind("\"}") {
            // Unescape JSON string (basic)
            remainder[..end].replace("\\r\\n", "\n").replace("\\n", "\n").replace("\\\"", "\"")
        } else {
             response_str.to_string()
        }
    } else {
        response_str.to_string()
    };

    std::fs::write(output_path, content)?;

    Ok(output_path.to_path_buf())
}

/// Result of a step execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepResult {
    Passed,
    Failed,
    Skipped,
}

impl StepResult {
    pub fn emoji(&self) -> &'static str {
        match self {
            StepResult::Passed => "✅",
            StepResult::Failed => "❌",
            StepResult::Skipped => "⏭️",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            StepResult::Passed => "passed",
            StepResult::Failed => "failed",
            StepResult::Skipped => "skipped",
        }
    }
}

/// Captures details about a single step execution.
#[derive(Debug, Clone)]
pub struct StepArtifacts {
    pub name: String,
    pub keyword: String,
    pub result: StepResult,
    pub dir: PathBuf,
    pub screenshot_before: Option<PathBuf>,
    pub screenshot_after: Option<PathBuf>,
    pub registers: Option<PathBuf>,
    pub serial_log: Option<PathBuf>,
    pub serial_excerpt: String,
    pub duration_ms: u64,
}

/// Captures details about a scenario execution.
#[derive(Debug, Clone)]
pub struct ScenarioArtifacts {
    pub name: String,
    pub dir: PathBuf,
    pub steps: Vec<StepArtifacts>,
    pub passed: bool,
}

/// Captures details about a feature execution.
#[derive(Debug, Clone)]
pub struct FeatureArtifacts {
    pub name: String,
    pub dir: PathBuf,
    pub scenarios: Vec<ScenarioArtifacts>,
}

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
        // Generate feature-level README
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

        // Add scenario to current feature
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
        // Use pending_scenario_serial which was set by the last step
        let serial_to_write = std::mem::take(&mut self.pending_scenario_serial);
        
        let scenario_to_write = if let Some(feature) = self.features.last_mut() {
            if let Some(scenario) = feature.scenarios.last_mut() {
                scenario.passed = passed;

                // Save full serial log
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

        // Write readme after mutable borrow ends
        if let Some(ref scenario) = scenario_to_write {
            let _ = self.write_scenario_readme(scenario);
        }
        self.current_scenario = None;
    }

    /// Set the full serial log for the current scenario (called from steps).
    pub fn set_scenario_serial(&mut self, serial: &str) {
        self.pending_scenario_serial = serial.to_string();
    }

    /// Called when a step starts. Records serial log position for excerpt.
    pub fn on_step_start(&mut self, keyword: &str, name: &str, serial_len: usize) {
        self.step_counter += 1;
        self.step_start_serial_len = serial_len;
        self.step_start_time = Some(std::time::Instant::now());

        let dir = self.step_dir();
        let _ = fs::create_dir_all(&dir);

        // Add step placeholder to current scenario
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

        // Extract serial log excerpt for this step
        let step_serial = if self.step_start_serial_len < full_serial.len() {
            full_serial[self.step_start_serial_len..].to_string()
        } else {
            String::new()
        };

        // Save step serial log to file
        let step_dir = self.step_dir();
        let log_path = step_dir.join("serial.log");
        if !step_serial.is_empty() {
            let _ = fs::write(&log_path, &step_serial);
        }

        // Update step in current scenario and collect data for readme
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

        // Write step README after mutable borrow ends
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

        // Summary table
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

        // Feature list
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

        // Steps table
        writeln!(file, "## Steps")?;
        writeln!(file)?;
        writeln!(file, "| # | Step | Result | Duration | Artifacts |")?;
        writeln!(file, "|---|------|--------|----------|-----------|")?;

        for (i, step) in scenario.steps.iter().enumerate() {
            let step_dir = format!("{:02}", i + 1);
            let screenshot_link = if step.screenshot_after.is_some() {
                format!("[📷](./{}/after.png)", step_dir)
            } else {
                "-".to_string()
            };
            let log_link = if step.serial_log.is_some() {
                format!("[📜](./{}/serial.log)", step_dir)
            } else {
                "-".to_string()
            };

            writeln!(
                file,
                "| {} | {} {} | {} | {}ms | {} {} |",
                i + 1,
                step.keyword,
                step.name,
                step.result.emoji(),
                step.duration_ms,
                screenshot_link,
                log_link
            )?;
        }
        writeln!(file)?;

        // Link to full serial log
        writeln!(file, "📜 [Full Serial Log](./serial.log)")?;

        Ok(())
    }

    fn write_step_readme(&self, step: &StepArtifacts) -> std::io::Result<()> {
        let readme_path = step.dir.join("README.md");
        let mut file = fs::File::create(&readme_path)?;

        writeln!(file, "# {} {} {}", step.result.emoji(), step.keyword, step.name)?;
        writeln!(file)?;
        writeln!(file, "**Result:** {} | **Duration:** {}ms", step.result.name(), step.duration_ms)?;
        writeln!(file)?;

        // Screenshots
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

        // Registers
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

        // Serial output
        if !step.serial_excerpt.is_empty() {
            writeln!(file, "## Serial Output")?;
            writeln!(file)?;
            writeln!(file, "```")?;
            // Limit to last 30 lines
            let lines: Vec<_> = step.serial_excerpt.lines().collect();
            let start = lines.len().saturating_sub(30);
            for line in &lines[start..] {
                writeln!(file, "{}", line)?;
            }
            writeln!(file, "```")?;
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

    /// Convert a name to a filesystem-safe slug.
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
