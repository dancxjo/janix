use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tokio::process::{Child, Command};
use tokio::time::{sleep, Duration};

use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};

pub struct QemuProcess {
    child: Child,
    qmp_sock_path: PathBuf,
    qmp_reader: Option<BufReader<OwnedReadHalf>>,
    qmp_writer: Option<OwnedWriteHalf>,
    pub log_buffer: Arc<Mutex<String>>,
}

impl QemuProcess {
    pub async fn spawn(
        arch: &str,
        iso_path: &Path,
        ovmf_code: &Path,
        ovmf_vars: &Path,
        qmp_sock_path: &Path,
    ) -> Result<Self> {
        let qemu_bin = match arch {
            "x86_64" => "qemu-system-x86_64",
            "aarch64" => "qemu-system-aarch64",
            "riscv64" => "qemu-system-riscv64",
            "loongarch64" => "qemu-system-loongarch64",
            _ => anyhow::bail!("Unsupported architecture: {}", arch),
        };

        let mut args = vec![
            "-m".to_string(),
            "2G".to_string(),
            "-display".to_string(),
            "none".to_string(),
            "-serial".to_string(),
            "stdio".to_string(),
            "-no-reboot".to_string(),
            "-qmp".to_string(),
            format!("unix:{},server,nowait", qmp_sock_path.display()),
            "-d".to_string(),
            "cpu_reset".to_string(),
            "-D".to_string(),
            "qemu.log".to_string(),
        ];

        // Check for /dev/kvm and architecture match
        let host_arch = std::env::consts::ARCH;
        if false /* std::path::Path::new("/dev/kvm").exists() && host_arch == arch */ {
            println!(
                "Probing KVM: available and arch matches ({}). Using -accel kvm",
                host_arch
            );
            args.extend_from_slice(&["-accel".to_string(), "kvm".to_string()]);
        } else {
            println!(
                "Probing KVM: missing or arch mismatch (host: {}, target: {}). Using -accel tcg",
                host_arch, arch
            );
            args.extend_from_slice(&["-accel".to_string(), "tcg".to_string()]);
        }

        // Quiet stdio by default. Toggle via env when debugging.
        let verbose_stdio = std::env::var("BDD_VERBOSE_STDIO").is_ok();

        match arch {
            "x86_64" => {
                args.extend_from_slice(&[
                    "-M".to_string(),
                    "q35".to_string(),
                    "-drive".to_string(),
                    format!(
                        "if=pflash,unit=0,format=raw,file={},readonly=on",
                        ovmf_code.display()
                    ),
                    "-drive".to_string(),
                    format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars.display()),
                    "-cdrom".to_string(),
                    iso_path.to_string_lossy().to_string(),
                    "-device".to_string(),
                    "isa-debug-exit,iobase=0xf4,iosize=0x04".to_string(),
                ]);
            }
            "aarch64" => {
                args.extend_from_slice(&[
                    "-M".to_string(),
                    "virt".to_string(),
                    "-cpu".to_string(),
                    "cortex-a72".to_string(),
                    "-device".to_string(),
                    "ramfb".to_string(),
                    "-device".to_string(),
                    "qemu-xhci".to_string(),
                    "-device".to_string(),
                    "usb-kbd".to_string(),
                    "-device".to_string(),
                    "usb-mouse".to_string(),
                    "-drive".to_string(),
                    format!(
                        "if=pflash,unit=0,format=raw,file={},readonly=on",
                        ovmf_code.display()
                    ),
                    "-drive".to_string(),
                    format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars.display()),
                    "-cdrom".to_string(),
                    iso_path.to_string_lossy().to_string(),
                ]);
            }
            "riscv64" => {
                args.extend_from_slice(&[
                    "-M".to_string(),
                    "virt".to_string(),
                    "-cpu".to_string(),
                    "rv64".to_string(),
                    "-device".to_string(),
                    "ramfb".to_string(),
                    "-device".to_string(),
                    "qemu-xhci".to_string(),
                    "-device".to_string(),
                    "usb-kbd".to_string(),
                    "-device".to_string(),
                    "usb-mouse".to_string(),
                    "-drive".to_string(),
                    format!(
                        "if=pflash,unit=0,format=raw,file={},readonly=on",
                        ovmf_code.display()
                    ),
                    "-drive".to_string(),
                    format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars.display()),
                    "-cdrom".to_string(),
                    iso_path.to_string_lossy().to_string(),
                ]);
            }
            "loongarch64" => {
                args.extend_from_slice(&[
                    "-M".to_string(),
                    "virt".to_string(),
                    "-cpu".to_string(),
                    "la464".to_string(),
                    "-drive".to_string(),
                    format!(
                        "if=pflash,unit=0,format=raw,file={},readonly=on",
                        ovmf_code.display()
                    ),
                    "-drive".to_string(),
                    format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars.display()),
                    "-cdrom".to_string(),
                    iso_path.to_string_lossy().to_string(),
                ]);
            }
            _ => {}
        }

        println!("Spawning QEMU: {} {}", qemu_bin, args.join(" "));

        let mut child = Command::new(qemu_bin)
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to spawn QEMU")?;

        let stdout = child.stdout.take().context("Failed to take stdout")?;
        let stderr = child.stderr.take().context("Failed to take stderr")?;
        let log_buffer = Arc::new(Mutex::new(String::new()));

        // Spawn stdout reader
        let log_clone = log_buffer.clone();
        let verbose_out = verbose_stdio;
        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if verbose_out {
                    println!("[QEMU IMPERIAL] {}", line);
                }

                let mut buf = log_clone.lock().unwrap();
                buf.push_str(&line);
                buf.push('\n');
            }
        });

        // Spawn stderr reader
        let log_clone_err = log_buffer.clone();
        let verbose_err = verbose_stdio;
        tokio::spawn(async move {
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if verbose_err {
                    eprintln!("[QEMU IMPERIAL ERR] {}", line);
                }

                let mut buf = log_clone_err.lock().unwrap();
                buf.push_str("STDERR: ");
                buf.push_str(&line);
                buf.push('\n');
            }
        });

        Ok(Self {
            child,
            qmp_sock_path: qmp_sock_path.to_path_buf(),
            qmp_reader: None,
            qmp_writer: None,
            log_buffer,
        })
    }

    pub fn is_connected(&self) -> bool {
        self.qmp_writer.is_some()
    }

    pub async fn connect_qmp(&mut self) -> Result<()> {
        let start = std::time::Instant::now();
        loop {
            if start.elapsed() > Duration::from_secs(5) {
                anyhow::bail!("Timeout waiting for QMP socket");
            }
            match UnixStream::connect(&self.qmp_sock_path).await {
                Ok(stream) => {
                    let (read_half, mut write_half) = stream.into_split();
                    let mut reader = BufReader::new(read_half);

                    // Handshake
                    let mut line = String::new();
                    reader
                        .read_line(&mut line)
                        .await
                        .context("Failed to read QMP greeting")?;

                    write_half
                        .write_all(br#"{"execute": "qmp_capabilities"}"#)
                        .await?;
                    write_half.write_all(b"\n").await?;

                    line.clear();
                    reader
                        .read_line(&mut line)
                        .await
                        .context("Failed to read QMP capabilities response")?;
                    if !line.contains("return") {
                        anyhow::bail!("QMP handshake failed: {}", line);
                    }

                    self.qmp_reader = Some(reader);
                    self.qmp_writer = Some(write_half);
                    break;
                }
                Err(_) => {
                    sleep(Duration::from_millis(100)).await;
                }
            }
        }
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

                // Read output
                let mut line = String::new();
                reader.read_line(&mut line).await?;

                if !line.contains("return") {
                    // Might start with error?
                    if line.contains("error") {
                        eprintln!("QMP screendump error: {}", line);
                    }
                }
                return Ok(());
            }
            anyhow::bail!("QMP not connected")
        })
        .await
        .context("Screendump timed out")?
    }

    pub async fn kill(&mut self) -> Result<()> {
        self.child.kill().await.context("Failed to kill QEMU")
    }

    pub fn check_status(&mut self) -> Option<std::process::ExitStatus> {
        match self.child.try_wait() {
            Ok(Some(status)) => Some(status),
            Ok(None) => None,
            Err(e) => {
                eprintln!("Error waiting for QEMU status: {}", e);
                None
            }
        }
    }
}

impl Drop for QemuProcess {
    fn drop(&mut self) {
        // Attempt to kill QEMU when dropped
        // Use blocking kill in drop if needed, but start_kill is non-blocking
        let _ = self.child.start_kill();
    }
}
