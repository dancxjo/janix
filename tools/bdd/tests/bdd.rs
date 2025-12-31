//! BDD tests for ThingOS boot process using Cucumber.
//!
//! These tests verify that the OS boots correctly for each architecture
//! by checking for expected output in the serial console.

use cucumber::{given, then, World};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::time::{timeout, Duration};

/// Test world state for boot scenarios
#[derive(Debug, Default, World)]
pub struct BootWorld {
    /// Architecture being tested (x86_64, aarch64, etc)
    arch: String,
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

#[given(expr = "I boot the OS in qemu for {string}")]
async fn boot_os_in_qemu(world: &mut BootWorld, arch: String) {
    world.arch = arch.clone();
    world.boot_attempted = true;
    let root = project_root();

    let output = Command::new("make")
        .args([&format!("template-{}.iso", arch)])
        .env("KARCH", &arch)
        .current_dir(&root)
        .output()
        .await
        .expect("Failed to run make");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        panic!("Failed to build ISO for {}.\nSTDOUT:\n{}\nSTDERR:\n{}", arch, stdout, stderr);
    }


    let iso_path = root.join(format!("template-{}.iso", arch));
    assert!(iso_path.exists(), "ISO file not found at {}", iso_path.display());

    // 2. Prepare OVMF paths
    let ovmf_code = root.join(format!("ovmf/ovmf-code-{}.fd", arch));
    let ovmf_vars = root.join(format!("ovmf/ovmf-vars-{}.fd", arch));

    // 3. Build QEMU command
    let mut cmd = match arch.as_str() {
        "x86_64" => {
            let mut c = Command::new("qemu-system-x86_64");
            c.args([
                "-M", "q35",
                "-m", "2G",
                "-display", "none",
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
                "-m", "2G",
                "-device", "ramfb",
                "-device", "qemu-xhci",
                "-device", "usb-kbd",
                "-device", "usb-mouse",
                "-display", "none",
                "-serial", "stdio",
                "-drive", &format!("if=pflash,unit=0,format=raw,file={},readonly=on", ovmf_code.display()),
                "-drive", &format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars.display()),
                "-cdrom", &iso_path.to_string_lossy(),
                "-no-reboot",
                "-d", "guest_errors", // Debug aid
            ]);
            c
        }
        "riscv64" => {
            let mut c = Command::new("qemu-system-riscv64");
            c.args([
                "-M", "virt",
                "-cpu", "rv64",
                "-m", "2G",
                "-device", "ramfb",
                "-device", "qemu-xhci",
                "-device", "usb-kbd",
                "-device", "usb-mouse",
                "-display", "none",
                "-serial", "stdio",
                "-drive", &format!("if=pflash,unit=0,format=raw,file={},readonly=on", ovmf_code.display()),
                "-drive", &format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars.display()),
                "-cdrom", &iso_path.to_string_lossy(),
                "-no-reboot",
            ]);
            c
        }
        "loongarch64" => {
            // Note: loongarch64 usually requires external bios if pflash not available
            // but we'll try standard way. If it fails, it fails.
            let mut c = Command::new("qemu-system-loongarch64");
            c.args([
                "-M", "virt",
                "-cpu", "la464",
                "-m", "2G",
                "-display", "none",
                "-serial", "stdio",
                "-drive", &format!("if=pflash,unit=0,format=raw,file={},readonly=on", ovmf_code.display()),
                "-drive", &format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars.display()),
                "-cdrom", &iso_path.to_string_lossy(),
                "-no-reboot",
            ]);
            c
        }
        _ => panic!("Unsupported architecture: {}", arch),
    };

    cmd.stdout(Stdio::piped())
       .stderr(Stdio::piped())
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
                        let mut buf = output_buffer_clone.lock().expect("Failed to lock output buffer");
                        buf.push_str(&line);
                        buf.push('\n');
                    }
                    
                    if line.contains("Booted.") {
                        return true;
                    }
                    if line.contains("panic") || line.contains("PANIC") {
                        return true;
                    }
                }
                false
            }).await;

            let _ = child.kill().await;

            let captured = output_buffer.lock().expect("Failed to lock output buffer").clone();
            
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

#[then(expr = "I expect to see {string} in the serial console")]
async fn expect_serial_output(world: &mut BootWorld, expected: String) {
    if !world.serial_output.contains(&expected) {
        panic!(
            "Expected '{}' in serial output. Got:\n--- START ---\n{}\n--- END ---",
            expected, world.serial_output
        );
    }
}

#[tokio::main]
async fn main() {
    BootWorld::run("features").await;
}
