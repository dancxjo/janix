//! BDD World - holds test state during scenario execution.

use cucumber::World;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
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
    /// Path to QMP socket for QEMU control
    #[world(skip)]
    pub qmp_socket: Option<PathBuf>,
    /// VNC display number (for screenshot capture)
    #[world(skip)]
    pub vnc_display: Option<u16>,
}

impl ThingOsWorld {
    /// Boot the OS in QEMU for the given architecture.
    pub async fn boot(&mut self, arch: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.arch = arch.to_string();

        let iso_path = format!("thing-os-{}.iso", arch);
        let ovmf_code = format!("ovmf/ovmf-code-{}.fd", arch);
        let ovmf_vars = format!("ovmf/ovmf-vars-{}.fd", arch);

        // Create unique socket path for this test run
        let pid = std::process::id();
        let qmp_socket_path = PathBuf::from(format!("/tmp/qemu-bdd-{}.sock", pid));
        self.qmp_socket = Some(qmp_socket_path.clone());

        // Use a random VNC display to avoid conflicts with other QEMU instances
        let vnc_display = ((std::process::id() % 100) + 50) as u16; // 50-149 range
        self.vnc_display = Some(vnc_display);

        let qemu_bin = match arch {
            "x86_64" => "qemu-system-x86_64",
            "aarch64" => "qemu-system-aarch64",
            "riscv64" => "qemu-system-riscv64",
            "loongarch64" => "qemu-system-loongarch64",
            _ => return Err(format!("Unsupported architecture: {}", arch).into()),
        };

        // Build QEMU command with serial output to stdio and QMP control
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
            // Disable default display, use VNC instead
            "-display", "none",
            // Serial to stdio for log capture
            "-serial", "stdio",
            // VNC for headless graphics (needed for screenshots)
            "-vnc", &format!(":{}", vnc_display),
            // QMP control socket (server mode, don't wait for connection)
            "-qmp", &format!("unix:{},server=on,wait=off", qmp_socket_path.display()),
            // UEFI firmware
            "-drive", &format!("if=pflash,unit=0,format=raw,file={},readonly=on", ovmf_code),
            "-drive", &format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars),
            "-cdrom", &iso_path,
        ]);

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped()); // Capture stderr too to see QEMU errors

        let mut child = cmd.spawn()?;

        // Spawn a task to read serial output and sync to global cache
        let stdout = child.stdout.take().expect("stdout was piped");
        let serial_log = self.serial_log.clone();

        tokio::spawn(async move {
            use crate::artifacts;
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let mut log = serial_log.lock().await;
                log.push_str(&line);
                log.push('\n');
                // Sync to global cache for reporter access
                artifacts::set_latest_serial(&log).await;
            }
        });

        // Also spawn a task to read stderr for QEMU errors
        if let Some(stderr) = child.stderr.take() {
            tokio::spawn(async move {
                let reader = BufReader::new(stderr);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    eprintln!("[qemu-stderr] {}", line);
                }
            });
        }

        self.qemu = Some(child);

        // Wait for QMP socket to become available
        let mut qmp_initialized = false;
        for i in 0..50 {
            if qmp_socket_path.exists() {
                match self.qmp_init().await {
                    Ok(()) => {
                        eprintln!("[bdd] QMP initialized after {}ms", i * 100);
                        qmp_initialized = true;
                        break;
                    }
                    Err(e) => {
                        if i % 10 == 0 {
                            eprintln!("[bdd] QMP init attempt {}: {}", i, e);
                        }
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        
        if !qmp_initialized {
            eprintln!("[bdd] Warning: QMP not initialized - screenshots will not work");
        } else {
            // Store QMP socket path globally for reporter access
            crate::artifacts::set_qmp_socket(Some(qmp_socket_path.clone()));
        }

        Ok(())
    }

    /// Initialize QMP connection (capabilities negotiation).
    async fn qmp_init(&self) -> Result<(), Box<dyn std::error::Error>> {
        let socket_path = self.qmp_socket.as_ref().ok_or("No QMP socket")?;
        let mut stream = UnixStream::connect(socket_path).await?;

        // Read greeting
        let mut buf = vec![0u8; 4096];
        let _ = stream.readable().await;
        let _ = tokio::io::AsyncReadExt::read(&mut stream, &mut buf).await?;

        // Send qmp_capabilities to enter command mode
        let caps_cmd = r#"{"execute": "qmp_capabilities"}"#;
        stream.write_all(caps_cmd.as_bytes()).await?;
        stream.write_all(b"\n").await?;

        // Read response
        let _ = stream.readable().await;
        let _ = tokio::io::AsyncReadExt::read(&mut stream, &mut buf).await?;

        Ok(())
    }

    /// Take a screenshot using QMP screendump command.
    /// 
    /// Saves as PNG by converting from QEMU's PPM format.
    /// Returns the path to the saved screenshot.
    pub async fn take_screenshot(&self, output_path: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let socket_path = self.qmp_socket.as_ref().ok_or("No QMP socket")?;
        let mut stream = UnixStream::connect(socket_path).await?;

        // Read any pending data
        let mut buf = vec![0u8; 4096];
        let _ = tokio::time::timeout(
            std::time::Duration::from_millis(100),
            tokio::io::AsyncReadExt::read(&mut stream, &mut buf)
        ).await;

        // Ensure output directory exists
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Use a temp file for the PPM, then convert to PNG
        let ppm_path = output_path.with_extension("ppm");

        // Send screendump command
        let screendump_cmd = format!(
            r#"{{"execute": "screendump", "arguments": {{"filename": "{}"}}}}"#,
            ppm_path.display()
        );
        stream.write_all(screendump_cmd.as_bytes()).await?;
        stream.write_all(b"\n").await?;

        // Wait a moment for the file to be written
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        // Read response
        let mut response = vec![0u8; 4096];
        let _ = tokio::time::timeout(
            std::time::Duration::from_millis(500),
            tokio::io::AsyncReadExt::read(&mut stream, &mut response)
        ).await;

        if !ppm_path.exists() {
            return Err(format!("Screenshot was not saved to {}", ppm_path.display()).into());
        }

        // Convert PPM to PNG using image crate
        let png_path = output_path.with_extension("png");
        let img = image::open(&ppm_path)?;
        img.save(&png_path)?;

        // Remove the temporary PPM file
        let _ = std::fs::remove_file(&ppm_path);

        Ok(png_path)
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

        // Clean up QMP socket
        if let Some(ref socket_path) = self.qmp_socket {
            let _ = std::fs::remove_file(socket_path);
        }
    }
}
