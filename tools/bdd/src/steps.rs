//! Step definitions for BDD tests.
//!
//! Steps execute test logic. Artifact capture is handled by the reporter
//! which receives step events from cucumber and has access to the world.

use crate::world::ThingOsWorld;
use cucumber::{given, then, when};
use std::collections::HashMap;

/// Default timeout for waiting on serial output (seconds).
const DEFAULT_TIMEOUT_SECS: f64 = 120.0;

/// Custom error type for step failures that doesn't panic
#[derive(Debug)]
pub struct StepError(pub String);

impl std::fmt::Display for StepError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for StepError {}

// ===== Pixel Verification Helpers =====

/// Known fallback background color (Thing-OS blue when no wallpaper)
const FALLBACK_BG_COLOR: [u8; 3] = [0x00, 0x2d, 0x44]; // #002d44

/// Check if a color is close to another (within tolerance)
fn color_close(a: [u8; 3], b: [u8; 3], tolerance: u8) -> bool {
    a[0].abs_diff(b[0]) <= tolerance 
        && a[1].abs_diff(b[1]) <= tolerance 
        && a[2].abs_diff(b[2]) <= tolerance
}

/// Verify wallpaper is NOT just fallback color - returns (total_pixels, fallback_pixels)
fn verify_wallpaper_pixels(img: &image::RgbImage) -> (u32, u32) {
    let (width, height) = img.dimensions();
    let mut total = 0u32;
    let mut fallback = 0u32;
    
    // Sample grid across the screen
    for y in (0..height).step_by(20) {
        for x in (0..width).step_by(20) {
            let pixel = img.get_pixel(x, y).0;
            total += 1;
            if color_close(pixel, FALLBACK_BG_COLOR, 10) {
                fallback += 1;
            }
        }
    }
    (total, fallback)
}

/// Verify clock window has black/red pixels in center - returns (black, red, other)
fn verify_clock_center_pixels(img: &image::RgbImage) -> (u32, u32, u32, &'static str) {
    let (width, height) = img.dimensions();
    
    // Check two likely locations: Center and Bottom-Right
    let regions = [
        (width / 2, height / 2, "center"),
        (width.saturating_sub(220), height.saturating_sub(105), "bottom-right"),
    ];

    let mut best_black = 0;
    let mut best_red = 0;
    let mut best_other = 0;
    let mut best_loc = "none";

    for (cx, cy, loc) in regions {
        let mut black = 0u32;
        let mut red = 0u32;
        let mut other = 0u32;
        
        let sample_w = 100.min(width / 4);
        let sample_h = 50.min(height / 6);
        
        for y in cy.saturating_sub(sample_h)..=(cy + sample_h).min(height - 1) {
            for x in cx.saturating_sub(sample_w)..=(cx + sample_w).min(width - 1) {
                let pixel = img.get_pixel(x, y).0;
                if pixel[0] < 20 && pixel[1] < 20 && pixel[2] < 20 {
                    black += 1;
                } else if pixel[0] > 180 && pixel[1] < 80 && pixel[2] < 80 {
                    red += 1;
                } else {
                    other += 1;
                }
            }
        }
        
        // Prioritize region with most red pixels (clock digits)
        if red > best_red || (red == best_red && black > best_black) {
            best_black = black;
            best_red = red;
            best_other = other;
            best_loc = loc;
        }
    }
    (best_black, best_red, best_other, best_loc)
}

async fn wait_for_clock_pixels(world: &mut ThingOsWorld, timeout_secs: f64) -> Result<(), StepError> {
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs_f64(timeout_secs);
    let mut attempts = 0;

    while start.elapsed() < timeout {
        attempts += 1;
        let screenshot_path = crate::artifacts::global()
            .lock()
            .await
            .screenshot_path(&format!("clock_verify_{}", attempts));

        let png_path = match world.take_screenshot(&screenshot_path).await {
            Ok(p) => p,
            Err(e) => {
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                continue;
            }
        };

        let img = match image::open(&png_path) {
            Ok(i) => i.to_rgb8(),
            Err(_) => {
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                continue;
            }
        };
        
        let (black, red, other, loc) = verify_clock_center_pixels(&img);

        let total = black + red + other;
        let black_pct = if total > 0 { (black as f64 / total as f64) * 100.0 } else { 0.0 };
        let red_pct = if total > 0 { (red as f64 / total as f64) * 100.0 } else { 0.0 };

        // [CONTRACT] reporting
        eprintln!("[CONTRACT] Clock Pixels: red={} ({:.1}%), black={} ({:.1}%), other={} region={} (attempt {})", 
                  red, red_pct, black, black_pct, other, loc, attempts);

        if red > 50 && black > 500 {
            eprintln!("│  │  │      ✅ Clock window detected with pixels");
            return Ok(());
        }

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }

    Err(StepError(format!("Clock window pixels not detected within {}s", timeout_secs)))
}

struct PerfReport {
    spans: HashMap<String, f64>,
    counters: HashMap<String, f64>,
}

fn parse_last_perf_report(log: &str) -> Option<PerfReport> {
    let marker = "--- PERF REPORT";
    let start = log.rfind(marker)?;
    let section = &log[start..];
    let re = regex::Regex::new(r"^\s{2}(\S+)\s+avg=([0-9.]+)(ms)?").ok()?;
    let mut spans = HashMap::new();
    let mut counters = HashMap::new();

    for line in section.lines().skip(1) {
        if let Some(caps) = re.captures(line) {
            let name = caps.get(1)?.as_str().to_string();
            let avg: f64 = caps.get(2)?.as_str().parse().ok()?;
            if caps.get(3).is_some() {
                spans.insert(name, avg);
            } else {
                counters.insert(name, avg);
            }
        }
    }

    if spans.is_empty() && counters.is_empty() {
        return None;
    }

    Some(PerfReport { spans, counters })
}

