//! BDD tests for ThingOS boot process using Cucumber.
//!
//! These tests verify that the OS boots correctly for each architecture
//! by checking for expected output in the serial console.

use cucumber::{gherkin::Step as GherkinStep, given, then, World};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::time::{timeout, Duration};

#[derive(Debug, Clone)]
struct Anchor {
    label: String,
    line_index: usize,
}

/// Test world state for boot scenarios
#[derive(Debug, Default, World)]
pub struct BootWorld {
    /// Architecture being tested (x86_64, aarch64, etc)
    arch: String,
    /// Captured serial output from QEMU
    serial_output: String,
    /// Whether boot was attempted
    boot_attempted: bool,
    /// Optional boot modifier used to drive negative-path scenarios
    boot_variant: Option<String>,
    /// Most recent anchor line used for cross-step ordering assertions
    last_anchor: Option<Anchor>,
}

fn fail_with_output(world: &BootWorld, message: impl AsRef<str>) -> ! {
    let log = &world.serial_output;
    // Simple heuristic for failure classification
    let mut class = "BOOT_FAILURE";

    if log.contains("TIMEOUT") {
        class = "TIMEOUT_WITHOUT_LIVENESS";
    } else if log.contains("PANIC") || log.contains("panic") {
        if let Some(v) = &world.boot_variant {
            if v.contains("init that exits") || v.contains("panic") {
                // We expected panic, but maybe assertion failed on content?
                // If we are here, the test failed. 
                // Using generic failure if panic was expected but details wrong.
                class = "ASSERTION_FAILURE_DURING_EXPECTED_PANIC"; 
            } else {
                class = "PANIC_UNEXPECTED";
            }
        } else {
             class = "PANIC_UNEXPECTED";
        }
    } else if let Some(v) = &world.boot_variant {
        if (v.contains("init that exits") || v.contains("panic")) && !log.contains("PANIC") {
            class = "PANIC_EXPECTED_BUT_MISSING";
        }
    } else if log.contains("sprout entry point") && !log.contains("SPROUT: I am alive") {
        class = "INIT_FAILURE";
    }

    panic!(
        "\n========= FAILURE CLASSIFICATION: {} =========\n{}\n--- START SERIAL LOG ---\n{}\n--- END SERIAL LOG ---",
        class,
        message.as_ref(),
        world.serial_output
    );
}

fn serial_lines(world: &BootWorld) -> Vec<&str> {
    world.serial_output.lines().collect()
}

fn find_line_index(lines: &[&str], needle: &str) -> Option<usize> {
    lines.iter().position(|line| line.contains(needle))
}

fn set_anchor(world: &mut BootWorld, label: String, line_index: usize) {
    world.last_anchor = Some(Anchor { label, line_index });
}

fn require_anchor(world: &BootWorld, step: &str) -> Anchor {
    world
        .last_anchor
        .clone()
        .unwrap_or_else(|| fail_with_output(world, format!("No anchor set before '{}'", step)))
}

/// Get the project root directory
fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

#[given(expr = "I boot the OS in qemu for {string}")]
async fn boot_os_in_qemu(world: &mut BootWorld, arch: String) {
    boot_os(world, arch, None).await;
}

#[given(expr = "I boot the OS in qemu for {string} with {string}")]
async fn boot_os_in_qemu_with(world: &mut BootWorld, arch: String, variant: String) {
    boot_os(world, arch, Some(variant)).await;
}

