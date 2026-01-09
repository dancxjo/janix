//! Step definitions for BDD tests.
//!
//! Steps execute test logic. Artifact capture is handled by the reporter
//! which receives step events from cucumber and has access to the world.

use cucumber::{given, then, when};
use crate::world::ThingOsWorld;

/// Default timeout for waiting on serial output (seconds).
const DEFAULT_TIMEOUT_SECS: f64 = 30.0;

#[when("I turn on the machine")]
async fn turn_on_machine(world: &mut ThingOsWorld) {
    let arch = std::env::var("BDD_ARCH").unwrap_or_else(|_| "x86_64".to_string());

    world.boot(&arch).await.expect("Failed to boot QEMU");
}

#[given("the machine is started")]
async fn machine_is_started(world: &mut ThingOsWorld) {
    turn_on_machine(world).await;
}

#[when("I wait for the system to boot")]
async fn wait_for_boot(world: &mut ThingOsWorld) {
    let found = world.wait_for_serial("System booted", 30.0).await;
    if !found {
        let log = world.get_serial_log().await;
        eprintln!("\n=== Serial Log (waiting for boot) ===");
        for line in log.lines().rev().take(50).collect::<Vec<_>>().into_iter().rev() {
            eprintln!("{}", line);
        }
        panic!("System did not boot within timeout");
    }
}

#[then(regex = r#"^I should see a message in the serial output that says "(.+)" within ([0-9.]+)s$"#)]
async fn check_serial_message_with_timeout(world: &mut ThingOsWorld, expected: String, timeout: String) {
    let timeout_secs = timeout.parse::<f64>().unwrap_or(DEFAULT_TIMEOUT_SECS);
    check_serial(world, &expected, timeout_secs).await;
}

#[then(regex = r#"^I should see a message in the serial output that says "(.+)"$"#)]
async fn check_serial_message(world: &mut ThingOsWorld, expected: String) {
    check_serial(world, &expected, DEFAULT_TIMEOUT_SECS).await;
}

#[then(regex = r#"^the serial output should contain "(.+)"$"#)]
async fn serial_contains(world: &mut ThingOsWorld, expected: String) {
    check_serial(world, &expected, DEFAULT_TIMEOUT_SECS).await;
}

async fn check_serial(world: &mut ThingOsWorld, expected: &str, timeout_secs: f64) {
    let found = world.wait_for_serial(expected, timeout_secs).await;

    if !found {
        let log = world.get_serial_log().await;
        eprintln!("\n=== Serial Log (last 100 lines) ===");
        for line in log.lines().rev().take(100).collect::<Vec<_>>().into_iter().rev() {
            eprintln!("{}", line);
        }
        eprintln!("=== End Serial Log ===\n");
        panic!("Expected to find '{}' in serial output, but it was not found within {}s", expected, timeout_secs);
    }
}

#[given("the system is shut down")]
async fn shutdown_system(world: &mut ThingOsWorld) {
    world.shutdown().await;
}

#[then("I should see that the machine has halted")]
async fn check_system_halted(world: &mut ThingOsWorld) {
    check_serial(world, "System halted", DEFAULT_TIMEOUT_SECS).await;
}
#[then(regex = r#"^the screen should be filled with "(.+)"$"#)]
async fn then_screen_fill(_world: &mut ThingOsWorld, color_name: String) {
    let expected_color = match color_name.as_str() {
        "Lilac" => [0xC8, 0xA2, 0xC8],
        _ => panic!("Unknown color: {}", color_name),
    };

    let screenshot_path = crate::artifacts::global().lock().await.screenshot_path("check_fill");
    
    // Use the world's private QMP connection for checked screenshots
    let png_path = _world.take_screenshot(&screenshot_path).await
        .expect("Failed to take screenshot");

    let img = image::open(&png_path).expect("Failed to open screenshot");
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
        if channels[0] == expected_color[0] && 
           channels[1] == expected_color[1] && 
           channels[2] == expected_color[2] {
            match_count += 1;
        }
        total_samples += 1;
    }

    if match_count < 95 { // Allow small failure rate for potential artifacts/cursors
         panic!("Screen does not look like {}! Matched {}/{} pixels. Expected RGB: {:?}. Sampled random pixels didn't match.", 
            color_name, match_count, total_samples, expected_color);
    }
}

#[then("the serial output should have monotonic timestamps")]
async fn check_serial_monotonic(world: &mut ThingOsWorld) {
    let log = world.get_serial_log().await;
    let mut last_ts = 0.0;
    
    // Regex to capture "[  12.345678]" -> 12.345678
    let re = regex::Regex::new(r"^\[\s*([0-9]+\.[0-9]+)\s*\]").expect("Invalid regex");

    let mut found_any = false;

    // We only care about line-by-line monotonicity for lines that *have* a timestamp.
    for line in log.lines() {
        if let Some(caps) = re.captures(line) {
            let ts_str = caps.get(1).unwrap().as_str();
            let ts: f64 = ts_str.parse().expect("Failed to parse timestamp");
            
            if ts < last_ts {
                panic!("Serial log timestamps went backwards! Previous: {}, Current: {}\nLine: {}", last_ts, ts, line);
            }
            
            last_ts = ts;
            found_any = true;
        }
    }

    if !found_any {
        panic!("No timestamps found in serial log to verify!");
    }

    if last_ts == 0.0 {
        panic!("Timestamps were monotonic but never advanced beyond 0.0! Timer likely broken.");
    }
}

#[given("the machine is booted")]
async fn machine_is_booted(world: &mut ThingOsWorld) {
    turn_on_machine(world).await;
    wait_for_boot(world).await;
}

#[then(regex = r#"^"(.+)" should appear at least (\d+) times$"#)]
async fn check_occurrence_count(world: &mut ThingOsWorld, pattern: String, count: usize) {
    let log = world.get_serial_log().await;
    let occurrences = log.lines().filter(|l| l.contains(&pattern)).count();
    if occurrences < count {
        panic!("Expected '{}' to appear at least {} times, but found {}", pattern, count, occurrences);
    }
}

#[then(regex = r#"^I should see "(.+)" after "(.+)"$"#)]
async fn check_ordering(world: &mut ThingOsWorld, second: String, first: String) {
    let log = world.get_serial_log().await;
    let first_pos = log.lines().position(|l| l.contains(&first));
    
    if first_pos.is_none() { 
        panic!("Could not find '{}'", first); 
    }
    let first_idx = first_pos.unwrap();
    
    let remainder = log.lines().skip(first_idx + 1);
    if !remainder.into_iter().any(|l| l.contains(&second)) {
         panic!("Did not find '{}' after '{}'", second, first);
    }
}

#[then(regex = r#"^I should see "(.+)"$"#)]
async fn should_see_simple(world: &mut ThingOsWorld, expected: String) {
    check_serial(world, &expected, DEFAULT_TIMEOUT_SECS).await;
}