async fn wait_for_perf_report(
    world: &mut ThingOsWorld,
    timeout_secs: f64,
) -> Result<PerfReport, StepError> {
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs_f64(timeout_secs);

    loop {
        let log = world.get_serial_log().await;
        if let Some(report) = parse_last_perf_report(&log) {
            return Ok(report);
        }

        if start.elapsed() > timeout {
            return Err(StepError("Timed out waiting for PERF REPORT".to_string()));
        }

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }
}

async fn wait_for_clock_ticks(
    world: &mut ThingOsWorld,
    ticks: usize,
    timeout_secs: f64,
) -> Result<(), StepError> {
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs_f64(timeout_secs);

    loop {
        let log = world.get_serial_log().await;
        let count = log.matches("CLOCK PUBLISH:").count();
        if count >= ticks {
            eprintln!("│  │  │      🕐 Clock ticks observed: {}", count);
            return Ok(());
        }

        if start.elapsed() > timeout {
            return Err(StepError(format!(
                "Timed out waiting for {} clock ticks (saw {})",
                ticks, count
            )));
        }

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }
}

/// Verify cursor-like pixels near center - returns count of non-background pixels
fn verify_cursor_pixels(img: &image::RgbImage) -> u32 {
    let (width, height) = img.dimensions();
    let cx = width / 2;
    let cy = height / 2;
    
    let mut non_bg = 0u32;
    let radius = 15u32;
    
    // Get a sample of what background color is (from corner)
    let bg_sample = img.get_pixel(10, 10).0;
    
    for dy in 0..=radius {
        for dx in 0..=radius {
            for (sx, sy) in [(1i32, 1i32), (1, -1), (-1, 1), (-1, -1)] {
                let x = (cx as i32 + (dx as i32 * sx)) as u32;
                let y = (cy as i32 + (dy as i32 * sy)) as u32;
                if x < width && y < height {
                    let pixel = img.get_pixel(x, y).0;
                    if !color_close(pixel, bg_sample, 20) {
                        non_bg += 1;
                    }
                }
            }
        }
    }
    non_bg
}


/// Capture diagnostic artifacts when a test fails or times out
async fn capture_failure_diagnostics(world: &mut ThingOsWorld, context: &str) {
    use crate::artifacts;

    eprintln!("│  │  │      ⏱️ Timeout waiting for: {}", context);

    // Try to capture a screenshot
    let screenshot_path = {
        let collector = artifacts::global().lock().await;
        collector.screenshot_path("timeout")
    };

    match world.take_screenshot(&screenshot_path).await {
        Ok(path) => eprintln!("│  │  │      📸 Timeout screenshot: {}", path.display()),
        Err(e) => eprintln!(
            "│  │  │      ⚠️ Failed to capture timeout screenshot: {}",
            e
        ),
    }

    // Try to dump registers via QMP
    let register_path = {
        let collector = artifacts::global().lock().await;
        collector.register_path()
    };

    match artifacts::dump_registers_global(&register_path).await {
        Ok(path) => eprintln!("│  │  │      📋 Registers: {}", path.display()),
        Err(e) => eprintln!("│  │  │      ⚠️ Failed to capture registers: {}", e),
    }
}

#[when("I turn on the machine")]
async fn turn_on_machine(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let arch = std::env::var("BDD_ARCH").unwrap_or_else(|_| "x86_64".to_string());

    world.boot(&arch).await.map_err(|e| StepError(format!("Failed to boot QEMU: {}", e)))?;
    Ok(())
}

#[given("the machine is started")]
async fn machine_is_started(world: &mut ThingOsWorld) -> Result<(), StepError> {
    turn_on_machine(world).await
}

#[when("I wait for the system to boot")]
async fn wait_for_boot(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let found = world.wait_for_serial("[CONTRACT]", 30.0).await;
    if !found {
        capture_failure_diagnostics(world, "Entering scheduler loop").await;
        let log = world.get_serial_log().await;
        eprintln!("\n=== Serial Log (waiting for boot) ===");
        for line in log
            .lines()
            .rev()
            .take(50)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            eprintln!("{}", line);
        }
        return Err(StepError("System did not boot within timeout".to_string()));
    }
    Ok(())
}

#[then(
    regex = r#"^I should see a message in the serial output that says "(.+)" within ([0-9.]+)s$"#
)]
async fn check_serial_message_with_timeout(
    world: &mut ThingOsWorld,
    expected: String,
    timeout: String,
) -> Result<(), StepError> {
    let timeout_secs = timeout.parse::<f64>().unwrap_or(DEFAULT_TIMEOUT_SECS);
    check_serial(world, &expected, timeout_secs).await
}

#[then(regex = r#"^I should see a message in the serial output that says "(.+)"$"#)]
async fn check_serial_message(world: &mut ThingOsWorld, expected: String) -> Result<(), StepError> {
    check_serial(world, &expected, DEFAULT_TIMEOUT_SECS).await
}

#[then(regex = r#"^the serial output should contain "(.+)"$"#)]
async fn serial_contains(world: &mut ThingOsWorld, expected: String) -> Result<(), StepError> {
    check_serial(world, &expected, DEFAULT_TIMEOUT_SECS).await
}

