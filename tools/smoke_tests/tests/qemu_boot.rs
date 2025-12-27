#[test]
#[ignore]
fn x86_64_uefi_smoke() {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let output = std::process::Command::new(cargo)
        .arg("run")
        .arg("-p")
        .arg("xtask")
        .arg("--")
        .arg("run")
        .arg("--env")
        .arg("x86_64")
        .arg("--timeout-secs")
        .arg("30")
        .output()
        .expect("Failed to run xtask");

    let stdout = String::from_utf8_lossy(&output.stdout);

    if !stdout.contains("THINGOS: kernel entry") {
        eprintln!("STDOUT:\n{}", stdout);
        // QEMU stderr is valuable
        eprintln!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    assert!(stdout.contains("THINGOS: kernel entry"));
    assert!(stdout.contains("THINGOS: bridge online"));
    assert!(stdout.contains("·"));
}

#[test]
#[ignore]
fn aarch64_uefi_smoke() {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let output = std::process::Command::new(cargo)
        .arg("run")
        .arg("-p")
        .arg("xtask")
        .arg("--")
        .arg("run")
        .arg("--env")
        .arg("aarch64")
        .arg("--timeout-secs")
        .arg("30")
        .output()
        .expect("Failed to run xtask");

    let stdout = String::from_utf8_lossy(&output.stdout);

    if !stdout.contains("THINGOS: kernel entry") {
        eprintln!("STDOUT:\n{}", stdout);
        eprintln!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    assert!(stdout.contains("THINGOS: kernel entry"));
    assert!(stdout.contains("THINGOS: bridge online"));
    assert!(stdout.contains("·"));
}
