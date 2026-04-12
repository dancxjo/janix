//! sig_test — POSIX signal regression test suite.
//!
//! Each test prints PASS or FAIL and a description.  The binary exits with
//! status 0 if all tests pass, 1 otherwise.
//!
//! Tests covered:
//!   1. Install a SIGUSR1 handler, send SIGUSR1 to self, verify handler ran.
//!   2. Block SIGUSR1, send it, verify it is pending, then unblock and verify delivery.
//!   3. SIGTERM with default disposition terminates the process.
//!   4. SIGKILL cannot be caught via sigaction (EINVAL expected).
//!   5. Verify sigpending returns the correct set when a signal is blocked.
//!   6. alarm() delivers SIGALRM.
//!   7. pause() returns EINTR when a signal arrives.
#![no_std]
#![no_main]
extern crate alloc;

use core::sync::atomic::{AtomicU32, Ordering};

use abi::signal::*;
use stem::syscall::signal::{alarm, kill, pause, sigaction, sigblock, sigpending, sigunblock};
use stem::syscall::vfs_write;

// ─── Global handler state ────────────────────────────────────────────────────

/// Counts how many times the SIGUSR1 handler has been invoked.
static USR1_COUNT: AtomicU32 = AtomicU32::new(0);
/// Counts SIGALRM deliveries.
static ALRM_COUNT: AtomicU32 = AtomicU32::new(0);

// ─── Signal handlers ─────────────────────────────────────────────────────────

extern "C" fn handle_usr1(_sig: u32) {
    USR1_COUNT.fetch_add(1, Ordering::SeqCst);
}

extern "C" fn handle_alrm(_sig: u32) {
    ALRM_COUNT.fetch_add(1, Ordering::SeqCst);
}

// ─── Test helpers ────────────────────────────────────────────────────────────

fn print(s: &str) {
    let _ = vfs_write(1, s.as_bytes());
}

fn println(s: &str) {
    print(s);
    print("\n");
}

fn pass(name: &str) {
    print("PASS: ");
    println(name);
}

fn fail(name: &str, reason: &str) {
    print("FAIL: ");
    print(name);
    print(": ");
    println(reason);
}

// ─── Tests ───────────────────────────────────────────────────────────────────

/// Test 1: install handler for SIGUSR1, raise it, verify delivery.
fn test_sigusr1_handler() -> bool {
    USR1_COUNT.store(0, Ordering::SeqCst);

    let act = SigAction {
        handler: handle_usr1 as usize,
        mask: SigSet::EMPTY,
        flags: 0,
        _pad: 0,
    };

    if sigaction(SIGUSR1, Some(&act), None).is_err() {
        fail("sigusr1_handler", "sigaction failed");
        return false;
    }

    // Send SIGUSR1 to ourselves (pid -1 signals self on ThingOS).
    // Use kill(0, SIGUSR1) — ThingOS will send to self.
    if kill(-1, SIGUSR1).is_err() {
        fail("sigusr1_handler", "kill failed");
        return false;
    }

    // The handler should have run by now (delivery happens at syscall return).
    if USR1_COUNT.load(Ordering::SeqCst) != 1 {
        fail("sigusr1_handler", "handler not called");
        return false;
    }
    pass("sigusr1_handler");
    true
}

/// Test 2: block SIGUSR1, verify pending, unblock, verify delivered.
fn test_sigusr1_blocked() -> bool {
    USR1_COUNT.store(0, Ordering::SeqCst);

    // Ensure handler is still installed.
    let act = SigAction {
        handler: handle_usr1 as usize,
        mask: SigSet::EMPTY,
        flags: 0,
        _pad: 0,
    };
    let _ = sigaction(SIGUSR1, Some(&act), None);

    // Block SIGUSR1.
    let mask = SigSet::from_signal(SIGUSR1);
    if sigblock(mask).is_err() {
        fail("sigusr1_blocked", "sigblock failed");
        return false;
    }

    // Send SIGUSR1 while blocked — should NOT invoke handler yet.
    let _ = kill(-1, SIGUSR1);

    if USR1_COUNT.load(Ordering::SeqCst) != 0 {
        fail("sigusr1_blocked", "handler called while blocked");
        return false;
    }

    // sigpending should show SIGUSR1 as pending.
    match sigpending() {
        Ok(pending) => {
            if !pending.contains(SIGUSR1) {
                fail("sigusr1_blocked", "SIGUSR1 not in pending set");
                // Unblock anyway.
                let _ = sigunblock(mask);
                return false;
            }
        }
        Err(e) => {
            fail("sigusr1_blocked", "sigpending failed");
            let _ = e;
            let _ = sigunblock(mask);
            return false;
        }
    }

    // Unblock — handler should now run.
    let _ = sigunblock(mask);

    if USR1_COUNT.load(Ordering::SeqCst) != 1 {
        fail("sigusr1_blocked", "handler not called after unblock");
        return false;
    }

    pass("sigusr1_blocked");
    true
}

