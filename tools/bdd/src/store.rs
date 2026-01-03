use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use anyhow::Result;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ResultsStore {
    pub run: RunInfo,
    pub features: BTreeMap<String, FeatureResult>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct RunInfo {
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct FeatureResult {
    pub scenarios: BTreeMap<String, ScenarioResult>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ScenarioResult {
    pub arches: BTreeMap<String, String>, // arch -> status
}

impl ResultsStore {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn update_result(&mut self, feature: String, scenario: String, arch: String, status: String) {
        self.features
            .entry(feature)
            .or_default()
            .scenarios
            .entry(scenario)
            .or_default()
            .arches
            .insert(arch, status);
    }
}
