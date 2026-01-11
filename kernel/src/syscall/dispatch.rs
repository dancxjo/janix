use abi::syscall::*;
use abi::errors::Errno;
use super::handlers;
use core::sync::atomic::{AtomicBool, Ordering};

// Optional syscall trace printing to avoid spamming the console during normal runs.
static TRACE_SYSCALLS: AtomicBool = AtomicBool::new(false);

/// Enable or disable syscall tracing logs.
pub fn set_syscall_tracing(enabled: bool) {
    TRACE_SYSCALLS.store(enabled, Ordering::Relaxed);
}

/// Dispatches a system call to the appropriate handler.
///
/// # Arguments
/// * `n` - Syscall number.
/// * `args` - Array of 6 arguments.
///
/// # Returns
/// * `isize` - Return value (success >= 0, error < 0).
pub fn dispatch(n: usize, args: [usize; 6]) -> isize {
    let syscall_id = n as u32;
    if TRACE_SYSCALLS.load(Ordering::Relaxed) {
        crate::kdebug!("Syscall: {} args={:x?}", syscall_id, args);
    }

    let result = match syscall_id {
        SYS_EXIT => handlers::sys_exit(args[0] as i32),
        SYS_DEBUG_WRITE => handlers::sys_debug_write(args[0], args[1]),
        SYS_LOG_WRITE => handlers::sys_debug_write(args[0], args[1]), // Alias to debug write for now
        SYS_SLEEP_MS => handlers::sys_sleep_ms(args[0] as u64),
        SYS_SLEEP_NS => handlers::sys_sleep_ns(args[0] as u64),
        SYS_DEVICE_CALL => handlers::sys_device_call(args[0]),
        SYS_YIELD => handlers::sys_yield(),
        SYS_SPAWN_THREAD => handlers::sys_spawn_thread(args[0], args[1]),
        SYS_SPAWN_PROCESS => handlers::sys_spawn_process(args[0], args[1]),
        SYS_TIME_MONOTONIC => handlers::sys_time_monotonic_ns(),
        SYS_RTC_READ => handlers::sys_rtc_read(args[0]),
        SYS_GET_TID => handlers::sys_get_tid(),
        _ => Err(Errno::ENOSYS),
    };

    match result {
        Ok(val) => val as isize,
        Err(e) => e.as_isize(),
    }
}
