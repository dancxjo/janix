use crate::syscall;
use core::time::Duration;

pub fn now_unix_seconds() -> u64 {
    unsafe {
        match syscall::syscall6(abi::syscall::SYS_TIME_NOW, 0, 0, 0, 0, 0, 0) {
            x if x >= 0 => x as u64,
            _ => 0, // Error fallback
        }
    }
}

pub fn sleep(duration: Duration) {
    syscall::sleep_ns(duration.as_nanos() as u64);
}

pub fn sleep_ms(ms: u64) {
    syscall::sleep_ms(ms);
}

pub fn monotonic_ns() -> u64 {
    syscall::monotonic_ns()
}