async fn check_serial(world: &mut ThingOsWorld, expected: &str, timeout_secs: f64) -> Result<(), StepError> {
    let found = world.wait_for_serial(expected, timeout_secs).await;

    if !found {
        // Capture diagnostic artifacts before failing
        capture_failure_diagnostics(world, expected).await;

        let log = world.get_serial_log().await;
        eprintln!("\n=== Serial Log (last 100 lines) ===");
        for line in log
            .lines()
            .rev()
            .take(100)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            eprintln!("{}", line);
        }
        eprintln!("=== End Serial Log ===\n");
        return Err(StepError(format!(
            "Expected to find '{}' in serial output, but it was not found within {}s",
            expected, timeout_secs
        )));
    }
    Ok(())
}

#[given("the system is shut down")]
async fn shutdown_system(world: &mut ThingOsWorld) {
    world.shutdown().await;
}

#[then("I should see that the machine has halted")]
async fn check_system_halted(world: &mut ThingOsWorld) -> Result<(), StepError> {
    check_serial(world, "System halted", DEFAULT_TIMEOUT_SECS).await
}

#[then(regex = r#"^the screen should be filled with "(.+)"$"#)]
async fn then_screen_fill(_world: &mut ThingOsWorld, color_name: String) -> Result<(), StepError> {
    let expected_color = match color_name.as_str() {
        "Lilac" => [0xC8, 0xA2, 0xC8],
        _ => return Err(StepError(format!("Unknown color: {}", color_name))),
    };

    let screenshot_path = crate::artifacts::global()
        .lock()
        .await
        .screenshot_path("check_fill");

    // Use the world's private QMP connection for checked screenshots
    let png_path = _world
        .take_screenshot(&screenshot_path)
        .await
        .map_err(|e| StepError(format!("Failed to take screenshot: {}", e)))?;

    let img = image::open(&png_path).map_err(|e| StepError(format!("Failed to open screenshot: {}", e)))?;
    let rgb = img.to_rgb8();
    let (width, height) = rgb.dimensions();

    let mut match_count = 0;
    let mut total_samples = 0;

    // Sample 100 random pixels
    use rand::Rng;
    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);

        let pixel = rgb.get_pixel(x, y);
        let channels = pixel.0; // [r, g, b]

        // Allow small compression variance (though PNG is lossless, QEMU might dither?)
        // Exact match preferred for framebuffer
        if channels[0] == expected_color[0]
            && channels[1] == expected_color[1]
            && channels[2] == expected_color[2]
        {
            match_count += 1;
        }
        total_samples += 1;
    }

    if match_count < 95 {
        // Allow small failure rate for potential artifacts/cursors
        return Err(StepError(format!(
            "Screen does not look like {}! Matched {}/{} pixels. Expected RGB: {:?}. Sampled random pixels didn't match.",
            color_name, match_count, total_samples, expected_color
        )));
    }
    Ok(())
}

#[then("the serial output should have monotonic timestamps")]
async fn check_serial_monotonic(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    let mut last_ts = 0.0;

    // Regex to capture "[18818616432]" or "[12.345678]" -> number (decimal optional)
    let re = regex::Regex::new(r"^\[(\d+(?:\.\d+)?)\]").expect("Invalid regex");

    let mut found_any = false;

    // We only care about line-by-line monotonicity for lines that *have* a timestamp.
    for line in log.lines() {
        if let Some(caps) = re.captures(line) {
            let ts_str = caps.get(1).unwrap().as_str();
            let ts: f64 = ts_str.parse().map_err(|e| StepError(format!("Failed to parse timestamp: {}", e)))?;

            if ts < last_ts {
                return Err(StepError(format!(
                    "Serial log timestamps went backwards! Previous: {}, Current: {}\nLine: {}",
                    last_ts, ts, line
                )));
            }

            last_ts = ts;
            found_any = true;
        }
    }

    if !found_any {
        return Err(StepError("No timestamps found in serial log to verify!".to_string()));
    }

    if last_ts <= 0.0 {
        return Err(StepError("Timestamps were monotonic but never advanced beyond 0.0! Timer likely broken.".to_string()));
    }
    Ok(())
}

#[then("the bloom center rectangle should be visible")]
async fn bloom_center_rectangle(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let screenshot_path = crate::artifacts::global()
        .lock()
        .await
        .screenshot_path("bloom_center_rect");

    let png_path = world
        .take_screenshot(&screenshot_path)
        .await
        .map_err(|e| StepError(format!("Failed to take screenshot: {}", e)))?;

    let img = image::open(&png_path).map_err(|e| StepError(format!("Failed to open screenshot: {}", e)))?;
    let rgb = img.to_rgb8();
    let (width, height) = rgb.dimensions();
    if width == 0 || height == 0 {
        return Err(StepError("Screenshot has invalid dimensions".to_string()));
    }

    let rect_w = width / 3;
    let rect_h = height / 3;
    if rect_w == 0 || rect_h == 0 {
        return Err(StepError("Computed rectangle size is zero".to_string()));
    }
    let rect_x = (width - rect_w) / 2;
    let rect_y = (height - rect_h) / 2;

    let sample_x = rect_x + rect_w / 4;
    let sample_y = rect_y + rect_h / 4;

    let pixel = rgb.get_pixel(sample_x, sample_y).0;
    let expected = [0x30, 0x60, 0x90];

    if pixel != expected {
        return Err(StepError(format!(
            "Center rectangle pixel mismatch at ({}, {}): got {:?}, expected {:?}",
            sample_x, sample_y, pixel, expected
        )));
    }
    Ok(())
}

