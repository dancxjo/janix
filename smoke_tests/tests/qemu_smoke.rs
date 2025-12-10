use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

const TIMEOUT_SECS: u64 = 60;
const LOG_DIR: &str = "../target/smoke_logs";

// Adjust these to match your actual log strings
const KERNEL_START_MARKER: &str = "ThingOS booting...";
const COMPOSITOR_START_MARKER: &str = "compositor";

struct QemuConfig<'a> {
    name: &'a str,
    make_target: &'a str,
}

fn run_qemu_and_capture(cfg: &QemuConfig<'_>) -> String {
    // Optional env gate
    if std::env::var("THINGOS_QEMU_SMOKE").ok().as_deref() != Some("1") {
        eprintln!(
            "[{}] Skipping QEMU smoke test; set THINGOS_QEMU_SMOKE=1 to enable.",
            cfg.name
        );
        return String::new();
    }

    // We use 'make' to launch QEMU via the existing Makefile targets.
    // This ensures we use the exact same flags and images as 'make run'.
    // We override QEMUFLAGS to disable the display but keep memory settings.
    let mut child = Command::new("make")
        .current_dir("..")
        .arg(cfg.make_target)
        .arg(format!("KARCH={}", cfg.name))
        .arg("QEMUFLAGS=-m 2G -display none")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("[{}] failed to spawn make: {e}", cfg.name));

    let mut stdout = child.stdout.take().expect("no stdout pipe");
    let mut stderr = child.stderr.take().expect("no stderr pipe");

    let (tx, rx) = mpsc::channel();
    let tx_out = tx.clone();
    let tx_err = tx.clone();

    // Read stdout in a background thread
    thread::spawn(move || {
        let mut buf = [0u8; 1024];
        while let Ok(n) = stdout.read(&mut buf) {
            if n == 0 {
                break;
            }
            let _ = tx_out.send(Vec::from(&buf[..n]));
        }
    });

    // Read stderr in a background thread
    thread::spawn(move || {
        let mut buf = [0u8; 1024];
        while let Ok(n) = stderr.read(&mut buf) {
            if n == 0 {
                break;
            }
            let _ = tx_err.send(Vec::from(&buf[..n]));
        }
    });

    let start = Instant::now();
    let mut combined = String::new();
    let timeout = Duration::from_secs(TIMEOUT_SECS);

    let mut found_kernel = false;
    let mut found_compositor = false;

    loop {
        if start.elapsed() > timeout {
            eprintln!("[{}] Timeout reached ({}s)", cfg.name, TIMEOUT_SECS);
            break;
        }

        // Try to receive data with a short timeout to allow checking child status
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(data) => {
                let chunk = String::from_utf8_lossy(&data);
                combined.push_str(&chunk);

                // Check for markers
                if !found_kernel && combined.contains(KERNEL_START_MARKER) {
                    found_kernel = true;
                    eprintln!("[{}] Found kernel start marker", cfg.name);
                }
                if !found_compositor && combined.contains(COMPOSITOR_START_MARKER) {
                    found_compositor = true;
                    eprintln!("[{}] Found compositor start marker", cfg.name);
                }

                if found_kernel && found_compositor {
                    eprintln!("[{}] All markers found, exiting early.", cfg.name);
                    break;
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                // Check if child exited
                if let Ok(Some(status)) = child.try_wait() {
                    eprintln!("[{}] Process exited with {}", cfg.name, status);
                    break;
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                // Both streams closed
                break;
            }
        }
    }

    // Kill process tree if possible, or just the make command
    let _ = child.kill();
    let _ = child.wait();

    // Write log to file
    let abs_log_dir = std::fs::canonicalize("..")
        .unwrap_or_default()
        .join("target/smoke_logs");
    std::fs::create_dir_all(&abs_log_dir).ok();
    let log_path = abs_log_dir.join(format!("{}.log", cfg.name));

    if let Err(e) = std::fs::write(&log_path, &combined) {
        eprintln!(
            "[{}] Failed to write log to {:?}: {}",
            cfg.name, log_path, e
        );
    } else {
        eprintln!("[{}] Log written to {:?}", cfg.name, log_path);
    }

    combined
}

fn assert_kernel_and_compositor_started(cfg: &QemuConfig<'_>) {
    let log = run_qemu_and_capture(cfg);
    if log.is_empty() {
        return;
    }

    // Take a larger snippet for debugging if needed
    let snippet: String = log.chars().take(4000).collect();

    assert!(
        log.contains(KERNEL_START_MARKER),
        "[{}] kernel start marker '{}' not found.\n--- log snippet ---\n{}\n-------------------",
        cfg.name,
        KERNEL_START_MARKER,
        snippet
    );

    assert!(
        log.contains(COMPOSITOR_START_MARKER),
        "[{}] compositor start marker '{}' not found.\n--- log snippet ---\n{}\n-------------------",
        cfg.name,
        COMPOSITOR_START_MARKER,
        snippet
    );
}

#[test]
#[ignore]
fn qemu_smoke_x86_64() {
    assert_kernel_and_compositor_started(&QemuConfig {
        name: "x86_64",
        make_target: "launch-hdd-x86_64",
    });
}

#[test]
#[ignore]
fn qemu_smoke_aarch64() {
    assert_kernel_and_compositor_started(&QemuConfig {
        name: "aarch64",
        make_target: "launch-hdd-aarch64",
    });
}

#[test]
#[ignore]
fn qemu_smoke_riscv64() {
    assert_kernel_and_compositor_started(&QemuConfig {
        name: "riscv64",
        make_target: "launch-hdd-riscv64",
    });
}

#[test]
#[ignore]
fn qemu_smoke_loongarch64() {
    assert_kernel_and_compositor_started(&QemuConfig {
        name: "loongarch64",
        make_target: "launch-hdd-loongarch64",
    });
}
