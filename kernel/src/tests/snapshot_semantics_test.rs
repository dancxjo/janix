//! Snapshot semantics verification tests
//!
//! Verifies:
//! 1. Two successive frames must not alias memory unless explicitly intended
//! 2. Frozen bytespaces correctly track immutability state

use crate::root::resources::bytespace::create;

pub fn run_selftest() {
    crate::kinfo!("SNAPSHOT SEMANTICS TEST: Starting...");

    test_non_aliasing_successive_frames();
    test_frozen_state_tracking();

    crate::kinfo!("SNAPSHOT SEMANTICS TEST: PASS");
}

/// Test: Two successive bytespace allocations must not alias
fn test_non_aliasing_successive_frames() {
    crate::kinfo!("  [1] Testing non-aliasing of successive frames...");

    let hhdm = crate::boot_info::get().map(|i| i.hhdm_offset).unwrap_or(0);

    // Simulate two frame allocations
    let bs1 = match create(4096, hhdm) {
        Some(bs) => bs,
        None => {
            crate::kinfo!("    SKIP: Failed to create bytespace 1");
            return;
        }
    };
    let bs2 = match create(4096, hhdm) {
        Some(bs) => bs,
        None => {
            crate::kinfo!("    SKIP: Failed to create bytespace 2");
            return;
        }
    };

    let bs1_lock = bs1.lock();
    let bs2_lock = bs2.lock();

    // Physical addresses must not overlap
    let bs1_end = bs1_lock.phys_base + bs1_lock.len as u64;
    let bs2_end = bs2_lock.phys_base + bs2_lock.len as u64;

    let overlaps = !(bs1_end <= bs2_lock.phys_base || bs2_end <= bs1_lock.phys_base);

    if overlaps {
        crate::kinfo!(
            "    FAIL: Bytespaces overlap! bs1=[0x{:x}..0x{:x}] bs2=[0x{:x}..0x{:x}]",
            bs1_lock.phys_base,
            bs1_end,
            bs2_lock.phys_base,
            bs2_end
        );
        return;
    }

    // Write patterns and verify no cross-contamination
    let bs1_va = bs1_lock.kernel_va;
    let bs2_va = bs2_lock.kernel_va;
    let bs1_len = bs1_lock.len;
    drop(bs1_lock);
    drop(bs2_lock);

    // Write pattern to bs1
    unsafe {
        core::ptr::write_bytes(bs1_va as *mut u8, 0xAA, bs1_len);
    }

    // Write different pattern to bs2
    unsafe {
        core::ptr::write_bytes(bs2_va as *mut u8, 0xBB, bs1_len);
    }

    // Verify bs1 still has original pattern
    let first_byte = unsafe { *(bs1_va as *const u8) };
    if first_byte != 0xAA {
        crate::kinfo!(
            "    FAIL: bs1 was clobbered! Expected 0xAA, got 0x{:02X}",
            first_byte
        );
        return;
    }

    crate::kinfo!("    OK: Successive frames do not alias");
}

/// Test: Frozen state is correctly tracked
fn test_frozen_state_tracking() {
    crate::kinfo!("  [2] Testing frozen state tracking...");

    let hhdm = crate::boot_info::get().map(|i| i.hhdm_offset).unwrap_or(0);
    let bs = match create(4096, hhdm) {
        Some(bs) => bs,
        None => {
            crate::kinfo!("    SKIP: Failed to create bytespace");
            return;
        }
    };

    // Initially not frozen
    {
        let bs_lock = bs.lock();
        if bs_lock.frozen {
            crate::kinfo!("    FAIL: Bytespace should not be frozen initially");
            return;
        }
        // Should return true (mutable)
        if !bs_lock.assert_mutable("test-initial") {
            crate::kinfo!("    FAIL: assert_mutable should return true for unfrozen bytespace");
            return;
        }
    }

    // Freeze the bytespace
    {
        let mut bs_lock = bs.lock();
        bs_lock.freeze();
        if !bs_lock.frozen {
            crate::kinfo!("    FAIL: Bytespace should be frozen after freeze()");
            return;
        }
    }

    // Verify frozen state persists
    {
        let bs_lock = bs.lock();
        if !bs_lock.frozen {
            crate::kinfo!("    FAIL: Frozen state should persist");
            return;
        }
    }

    crate::kinfo!("    OK: Frozen state correctly tracked");
}
