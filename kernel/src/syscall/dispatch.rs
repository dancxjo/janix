use abi::syscall::*;
use abi::errors::Errno;
use super::handlers;

/// Dispatches a system call to the appropriate handler.
///
/// # Arguments
/// * `n` - Syscall number.
/// * `args` - Array of 6 arguments.
///
/// # Returns
/// * `isize` - Return value (success >= 0, error < 0).
pub fn dispatch(n: usize, args: [usize; 6]) -> isize {
     // Log trace (optional, maybe behind feature flag or debug level)
    let syscall_id = n as u32;
    // crate::kinfo!("Syscall: {}", syscall_id); // noisy

    let result = match syscall_id {
        SYS_EXIT => handlers::sys_exit(args[0] as i32),
        SYS_DEBUG_WRITE => handlers::sys_debug_write(args[0], args[1]),
        SYS_SLEEP_MS => handlers::sys_sleep_ms(args[0] as u64),
        SYS_DEVICE_CALL => handlers::sys_device_call(args[0]),
        SYS_YIELD => handlers::sys_yield(),
        SYS_SPAWN_THREAD => handlers::sys_spawn_thread(args[0], args[1]),
        SYS_SPAWN_PROCESS => handlers::sys_spawn_process(args[0], args[1]),
        _ => Err(Errno::ENOSYS),
    };

    match result {
        Ok(val) => val as isize,
        Err(e) => e.as_isize(),
    }
}
