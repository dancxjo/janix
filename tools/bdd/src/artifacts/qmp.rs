use std::path::PathBuf;
use std::sync::OnceLock;
use tokio::sync::Mutex;
use tokio::net::UnixStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Global QMP stream (for reporter access to screenshots).
/// Kept open to avoid reconnection issues.
pub(crate) static QMP_STREAM: OnceLock<Mutex<Option<UnixStream>>> = OnceLock::new();

/// Set the global QMP stream (called from world after init).
pub async fn set_qmp_stream(stream: Option<UnixStream>) {
    if let Some(cache) = QMP_STREAM.get() {
        let mut guard = cache.lock().await;
        *guard = stream;
    }
}

async fn qmp_execute(command: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mutex = QMP_STREAM.get().ok_or("Artifacts system not initialized")?;
    let mut guard = mutex.lock().await;

    let stream = match guard.as_mut() {
        Some(s) => s,
        None => return Err("No QMP connection active".into()),
    };

    // Helper to read a QMP line
    async fn read_line(stream: &mut UnixStream) -> std::io::Result<String> {
        let mut buf = [0u8; 1];
        let mut line = String::new();
        loop {
            // Use a timeout for each byte
            match tokio::time::timeout(std::time::Duration::from_millis(2000), stream.read(&mut buf)).await {
                Ok(Ok(n)) if n > 0 => {
                    let c = buf[0] as char;
                    line.push(c);
                    if c == '\n' {
                        break;
                    }
                }
                Ok(Ok(0)) => return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "EOF")),
                Ok(Err(e)) => return Err(e),
                Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "Read timeout")),
                Ok(Ok(_)) => unreachable!("Buffer is size 1"),
            }
        }
        Ok(line)
    }

    // Send command
    if let Err(e) = stream.write_all(command.as_bytes()).await {
        return Err(format!("Failed to send QMP command: {}", e).into());
    }
    if let Err(e) = stream.write_all(b"\n").await {
        return Err(format!("Failed to send QMP newline: {}", e).into());
    }

    // Read response, filtering out asynchronous events
    loop {
        match read_line(stream).await {
            Ok(res) => {
                let _trimmed = res.trim();
                // eprintln!("QMP READ: {}", _trimmed); // Debug logging

                // Ignore asynchronous events
                if res.contains(r#""event":"#) {
                     continue;
                }
                return Ok(res);
            }
            Err(e) => return Err(format!("Failed to read QMP response: {}", e).into()),
        }
    }
}


/// Take a screenshot using the global QMP socket (for reporter).
pub async fn take_screenshot_global(output_path: &std::path::Path) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    // Ensure output directory exists
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Get absolute path for QEMU
    let ppm_path = output_path.with_extension("ppm");
    let ppm_abs = std::fs::canonicalize(output_path.parent().unwrap())?
        .join(ppm_path.file_name().unwrap());

    // Retry a few times if "device not ready" or similar transient errors occur
    let mut success = false;
    for _ in 0..3 {
        let screendump_cmd = format!(
            r#"{{"execute": "screendump", "arguments": {{"filename": "{}"}}}}"#,
            ppm_abs.display()
        );

        match qmp_execute(&screendump_cmd).await {
            Ok(resp) => {
                if !resp.contains("error") {
                    success = true;
                    break;
                }
                eprintln!("[bdd-debug] QMP returned error: {}", resp);
                tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            }
            Err(e) => {
                let msg = e.to_string();
                if msg.contains("No QMP connection active") || msg.contains("Broken pipe") || msg.contains("EOF") {
                    return Err(e); // Fatal connection loss
                }
                eprintln!("[bdd-debug] QMP execute failed: {}", msg);
                tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            }
        }
    }

    if !success {
        return Err("Failed to capture screenshot after retries".into());
    }

    // Wait for file to appear - increased to 40 iterations (2s) for reliability
    for _ in 0..40 {
        if ppm_path.exists() {
            // Found it! Small extra sleep to ensure flushed?
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            break;
        }
        let _ = tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }

    if !ppm_path.exists() {
        return Err(format!("Screenshot file not created at {}", ppm_path.display()).into());
    }

    // Convert PPM to PNG
    let png_path = output_path.with_extension("png");
    let img = image::open(&ppm_path)?;
    img.save(&png_path)?;
    let _ = std::fs::remove_file(&ppm_path);

    Ok(png_path)
}

/// Dump CPU registers using the global QMP socket (for reporter).
pub async fn dump_registers_global(output_path: &std::path::Path) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    // Ensure output directory exists
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let info_regs_cmd = r#"{"execute": "human-monitor-command", "arguments": {"command-line": "info registers"}}"#;

    let response_str = match qmp_execute(info_regs_cmd).await {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    // Parse JSON response to extract the actual output
    let content = if let Some(start) = response_str.find("\"return\": \"") {
        let remainder = &response_str[start + 11..];
        if let Some(end) = remainder.rfind("\"}") {
            remainder[..end].replace("\\r\\n", "\n").replace("\\n", "\n").replace("\\\"", "\"")
        } else {
             response_str
        }
    } else {
        response_str
    };

    std::fs::write(output_path, content)?;

    Ok(output_path.to_path_buf())
}
