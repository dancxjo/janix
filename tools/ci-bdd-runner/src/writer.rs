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
            // Screenshot
            let screen_path_ppm = step_dir.join("screen.ppm");
            let screen_path_png = step_dir.join("screen.png");
            
            if qemu.screendump(&screen_path_ppm).await.is_ok() {
                 if screen_path_ppm.exists() {
                    // Convert to PNG using the image crate
                    // We spawn blocking because image conversion is CPU intensive
                    // and we don't want to block the async runtime logic too much, 
                    // though for this tool it's likely fine.
                    let ppm_path = screen_path_ppm.clone();
                    let png_path = screen_path_png.clone();
                    
                    let conversion_result = tokio::task::spawn_blocking(move || {
                        if let Ok(img) = image::open(&ppm_path) {
                            if img.save(&png_path).is_ok() {
                                return true;
                            }
                        }
                        false
                    }).await;

                    if let Ok(true) = conversion_result {
                         meta.artifacts.screenshot = Some("screen.png".to_string());
                         let _ = std::fs::remove_file(&screen_path_ppm);
                    }
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