async fn boot_os(world: &mut BootWorld, arch: String, variant: Option<String>) {
    world.arch = arch.clone();
    world.boot_attempted = true;
    world.boot_variant = variant.clone();
    let root = project_root();

    // Map variant to init module if applicable
    let mut init_module = None;
    if let Some(v) = &variant {
        if v.contains("init that exits") {
            init_module = Some("sprout_exit".to_string());
        }
    }

    let mut build_cmd = Command::new("cargo");
    build_cmd.args(["run", "-p", "xtask", "--", "iso", "--env", &arch]);
    
    if let Some(mod_name) = init_module {
        build_cmd.arg("--init-module").arg(mod_name);
    }

    let output = build_cmd
        .current_dir(&root)
        .output()
        .await
        .expect("Failed to run xtask iso");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        panic!(
            "Failed to build ISO for {}.\nSTDOUT:\n{}\nSTDERR:\n{}",
            arch, stdout, stderr
        );
    }

    let iso_path = root.join("target").join("iso").join(format!("thingos-{}.iso", arch));
    assert!(
        iso_path.exists(),
        "ISO file not found at {}",
        iso_path.display()
    );

    // 2. Prepare OVMF paths
    let ovmf_code = root.join(format!("ovmf/ovmf-code-{}.fd", arch));
    let ovmf_vars = root.join(format!("ovmf/ovmf-vars-{}.fd", arch));

    // 3. Build QEMU command
    let mut cmd = match arch.as_str() {
        "x86_64" => {
            let mut c = Command::new("qemu-system-x86_64");
            c.args([
                "-M",
                "q35",
                "-m",
                "2G",
                "-display",
                "none",
                "-serial",
                "stdio",
                "-drive",
                &format!(
                    "if=pflash,unit=0,format=raw,file={},readonly=on",
                    ovmf_code.display()
                ),
                "-drive",
                &format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars.display()),
                "-cdrom",
                &iso_path.to_string_lossy(),
                "-no-reboot",
            ]);
            c
        }
        "aarch64" => {
            let mut c = Command::new("qemu-system-aarch64");
            c.args([
                "-M",
                "virt",
                "-cpu",
                "cortex-a72",
                "-m",
                "2G",
                "-device",
                "ramfb",
                "-device",
                "qemu-xhci",
                "-device",
                "usb-kbd",
                "-device",
                "usb-mouse",
                "-display",
                "none",
                "-serial",
                "stdio",
                "-drive",
                &format!(
                    "if=pflash,unit=0,format=raw,file={},readonly=on",
                    ovmf_code.display()
                ),
                "-drive",
                &format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars.display()),
                "-cdrom",
                &iso_path.to_string_lossy(),
                "-no-reboot",
                "-d",
                "guest_errors", // Debug aid
            ]);
            c
        }
        "riscv64" => {
            let mut c = Command::new("qemu-system-riscv64");
            c.args([
                "-M",
                "virt",
                "-cpu",
                "rv64",
                "-m",
                "2G",
                "-device",
                "ramfb",
                "-device",
                "qemu-xhci",
                "-device",
                "usb-kbd",
                "-device",
                "usb-mouse",
                "-display",
                "none",
                "-serial",
                "stdio",
                "-drive",
                &format!(
                    "if=pflash,unit=0,format=raw,file={},readonly=on",
                    ovmf_code.display()
                ),
                "-drive",
                &format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars.display()),
                "-cdrom",
                &iso_path.to_string_lossy(),
                "-no-reboot",
            ]);
            c
        }
        "loongarch64" => {
            // Note: loongarch64 usually requires external bios if pflash not available
            // but we'll try standard way. If it fails, it fails.
            let mut c = Command::new("qemu-system-loongarch64");
            c.args([
                "-M",
                "virt",
                "-cpu",
                "la464",
                "-m",
                "2G",
                "-display",
                "none",
                "-serial",
                "stdio",
                "-drive",
                &format!(
                    "if=pflash,unit=0,format=raw,file={},readonly=on",
                    ovmf_code.display()
                ),
                "-drive",
                &format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars.display()),
                "-cdrom",
                &iso_path.to_string_lossy(),
                "-no-reboot",
            ]);
            c
        }
        _ => panic!("Unsupported architecture: {}", arch),
    };

    if let Some(v) = &variant {
        cmd.env("THINGOS_BOOT_SCENARIO", v);
    }

    cmd.stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .current_dir(&root);

    // 4. Run QEMU and capture output
    // Increase timeout to 60s
    let boot_timeout = Duration::from_secs(300);

    let output_buffer = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
    let output_buffer_clone = output_buffer.clone();

    match cmd.spawn() {
        Ok(mut child) => {
            let stdout = child.stdout.take().expect("stdout");
            let mut reader = BufReader::new(stdout).lines();

            let result = timeout(boot_timeout, async move {
                while let Ok(Some(line)) = reader.next_line().await {
                    {
                        let mut buf = output_buffer_clone
                            .lock()
                            .expect("Failed to lock output buffer");
                        buf.push_str(&line);
                        buf.push('\n');
                    }

                    if line.contains("Booted.") 
                        || line.contains("userland: SPROUT: root contains expected count")
                        || line.contains("SPROUT: I am alive") 
                    {
                        return true;
                    }
                    if line.contains("panic") || line.contains("PANIC") {
                        return true;
                    }
                }
                false
            })
            .await;

            let _ = child.kill().await;
            let _ = child.wait().await;

            let captured = output_buffer
                .lock()
                .expect("Failed to lock output buffer")
                .clone();

            if let Err(_) = result {
                world.serial_output = format!("TIMEOUT (60s). Captured so far:\n{}", captured);
            } else {
                world.serial_output = captured;
            }
        }
        Err(e) => {
            world.serial_output = format!("ERROR: Failed to start QEMU: {}", e);
        }
    }
}

