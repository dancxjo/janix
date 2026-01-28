//! Kernel Time Anchoring Unit Test
//!
//! Verifies that system time is gated until anchored.

use crate::syscall::handlers::sys_time_now;
use abi::errors::Errno;

pub fn run_selftest() {
    crate::kinfo!("TIME ANCHORING TEST: Starting...");

    // 1. Before anchor: sys_time_now should return EAGAIN
    let ret = sys_time_now();
    match ret {
        Err(Errno::EAGAIN) => crate::kinfo!(
            "TIME ANCHORING TEST: Success - sys_time_now returned EAGAIN before anchor"
        ),
        Ok(t) => {
            crate::kinfo!(
                "TIME ANCHORING TEST: FAIL - sys_time_now returned {} but should have failed before anchor",
                t
            );
            return;
        }
        Err(e) => {
            crate::kinfo!(
                "TIME ANCHORING TEST: FAIL - sys_time_now returned unexpected error {:?}",
                e
            );
            return;
        }
    }

    // 2. Anchor the clock
    crate::kinfo!("TIME ANCHORING TEST: Anchoring clock...");
    crate::time::anchor_system_clock(1706126400, 1000); // Sample Unix time: 2024-01-24

    // 3. After anchor: sys_time_now should return a valid timestamp
    let ret = sys_time_now();
    match ret {
        Ok(t) => {
            crate::kinfo!(
                "TIME ANCHORING TEST: Success - sys_time_now returned valid time after anchor: {}",
                t
            );
            if t < 1700000000 {
                crate::kinfo!(
                    "TIME ANCHORING TEST: FAIL - returned time {} seems too small for 2024",
                    t
                );
                return;
            }
        }
        Err(e) => {
            crate::kinfo!(
                "TIME ANCHORING TEST: FAIL - sys_time_now failed after anchor with error {:?}",
                e
            );
            return;
        }
    }

    crate::kinfo!("TIME ANCHORING TEST: PASS - All invariants verified");
}
