use core::sync::atomic::{AtomicU64, Ordering};

pub static SYSTEM_TIME_OFFSET: AtomicU64 = AtomicU64::new(0);

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
    crate::kinfo!(
        "System clock anchored: unix_secs={}, mono_ns={}, offset={}ns",
        unix_secs,
        mono_ns,
        offset
    );
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
