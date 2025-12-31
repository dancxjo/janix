use crate::shared::GLOBAL_QEMU;
use cucumber::event::{Event, Cucumber};
use cucumber::Writer;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize)]
struct ArtifactMeta {
    version: u32,
    arch: String,
    feature: String,
    scenario: String,
    step: StepMeta,
    artifacts: Artifacts,
}

#[derive(Serialize)]
struct StepMeta {
    index: usize,
    text: String,
    status: String,
    timestamp: String,
}

#[derive(Serialize)]
struct Artifacts {
    screenshot: Option<String>,
    serial_tail: Option<String>,
}

pub struct ArtifactWriter {
    pub out_dir: PathBuf,
    pub arch: String,
    pub run_id: String,
    pub current_feature: String,
    pub current_scenario: String,
    pub step_index: usize,
}

impl<World: std::fmt::Debug + cucumber::World> Writer<World> for ArtifactWriter {
    type Cli = cucumber::cli::Empty;

    async fn handle_event(
        &mut self,
        event: cucumber::parser::Result<Event<Cucumber<World>>>,
        _cli: &Self::Cli,
    ) {
        use cucumber::event::{Cucumber, Feature, Rule, Scenario, Step};

        let event = match event {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Parsing error: {:?}", e);
                return;
            }
        };

        match event.value {
            Cucumber::Feature(f, feature_event) => {
                self.current_feature = f.name.clone();
                match feature_event {
                    Feature::Scenario(s, retryable_scenario) => {
                        self.current_scenario = s.name.clone();
                        match retryable_scenario.event {
                            Scenario::Started => {
                                self.step_index = 0;
                            }
                            Scenario::Step(step, step_event) => {
                                match step_event {
                                    Step::Passed(..) | Step::Failed(..) | Step::Skipped => {
                                        let status = match step_event {
                                            Step::Passed(..) => "passed",
                                            Step::Failed(..) => "failed",
                                            Step::Skipped => "skipped",
                                            _ => "unknown",
                                        };
                                        self.capture_artifact(&step.value, status).await;
                                        self.step_index += 1;
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    Feature::Rule(_r, _rule_event) => {
                        // Handle rule if needed, ignoring for now
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

impl cucumber::writer::Normalized for ArtifactWriter {}

impl ArtifactWriter {
    async fn capture_artifact(&self, step_text: &str, status: &str) {
        // ... (slugify setup) ...
        // I'll assume capture_artifact internal logic is fine, but I need to call qemu properly.
        // Let's replace only handle_event first, assuming capture_artifact is below.
        // Wait, I need to replace capture_artifact too because of `qemu.qmp_stream.is_none()` usage.
        
        let feature_slug = slugify(&self.current_feature);
        let scenario_slug = slugify(&self.current_scenario);
        let step_slug = format!("{:03}_{}", self.step_index, slugify(step_text));

        let step_dir = self.out_dir
            .join(&self.arch)
            .join(&feature_slug)
            .join(&scenario_slug)
            .join("steps")
            .join(&step_slug);

        if let Err(e) = fs::create_dir_all(&step_dir) {
            eprintln!("Failed to create step dir: {}", e);
            return;
        }

        let mut meta = ArtifactMeta {
            version: 1,
            arch: self.arch.clone(),
            feature: self.current_feature.clone(),
            scenario: self.current_scenario.clone(),
            step: StepMeta {
                index: self.step_index,
                text: step_text.to_string(),
                status: status.to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            artifacts: Artifacts {
                screenshot: None,
                serial_tail: None,
            },
        };

        // Take screenshot and grab log
        let mut guard = GLOBAL_QEMU.lock().await;
        if let Some(qemu) = guard.as_mut() {
            // Re-connect if steam is Missing?
            if !qemu.is_connected() {
                 let _ = qemu.connect_qmp().await;
            }

            // Screenshot
            let screen_path = step_dir.join("screen.ppm");
            if let Err(_e) = qemu.screendump(&screen_path).await {
                // Ignore errors
            } else {
                if screen_path.exists() {
                     meta.artifacts.screenshot = Some("screen.ppm".to_string());
                }
            }

            // Log tail
            if let Ok(log) = qemu.log_buffer.lock() {
                let tail = log.lines().rev().take(50).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("\n");
                let log_path = step_dir.join("serial_tail.txt");
                if let Ok(_) = fs::write(&log_path, tail) {
                    meta.artifacts.serial_tail = Some("serial_tail.txt".to_string());
                }
            }
        }
        
        let meta_path = step_dir.join("meta.json");
        let file = fs::File::create(meta_path).unwrap();
        serde_json::to_writer_pretty(file, &meta).unwrap();
    }
}


fn slugify(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { '_' })
        .collect()
}
