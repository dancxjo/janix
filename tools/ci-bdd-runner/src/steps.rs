use crate::qemu::QemuProcess;
use crate::shared::{ANY_FAILURE, GLOBAL_LAST_ERROR, GLOBAL_QEMU};
use anyhow::{anyhow, Context, Result};
use cucumber::{gherkin::Step, given, then, World};
use std::path::PathBuf;
use tokio::process::Command;
use tokio::time::{sleep, Duration};

#[derive(Debug, Default, World)]
pub struct BootWorld {
    arch: String,
    /// Stores the last anchor position for "after that point" steps
    last_anchor_position: Option<usize>,
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

/// Strip ANSI escape codes from a string
pub fn strip_ansi_codes(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Skip escape sequence
            if let Some(&next) = chars.peek() {
                if next == '[' {
                    chars.next(); // consume '['
                    // Skip until we hit a letter (the command)
                    while let Some(&ch) = chars.peek() {
                        chars.next();
                        if ch.is_ascii_alphabetic() {
                            break;
                        }
                    }
                    continue;
                }
            }
        }
        result.push(c);
    }
    result
}

/// Get the current serial log, with ANSI codes stripped
async fn get_clean_log() -> String {
    let guard = GLOBAL_QEMU.lock().await;
    if let Some(qemu) = guard.as_ref() {
        let log = qemu.log_buffer.lock().unwrap().clone();
        strip_ansi_codes(&log)
    } else {
        String::new()
    }
}

/// Wait for the boot to complete (Booted. message) or timeout
async fn wait_for_boot_completion() -> Result<String> {
    let timeout_secs = std::env::var("BDD_TIMEOUT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(30);
    let timeout = Duration::from_secs(timeout_secs);
    let start = std::time::Instant::now();

    loop {
        let log = get_clean_log().await;
        
        // Check if boot completed
        if log.contains("Booted.") {
            return Ok(log);
        }

        // Check if QEMU exited
        {
            let mut guard = GLOBAL_QEMU.lock().await;
            if let Some(qemu) = guard.as_mut() {
                if let Some(status) = qemu.check_status() {
                    return Err(anyhow!("QEMU exited with status {:?} before boot completed", status));
                }
            }
        }

        if start.elapsed() > timeout {
            return Err(anyhow!("Timeout waiting for boot completion"));
        }

        sleep(Duration::from_millis(500)).await;
    }
}

/// Set a soft failure (allows test to continue but marks as failed)
async fn soft_fail(msg: String) {
    eprintln!("SOFT FAIL: {}", msg);
    {
        let mut guard = GLOBAL_LAST_ERROR.lock().await;
        *guard = Some(msg);
    }
    ANY_FAILURE.store(true, std::sync::atomic::Ordering::SeqCst);
}

#[given(expr = "I boot the OS in qemu for {string}")]
async fn boot_os_in_qemu(world: &mut BootWorld, arch: String) -> Result<()> {
    world.arch = arch.clone();
    let root = project_root();

    // 1. Build ISO quietly (capture output for error reporting)
    let output = Command::new("make")
        .args([&format!("template-{}.iso", arch)])
        .env("KARCH", &arch)
        .current_dir(&root)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .await
        .context("Failed to run make")?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!(
            "Failed to build ISO for {}. Exit status: {}.\nstdout:\n{}\nstderr:\n{}",
            arch,
            output.status,
            stdout,
            stderr
        ));
    }

    let iso_path = root.join(format!("template-{}.iso", arch));
    if !iso_path.exists() {
        return Err(anyhow!("ISO file not found at {}", iso_path.display()));
    }

    // 2. Prepare OVMF paths
    let ovmf_code = root.join(format!("ovmf/ovmf-code-{}.fd", arch));
    let ovmf_vars = root.join(format!("ovmf/ovmf-vars-{}.fd", arch));

    // 3. Socket path
    // Use a random or specific path
    let rand_id: u32 = rand::random();
    let qmp_sock = std::env::temp_dir().join(format!("thingos-qmp-{}.sock", rand_id));

    // Ensure previous QEMU is killed?
    {
        let mut g = GLOBAL_QEMU.lock().await;
        if let Some(mut old_qemu) = g.take() {
            let _ = old_qemu.kill().await;
        }
    }

    // 4. Spawn QEMU
    let mut qemu = QemuProcess::spawn(&arch, &iso_path, &ovmf_code, &ovmf_vars, &qmp_sock)
        .await
        .context("Failed to spawn QEMU")?;

    // Connect QMP now so it's ready for steps
    qemu.connect_qmp().await.context("Failed to connect QMP")?;

    *GLOBAL_QEMU.lock().await = Some(qemu);

    Ok(())
}