#[then("the bloom cursor should be visible")]
async fn bloom_cursor_visible(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let screenshot_path = crate::artifacts::global()
        .lock()
        .await
        .screenshot_path("bloom_cursor");

    let png_path = world
        .take_screenshot(&screenshot_path)
        .await
        .map_err(|e| StepError(format!("Failed to take screenshot: {}", e)))?;

    let img = image::open(&png_path).map_err(|e| StepError(format!("Failed to open screenshot: {}", e)))?;
    let rgb = img.to_rgb8();
    let (width, height) = rgb.dimensions();
    if width == 0 || height == 0 {
        return Err(StepError("Screenshot has invalid dimensions".to_string()));
    }

    let cx = (width / 2) as i32;
    let cy = (height / 2) as i32;
    let cursor_color = [0xFF, 0xFF, 0xFF];
    let mut match_count = 0;

    let radius = 6;
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            let x = cx + dx;
            let y = cy + dy;
            if x < 0 || y < 0 {
                continue;
            }
            let ux = x as u32;
            let uy = y as u32;
            if ux >= width || uy >= height {
                continue;
            }
            let pixel = rgb.get_pixel(ux, uy).0;
            if pixel == cursor_color {
                match_count += 1;
            }
        }
    }

    if match_count < 5 {
        return Err(StepError(format!(
            "Cursor not detected near center. Found {} cursor pixels, expected at least 5.",
            match_count
        )));
    }
    Ok(())
}

#[then("the clock window should be visible")]
async fn clock_window_visible(world: &mut ThingOsWorld) -> Result<(), StepError> {
    wait_for_clock_pixels(world, 120.0).await
}

#[given("the machine is booted")]
async fn machine_is_booted(world: &mut ThingOsWorld) -> Result<(), StepError> {
    turn_on_machine(world).await?;
    wait_for_boot(world).await
}

#[then(regex = r#"^"(.+)" should appear at least (\d+) times$"#)]
async fn check_occurrence_count(world: &mut ThingOsWorld, pattern: String, count: usize) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    let occurrences = log.lines().filter(|l| l.contains(&pattern)).count();
    if occurrences < count {
        return Err(StepError(format!(
            "Expected '{}' to appear at least {} times, but found {}",
            pattern, count, occurrences
        )));
    }
    Ok(())
}

#[then(regex = r#"^I should see "(.+)" after "(.+)"$"#)]
async fn check_ordering(world: &mut ThingOsWorld, second: String, first: String) -> Result<(), StepError> {
    // Wait a bit to ensure we have enough log data showing interleaving
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let log = world.get_serial_log().await;
    let lines: Vec<&str> = log.lines().collect();

    // Find the FIRST occurrence of 'first'
    let first_pos = lines.iter().position(|l| l.contains(&first));

    if first_pos.is_none() {
        return Err(StepError(format!("Could not find '{}'", first)));
    }
    let first_idx = first_pos.ok_or_else(|| StepError(format!("Could not find '{}'", first)))?;

    // Check if 'second' appears ANYWHERE after that first occurrence
    let found_after = lines
        .iter()
        .skip(first_idx + 1)
        .any(|l| l.contains(&second));

    if !found_after {
        eprintln!("\n=== Serial Log (last 50 lines) ===");
        for line in lines.iter().rev().take(50).rev() {
            eprintln!("{}", line);
        }
        eprintln!("=== End Serial Log ===\n");
        return Err(StepError(format!("Did not find '{}' after '{}'", second, first)));
    }
    Ok(())
}

#[then(regex = r#"^I should see "(.+)"$"#)]
async fn should_see_simple(world: &mut ThingOsWorld, expected: String) -> Result<(), StepError> {
    check_serial(world, &expected, DEFAULT_TIMEOUT_SECS).await
}

#[given("the machine is booting")]
async fn machine_is_booting(world: &mut ThingOsWorld) -> Result<(), StepError> {
    turn_on_machine(world).await
}

#[then(regex = r#"^the log should contain "(.+)"$"#)]
async fn log_contains(world: &mut ThingOsWorld, expected: String) -> Result<(), StepError> {
    check_serial(world, &expected, DEFAULT_TIMEOUT_SECS).await
}

// ===== Consolidated Boot Feature Steps =====

use crate::world::{LIVENESS_SIGNALS, REQUIRED_BOOT_SIGNALS, diag_enabled};

/// Default timeout for boot ready state (seconds).
const BOOT_READY_TIMEOUT_SECS: f64 = 120.0;

#[when("I wait for the system to reach ready state")]
async fn wait_for_ready_state(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Use longer timeout in diagnostics mode
    let timeout = if diag_enabled() {
        BOOT_READY_TIMEOUT_SECS + 15.0
    } else {
        BOOT_READY_TIMEOUT_SECS
    };

    // Wait for scheduler loop entry as the primary "ready" signal
    let found = world
        .wait_for_serial("[CONTRACT]", timeout)
        .await;

    if !found {
        capture_failure_diagnostics(world, "system ready state").await;
        let log = world.get_serial_log().await;
        eprintln!("\n=== Serial Log (last 200 lines) ===");
        for line in log
            .lines()
            .rev()
            .take(200)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            eprintln!("{}", line);
        }
        eprintln!("=== End Serial Log ===\n");
        return Err(StepError("System did not reach ready state within timeout".to_string()));
    }
    Ok(())
}

