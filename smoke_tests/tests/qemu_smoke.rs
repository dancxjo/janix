use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

const TIMEOUT_SECS: u64 = 60;
const LOG_DIR: &str = "../target/smoke_logs";

// Stronger markers (match “actually alive” moments)
const KERNEL_START_MARKER: &str = "ThingOS booting...";
const COMPOSITOR_START_MARKER: &str = "clouds.bmp: mapped"; // or "clouds.bmp: mapped"
const ROLLING_WINDOW_BYTES: usize = 64 * 1024;

struct QemuConfig<'a> {
    name: &'a str,
    make_target: &'a str,
}

fn enabled() -> bool {
    std::env::var("THINGOS_QEMU_SMOKE").ok().as_deref() == Some("1")
}

fn run_qemu_and_capture(cfg: &QemuConfig<'_>) -> String {
    if !enabled() {
        eprintln!(
            "[{}] Skipping QEMU smoke test; set THINGOS_QEMU_SMOKE=1 to enable.",
            cfg.name
        );
        return String::new();
    }

    // NOTE: This assumes Makefile respects these variables.
    // If your Makefile doesn’t, remove them and just run default.
    let mut cmd = Command::new("make");
    cmd.current_dir("..")
        .arg(cfg.make_target)
        .arg(format!("KARCH={}", cfg.name))
        // Prefer additive variables if Makefile supports them.
        .arg("QEMU_DISPLAY=none")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // Unix: put child in its own process group so we can kill the whole tree.
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        unsafe {
            cmd.pre_exec(|| {
                // setsid(): new session and process group
                libc::setsid();
                Ok(())
            });
        }
    }

    let mut child = cmd
        .spawn()
        .unwrap_or_else(|e| panic!("[{}] failed to spawn make: {e}", cfg.name));

    let mut stdout = child.stdout.take().expect("no stdout pipe");
    let mut stderr = child.stderr.take().expect("no stderr pipe");

    let (tx, rx) = mpsc::channel::<Vec<u8>>();
    let tx_out = tx.clone();
    let tx_err = tx.clone();

    thread::spawn(move || {
        let mut buf = [0u8; 4096];
        while let Ok(n) = stdout.read(&mut buf) {
            if n == 0 {
                break;
            }
            let _ = tx_out.send(buf[..n].to_vec());
        }
    });

    thread::spawn(move || {
        let mut buf = [0u8; 4096];
        while let Ok(n) = stderr.read(&mut buf) {
            if n == 0 {
                break;
            }
            let _ = tx_err.send(buf[..n].to_vec());
        }
    });

    // Parent holds tx; drop it so Disconnected can happen once readers finish.
    drop(tx);

    let start = Instant::now();
    let timeout = Duration::from_secs(TIMEOUT_SECS);

    let mut full_log = String::new();
    let mut rolling = String::new();

    let mut found_kernel = false;
    let mut found_compositor = false;

    loop {
        if start.elapsed() > timeout {
            eprintln!("[{}] Timeout reached ({}s)", cfg.name, TIMEOUT_SECS);
            break;
        }

        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(data) => {
                let chunk = String::from_utf8_lossy(&data);

                // Always keep full log for file output
                full_log.push_str(&chunk);

                // Rolling window for marker searching (avoid OOM)
                rolling.push_str(&chunk);
                if rolling.len() > ROLLING_WINDOW_BYTES {
                    let drain = rolling.len() - ROLLING_WINDOW_BYTES;
                    rolling.drain(..drain);
                }

                if !found_kernel && rolling.contains(KERNEL_START_MARKER) {
                    found_kernel = true;
                    eprintln!("[{}] Found kernel marker", cfg.name);
                }
                if !found_compositor && rolling.contains(COMPOSITOR_START_MARKER) {
                    found_compositor = true;
                    eprintln!("[{}] Found compositor marker", cfg.name);
                }

                if found_kernel && found_compositor {
                    eprintln!("[{}] All markers found, exiting early.", cfg.name);
                    break;
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                if let Ok(Some(status)) = child.try_wait() {
                    eprintln!("[{}] Process exited with {}", cfg.name, status);
                    break;
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }

    // Kill the process group (Unix) or just the child (fallback)
    #[cfg(unix)]
    {
        // If we created a new process group/session, PID == PGID
        let pid = child.id() as i32;
        unsafe {
            // kill(-pgid, SIGKILL)
            libc::kill(-pid, libc::SIGKILL);
        }
    }
    #[cfg(not(unix))]
    {
        let _ = child.kill();
    }

    let _ = child.wait();

    // Write log to file
    let abs_log_dir = std::fs::canonicalize(LOG_DIR).unwrap_or_else(|_| {
        // fallback: relative
        std::path::PathBuf::from(LOG_DIR)
    });
    let _ = std::fs::create_dir_all(&abs_log_dir);
    let log_path = abs_log_dir.join(format!("{}.log", cfg.name));

    if let Err(e) = std::fs::write(&log_path, &full_log) {
        eprintln!("[{}] Failed to write log to {:?}: {}", cfg.name, log_path, e);
    } else {
        eprintln!("[{}] Log written to {:?}", cfg.name, log_path);
    }

    full_log
}

fn assert_kernel_and_compositor_started(cfg: &QemuConfig<'_>) {
    let log = run_qemu_and_capture(cfg);
    if log.is_empty() {
        return;
    }

    let snippet: String = log.chars().rev().take(6000).collect::<String>().chars().rev().collect();

    assert!(
        log.contains(KERNEL_START_MARKER),
        "[{}] kernel marker '{}' not found.\n--- tail ---\n{}\n-----------",
        cfg.name,
        KERNEL_START_MARKER,
        snippet
    );

    assert!(
        log.contains(COMPOSITOR_START_MARKER),
        "[{}] compositor marker '{}' not found.\n--- tail ---\n{}\n-----------",
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
        make_target: "launch-x86_64",
    });
}
