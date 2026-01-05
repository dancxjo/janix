use super::*;

pub fn monotonic_now() -> u64 {
    unsafe { syscall(nr::SYS_TIME_MONOTONIC_NOW, 0, 0, 0, 0, 0, 0).status }
}

pub fn system_now() -> u64 {
    unsafe { syscall(nr::SYS_TIME_SYSTEM_NOW, 0, 0, 0, 0, 0, 0).status }
}

pub fn sleep_ms(ms: u64) {
    let now = monotonic_now();
    unsafe { syscall(nr::SYS_SLEEP_UNTIL, now + ms, 0, 0, 0, 0, 0) };
}