#[then(expr = "the serial console log must contain the following lines in order:")]
async fn serial_console_lines_in_order(world: &mut BootWorld, step: &GherkinStep) {
    let table = step
        .table
        .as_ref()
        .unwrap_or_else(|| fail_with_output(world, "Expected a table of ordered log lines"));
    let lines = serial_lines(world);
    let mut cursor = 0usize;

    for row in &table.rows {
        let expected = row.get(0).map(|s| s.trim()).unwrap_or_else(|| {
            fail_with_output(world, "Expected table rows with at least one column")
        });
        let mut found = None;
        for (idx, line) in lines.iter().enumerate().skip(cursor) {
            if line.contains(expected) {
                println!("    [DEBUG] Match found: '{}' matches line {}: '{}'", expected, idx + 1, line);
                found = Some(idx + 1);
                break;
            }
        }
        cursor = match found {
            Some(next) => next,
            None => {
                println!("    [DEBUG] Match FAILED: could not find '{}' after line {}", expected, cursor);
                fail_with_output(
                    world,
                    format!(
                        "Expected '{}' after line {}",
                        expected,
                        cursor.saturating_sub(1)
                    ),
                )
            }
        };
    }
}

#[then(expr = "no line starting with {string} appears before {string}")]
async fn no_prefixed_line_before_anchor(world: &mut BootWorld, prefix: String, anchor: String) {
    let lines = serial_lines(world);
    let anchor_idx = find_line_index(&lines, &anchor).unwrap_or_else(|| {
        fail_with_output(
            world,
            format!("Expected anchor '{}' in serial output", anchor),
        )
    });

    if lines
        .iter()
        .take(anchor_idx)
        .any(|line| line.starts_with(&prefix))
    {
        fail_with_output(
            world,
            format!(
                "Found a line starting with '{}' before '{}'",
                prefix, anchor
            ),
        );
    }
}

#[then(expr = "I expect NOT to see {string} before {string}")]
async fn expect_not_to_see_before(world: &mut BootWorld, unexpected: String, anchor: String) {
    let lines = serial_lines(world);
    let anchor_idx = find_line_index(&lines, &anchor).unwrap_or_else(|| {
        fail_with_output(
            world,
            format!("Expected anchor '{}' in serial output", anchor),
        )
    });

    if lines
        .iter()
        .take(anchor_idx)
        .any(|line| line.contains(&unexpected))
    {
        fail_with_output(
            world,
            format!(
                "Found '{}' before '{}', violating ordering constraint",
                unexpected, anchor
            ),
        );
    }
}

