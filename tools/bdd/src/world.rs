//! BDD World - holds test state during scenario execution.

use cucumber::World;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::Mutex;

/// The test world shared across all steps in a scenario.
#[derive(Debug, Default, World)]
pub struct ThingOsWorld {
    /// Target architecture for this test run
    pub arch: String,
    /// QEMU child process
    #[world(skip)]
    pub qemu: Option<Child>,
    /// Accumulated serial output
    pub serial_log: Arc<Mutex<String>>,
}

impl ThingOsWorld {
    /// Boot the OS in QEMU for the given architecture.
    pub async fn boot(&mut self, arch: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.arch = arch.to_string();

        let iso_path = format!("thing-os-{}.iso", arch);
        let ovmf_code = format!("ovmf/ovmf-code-{}.fd", arch);
        let ovmf_vars = format!("ovmf/ovmf-vars-{}.fd", arch);

        let qemu_bin = match arch {
            "x86_64" => "qemu-system-x86_64",
            "aarch64" => "qemu-system-aarch64",
            "riscv64" => "qemu-system-riscv64",
            "loongarch64" => "qemu-system-loongarch64",
            _ => return Err(format!("Unsupported architecture: {}", arch).into()),
        };

        // Build QEMU command with serial output to stdio
        let mut cmd = Command::new(qemu_bin);
        cmd.args(["-M", if arch == "x86_64" { "q35" } else { "virt" }]);

        // Add CPU for non-x86 architectures
        match arch {
            "aarch64" => { cmd.args(["-cpu", "cortex-a72"]); }
            "riscv64" => { cmd.args(["-cpu", "rv64"]); }
            "loongarch64" => { cmd.args(["-cpu", "la464"]); }
            _ => {}
        }

        cmd.args([
            "-m", "2G",
            "-nographic",
            "-serial", "stdio",
            "-drive", &format!("if=pflash,unit=0,format=raw,file={},readonly=on", ovmf_code),
            "-drive", &format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars),
            "-cdrom", &iso_path,
        ]);

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::null());

        let mut child = cmd.spawn()?;

        // Spawn a task to read serial output
        let stdout = child.stdout.take().expect("stdout was piped");
        let serial_log = self.serial_log.clone();

        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let mut log = serial_log.lock().await;
                log.push_str(&line);
                log.push('\n');
            }
        });

        self.qemu = Some(child);
        Ok(())
    }

    /// Wait for a string to appear in the serial log.
    pub async fn wait_for_serial(&self, needle: &str, timeout_secs: f64) -> bool {
        let start = std::time::Instant::now();
        let timeout = std::time::Duration::from_secs_f64(timeout_secs);

        loop {
            {
                let log = self.serial_log.lock().await;
                if log.contains(needle) {
                    return true;
                }
            }

            if start.elapsed() > timeout {
                return false;
            }

            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }

    /// Get the current serial log contents.
    pub async fn get_serial_log(&self) -> String {
        self.serial_log.lock().await.clone()
    }

    /// Kill the QEMU process if running.
    pub async fn shutdown(&mut self) {
        if let Some(ref mut child) = self.qemu {
            let _ = child.kill().await;
        }
    }
}
