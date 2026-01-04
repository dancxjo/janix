use crate::time;
use abi::syscall::err;
use abi::wire::SyscallResult;

pub fn sys_time_monotonic_now() -> SyscallResult {
    let now = time::monotonic_now();
    SyscallResult::new(0, now, 0)
}

pub fn sys_time_system_now() -> SyscallResult {
    match time::system_now() {
        Ok(now) => {
            // Return i64 as (low, high) or just low if it fits?
            // Spec says: If you don’t want 128-bit, return i64 via val0 and ignore val1.
            SyscallResult::new(0, now as u64, 0)
        }
        Err(_) => SyscallResult::new(err::EAGAIN, 0, 0), // Unset
    }
}

pub fn sys_time_set_system(unix_epoch_ns: i64) -> SyscallResult {
    time::set_system_time(unix_epoch_ns);
    SyscallResult::new(0, 0, 0)
}

pub fn sys_sleep_until(deadline_mono_ns: u64) -> SyscallResult {
    // For now, just yield until deadline is reached.
    // A better implementation would block the task and wake it up.
    while time::monotonic_now() < deadline_mono_ns {
        crate::sched::yield_current();
    }
    SyscallResult::new(0, 0, 0)
}