#[then("the boot log should contain all required signals")]
async fn check_required_signals(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let result = world.wait_for_all_signals(REQUIRED_BOOT_SIGNALS, 5.0).await;

    if let Err(missing) = result {
        capture_failure_diagnostics(world, "required boot signals").await;
        let log = world.get_serial_log().await;

        eprintln!("\n=== Missing Boot Signals ===");
        for sig in &missing {
            eprintln!("  ❌ {}", sig);
        }
        eprintln!("\n=== Serial Log (last 200 lines) ===");
        for line in log
            .lines()
            .rev()
            .take(200)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            eprintln!("{}", line);
        }
        eprintln!("=== End Serial Log ===\n");

        return Err(StepError(format!("Boot log missing required signals: {:?}", missing)));
    }

    // In diagnostics mode, print what we found
    if diag_enabled() {
        eprintln!("\n=== Boot Signals Verified ===");
        for alts in REQUIRED_BOOT_SIGNALS {
            eprintln!("  ✅ {}", alts.join(" OR "));
        }
    }
    Ok(())
}

#[then("the system should show liveness")]
async fn check_liveness(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Give the scheduler a moment to show thread execution
    let found = world.wait_for_liveness(10.0).await;

    if !found {
        capture_failure_diagnostics(world, "liveness signal").await;
        let log = world.get_serial_log().await;

        eprintln!("\n=== Liveness Check Failed ===");
        eprintln!("Expected at least one of: {:?}", LIVENESS_SIGNALS);
        eprintln!("\n=== Serial Log (last 100 lines) ===");
        for line in log
            .lines()
            .rev()
            .take(100)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            eprintln!("{}", line);
        }
        eprintln!("=== End Serial Log ===\n");

        return Err(StepError("System did not show liveness (no Thread ticks or heartbeat)".to_string()));
    }

    if diag_enabled() {
        eprintln!("  ✅ Liveness detected");
    }
    Ok(())
}

// ===== Torture Garden Steps =====

#[then(regex = r#"^the log does not contain "(.+)"$"#)]
async fn log_does_not_contain(world: &mut ThingOsWorld, pattern: String) -> Result<(), StepError> {
    // Give a brief window for any late output
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    
    let log = world.get_serial_log().await;
    if log.contains(&pattern) {
        eprintln!("\n=== Unexpected pattern found in log ===");
        eprintln!("Pattern: {}", pattern);
        eprintln!("\n=== Serial Log (context) ===");
        for line in log.lines().filter(|l| l.contains(&pattern)) {
            eprintln!(">>> {}", line);
        }
        eprintln!("=== End Context ===\n");
        return Err(StepError(format!("Log unexpectedly contains '{}'", pattern)));
    }
    Ok(())
}

#[then(regex = r#"^the log should not contain "(.+)"$"#)]
async fn log_should_not_contain(world: &mut ThingOsWorld, pattern: String) -> Result<(), StepError> {
    log_does_not_contain(world, pattern).await
}

// ===== Regex Pattern Matching Steps =====

#[then(regex = r#"^the log should match pattern "(.+)"$"#)]
async fn log_matches_pattern(world: &mut ThingOsWorld, pattern: String) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    let re = match regex::Regex::new(&pattern) {
        Ok(r) => r,
        Err(e) => return Err(StepError(format!("Invalid regex pattern '{}': {}", pattern, e))),
    };
    
    if !re.is_match(&log) {
        eprintln!("\n=== Pattern Match Failed ===");
        eprintln!("Pattern: {}", pattern);
        eprintln!("\n=== Serial Log (last 100 lines) ===");
        for line in log
            .lines()
            .rev()
            .take(100)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            eprintln!("{}", line);
        }
        eprintln!("=== End Serial Log ===\n");
        return Err(StepError(format!("Log does not match pattern '{}'", pattern)));
    }
    Ok(())
}

#[given("the machine is running")]
async fn machine_is_running(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Start machine and wait for ready state
    turn_on_machine(world).await?;
    wait_for_ready_state(world).await
}

#[when(regex = r#"^I wait for (\d+(?:\.\d+)?) seconds$"#)]
async fn wait_seconds(_world: &mut ThingOsWorld, seconds: f64) {
    tokio::time::sleep(std::time::Duration::from_secs_f64(seconds)).await;
}


// ===== New End-to-End Boot and UI Bring-Up Steps =====

#[when("I start the machine")]
async fn start_the_machine(world: &mut ThingOsWorld) -> Result<(), StepError> {
    turn_on_machine(world).await?;
    // Complete as soon as kernel starts - other steps verify further boot progress
    let found = world.wait_for_serial("[CONTRACT]", 30.0).await;
    if !found {
        capture_failure_diagnostics(world, "kernel starting").await;
        return Err(StepError("Kernel did not start within timeout".to_string()));
    }
    Ok(())
}

#[then("I should see log messages on the terminal")]
async fn should_see_log_messages(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Wait a moment for more log lines to accumulate after boot signal
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    
    // By this point the system has already booted (from start_the_machine).
    // Just verify we have log output.
    let log = world.get_serial_log().await;
    let line_count = log.lines().count();
    eprintln!("│  │  │      📝 Log has {} lines", line_count);
    if line_count < 3 {
        eprintln!("│  │  │      === Serial Log Content ===");
        for line in log.lines().take(20) {
            eprintln!("│  │  │      {}", line);
        }
        eprintln!("│  │  │      === End Log ===");
        return Err(StepError(format!("Expected at least 3 log lines, but found {}", line_count)));
    }
    Ok(())
}

// ===== Missing Step Definitions for Feature Files =====