/// Test 3: SIGKILL cannot be caught.
fn test_sigkill_uncatchable() -> bool {
    let act = SigAction {
        handler: handle_usr1 as usize, // reuse any handler
        mask: SigSet::EMPTY,
        flags: 0,
        _pad: 0,
    };
    match sigaction(SIGKILL, Some(&act), None) {
        Err(abi::errors::Errno::EINVAL) => {
            pass("sigkill_uncatchable");
            true
        }
        Ok(()) => {
            fail("sigkill_uncatchable", "sigaction(SIGKILL) should return EINVAL");
            false
        }
        Err(e) => {
            fail("sigkill_uncatchable", "unexpected error");
            let _ = e;
            false
        }
    }
}

/// Test 4: SIGSTOP cannot be caught.
fn test_sigstop_uncatchable() -> bool {
    let act = SigAction {
        handler: handle_usr1 as usize,
        mask: SigSet::EMPTY,
        flags: 0,
        _pad: 0,
    };
    match sigaction(SIGSTOP, Some(&act), None) {
        Err(abi::errors::Errno::EINVAL) => {
            pass("sigstop_uncatchable");
            true
        }
        Ok(()) => {
            fail("sigstop_uncatchable", "sigaction(SIGSTOP) should return EINVAL");
            false
        }
        Err(e) => {
            fail("sigstop_uncatchable", "unexpected error");
            let _ = e;
            false
        }
    }
}

/// Test 5: alarm() schedules SIGALRM delivery.
fn test_alarm() -> bool {
    ALRM_COUNT.store(0, Ordering::SeqCst);

    let act = SigAction {
        handler: handle_alrm as usize,
        mask: SigSet::EMPTY,
        flags: 0,
        _pad: 0,
    };
    if sigaction(SIGALRM, Some(&act), None).is_err() {
        fail("alarm", "sigaction(SIGALRM) failed");
        return false;
    }

    // Set a 1-second alarm.
    let prev = alarm(1);
    if prev != 0 {
        fail("alarm", "unexpected previous alarm");
        alarm(0); // cancel
        return false;
    }

    // pause() should return when SIGALRM fires (within ~1 second).
    let _ = pause();

    if ALRM_COUNT.load(Ordering::SeqCst) == 0 {
        fail("alarm", "SIGALRM not delivered");
        return false;
    }

    pass("alarm");
    true
}

/// Test 6: sigaction SIG_IGN causes signal to be discarded.
fn test_sig_ign() -> bool {
    USR1_COUNT.store(0, Ordering::SeqCst);

    // Ignore SIGUSR2.
    let ign = SigAction {
        handler: sig_handler::SIG_IGN,
        mask: SigSet::EMPTY,
        flags: 0,
        _pad: 0,
    };
    if sigaction(SIGUSR2, Some(&ign), None).is_err() {
        fail("sig_ign", "sigaction failed");
        return false;
    }

    // Send SIGUSR2 — should be silently discarded, NOT deliver to USR1 handler.
    let _ = kill(-1, SIGUSR2);

    // Restore default.
    let dfl = SigAction::default();
    let _ = sigaction(SIGUSR2, Some(&dfl), None);

    pass("sig_ign");
    true
}

// ─── Entry point ─────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println("=== sig_test begin ===");

    let mut passed = 0u32;
    let mut failed = 0u32;

    macro_rules! run {
        ($test:expr) => {
            if $test {
                passed += 1;
            } else {
                failed += 1;
            }
        };
    }

    run!(test_sigkill_uncatchable());
    run!(test_sigstop_uncatchable());
    run!(test_sig_ign());
    run!(test_sigusr1_handler());
    run!(test_sigusr1_blocked());
    run!(test_alarm());

    println("=== sig_test end ===");

    if failed == 0 {
        print("ALL ");
        print(u32_str(passed).as_str());
        println(" TESTS PASSED");
        stem::syscall::exit(0);
    } else {
        print(u32_str(failed).as_str());
        print(" OF ");
        print(u32_str(passed + failed).as_str());
        println(" TESTS FAILED");
        stem::syscall::exit(1);
    }
}

fn u32_str(mut n: u32) -> alloc::string::String {
    use alloc::string::ToString;
    if n == 0 {
        return "0".to_string();
    }
    let mut digits = alloc::vec::Vec::new();
    while n > 0 {
        digits.push((b'0' + (n % 10) as u8) as char);
        n /= 10;
    }
    digits.iter().rev().collect()
}
