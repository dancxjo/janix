use anyhow::{anyhow, Context, Result};
use cucumber::{given, then, World};
use std::path::PathBuf;
use tokio::process::Command;
use crate::qemu::QemuProcess;
use crate::shared::GLOBAL_QEMU;
use tokio::time::{sleep, Duration};

#[derive(Debug, Default, World)]
pub struct BootWorld {
    arch: String,
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

#[given(expr = "I boot the OS in qemu for {string}")]
async fn boot_os_in_qemu(world: &mut BootWorld, arch: String) -> Result<()> {
    world.arch = arch.clone();
    let root = project_root();

    // 1. Build ISO
    println!("Building ISO for {}...", arch);
    let output = Command::new("make")
        .args([&format!("template-{}.iso", arch)])
        .env("KARCH", &arch)
        .current_dir(&root)
        .output()
        .await
        .context("Failed to run make")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(anyhow!("Failed to build ISO for {}.\nSTDOUT:\n{}\nSTDERR:\n{}", arch, stdout, stderr));
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
    let mut qemu = QemuProcess::spawn(
        &arch,
        &iso_path,
        &ovmf_code,
        &ovmf_vars,
        &qmp_sock
    ).await.context("Failed to spawn QEMU")?;
    
    // Connect QMP now so it's ready for steps
    qemu.connect_qmp().await.context("Failed to connect QMP")?;
    
    *GLOBAL_QEMU.lock().await = Some(qemu);

    Ok(())
}

#[then(expr = "I expect to see {string} in the serial console")]
async fn expect_serial_output(_world: &mut BootWorld, expected: String) -> Result<()> {
    // Wait for output to appear (up to timeout)
    let timeout = Duration::from_secs(120);
    let start = std::time::Instant::now();
    
    loop {
        let current_log = {
            let guard = GLOBAL_QEMU.lock().await;
            if let Some(qemu) = guard.as_ref() {
                let log = qemu.log_buffer.lock().unwrap();
                log.clone()
            } else {
                String::new()
            }
        };

        if current_log.contains(&expected) {
            return Ok(());
        }

        if start.elapsed() > timeout {
            return Err(anyhow!(
                "Expected '{}' in serial output. Got:\n--- START ---\n{}\n--- END ---",
                expected, current_log
            ));
        }

        sleep(Duration::from_millis(500)).await;
    }
}
