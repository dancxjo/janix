use anyhow::Result;
use std::process::{Command, Stdio};

#[test]
fn hosted_boot_smoke() -> Result<()> {
    let mut child = Command::new("cargo")
        .args(["run", "-p", "xtask", "--", "run", "hosted"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Minimal: let it run briefly then kill.
    std::thread::sleep(std::time::Duration::from_millis(2000));
    let _ = child.kill();

    Ok(())
}
