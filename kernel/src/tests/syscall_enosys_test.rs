//! ENOSYS Guard Test
//!
//! Verifies that old syscall numbers return ENOSYS.
//! This ensures no backwards compatibility is accidentally maintained.

/// Test that old syscall IDs return ENOSYS
pub fn run_selftest() {
    crate::kinfo!("ENOSYS GUARD TEST: Starting...");

    // Old syscall IDs that must now be invalid
    // These were the old values before the realignment
    const OLD_SYS_RTC_READ: usize = 9;
    const OLD_SYS_PORT_SEND: usize = 33;
    const OLD_SYS_ROOT_GET_KIND: usize = 256;

    // Verify they all return ENOSYS (negative error code)
    let ret = crate::syscall::dispatch::dispatch(OLD_SYS_RTC_READ, [0; 6]);
    if ret >= 0 {
        crate::kinfo!(
            "ENOSYS GUARD TEST: FAIL - Old SYS_RTC_READ (9) should fail but returned {}",
            ret
        );
        return;
    }

    let ret = crate::syscall::dispatch::dispatch(OLD_SYS_PORT_SEND, [0; 6]);
    if ret >= 0 {
        crate::kinfo!(
            "ENOSYS GUARD TEST: FAIL - Old SYS_PORT_SEND (33) should fail but returned {}",
            ret
        );
        return;
    }

    let ret = crate::syscall::dispatch::dispatch(OLD_SYS_ROOT_GET_KIND, [0; 6]);
    if ret >= 0 {
        crate::kinfo!(
            "ENOSYS GUARD TEST: FAIL - Old SYS_ROOT_GET_KIND (256) should fail but returned {}",
            ret
        );
        return;
    }

    crate::kinfo!("ENOSYS GUARD TEST: PASS - All old syscall numbers correctly rejected");
}
