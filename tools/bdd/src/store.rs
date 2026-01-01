use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RunMeta {
    pub git_sha: Option<String>,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ResultsStore {
    #[serde(default)]
    pub run: RunMeta,
    #[serde(default)]
    pub scenarios: Vec<ScenarioResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScenarioResult {
    pub feature: String,
    pub scenario: String,
    pub results: BTreeMap<String, String>, // arch -> status
}

impl ResultsStore {
    pub fn load(path: &Path) -> Result<Self> {
        if path.exists() {
            let content = fs::read_to_string(path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let f = fs::File::create(path)?;
        serde_json::to_writer_pretty(f, self)?;
        Ok(())
    }

    pub fn update_result(&mut self, feature: String, scenario: String, arch: String, status: String) {
        // Find existing scenario or create new
        if let Some(scen) = self.scenarios.iter_mut().find(|s| s.feature == feature && s.scenario == scenario) {
            scen.results.insert(arch, status);
        } else {
            let mut results = BTreeMap::new();
            results.insert(arch, status);
            self.scenarios.push(ScenarioResult {
                feature,
                scenario,
                results,
            });
        }
    }
}
