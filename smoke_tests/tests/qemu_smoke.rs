use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

const TIMEOUT_SECS: u64 = 60;
const LOG_DIR: &str = "../target/smoke_logs";

// Milestones
const KERNEL_HEAP_MARKER: &str = "Kernel heap initialized";
const GRAPH_SEEDED_MARKER: &str = "Memory graph seeded.";
const SYSCALL_HANDLER_MARKER: &str = "Installing syscall handler...";
const TIMEKEEPING_MARKER: &str = "Initializing timekeeping...";
const DRIVERS_INIT_MARKER: &str = "Initializing hardware drivers...";
const INIT_LAUNCH_MARKER: &str = "Launching init (PID 1) ...";
const SCHEDULER_MARKER: &str = "Handing control to scheduler...";
const COMPOSITOR_START_MARKER: &str = "compositor: starting";
const CLOUDS_MAPPED_MARKER: &str = "clouds.bmp: mapped";

const ALL_MARKERS: &[&str] = &[
    KERNEL_HEAP_MARKER,
    GRAPH_SEEDED_MARKER,
    SYSCALL_HANDLER_MARKER,
    TIMEKEEPING_MARKER,
    DRIVERS_INIT_MARKER,
    INIT_LAUNCH_MARKER,
    SCHEDULER_MARKER,
    COMPOSITOR_START_MARKER,
    CLOUDS_MAPPED_MARKER,
];

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

    // Track which markers we have found
    let mut found_markers = vec![false; ALL_MARKERS.len()];

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

                // Check for failure patterns
                if rolling.contains("panicked at") {
                    eprintln!("[{}] CRITICAL FAILURE: Panic detected!", cfg.name);
                    break; // Exit loop to fail assertion
                }
                if rolling.contains("Out of memory") {
                    eprintln!("[{}] CRITICAL FAILURE: OOM detected!", cfg.name);
                    break;
                }

                // Check for markers
                for (i, marker) in ALL_MARKERS.iter().enumerate() {
                    if !found_markers[i] && rolling.contains(marker) {
                        found_markers[i] = true;
                        eprintln!("[{}] Found marker: '{}'", cfg.name, marker);
                    }
                }

                if found_markers.iter().all(|&f| f) {
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
        eprintln!(
            "[{}] Failed to write log to {:?}: {}",
            cfg.name, log_path, e
        );
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

    let snippet: String = log
        .chars()
        .rev()
        .take(6000)
        .collect::<String>()
        .chars()
        .rev()
        .collect();

    // 1. Check for panics
    assert!(
        !log.contains("panicked at"),
        "[{}] Kernel panic detected!\n--- tail ---\n{}\n-----------",
        cfg.name,
        snippet
    );
    assert!(
        !log.contains("Out of memory"),
        "[{}] Out of memory detected!\n--- tail ---\n{}\n-----------",
        cfg.name,
        snippet
    );

    // 2. Check for all milestones
    for marker in ALL_MARKERS {
        assert!(
            log.contains(marker),
            "[{}] Milestone '{}' not found.\n--- tail ---\n{}\n-----------",
            cfg.name,
            marker,
            snippet
        );
    }
}

#[test]
#[ignore]
fn qemu_smoke_x86_64() {
    assert_kernel_and_compositor_started(&QemuConfig {
        name: "x86_64",
        make_target: "launch-x86_64",
    });
}
