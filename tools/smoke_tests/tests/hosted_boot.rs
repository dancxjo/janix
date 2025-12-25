#[test]
fn hosted_boot() {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let output = std::process::Command::new(cargo)
        .arg("run")
        .arg("-p")
        .arg("xtask")
        .arg("--")
        .arg("run")
        .arg("--env")
        .arg("hosted")
        .arg("--timeout-secs")
        .arg("5") // increased locally for safety, though 1 might suffice
        .output()
        .expect("Failed to run xtask");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Debug output if failure
    if !stdout.contains("THINGOS: kernel entry") {
        eprintln!("STDOUT:\n{}", stdout);
        eprintln!("STDERR:\n{}", stderr);
    }

    assert!(
        stdout.contains("THINGOS: kernel entry"),
        "Missing kernel entry log"
    );
    assert!(
        stdout.contains("THINGOS: bridge online"),
        "Missing bridge online log"
    );
    assert!(
        stdout.contains("THINGOS: idle loop"),
        "Missing idle loop log"
    );

    // Verify order
    let p1 = stdout.find("THINGOS: kernel entry").expect("p1 missing");
    let p2 = stdout.find("THINGOS: bridge online").expect("p2 missing");
    let p3 = stdout.find("THINGOS: idle loop").expect("p3 missing");

    assert!(p1 < p2, "kernel entry should be before bridge online");
    assert!(p2 < p3, "bridge online should be before idle loop");
}
