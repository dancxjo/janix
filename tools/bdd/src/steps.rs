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
    let found = world.wait_for_serial("System booted", 10.0).await;
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
async fn check_screen_fill(world: &mut ThingOsWorld, color_name: String) {
    let expected_color = match color_name.as_str() {
        "Mallard Teal" => [0x00, 0x47, 0x4F],
        _ => panic!("Unknown color: {}", color_name),
    };

    let screenshot_path = crate::artifacts::global().lock().await.screenshot_path("check_fill");
    
    // Use the global artifacts helper to take the screenshot
    let png_path = crate::artifacts::take_screenshot_global(&screenshot_path).await
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
