use anyhow::Result;
use cucumber::World;
use std::path::PathBuf;

mod extra_steps;
mod keyboard_steps;
mod qemu;
mod report;
mod shared;
mod steps;
mod store;
mod writer;

#[tokio::main]
async fn main() -> Result<()> {
    // Read configuration from environment variables passed by xtask
    let arch = std::env::var("BDD_ARCH").unwrap_or_else(|_| "x86_64".to_string());

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let feature_dir = manifest_dir.join("features");

    let writer = writer::ArtifactWriter {
        out_dir: root.join("artifacts"),
        arch: arch.clone(),
        _run_id: uuid::Uuid::new_v4().to_string(),
        current_feature: String::new(),
        current_scenario: String::new(),
        step_index: 0,
        scenario_failed: false,
        current_steps: Vec::new(),
    };

    println!("Running BDD suite from: {}", feature_dir.display());
    println!("Architecture: {}", arch);

    use crate::steps::BootWorld;

    let feature_filter_raw = std::env::var("BDD_FEATURE").unwrap_or_else(|_| "all".to_string());
    let tag_filter = std::env::var("BDD_TAG").ok();
    let feature_filters: Vec<String> = feature_filter_raw
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    let filter_all = feature_filters.is_empty()
        || (feature_filters.len() == 1 && feature_filters[0] == "all");

    BootWorld::cucumber()
        .max_concurrent_scenarios(1)
        .with_writer(writer)
        .filter_run(feature_dir, move |f, r, s| {
            // Feature filtering
            if !filter_all {
                let matches_feature = f
                    .path
                    .as_ref()
                    .and_then(|p: &std::path::PathBuf| p.file_name())
                    .and_then(|n: &std::ffi::OsStr| n.to_str())
                    .map(|n: &str| feature_filters.iter().any(|f| n.contains(f)))
                    .unwrap_or(false);

                if !matches_feature {
                    return false;
                }
            }

            // Tag filtering
            if let Some(raw_tag) = &tag_filter {
                let desired_tag = raw_tag.strip_prefix('@').unwrap_or(raw_tag);
                let feature_tags = &f.tags;
                let rule_tags = r.map(|r| &r.tags).into_iter().flatten();
                let scenario_tags = &s.tags;

                let has_tag = feature_tags
                    .iter()
                    .chain(rule_tags)
                    .chain(scenario_tags.iter())
                    .any(|t| t == desired_tag);

                if !has_tag {
                    return false;
                }
            }

            true
        })
        .await;

    report::generate_report(&root.join("artifacts"))?;

    if shared::ANY_FAILURE.load(std::sync::atomic::Ordering::SeqCst) {
        std::process::exit(101);
    }

    Ok(())
}
