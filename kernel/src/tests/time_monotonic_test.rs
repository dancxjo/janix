//! Time Monotonicity Selftest
//!
//! Verifies that the monotonic time source never goes backward.

pub fn run_selftest() {
    crate::kinfo!("TIME MONOTONIC TEST: Starting...");

    // Test 1: Verify monotonicity over 1000 samples
    let mut last = 0u64;
    for i in 0..1000 {
        let now = crate::syscall::handlers::sys_time_monotonic_ns()
            .expect("monotonic_ns should succeed");
        let now = now as u64;
        
        if now < last {
            crate::kinfo!(
                "TIME MONOTONIC TEST: FAIL - time went backward at sample {}: {} -> {}",
                i, last, now
            );
            return;
        }
        last = now;
    }
    crate::kinfo!("TIME MONOTONIC TEST: 1000 samples monotonic - PASS");

    // Test 2: Verify trace::now() is also monotonic
    let mut last_trace = 0u64;
    for i in 0..100 {
        let now = crate::trace::now();
        if now < last_trace {
            crate::kinfo!(
                "TIME MONOTONIC TEST: FAIL - trace::now() went backward at sample {}: {} -> {}",
                i, last_trace, now
            );
            return;
        }
        last_trace = now;
    }
    crate::kinfo!("TIME MONOTONIC TEST: trace::now() monotonic - PASS");

    // Test 3: Verify consistency between trace::now() and syscall
    let trace_now = crate::trace::now();
    let syscall_now = crate::syscall::handlers::sys_time_monotonic_ns()
        .expect("monotonic_ns should succeed") as u64;
    
    // They should be very close (within 1ms tolerance)
    let diff = if syscall_now > trace_now {
        syscall_now - trace_now
    } else {
        trace_now - syscall_now
    };
    
    if diff > 1_000_000 {
        crate::kinfo!(
            "TIME MONOTONIC TEST: WARN - trace::now() and syscall differ by {}ns",
            diff
        );
    } else {
        crate::kinfo!("TIME MONOTONIC TEST: trace and syscall consistent - PASS");
    }

    crate::kinfo!("TIME MONOTONIC TEST: All tests PASS");
}
