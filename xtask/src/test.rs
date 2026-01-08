use anyhow::{Context, Result};
use std::process::Command;

pub fn run(
    arch: Option<String>,
    smoke: bool,
    suite: Option<String>,
    feature: Option<String>,
    tag: Option<String>,
    force_all: bool,
) -> Result<()> {
    let arch = arch.unwrap_or_else(|| "all".to_string());
    let mut feature = feature.unwrap_or_else(|| "all".to_string());
    let suite = suite.unwrap_or_else(|| "all".to_string());

    if suite != "all" {
        if feature != "all" {
            anyhow::bail!("Use either --suite or --feature, not both.");
        }
        let suite_features = resolve_suite_features(&suite)?;
        println!("Suite '{}' -> {}", suite, suite_features.join(", "));
        feature = suite_features.join(",");
    }

    // Safety check: Prevent accidental massive test runs
    if arch == "all" && feature == "all" && tag.is_none() && !smoke && !force_all {
        anyhow::bail!(
            "SAFETY CHECK FAILED: You are trying to run ALL tests on ALL architectures.\n\
             This takes a long time and is usually not what you want.\n\
             \n\
             Please narrow your scope:\n\
             1. Use --tag to select specific scenarios (e.g. --tag '@mouse')\n\
             2. Use --feature to select a specific feature file\n\
             3. Use --arch to select a specific architecture\n\
             4. Use --smoke for a quick sanity check\n\
             \n\
             If you REALLY want to run everything, use --force-all."
        );
    }

    let archs = if arch == "all" {
        vec!["x86_64", "aarch64", "riscv64", "loongarch64"]
    } else {
        vec![arch.as_str()]
    };

    for a in archs {
        println!("\n==> Running BDD tests for {}...", a);
        let mut cmd = Command::new("cargo");
        cmd.args(["run", "-p", "bdd"]);

        cmd.env("BDD_ARCH", a);

        if smoke {
            cmd.env("BDD_SMOKE", "true");
        }

        // Always pass BDD_FEATURE so the bdd tool knows (it defaults to "all" if missing, but let's be explicit)
        cmd.env("BDD_FEATURE", &feature);

        if let Some(t) = &tag {
            cmd.env("BDD_TAG", t);
        }

        let status = cmd.status().context("Failed to run BDD tests")?;

        if !status.success() {
            anyhow::bail!("BDD tests failed for {}", a);
        }
    }

    Ok(())
}

fn resolve_suite_features(suite: &str) -> Result<Vec<String>> {
    let suite_names: Vec<&str> = suite
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let mut features: Vec<&'static str> = Vec::new();
    for name in suite_names {
        match name {
            "core" => features.extend([
                "boot.feature",
                "multi_arch.feature",
                "memory.feature",
                "scheduler.feature",
                "threads.feature",
                "capabilities.feature",
                "graph.feature",
                "thingify.feature",
                "thing_envelope.feature",
                "watch_wait.feature",
                "rtc.feature",
                "lapic.feature",
                "simd.feature",
            ]),
            "input" => features.extend([
                "input.feature",
                "event_stream.feature",
                "keyboard_pipeline.feature",
                "usb_mouse.feature",
                "pointer_cursor.feature",
                "cursor.feature",
                "cursor_animation.feature",
            ]),
            "bloom" => features.extend([
                "graphics.feature",
                "display_backends.feature",
                "bloom_watches_scene.feature",
                "wallpaper_clouds.feature",
                "cursor_perf.feature",
            ]),
            "apps" => features.extend([
                "demo_app.feature",
                "graphviewer.feature",
                "repl.feature",
            ]),
            "all" => {
                return Ok(vec!["all".to_string()]);
            }
            _ => {
                anyhow::bail!(
                    "Unknown suite '{}'. Available suites: core, input, bloom, apps, all.",
                    name
                );
            }
        }
    }

    if features.is_empty() {
        anyhow::bail!("Suite list resolved to no features.");
    }

    Ok(features.into_iter().map(|f| f.to_string()).collect())
}