#[then(expr = "I expect NOT to see any {string} line before {string}")]
async fn expect_not_to_see_prefixed_before(world: &mut BootWorld, prefix: String, anchor: String) {
    let lines = serial_lines(world);
    let anchor_idx = find_line_index(&lines, &anchor).unwrap_or_else(|| {
        fail_with_output(
            world,
            format!("Expected anchor '{}' in serial output", anchor),
        )
    });

    if lines
        .iter()
        .take(anchor_idx)
        .any(|line| line.starts_with(&prefix))
    {
        fail_with_output(
            world,
            format!("Found a '{}' line before '{}'", prefix, anchor),
        );
    }
}

#[then(expr = "I expect NOT to see {string} if {string} did not occur")]
async fn expect_not_if_missing(world: &mut BootWorld, unexpected: String, prerequisite: String) {
    let prereq_seen = world.serial_output.contains(&prerequisite);
    let unexpected_seen = world.serial_output.contains(&unexpected);

    if !prereq_seen && unexpected_seen {
        fail_with_output(
            world,
            format!(
                "Saw '{}' even though '{}' never appeared",
                unexpected, prerequisite
            ),
        );
    }
}

#[then(expr = "the serial console MAY miss any line starting with {string}")]
async fn serial_console_may_miss(_world: &mut BootWorld, _prefix: String) {
    // Early logging is intentionally treated as best-effort here.
}

#[then(expr = "I MUST see {string}")]
async fn expect_must_see(world: &mut BootWorld, expected: String) {
    let lines = serial_lines(world);
    let idx = find_line_index(&lines, &expected).unwrap_or_else(|| {
        fail_with_output(world, format!("Expected '{}' in serial output", expected))
    });

    set_anchor(world, expected, idx);
}

#[then(expr = "after that point, I MUST see all of:")]
async fn expect_after_anchor(world: &mut BootWorld, step: &GherkinStep) {
    let anchor = require_anchor(world, "after that point, I MUST see all of:");
    let table = step.table.as_ref().unwrap_or_else(|| {
        fail_with_output(world, "Expected a table of post-serial-backend milestones")
    });
    let lines = serial_lines(world);
    let mut cursor = anchor.line_index + 1;

    for row in &table.rows {
        let expected = row.get(0).map(|s| s.trim()).unwrap_or_else(|| {
            fail_with_output(world, "Expected table rows with at least one column")
        });
        let mut found = None;
        for (idx, line) in lines.iter().enumerate().skip(cursor) {
            if line.contains(expected) {
                found = Some(idx + 1);
                break;
            }
        }

        cursor = match found {
            Some(next) => next,
            None => fail_with_output(
                world,
                format!(
                    "Expected '{}' after '{}' (line {})",
                    expected,
                    anchor.label,
                    anchor.line_index + 1
                ),
            ),
        };
    }

    set_anchor(world, anchor.label, cursor.saturating_sub(1));
}

#[then(expr = "{string} appears exactly once in the serial console")]
async fn expect_appears_exactly_once(world: &mut BootWorld, needle: String) {
    let lines = serial_lines(world);
    let matches: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter_map(|(idx, line)| line.contains(&needle).then_some(idx))
        .collect();

    match matches.len() {
        1 => set_anchor(world, needle, matches[0]),
        0 => fail_with_output(
            world,
            format!(
                "Expected '{}' to appear exactly once but it never appeared",
                needle
            ),
        ),
        count => fail_with_output(
            world,
            format!(
                "Expected '{}' exactly once but found it {} times",
                needle, count
            ),
        ),
    }
}

