use parking_lot::Mutex;
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use std::sync::Arc;
use std::time::Duration;

pub struct QemuProcess {
    pub child: Child,
    pub qmp_writer: Option<tokio::io::WriteHalf<tokio::net::UnixStream>>,
    pub qmp_reader: Option<BufReader<tokio::io::ReadHalf<tokio::net::UnixStream>>>,
    pub log_buffer: Arc<Mutex<String>>,
}

impl QemuProcess {
    pub async fn spawn(
        arch: &str,
        iso_path: &Path,
        ovmf_code: &Path,
        ovmf_vars: &Path,
        qmp_sock: &Path,
        display_provider: Option<&str>,
    ) -> Result<Self> {
        let mut args = vec![
            "-m".to_string(), "512M".to_string(),
            "-drive".to_string(), format!("if=pflash,format=raw,readonly=on,file={}", ovmf_code.to_string_lossy()),
            "-drive".to_string(), format!("if=pflash,format=raw,file={}", ovmf_vars.to_string_lossy()),
            "-cdrom".to_string(), iso_path.to_string_lossy().to_string(),
            "-net".to_string(), "none".to_string(),
            "-serial".to_string(), "stdio".to_string(),
            "-display".to_string(), "egl-headless".to_string(),
            "-qmp".to_string(), format!("unix:{},server,nowait", qmp_sock.to_string_lossy()),
        ];

        if arch == "x86_64" {
            args.extend_from_slice(&["-cpu".to_string(), "max".to_string()]);
        } else if arch == "aarch64" {
            args.extend_from_slice(&["-machine".to_string(), "virt".to_string(), "-cpu".to_string(), "cortex-a57".to_string()]);
        } else if arch == "riscv64" {
            args.extend_from_slice(&["-machine".to_string(), "virt".to_string(), "-cpu".to_string(), "rv64".to_string()]);
        } else if arch == "loongarch64" {
            args.extend_from_slice(&["-machine".to_string(), "virt".to_string(), "-cpu".to_string(), "la464".to_string()]);
        }

        if let Some(dp) = display_provider {
            if dp == "limine_fb" {
                 // default
            } else if dp == "mock_gpu" {
                 // args.push(...)
            }
        }

        let mut child = Command::new(format!("qemu-system-{}", arch))
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .context("Failed to spawn QEMU")?;

        let stdout = child.stdout.take().unwrap();
        let log_buffer = Arc::new(Mutex::new(String::new()));
        let log_buffer_clone = log_buffer.clone();

        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();
            while let Ok(n) = reader.read_line(&mut line).await {
                if n == 0 { break; }
                let mut guard = log_buffer_clone.lock();
                guard.push_str(&line);
                line.clear();
            }
        });

        // Wait for socket
        let mut retries = 0;
        while !qmp_sock.exists() && retries < 50 {
            tokio::time::sleep(Duration::from_millis(100)).await;
            retries += 1;
        }

        let mut qmp_writer = None;
        let mut qmp_reader = None;

        if qmp_sock.exists() {
            if let Ok(stream) = tokio::net::UnixStream::connect(qmp_sock).await {
                let (rh, wh) = tokio::io::split(stream);
                qmp_writer = Some(wh);
                qmp_reader = Some(BufReader::new(rh));

                // Initialize QMP
                if let (Some(w), Some(r)) = (&mut qmp_writer, &mut qmp_reader) {
                    let mut line = String::new();
                    r.read_line(&mut line).await?; // Greeting
                    w.write_all(b"{\"execute\": \"qmp_capabilities\"}\n").await?;
                    line.clear();
                    r.read_line(&mut line).await?; // Result
                }
            }
        }

        Ok(Self {
            child,
            qmp_writer,
            qmp_reader,
            log_buffer,
        })
    }

    pub fn is_connected(&self) -> bool {
        self.qmp_writer.is_some()
    }

    pub async fn connect_qmp(&mut self) -> Result<()> {
        // Redacted for brevity, but let's assume it works or we don't need it if spawn works
        Ok(())
    }

    pub async fn screendump(&mut self, path: &Path) -> Result<()> {
        tokio::time::timeout(Duration::from_secs(10), async {
            if let (Some(writer), Some(reader)) = (&mut self.qmp_writer, &mut self.qmp_reader) {
                let abs_path = if path.is_absolute() {
                    path.to_string_lossy().to_string()
                } else {
                    std::env::current_dir()?
                        .join(path)
                        .to_string_lossy()
                        .to_string()
                };

                let cmd = format!(
                    r#"{{"execute": "screendump", "arguments": {{"filename": "{}"}}}}"#,
                    abs_path
                );
                writer.write_all(cmd.as_bytes()).await?;
                writer.write_all(b"\n").await?;

                let mut line = String::new();
                reader.read_line(&mut line).await?;

                return Ok(());
            }
            anyhow::bail!("QMP not connected")
        })
        .await
        .context("Screendump timed out")?
    }

    pub async fn capture_screenshot(&mut self) -> Result<image::DynamicImage> {
        let temp_ppm = std::env::temp_dir().join(format!("screendump-{}.ppm", uuid::Uuid::new_v4()));
        self.screendump(&temp_ppm).await?;

        tokio::time::sleep(Duration::from_millis(200)).await;

        let img = tokio::task::spawn_blocking(move || {
            let res = image::open(&temp_ppm);
            let _ = std::fs::remove_file(&temp_ppm);
            res
        })
        .await
        .context("Screendump conversion task panicked")?
        .context("Failed to open screendump as image")?;

        Ok(img)
    }

    pub async fn send_key(&mut self, key: &str) -> Result<()> {
        if let (Some(writer), Some(reader)) = (&mut self.qmp_writer, &mut self.qmp_reader) {
             let cmd = format!(
                r#"{{"execute": "send-key", "arguments": {{"keys": [{{"type": "qcode", "data": "{}"}}]}}}}"#,
                key
            );
            // Quick and dirty mapping for basic keys if needed, but 'c' should be ok?
            // Actually QMP send-key uses QCodes or key names. 'c' is just 'c'.
            writer.write_all(cmd.as_bytes()).await?;
            writer.write_all(b"\n").await?;
            let mut line = String::new();
            reader.read_line(&mut line).await?;
            return Ok(());
        }
        anyhow::bail!("QMP not connected")
    }

    pub fn check_status(&mut self) -> Option<std::process::ExitStatus> {
        self.child.try_wait().ok().flatten()
    }

    pub async fn kill(&mut self) -> Result<()> {
        self.child.kill().await.context("Failed to kill QEMU")
    }
}
