use std::env;
use std::process::Command;
use std::path::PathBuf;

#[test]
fn test_hosted_symbol_persistence() {
    let temp_dir = std::env::temp_dir().join("thingos_smoke");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let sym_file = temp_dir.join(format!("symbols_{}.tsym", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    
    // 1. Intern "smoke_test_symbol"
    let output1 = Command::new("cargo")
        .args(&["run", "--bin", "kernel_hosted"])
        .env("THINGOS_SYMBOLS_PATH", &sym_file)
        .env("THINGOS_SMOKE_ACTION", "intern")
        .env("THINGOS_SMOKE_VAL", "smoke_test_symbol")
        .current_dir("../../")
        .output()
        .expect("Failed to run intern step");

    let stdout1 = String::from_utf8_lossy(&output1.stdout);
    if !output1.status.success() {
        println!("STDOUT 1:\n{}", stdout1);
        println!("STDERR 1:\n{}", String::from_utf8_lossy(&output1.stderr));
        panic!("Intern step failed");
    }

    let id_line = stdout1.lines().find(|l| l.starts_with("ID: ")).expect("No ID returned");
    let id_str = id_line.trim_start_matches("ID: ");
    let id: u64 = id_str.parse().expect("Failed to parse ID");

    // 2. Resolve ID
    let output2 = Command::new("cargo")
        .args(&["run", "--bin", "kernel_hosted"])
        .env("THINGOS_SYMBOLS_PATH", &sym_file)
        .env("THINGOS_SMOKE_ACTION", "resolve")
        .env("THINGOS_SMOKE_ID", id.to_string())
        .current_dir("../../")
        .output()
        .expect("Failed to run resolve step");

    let stdout2 = String::from_utf8_lossy(&output2.stdout);
    if !output2.status.success() {
        println!("STDOUT 2:\n{}", stdout2);
        println!("STDERR 2:\n{}", String::from_utf8_lossy(&output2.stderr));
        panic!("Resolve step failed");
    }

    let val_line = stdout2.lines().find(|l| l.starts_with("VAL: ")).expect("No VAL returned");
    let val = val_line.trim_start_matches("VAL: ");
    
    assert_eq!(val, "smoke_test_symbol", "Persistence failed: resolved value mismatch");

    // Cleanup
    let _ = std::fs::remove_file(sym_file);
}
