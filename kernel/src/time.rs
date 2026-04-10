use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub static SYSTEM_TIME_OFFSET: AtomicU64 = AtomicU64::new(0);
static IS_ANCHORED: AtomicBool = AtomicBool::new(false);

/// Returns true if the system clock has been anchored to a wall-clock time source.
pub fn is_anchored() -> bool {
    IS_ANCHORED.load(Ordering::Relaxed)
}

/// Returns system time in nanoseconds (monotonic + offset)
pub fn get_system_time_ns(mono_ns: u64) -> u64 {
    mono_ns + SYSTEM_TIME_OFFSET.load(Ordering::Relaxed)
}

/// Set the system time offset directly (in nanoseconds)
pub fn set_system_time_offset(offset_ns: u64) {
    SYSTEM_TIME_OFFSET.store(offset_ns, Ordering::Relaxed);
}

/// Anchor system clock: given a Unix timestamp and the corresponding monotonic time,
/// compute and store the offset so that future time queries return correct wall-clock time.
pub fn anchor_system_clock(unix_secs: u64, mono_ns: u64) {
    let unix_ns = unix_secs.saturating_mul(1_000_000_000);
    let offset = unix_ns.saturating_sub(mono_ns);
    set_system_time_offset(offset);
    IS_ANCHORED.store(true, Ordering::Relaxed);
    crate::kinfo!(
        "System clock anchored: unix_secs={}, mono_ns={}, offset={}ns",
        unix_secs,
        mono_ns,
        offset
    );
}

/// Return the current wall-clock time as `(seconds, nanoseconds)`.
///
/// If the runtime has not yet been initialized (e.g. during early boot or
/// in unit tests), or if the clock has not been anchored to a real-time
/// source, returns `(0, 0)` — which corresponds to the Unix epoch.
/// Callers that need precise wall-clock time should check [`is_anchored`] first.
///
/// This is the canonical time source for VFS node timestamps.
pub fn now_timespec() -> (u64, u32) {
    if !crate::is_runtime_initialized() {
        return (0, 0);
    }
    let rt = crate::runtime_base();
    let ticks = rt.mono_ticks();
    let freq = rt.mono_freq_hz();
    if freq == 0 {
        return (0, 0);
    }
    let mono_ns = (ticks as u128 * 1_000_000_000) / (freq as u128);
    let sys_ns = if is_anchored() {
        get_system_time_ns(mono_ns as u64)
    } else {
        0
    };
    let sec = sys_ns / 1_000_000_000;
    let nsec = (sys_ns % 1_000_000_000) as u32;
    (sec, nsec)
}

pub struct MonotonicClamp {
    last: AtomicU64,
}

impl MonotonicClamp {
    pub const fn new() -> Self {
        Self {
            last: AtomicU64::new(0),
        }
    }

    pub fn clamp(&self, raw: u64) -> u64 {
        let last = self.last.load(Ordering::Relaxed);
        if raw > last {
            if let Err(actual) =
                self.last
                    .compare_exchange(last, raw, Ordering::Relaxed, Ordering::Relaxed)
            {
                if actual > raw { actual } else { raw }
            } else {
                raw
            }
        } else {
            last
        }
    }
}
