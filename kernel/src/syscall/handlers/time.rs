//! Time and scheduling syscalls

use abi::errors::SysResult;

pub fn sys_yield() -> SysResult<usize> {
    unsafe {
        crate::task::scheduler::yield_now_current();
    }
    Ok(0)
}

pub fn sys_sleep_ns(ns: u64) -> SysResult<usize> {
    let rt = crate::runtime_base();
    let freq = rt.mono_freq_hz();
    let ticks = (ns as u128 * freq as u128) / 1_000_000_000;
    let start = rt.mono_ticks();
    let deadline = start + ticks as u64;
    loop {
        let now = rt.mono_ticks();
        if now >= deadline {
            break;
        }
        unsafe {
            crate::task::scheduler::yield_now_current();
        }
    }
    Ok(0)
}

pub fn sys_sleep_ms(ms: u64) -> SysResult<usize> {
    let rt = crate::runtime_base();
    let freq = rt.mono_freq_hz();
    let ticks = (ms * freq) / 1000;
    let start = rt.mono_ticks();
    let deadline = start + ticks;
    loop {
        let now = rt.mono_ticks();
        if now >= deadline {
            break;
        }
        unsafe {
            crate::task::scheduler::yield_now_current();
        }
    }
    Ok(0)
}

pub fn sys_time_monotonic_ns() -> SysResult<usize> {
    let rt = crate::runtime_base();
    let ticks = rt.mono_ticks();
    let freq = rt.mono_freq_hz();
    let ns = (ticks as u128 * 1_000_000_000) / (freq as u128);
    Ok(ns as usize)
}

pub fn sys_time_now() -> SysResult<usize> {
    let rt = crate::runtime_base();
    let ticks = rt.mono_ticks();
    let freq = rt.mono_freq_hz();
    let mono_ns = (ticks as u128 * 1_000_000_000) / (freq as u128);
    let sys_ns = crate::time::get_system_time_ns(mono_ns as u64);
    let sys_sec = sys_ns / 1_000_000_000;
    Ok(sys_sec as usize)
}

pub fn sys_time_anchor(unix_secs: u64) -> SysResult<usize> {
    let rt = crate::runtime_base();
    let ticks = rt.mono_ticks();
    let freq = rt.mono_freq_hz();
    let mono_ns = (ticks as u128 * 1_000_000_000) / (freq as u128);
    crate::time::anchor_system_clock(unix_secs, mono_ns as u64);
    Ok(0)
}
