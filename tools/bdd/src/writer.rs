use crate::shared::{GLOBAL_LAST_ERROR, GLOBAL_QEMU};
use crate::steps::strip_ansi_codes;
use cucumber::event::{Cucumber, Event};
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
    pub _run_id: String,
    pub current_feature: String,
    pub current_scenario: String,
    pub step_index: usize,
    pub scenario_failed: bool,
}

impl<World: std::fmt::Debug + cucumber::World> Writer<World> for ArtifactWriter {
    type Cli = cucumber::cli::Empty;

    async fn handle_event(
        &mut self,
        event: cucumber::parser::Result<Event<Cucumber<World>>>,
        _cli: &Self::Cli,
    ) {
        use cucumber::event::{Cucumber, Feature, Scenario, Step};

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
                                println!(
                                    "\nFeature: {}  Scenario: {}",
                                    self.current_feature, self.current_scenario
                                );
                                self.step_index = 0;
                                self.scenario_failed = false;
                            }
                            Scenario::Step(step, step_event) => {
                                match step_event {
                                    Step::Started => {
                                        println!("→ Step {}: {}", self.step_index + 1, step.value);
                                    }
                                    Step::Passed(..) | Step::Failed(..) | Step::Skipped => {
                                        let mut status = match step_event {
                                            Step::Passed(..) => "passed",
                                            Step::Failed(..) => "failed",
                                            Step::Skipped => "skipped",
                                            Step::Started => unreachable!(),
                                        };

                                        // Check for soft fail
                                        if status == "passed" {
                                            let guard = GLOBAL_LAST_ERROR.lock().await;
                                            if guard.is_some() {
                                                status = "failed";
                                                println!("Writer: Soft Fail detected. Marking step as failed.");
                                            }
                                        }

                                        if status == "failed" {
                                            self.scenario_failed = true;
                                        }

                                        let status_label = match status {
                                            "passed" => "PASS",
                                            "failed" => "FAIL",
                                            "skipped" => "SKIP",
                                            _ => "???",
                                        };
                                        println!(
                                            "[{}] Step {}: {}",
                                            status_label,
                                            self.step_index + 1,
                                            step.value
                                        );

                                        self.capture_artifact(&step.value, status).await;

                                        // Clear error after handling step
                                        {
                                            let mut guard = GLOBAL_LAST_ERROR.lock().await;
                                            *guard = None;
                                        }

                                        self.step_index += 1;
                                    }
                                }
                            }
                            Scenario::Finished => {
                                // Ensure QEMU is killed at the end of the scenario
                                let mut guard = GLOBAL_QEMU.lock().await;
                                if let Some(qemu) = guard.as_mut() {
                                    println!("Scenario finished. Killing QEMU...");
                                    let _ = qemu.kill().await;
                                }
                                *guard = None;

                                // Write scenario JSON report
                                self.write_scenario_report().await;
                            }
                            _ => {}
                        }
                    }
                    Feature::Rule(_r, _rule_event) => {
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
    async fn write_scenario_report(&self) {
        let feature_slug = slugify(&self.current_feature);
        let scenario_slug = slugify(&self.current_scenario);

        let report_dir = self.out_dir.join("bdd");

        if let Err(e) = fs::create_dir_all(&report_dir) {
            eprintln!("Failed to create report dir: {}", e);
            return;
        }

        let status = if self.scenario_failed {
            "failed"
        } else {
            "pass"
        };

        let meta = serde_json::json!({
            "feature": self.current_feature,
            "scenario": self.current_scenario,
            "arch": self.arch,
            "status": status,
            "artifacts_dir": format!("{}/{}/{}", self.arch, feature_slug, scenario_slug)
        });

        let filename = format!("{}_{}_{}.json", self.arch, feature_slug, scenario_slug);
        let path = report_dir.join(filename);

        if let Ok(file) = fs::File::create(path) {
            let _ = serde_json::to_writer_pretty(file, &meta);
        }

        let results_path = self.out_dir.join("bdd/results.json");
        let mut store = crate::store::ResultsStore::load(&results_path).unwrap_or_default();

        if store.run.timestamp.is_empty() {
            store.run.timestamp = chrono::Utc::now().to_rfc3339();
        }

        store.update_result(
            self.current_feature.clone(),
            self.current_scenario.clone(),
            self.arch.clone(),
            status.to_string(),
        );

        if let Err(e) = store.save(&results_path) {
            eprintln!("Failed to save canonical results.json: {}", e);
        }
    }

    async fn capture_artifact(&self, step_text: &str, status: &str) {
        let feature_slug = slugify(&self.current_feature);
        let scenario_slug = slugify(&self.current_scenario);
        let step_slug = format!("{:03}_{}", self.step_index, slugify(step_text));

        let step_dir = self
            .out_dir
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

        let mut guard = GLOBAL_QEMU.lock().await;
        if let Some(qemu) = guard.as_mut() {
            if !qemu.is_connected() {
                let _ = qemu.connect_qmp().await;
            }

            let screen_path_ppm = step_dir.join("screen.ppm");
            let dump_res = qemu.screendump(&screen_path_ppm).await;

            if let Ok(_) = dump_res {
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;

                if screen_path_ppm.exists() {
                    let ppm_path = screen_path_ppm.clone();
                    let png_path = step_dir.join("screen.png");

                    let conversion_result = tokio::task::spawn_blocking(move || {
                        let res = (|| -> anyhow::Result<()> {
                            let img = image::open(&ppm_path)
                                .map_err(|e| anyhow::anyhow!("Open failed: {}", e))?;
                            img.save(&png_path)
                                .map_err(|e| anyhow::anyhow!("Save failed: {}", e))?;
                            Ok(())
                        })();
                        res
                    })
                    .await;

                    match conversion_result {
                        Ok(Ok(_)) => {
                            meta.artifacts.screenshot = Some("screen.png".to_string());
                        }
                        Ok(Err(e)) => {
                            eprintln!("Image conversion failed: {}", e);
                        }
                        Err(e) => {
                            eprintln!("Image conversion task panicked: {}", e);
                        }
                    }
                }
            } else if let Err(e) = dump_res {
                eprintln!("Screendump failed: {}", e);
            }

            if screen_path_ppm.exists() {
                if let Err(e) = std::fs::remove_file(&screen_path_ppm) {
                    eprintln!("Failed to remove PPM: {}", e);
                }
            }

            if let Ok(log) = qemu.log_buffer.lock() {
                let cleaned_log = strip_ansi_codes(&log);
                let tail = cleaned_log
                    .lines()
                    .rev()
                    .take(50)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>()
                    .join("\n");
                let log_path = step_dir.join("serial_tail.txt");
                if let Ok(_) = fs::write(&log_path, tail) {
                    meta.artifacts.serial_tail = Some("serial_tail.txt".to_string());
                }
            }
        }

        let meta_path = step_dir.join("meta.json");
        match fs::File::create(&meta_path) {
            Ok(file) => {
                if let Err(e) = serde_json::to_writer_pretty(file, &meta) {
                    eprintln!("Failed to write meta.json: {}", e);
                }
            }
            Err(e) => {
                eprintln!(
                    "Failed to create meta.json at {}: {}",
                    meta_path.display(),
                    e
                );
            }
        }
    }
}

fn slugify(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect()
}
