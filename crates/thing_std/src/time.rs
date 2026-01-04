use super::*;

pub fn monotonic_now() -> u64 {
    let res = unsafe {
        syscall(
            nr::SYS_TIME_MONOTONIC_NOW,
            0,
            0,
            0,
            0,
            0,
            0,
        )
    };
    res.val0
}

pub fn system_now() -> i64 {
    let res = unsafe {
        syscall(
            nr::SYS_TIME_SYSTEM_NOW,
            0,
            0,
            0,
            0,
            0,
            0,
        )
    };
    res.val0 as i64
}

pub fn set_system_time(nanos: i64) {
    unsafe {
        syscall(
            nr::SYS_TIME_SET_SYSTEM,
            nanos as u64,
            0,
            0,
            0,
            0,
            0,
        )
    };
}

pub fn sleep_until(nanos: u64) {
    unsafe {
        syscall(
            nr::SYS_SLEEP_UNTIL,
            nanos,
            0,
            0,
            0,
            0,
            0,
        )
    };
}

pub fn sleep_ms(ms: u64) {
    let now = monotonic_now();
    sleep_until(now + ms * 1_000_000);
}
