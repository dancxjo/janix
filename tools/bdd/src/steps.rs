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

#[when("I wait for the system to boot")]
async fn wait_for_boot(world: &mut ThingOsWorld) {
    let found = world.wait_for_serial("Thing-OS", 10.0).await;
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
