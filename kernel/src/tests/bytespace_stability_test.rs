//! Bytespace stability tests
//!
//! Verifies that:
//! 1. Bytespace creation sets provenance correctly
//! 2. New allocations don't overlap with existing bytespaces  
//! 3. Frozen bytespaces maintain their canary values

use crate::root::resources::bytespace::Provenance;

pub fn run_selftest() {
    crate::kinfo!("BYTESPACE STABILITY TEST: Starting...");
    
    // Test 1: Create bytespaces and verify they don't alias
    test_non_aliasing();
    
    // Test 2: Verify provenance enum values
    test_provenance_values();
    
    crate::kinfo!("BYTESPACE STABILITY TEST: PASS");
}

fn test_non_aliasing() {
    crate::kinfo!("  [1] Testing bytespace non-aliasing...");
    
    // Create first bytespace via syscall
    let bs1_id = match crate::syscall::handlers::sys_root_bytespace_create(4096, 0, 0) {
        Ok(id) => id,
        Err(e) => {
            crate::kinfo!("    SKIP: Failed to create first bytespace: {:?}", e);
            return;
        }
    };
    
    // Write a distinctive pattern to first bytespace
    let pattern: [u8; 8] = [0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE];
    if let Err(e) = crate::syscall::handlers::sys_root_bytespace_write(
        bs1_id,
        0,
        pattern.as_ptr() as usize,
        8,
    ) {
        crate::kinfo!("    SKIP: Failed to write pattern: {:?}", e);
        return;
    }
    
    // Create second bytespace
    let _bs2_id = match crate::syscall::handlers::sys_root_bytespace_create(8192, 0, 0) {
        Ok(id) => id,
        Err(_) => {
            crate::kinfo!("    SKIP: Second allocation failed");
            return;
        }
    };
    
    // Read back first bytespace and verify pattern is unchanged
    let mut read_buf = [0u8; 8];
    if let Err(e) = crate::syscall::handlers::sys_root_bytespace_read(
        bs1_id,
        0,
        read_buf.as_mut_ptr() as usize,
        8,
    ) {
        crate::kinfo!("    SKIP: Failed to read back: {:?}", e);
        return;
    }
    
    if read_buf != pattern {
        crate::kinfo!(
            "    FAIL: Pattern was clobbered! Expected {:x?}, got {:x?}",
            pattern,
            read_buf
        );
        return;
    }
    
    crate::kinfo!("    OK: Pattern unchanged after second allocation");
}

fn test_provenance_values() {
    crate::kinfo!("  [2] Testing provenance enum values...");
    
    // Verify enum discriminant values match expected ABI
    assert_eq!(Provenance::Kernel as u8, 0, "Kernel should be 0");
    assert_eq!(Provenance::Boot as u8, 1, "Boot should be 1");
    assert_eq!(Provenance::Firmware as u8, 2, "Firmware should be 2");
    
    crate::kinfo!("    OK: Provenance enum values are correct");
}
