//! Time and scheduling syscalls

use abi::errors::SysResult;

pub fn sys_yield() -> SysResult<usize> {
    unsafe {
        crate::sched::yield_now_current();
    }
    Ok(0)
}

pub fn sys_sleep_ns(ns: u64) -> SysResult<usize> {
    // Timer runs at 100Hz = 1 tick per 10ms = 10,000,000ns per tick
    // Convert ns to ticks, rounding up to avoid sleeping less than requested
    let ticks = (ns + 9_999_999) / 10_000_000;
    if ticks == 0 {
        // Very short sleep, just yield once
        unsafe {
            crate::sched::yield_now_current();
        }
    } else {
        // Use true blocking sleep
        crate::sched::sleep_ticks_current(ticks);
    }
    Ok(0)
}

pub fn sys_sleep_ms(ms: u64) -> SysResult<usize> {
    // Timer runs at 100Hz = 1 tick per 10ms
    // Convert ms to ticks, rounding up
    let ticks = (ms + 9) / 10;
    if ticks == 0 {
        // Very short sleep, just yield once
        unsafe {
            crate::sched::yield_now_current();
        }
    } else {
        // Use true blocking sleep
        crate::sched::sleep_ticks_current(ticks);
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
    // Return system time (wall-clock time).
    // If not anchored yet, return EAGAIN - callers must wait for RTC anchoring.
    // For early timing, use sys_time_monotonic_ns instead.
    if !crate::time::is_anchored() {
        return Err(abi::errors::Errno::EAGAIN);
    }
    let rt = crate::runtime_base();
    let ticks = rt.mono_ticks();
    let freq = rt.mono_freq_hz();
    let mono_ns = (ticks as u128 * 1_000_000_000) / (freq as u128);
    let sys_ns = crate::time::get_system_time_ns(mono_ns as u64);
    Ok(sys_ns as usize)
}

pub fn sys_time_anchor(unix_secs: u64) -> SysResult<usize> {
    let rt = crate::runtime_base();
    let ticks = rt.mono_ticks();
    let freq = rt.mono_freq_hz();
    let mono_ns = (ticks as u128 * 1_000_000_000) / (freq as u128);
    crate::time::anchor_system_clock(unix_secs, mono_ns as u64);
    Ok(0)
}
