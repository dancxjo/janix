use crate::syscall;

pub fn now_unix_seconds() -> u64 {
    unsafe {
        match syscall::syscall6(abi::syscall::SYS_TIME_NOW, 0, 0, 0, 0, 0, 0) {
            x if x >= 0 => x as u64,
            _ => 0, // Error fallback
        }
    }
}

pub fn sleep(duration: core::time::Duration) {
    syscall::sleep_ns(duration.as_nanos() as u64);
}