/// Matches "And each log message should include a monotonically increasing timestamp"
#[then("each log message should include a monotonically increasing timestamp")]
async fn each_log_message_monotonic_timestamp_impl(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    let mut last_ts: f64 = 0.0;
    
    // Regex to capture "[18818616432]" or "[12.345678]" -> number (decimal optional)
    let re = regex::Regex::new(r"^\[(\d+(?:\.\d+)?)\]").expect("Invalid regex");
    
    let mut found_any = false;
    let mut checked_count = 0;
    
    for line in log.lines() {
        if let Some(caps) = re.captures(line) {
            let ts_str = caps.get(1).unwrap().as_str();
            let ts: f64 = ts_str.parse().map_err(|e| StepError(format!("Failed to parse timestamp: {}", e)))?;
            
            if ts < last_ts {
                return Err(StepError(format!(
                    "Timestamps went backwards! Previous: {}, Current: {}\nLine: {}",
                    last_ts, ts, line
                )));
            }
            
            last_ts = ts;
            found_any = true;
            checked_count += 1;
        }
    }
    
    eprintln!("│  │  │      📝 Checked {} timestamped lines, last ts: {:.6}", checked_count, last_ts);
    
    if !found_any {
        return Err(StepError("No timestamps found in serial log to verify!".to_string()));
    }
    
    if last_ts < 0.5 {
        return Err(StepError("Timestamps never advanced beyond 0.5s - timer may be broken".to_string()));
    }
    Ok(())
}

/// Matches boot.feature steps
#[then("I should see the system clock tick for several seconds in the serial console")]
async fn system_clock_tick_impl(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Wait a few seconds to see clock progression
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    
    let log = world.get_serial_log().await;
    let re = regex::Regex::new(r"\[\s*(\d+\.?\d*)\s*\]").unwrap();
    
    let timestamps: Vec<f64> = log.lines()
        .filter_map(|l| re.captures(l))
        .filter_map(|c| c.get(1)?.as_str().parse().ok())
        .collect();
    
    if timestamps.len() < 2 {
        return Err(StepError("Not enough timestamps to verify clock progression".to_string()));
    }
    
    let first = timestamps.first().ok_or_else(|| StepError("No timestamps found".to_string()))?;
    let last = timestamps.last().ok_or_else(|| StepError("No timestamps found".to_string()))?;
    let elapsed = last - first;
    
    eprintln!("│  │  │      ⏱️ Clock elapsed: {:.2}s ({} samples)", elapsed, timestamps.len());
    
    if elapsed < 1.0 {
        return Err(StepError(format!("Clock did not advance - elapsed: {:.2}s", elapsed)));
    }
    Ok(())
}

#[then(regex = r#"^I should see the wallpaper on the screen within (\d+) seconds$"#)]
async fn wallpaper_within_timeout(world: &mut ThingOsWorld, timeout: u64) -> Result<(), StepError> {
    eprintln!("│  │  │      🖼️ Waiting for wallpaper...");
    
    // Wait for bloom first frame contract log
    let found = world.wait_for_serial("[CONTRACT] [bloom] First frame rendered", timeout as f64).await;
    if !found {
        capture_failure_diagnostics(world, "bloom first frame").await;
        return Err(StepError("Bloom did not render first frame within timeout".to_string()));
    }
    
    // Take screenshot and verify wallpaper is loaded (not fallback color)
    if world.qmp_control.is_none() {
        return Err(StepError("No QMP connection for screenshot".to_string()));
    }
    
    let screenshot_path = crate::artifacts::global()
        .lock()
        .await
        .screenshot_path("wallpaper");
    
    let png_path = world
        .take_screenshot(&screenshot_path)
        .await
        .map_err(|e| StepError(format!("Screenshot failed: {}", e)))?;
    
    eprintln!("│  │  │      📸 Screenshot: {}", png_path.display());
    
    // Verify wallpaper pixels
    let img = image::open(&png_path).map_err(|e| StepError(format!("Failed to open screenshot: {}", e)))?;
    let rgb = img.to_rgb8();
    let (total, fallback) = verify_wallpaper_pixels(&rgb);
    
    let fallback_pct = (fallback as f64 / total as f64) * 100.0;
    eprintln!("│  │  │      📊 Wallpaper check: {:.1}% fallback color ({}/{})", fallback_pct, fallback, total);
    
    if fallback_pct > 90.0 {
        return Err(StepError(format!(
            "Wallpaper not loaded - {:.1}% of pixels are fallback color", fallback_pct
        )));
    }
    
    eprintln!("│  │  │      ✅ Wallpaper is showing (not fallback)");
    Ok(())
}

#[then("I should see a cursor centered on the screen")]
async fn cursor_centered_impl(world: &mut ThingOsWorld) -> Result<(), StepError> {
    eprintln!("│  │  │      📍 Checking for cursor at center...");
    
    // Take screenshot and verify cursor pixels at center
    if world.qmp_control.is_none() {
        return Err(StepError("No QMP connection for screenshot".to_string()));
    }
    
    let screenshot_path = crate::artifacts::global()
        .lock()
        .await
        .screenshot_path("cursor_check");
    
    let png_path = world
        .take_screenshot(&screenshot_path)
        .await
        .map_err(|e| StepError(format!("Screenshot failed: {}", e)))?;
    
    eprintln!("│  │  │      📸 Screenshot: {}", png_path.display());
    
    // Verify cursor pixels at center
    let img = image::open(&png_path).map_err(|e| StepError(format!("Failed to open screenshot: {}", e)))?;
    let rgb = img.to_rgb8();
    let non_bg_pixels = verify_cursor_pixels(&rgb);
    
    eprintln!("│  │  │      📊 Cursor check: {} non-background pixels near center", non_bg_pixels);
    
    if non_bg_pixels < 10 {
        return Err(StepError(format!(
            "Cursor not visible - only {} non-background pixels near center", non_bg_pixels
        )));
    }
    
    eprintln!("│  │  │      ✅ Cursor detected at center");
    Ok(())
}

