//! BDD tests for ThingOS boot process using Cucumber.
//!
//! These tests verify that the OS boots correctly for each architecture
//! by checking for expected output in the serial console.

use cucumber::{given, then, when, World};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::time::{timeout, Duration};

/// Test world state for boot scenarios
#[derive(Debug, Default, World)]
pub struct BootWorld {
    /// Architecture being tested (x86_64, aarch64)
    arch: String,
    /// Path to the ISO file
    iso_path: Option<PathBuf>,
    /// Captured serial output from QEMU
    serial_output: String,
    /// Whether boot was attempted
    boot_attempted: bool,
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

#[given(expr = "I have an iso for {string}")]
async fn have_iso_for_arch(world: &mut BootWorld, arch: String) {
    world.arch = arch.clone();

    let root = project_root();

    // Build the ISO using make
    let iso_result = Command::new("make")
        .args([&format!("KARCH={}", arch)])
        .current_dir(&root)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .status()
        .await;

    match iso_result {
        Ok(status) if status.success() => {
            let iso_path = root.join(format!("template-{}.iso", arch));
            if iso_path.exists() {
                world.iso_path = Some(iso_path);
            }
        }
        Ok(_) => {
            world.iso_path = None;
        }
        Err(e) => {
            eprintln!("Failed to build ISO: {}", e);
            world.iso_path = None;
        }
    }
}

#[when("I boot")]
async fn boot_system(world: &mut BootWorld) {
    world.boot_attempted = true;

    let iso_path = match &world.iso_path {
        Some(p) => p.clone(),
        None => {
            world.serial_output = "ERROR: No ISO available".to_string();
            return;
        }
    };

    let root = project_root();
    let ovmf_code = root.join(format!("ovmf/ovmf-code-{}.fd", world.arch));
    let ovmf_vars = root.join(format!("ovmf/ovmf-vars-{}.fd", world.arch));

    // Build QEMU command based on architecture
    let mut cmd = match world.arch.as_str() {
        "x86_64" => {
            let mut c = Command::new("qemu-system-x86_64");
            c.args([
                "-M", "q35",
                "-m", "256M",
                "-nographic",
                "-serial", "stdio",
                "-drive", &format!("if=pflash,unit=0,format=raw,file={},readonly=on", ovmf_code.display()),
                "-drive", &format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars.display()),
                "-cdrom", &iso_path.to_string_lossy(),
                "-no-reboot",
            ]);
            c
        }
        "aarch64" => {
            let mut c = Command::new("qemu-system-aarch64");
            c.args([
                "-M", "virt",
                "-cpu", "cortex-a72",
                "-m", "256M",
                "-nographic",
                "-serial", "stdio",
                "-drive", &format!("if=pflash,unit=0,format=raw,file={},readonly=on", ovmf_code.display()),
                "-drive", &format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars.display()),
                "-cdrom", &iso_path.to_string_lossy(),
                "-no-reboot",
            ]);
            c
        }
        "riscv64" => {
            let mut c = Command::new("qemu-system-riscv64");
            c.args([
                "-M", "virt",
                "-cpu", "rv64",
                "-m", "256M",
                "-nographic",
                "-serial", "stdio",
                "-drive", &format!("if=pflash,unit=0,format=raw,file={},readonly=on", ovmf_code.display()),
                "-drive", &format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars.display()),
                "-cdrom", &iso_path.to_string_lossy(),
                "-no-reboot",
            ]);
            c
        }
        "loongarch64" => {
            let mut c = Command::new("qemu-system-loongarch64");
            c.args([
                "-M", "virt",
                "-cpu", "la464",
                "-m", "256M",
                "-nographic",
                "-serial", "stdio",
                "-drive", &format!("if=pflash,unit=0,format=raw,file={},readonly=on", ovmf_code.display()),
                "-drive", &format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars.display()),
                "-cdrom", &iso_path.to_string_lossy(),
                "-no-reboot",
            ]);
            c
        }
        _ => {
            world.serial_output = format!("ERROR: Unsupported architecture: {}", world.arch);
            return;
        }
    };

    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.current_dir(&root);

    // Start QEMU and capture output with timeout
    let boot_timeout = Duration::from_secs(60);

    match cmd.spawn() {
        Ok(mut child) => {
            let stdout = child.stdout.take().expect("stdout");
            let mut reader = BufReader::new(stdout).lines();

            let result = timeout(boot_timeout, async {
                let mut output = String::new();
                while let Ok(Some(line)) = reader.next_line().await {
                    output.push_str(&line);
                    output.push('\n');

                    // Check for boot success or failure indicators
                    if line.contains("Booted.") || line.contains("panic") || line.contains("PANIC") {
                        break;
                    }
                }
                output
            })
            .await;

            // Kill QEMU
            let _ = child.kill().await;

            world.serial_output = match result {
                Ok(out) => out,
                Err(_) => {
                    "TIMEOUT: Boot did not complete within 60 seconds".to_string()
                }
            };
        }
        Err(e) => {
            world.serial_output = format!("ERROR: Failed to start QEMU: {}", e);
        }
    }
}

#[then(expr = "I should see {string} in the serial console")]
async fn check_serial_output(world: &mut BootWorld, expected: String) {
    assert!(
        world.boot_attempted,
        "Boot was not attempted"
    );

    assert!(
        world.serial_output.contains(&expected),
        "Expected to find '{}' in serial output, but got:\n{}",
        expected,
        world.serial_output
    );
}

#[tokio::main]
async fn main() {
    BootWorld::run("features").await;
}