#[then(expr = "I expect to see {string} in the serial console")]
async fn expect_serial_output(_world: &mut BootWorld, expected: String) -> Result<()> {
    // Wait for output to appear (up to timeout)
    let timeout_secs = std::env::var("BDD_TIMEOUT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(30);
    let timeout = Duration::from_secs(timeout_secs);
    let start = std::time::Instant::now();

    loop {
        let log = get_clean_log().await;

        if log.contains(&expected) {
            return Ok(());
        }

        // Check QEMU status
        {
            let mut guard = GLOBAL_QEMU.lock().await;
            if let Some(qemu) = guard.as_mut() {
                if let Some(status) = qemu.check_status() {
                    let err_msg = format!(
                        "QEMU exited with status {:?}. Expected '{}' not found.",
                        status, expected
                    );
                    soft_fail(err_msg).await;
                    return Ok(());
                }
            }
        }

        if start.elapsed() > timeout {
            let log = get_clean_log().await;
            let err_msg = format!(
                "Expected '{}' in serial output. Got:\n--- START ---\n{}\n--- END ---",
                expected, log
            );
            soft_fail(err_msg).await;
            return Ok(());
        }

        sleep(Duration::from_millis(500)).await;
    }
}

// ============================================================================
// New step definitions
// ============================================================================

#[then("the serial console log must contain the following lines in order:")]
async fn check_ordered_lines(_world: &mut BootWorld, step: &Step) -> Result<()> {
    // Wait for boot to complete
    let log = match wait_for_boot_completion().await {
        Ok(l) => l,
        Err(e) => {
            soft_fail(format!("Boot did not complete: {}", e)).await;
            return Ok(());
        }
    };

    // Extract expected lines from data table
    let table = step.table.as_ref().ok_or_else(|| anyhow!("No data table in step"))?;
    let expected_lines: Vec<String> = table
        .rows
        .iter()
        .filter_map(|row| row.first().map(|s| s.trim().to_string()))
        .collect();

    // Find each line in order
    let mut search_pos = 0;
    for expected in &expected_lines {
        if let Some(pos) = log[search_pos..].find(expected) {
            search_pos += pos + expected.len();
        } else {
            let err_msg = format!(
                "Line '{}' not found in order. Log from position {}:\n{}",
                expected,
                search_pos,
                &log[search_pos..]
            );
            soft_fail(err_msg).await;
            return Ok(());
        }
    }

    Ok(())
}

#[then(expr = "I expect NOT to see {string} before {string}")]
async fn expect_not_before(_world: &mut BootWorld, needle: String, anchor: String) -> Result<()> {
    let log = match wait_for_boot_completion().await {
        Ok(l) => l,
        Err(e) => {
            soft_fail(format!("Boot did not complete: {}", e)).await;
            return Ok(());
        }
    };

    let anchor_pos = log.find(&anchor);
    let needle_pos = log.find(&needle);

    match (needle_pos, anchor_pos) {
        (Some(n), Some(a)) if n < a => {
            let err_msg = format!(
                "'{}' appeared at position {} before '{}' at position {}",
                needle, n, anchor, a
            );
            soft_fail(err_msg).await;
        }
        _ => {
            // Either needle not found, or anchor not found, or needle after anchor - all OK
        }
    }

    Ok(())
}

#[then(expr = "the serial console MAY miss any line starting with {string}")]
async fn may_miss_pattern(_world: &mut BootWorld, _pattern: String) -> Result<()> {
    // This is an acknowledgment step - no validation needed
    // It documents that missing BRAN: lines on some architectures is acceptable
    Ok(())
}

#[then(expr = "I MUST see {string}")]
async fn must_see(world: &mut BootWorld, expected: String) -> Result<()> {
    let log = match wait_for_boot_completion().await {
        Ok(l) => l,
        Err(e) => {
            soft_fail(format!("Boot did not complete: {}", e)).await;
            return Ok(());
        }
    };

    if !log.contains(&expected) {
        let err_msg = format!("Required line '{}' not found in log", expected);
        soft_fail(err_msg).await;
    } else {
        // Store anchor position for "after that point" steps
        if let Some(pos) = log.find(&expected) {
            world.last_anchor_position = Some(pos);
        }
    }

    Ok(())
}

#[then("after that point, I MUST see all of:")]
async fn after_point_must_see_all(world: &mut BootWorld, step: &Step) -> Result<()> {
    let log = match wait_for_boot_completion().await {
        Ok(l) => l,
        Err(e) => {
            soft_fail(format!("Boot did not complete: {}", e)).await;
            return Ok(());
        }
    };

    let anchor_pos = world.last_anchor_position.unwrap_or(0);
    let log_after = &log[anchor_pos..];

    // Extract expected lines from data table
    let table = step.table.as_ref().ok_or_else(|| anyhow!("No data table in step"))?;
    let expected_lines: Vec<String> = table
        .rows
        .iter()
        .filter_map(|row| row.first().map(|s| s.trim().to_string()))
        .collect();

    for expected in &expected_lines {
        if !log_after.contains(expected) {
            let err_msg = format!(
                "Required line '{}' not found after anchor position {}",
                expected, anchor_pos
            );
            soft_fail(err_msg).await;
            return Ok(());
        }
    }

    Ok(())
}

#[then(expr = "{string} appears exactly once in the serial console")]
async fn appears_exactly_once(_world: &mut BootWorld, expected: String) -> Result<()> {
    let log = match wait_for_boot_completion().await {
        Ok(l) => l,
        Err(e) => {
            soft_fail(format!("Boot did not complete: {}", e)).await;
            return Ok(());
        }
    };

    let count = log.matches(&expected).count();
    if count != 1 {
        let err_msg = format!(
            "'{}' appeared {} times, expected exactly once",
            expected, count
        );
        soft_fail(err_msg).await;
    }

    Ok(())
}

#[then(expr = "I expect to see {string}")]
async fn expect_to_see_simple(_world: &mut BootWorld, expected: String) -> Result<()> {
    let log = match wait_for_boot_completion().await {
        Ok(l) => l,
        Err(e) => {
            soft_fail(format!("Boot did not complete: {}", e)).await;
            return Ok(());
        }
    };

    if !log.contains(&expected) {
        let err_msg = format!("Expected '{}' not found in log", expected);
        soft_fail(err_msg).await;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_ansi_codes() {
        let input = "\x1b[2J\x1b[01;01HHello World";
        let output = strip_ansi_codes(input);
        assert_eq!(output, "Hello World");

        let input2 = "BRAN: starting\n\x1b[0mKERNEL: init";
        let output2 = strip_ansi_codes(input2);
        assert_eq!(output2, "BRAN: starting\nKERNEL: init");
    }
}
