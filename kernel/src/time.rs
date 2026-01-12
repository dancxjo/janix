use core::sync::atomic::{AtomicU64, Ordering};

pub static SYSTEM_TIME_OFFSET: AtomicU64 = AtomicU64::new(0);

pub fn get_system_time_ns(mono_ns: u64) -> u64 {
    mono_ns + SYSTEM_TIME_OFFSET.load(Ordering::Relaxed)
}

pub fn set_system_time_offset(offset_ns: u64) {
    SYSTEM_TIME_OFFSET.store(offset_ns, Ordering::Relaxed);
}

pub struct MonotonicClamp {
    last: AtomicU64,
}

impl MonotonicClamp {
    pub const fn new() -> Self {
        Self { last: AtomicU64::new(0) }
    }

    pub fn clamp(&self, raw: u64) -> u64 {
        let mut last = self.last.load(Ordering::Relaxed);
        if raw > last {
            if let Err(actual) = self.last.compare_exchange(last, raw, Ordering::Relaxed, Ordering::Relaxed) {
                if actual > raw { actual } else { raw }
            } else {
                raw
            }
        } else {
            last
        }
    }
}