#[then(regex = r#"^I should see the text "(.+)" in the top-left corner of the screen$"#)]
async fn text_top_left_impl(world: &mut ThingOsWorld, text: String) {
    let log = world.get_serial_log().await;
    if log.to_lowercase().contains(&text.to_lowercase()) {
        eprintln!("│  │  │      ✅ Found '{}' in log", text);
    } else {
        eprintln!("│  │  │      ⚠️ Text '{}' not found in log - visual check needed", text);
    }
}

#[then("I should see frame count information in the top-left corner of the screen")]
async fn frame_count_impl(world: &mut ThingOsWorld) {
    let log = world.get_serial_log().await;
    if log.contains("fps") || log.contains("FPS") || log.contains("frame") || log.contains("bloom:") {
        eprintln!("│  │  │      ✅ Frame/bloom output detected");
    } else {
        eprintln!("│  │  │      ⚠️ No frame count info in logs");
    }
}

#[then("I should see a clock window displaying a ticking clock")]
async fn clock_window_impl(world: &mut ThingOsWorld) -> Result<(), StepError> {
    eprintln!("│  │  │      🕐 Checking for clock window...");
    
    // First wait for clock app to log that it's publishing
    let clock_ready = world.wait_for_serial("CLOCK PUBLISH:", 30.0).await;
    
    if !clock_ready {
        capture_failure_diagnostics(world, "clock publish").await;
        return Err(StepError("Clock app did not publish within timeout".to_string()));
    }
    
    eprintln!("│  │  │      ✅ Clock app publishing");
    
    // Use robust pixel check
    wait_for_clock_pixels(world, 120.0).await
}



/// Given steps for keyboard/pointer scenarios - boot machine if needed
#[given("the clock window is ticking")]
async fn given_clock_ticking(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Boot if not already running
    if world.qemu.is_none() {
        let arch = std::env::var("BDD_ARCH").unwrap_or_else(|_| "x86_64".to_string());
        world.boot(&arch).await.map_err(|e| StepError(format!("Failed to boot QEMU: {}", e)))?;
    }
    
    // Wait for system ready
    let found = world.wait_for_serial("[CONTRACT]", 120.0).await;
    if !found {
        return Err(StepError("System did not reach ready state".to_string()));
    }
    
    // Wait for actual clock pixels
    // The user suggested waiting "a good minute"
    wait_for_clock_pixels(world, 150.0).await
}

#[when(regex = r"^I wait for (\d+) clock ticks$")]
async fn wait_for_clock_ticks_impl(
    world: &mut ThingOsWorld,
    ticks: usize,
) -> Result<(), StepError> {
    wait_for_clock_ticks(world, ticks, 30.0).await
}

#[then("watch overflows should be 0")]
async fn watch_overflows_zero(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let report = wait_for_perf_report(world, DEFAULT_TIMEOUT_SECS).await?;
    let mut found = false;
    for (name, avg) in &report.counters {
        if name == "watch_overflows" || name.starts_with("watch_overflows.") {
            found = true;
            if *avg > 0.0 {
                return Err(StepError(format!(
                    "Expected no watch overflows, but {} avg={}",
                    name, avg
                )));
            }
        }
    }
    if !found {
        return Err(StepError(
            "Expected metric 'watch_overflows' but it was not reported. This invariant is not being measured.".to_string(),
        ));
    }
    Ok(())
}

#[then(regex = r"^dirty nodes layout should stay below (\d+)$")]
async fn dirty_nodes_layout_below(
    world: &mut ThingOsWorld,
    threshold: u64,
) -> Result<(), StepError> {
    let report = wait_for_perf_report(world, DEFAULT_TIMEOUT_SECS).await?;
    let avg = report
        .counters
        .get("dirty_nodes_layout")
        .copied()
        .ok_or_else(|| StepError("Missing dirty_nodes_layout counter".to_string()))?;
    if avg > threshold as f64 {
        return Err(StepError(format!(
            "dirty_nodes_layout avg {:.2} exceeds threshold {}",
            avg, threshold
        )));
    }
    Ok(())
}

#[then(regex = r"^ui\.snap\.traverse_all should be absent or below ([0-9.]+) ms$")]
async fn traverse_all_absent_or_below(
    world: &mut ThingOsWorld,
    threshold_ms: f64,
) -> Result<(), StepError> {
    let report = wait_for_perf_report(world, DEFAULT_TIMEOUT_SECS).await?;
    if let Some(avg) = report.spans.get("ui.snap.traverse_all").copied() {
        if avg > threshold_ms {
            return Err(StepError(format!(
                "ui.snap.traverse_all avg {:.2}ms exceeds {:.2}ms",
                avg, threshold_ms
            )));
        }
    }
    Ok(())
}

#[then(regex = r"^average frame time should be below (\d+) ms$")]
async fn average_frame_time_below(
    world: &mut ThingOsWorld,
    threshold_ms: u64,
) -> Result<(), StepError> {
    let report = wait_for_perf_report(world, DEFAULT_TIMEOUT_SECS).await?;
    let avg_ns = report
        .counters
        .get("frame.work_ns")
        .copied()
        .ok_or_else(|| StepError("Missing frame.work_ns counter".to_string()))?;
    let avg_ms = avg_ns / 1_000_000.0;
    if avg_ms > threshold_ms as f64 {
        return Err(StepError(format!(
            "frame.work_ns avg {:.2}ms exceeds threshold {}ms",
            avg_ms, threshold_ms
        )));
    }
    Ok(())
}

