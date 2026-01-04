use cucumber::Writer;
use std::fs;
use std::path::PathBuf;
use chrono;
use serde::Serialize;
use crate::shared::{GLOBAL_QEMU, slugify};
use crate::steps::strip_ansi_codes;

#[derive(Serialize, Clone, Debug)]
pub struct StepMeta {
    pub index: usize,
    pub text: String,
    pub status: String,
    pub timestamp: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct Artifacts {
    pub screenshot: Option<String>,
    pub serial_tail: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ArtifactMeta {
    pub version: i32,
    pub arch: String,
    pub feature: String,
    pub scenario: String,
    pub step: StepMeta,
    pub artifacts: Artifacts,
}

pub struct ArtifactWriter {
    pub out_dir: PathBuf,
    pub arch: String,
    pub _run_id: String,
    pub current_feature: String,
    pub current_scenario: String,
    pub scenario_failed: bool,
    pub current_steps: Vec<ArtifactMeta>,
    pub step_index: usize,
}

impl ArtifactWriter {
    async fn capture_artifact(&mut self, step_text: &str, status: &str) {
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

        if let Err(_) = fs::create_dir_all(&step_dir) {
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
                        let img = image::open(&ppm_path)?;
                        img.save(&png_path)?;
                        Ok::<(), anyhow::Error>(())
                    }).await;

                    if let Ok(Ok(_)) = conversion_result {
                        meta.artifacts.screenshot = Some("screen.png".to_string());
                    }
                    let _ = std::fs::remove_file(&screen_path_ppm);
                }
            }

            {
                let log = qemu.log_buffer.lock();
                let cleaned_log = strip_ansi_codes(&log);
                let tail = cleaned_log.lines().rev().take(50).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("\n");
                let log_path = step_dir.join("serial_tail.txt");
                if let Ok(_) = fs::write(&log_path, tail) {
                    meta.artifacts.serial_tail = Some("serial_tail.txt".to_string());
                }
            }
        }

        self.current_steps.push(meta.clone());

        let meta_path = step_dir.join("meta.json");
        if let Ok(file) = fs::File::create(&meta_path) {
            let _ = serde_json::to_writer_pretty(file, &meta);
        }
    }
}

impl<W: std::fmt::Debug + cucumber::World> Writer<W> for ArtifactWriter {
    type Cli = cucumber::cli::Empty;

    async fn handle_event(
        &mut self,
        event: cucumber::parser::Result<cucumber::event::Event<cucumber::event::Cucumber<W>>>,
        _cli: &Self::Cli,
    ) {
        use cucumber::event::{Cucumber, Feature, Scenario, Step};

        if let Ok(ev) = event {
            match ev.into_inner() {
                Cucumber::Feature(f, Feature::Started) => {
                    self.current_feature = f.name.clone();
                }
                Cucumber::Feature(_, Feature::Scenario(sc, retryable)) => {
                    match retryable.event {
                        Scenario::Started => {
                            self.current_scenario = sc.name.clone();
                            self.current_steps.clear();
                            self.step_index = 0;
                            self.scenario_failed = false;
                        }
                        Scenario::Step(st, step_ev) => {
                            match step_ev {
                                Step::Passed(..) => {
                                    self.capture_artifact(&st.value, "passed").await;
                                    self.step_index += 1;
                                }
                                Step::Failed(..) => {
                                    self.capture_artifact(&st.value, "failed").await;
                                    self.step_index += 1;
                                    self.scenario_failed = true;
                                }
                                Step::Skipped => {
                                    self.capture_artifact(&st.value, "skipped").await;
                                    self.step_index += 1;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}

impl cucumber::writer::Normalized for ArtifactWriter {}