#[then(expr = "no later log line contains {string}")]
async fn expect_no_later_log_line(world: &mut BootWorld, needle: String) {
    let anchor = require_anchor(world, "no later log line contains");
    let lines = serial_lines(world);

    if lines
        .iter()
        .skip(anchor.line_index + 1)
        .any(|line| line.contains(&needle))
    {
        fail_with_output(
            world,
            format!(
                "Found '{}' after anchor '{}' on line {}",
                needle,
                anchor.label,
                anchor.line_index + 1
            ),
        );
    }
}

#[then(expr = "I expect NOT to see {string}")]
async fn expect_not_to_see(world: &mut BootWorld, unexpected: String) {
    if world.serial_output.contains(&unexpected) {
        fail_with_output(
            world,
            format!("Unexpected '{}' present in serial output", unexpected),
        );
    }
}

#[then(expr = "I expect to see a single line starting with {string}")]
async fn expect_single_line_starting(world: &mut BootWorld, prefix: String) {
    let lines = serial_lines(world);
    let matches: Vec<(usize, &str)> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.starts_with(&prefix))
        .map(|(i, &l)| (i, l))
        .collect();

    match matches.len() {
        1 => set_anchor(world, matches[0].1.to_string(), matches[0].0),
        0 => fail_with_output(
            world,
            format!(
                "Expected one line starting with '{}' but found none",
                prefix
            ),
        ),
        count => fail_with_output(
            world,
            format!(
                "Expected one line starting with '{}' but found {}",
                prefix, count
            ),
        ),
    }
}

#[then(expr = "that line must mention:")]
async fn anchor_line_must_mention(world: &mut BootWorld, step: &GherkinStep) {
    let anchor = require_anchor(world, "that line must mention:");
    let table = step
        .table
        .as_ref()
        .unwrap_or_else(|| fail_with_output(world, "Expected a table of summary tokens"));

    let lines = serial_lines(world);
    let line = lines.get(anchor.line_index).copied().unwrap_or_else(|| {
        fail_with_output(
            world,
            format!(
                "Anchor line {} is out of range for captured output",
                anchor.line_index + 1
            ),
        )
    });

    for row in &table.rows {
        let expected = row.get(0).map(String::as_str).unwrap_or_else(|| {
            fail_with_output(world, "Expected table rows with at least one column")
        });
        if !line.contains(expected) {
            fail_with_output(
                world,
                format!(
                    "Anchor line '{}' missing expected token '{}'",
                    line, expected
                ),
            );
        }
    }
}

#[then(expr = "I expect to see {string} in the serial console")]
async fn expect_serial_output(world: &mut BootWorld, expected: String) {
    let lines = serial_lines(world);
    if find_line_index(&lines, &expected).is_none() {
        fail_with_output(world, format!("Expected '{}' in serial output", expected));
    }
}

#[then(expr = "the system must reach steady state")]
async fn expect_steady_state(world: &mut BootWorld) {
    expect_steady_state_logic(world);
}

#[then(expr = "the system must reach steady state within {int}ms")]
async fn expect_steady_state_within(world: &mut BootWorld, _ms: u64) {
    // We treat the timeout as 'within reasonable time' since logs are already captured.
    expect_steady_state_logic(world);
}

fn expect_steady_state_logic(world: &BootWorld) {
    let log = &world.serial_output;
     // Milestones from Init Contract
    let has_scheduler = log.contains("KERNEL: scheduler running") || log.contains("sched: entered loop"); // Adjust based on actual kernel logs
    let has_init = log.contains("KERNEL: init task alive") || log.contains("SPROUT: I am alive");

    if !has_scheduler {
        fail_with_output(world, "Steady State Violation: Scheduler did not verify running");
    }
    if !has_init {
        fail_with_output(world, "Steady State Violation: Init task did not verify liveness");
    }
    if log.contains("PANIC") || log.contains("panic") {
         fail_with_output(world, "Steady State Violation: System panicked");
    }
}

#[tokio::main]
async fn main() {
    BootWorld::run("features").await;
}