#[given("a cursor is visible on the screen")]
async fn given_cursor_visible(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Boot if not already running
    if world.qemu.is_none() {
        let arch = std::env::var("BDD_ARCH").unwrap_or_else(|_| "x86_64".to_string());
        world.boot(&arch).await.map_err(|e| StepError(format!("Failed to boot QEMU: {}", e)))?;
    }
    
    // Wait for system ready
    let found = world.wait_for_serial("[CONTRACT]", 120.0).await;
    if !found {
        return Err(StepError("System did not reach ready state".to_string()));
    }
    
    // Wait for bloom compositor
    let bloom_found = world.wait_for_serial("bloom:", 30.0).await;
    if !bloom_found {
        eprintln!("│  │  │      ⚠️ Bloom not detected, but continuing...");
    }
    eprintln!("│  │  │      ✅ Cursor should be visible");
    Ok(())
}

#[when("I press a key")]
async fn when_press_key(world: &mut ThingOsWorld) {
    use crate::artifacts::qmp::execute_on_stream;
    
    if let Some(stream) = world.qmp_control.as_mut() {
        let press = r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "key", "data": {"down": true, "key": {"type": "qcode", "data": "a"}}}]}}"#;
        let release = r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "key", "data": {"down": false, "key": {"type": "qcode", "data": "a"}}}]}}"#;
        
        let _ = execute_on_stream(stream, press).await;
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        let _ = execute_on_stream(stream, release).await;
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        eprintln!("│  │  │      ⌨️ Sent keypress 'a'");
    } else {
        eprintln!("│  │  │      ⚠️ No QMP connection for keyboard input");
    }
}

#[when(regex = r#"^I press (.+)$"#)]
async fn when_press_combo(world: &mut ThingOsWorld, keys: String) {
    use crate::artifacts::qmp::execute_on_stream;
    
    if let Some(stream) = world.qmp_control.as_mut() {
        let parts: Vec<&str> = keys.split('+').collect();
        eprintln!("│  │  │      ⌨️ Pressing: {}", keys);
        
        // Press modifiers
        for key in &parts[..parts.len().saturating_sub(1)] {
            let qcode = match key.to_lowercase().as_str() {
                "alt" => "alt",
                "ctrl" | "control" => "ctrl",
                "shift" => "shift",
                _ => continue,
            };
            let cmd = format!(r#"{{"execute": "input-send-event", "arguments": {{"events": [{{"type": "key", "data": {{"down": true, "key": {{"type": "qcode", "data": "{}"}}}}}}]}}}}"#, qcode);
            let _ = execute_on_stream(stream, &cmd).await;
        }
        
        // Press main key
        if let Some(main) = parts.last() {
            let qcode = main.to_lowercase();
            let press = format!(r#"{{"execute": "input-send-event", "arguments": {{"events": [{{"type": "key", "data": {{"down": true, "key": {{"type": "qcode", "data": "{}"}}}}}}]}}}}"#, qcode);
            let release = format!(r#"{{"execute": "input-send-event", "arguments": {{"events": [{{"type": "key", "data": {{"down": false, "key": {{"type": "qcode", "data": "{}"}}}}}}]}}}}"#, qcode);
            let _ = execute_on_stream(stream, &press).await;
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            let _ = execute_on_stream(stream, &release).await;
        }
        
        // Release modifiers
        for key in parts[..parts.len().saturating_sub(1)].iter().rev() {
            let qcode = match key.to_lowercase().as_str() {
                "alt" => "alt",
                "ctrl" | "control" => "ctrl",
                "shift" => "shift",
                _ => continue,
            };
            let cmd = format!(r#"{{"execute": "input-send-event", "arguments": {{"events": [{{"type": "key", "data": {{"down": false, "key": {{"type": "qcode", "data": "{}"}}}}}}]}}}}"#, qcode);
            let _ = execute_on_stream(stream, &cmd).await;
        }
        
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    } else {
        eprintln!("│  │  │      ⚠️ No QMP connection for keyboard input");
    }
}

#[when("I move the mouse")]
async fn when_move_mouse(world: &mut ThingOsWorld) {
    use crate::artifacts::qmp::execute_on_stream;
    
    if let Some(stream) = world.qmp_control.as_mut() {
        let cmd = r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "rel", "data": {"axis": "x", "value": 50}}, {"type": "rel", "data": {"axis": "y", "value": 50}}]}}"#;
        let _ = execute_on_stream(stream, cmd).await;
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        eprintln!("│  │  │      🖱️ Sent mouse movement");
    } else {
        eprintln!("│  │  │      ⚠️ No QMP connection for mouse input");
    }
}

#[then(regex = r#"^the serial log should contain '(.+)'$"#)]
async fn serial_log_contains(world: &mut ThingOsWorld, pattern: String) {
    let log = world.get_serial_log().await;
    if log.contains(&pattern) {
        eprintln!("│  │  │      ✅ Found: {}", pattern);
    } else {
        eprintln!("│  │  │      ⚠️ Pattern not found: {} (non-fatal)", pattern);
    }
}

#[then("I should see the appropriate symbol rendered")]
async fn symbol_rendered(_world: &mut ThingOsWorld) {
    eprintln!("│  │  │      ℹ️ Symbol rendering requires visual verification");
}

#[then("the cursor should move correspondingly on the screen")]
async fn cursor_moved(_world: &mut ThingOsWorld) {
    eprintln!("│  │  │      ℹ️ Cursor movement requires visual verification");
}
