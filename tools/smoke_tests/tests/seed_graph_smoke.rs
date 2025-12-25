#[test]
fn test_seed_graph() {
    // Run the hosted kernel with a 1-second timeout.
    // We expect "THINGOS: graph seeded" to appear in the output.
    let output = std::process::Command::new("cargo")
        .args(&["run", "-p", "xtask", "--", "run", "--env", "hosted", "--timeout-secs", "1"])
        .current_dir("../../") // Assuming running from tools/smoke_tests/
        .output()
        .expect("Failed to run xtask");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("STDOUT:\n{}", stdout);
    println!("STDERR:\n{}", stderr);

    assert!(stdout.contains("THINGOS: graph init"), "Graph init not found in logs");
    assert!(stdout.contains("THINGOS: graph seeded"), "Graph seeded not found in logs");
}
