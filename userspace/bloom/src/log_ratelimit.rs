//! Rate-limited logging helper to prevent log flooding

use core::sync::atomic::{AtomicU64, Ordering};

static LAST_LOG_MS: AtomicU64 = AtomicU64::new(0);

/// Check if enough time has elapsed since the last log.
/// Returns true if at least `interval_ms` milliseconds have passed since the last logged event.
pub fn log_every(interval_ms: u64, now_ms: u64) -> bool {
    let last = LAST_LOG_MS.load(Ordering::Relaxed);
    if now_ms.wrapping_sub(last) >= interval_ms {
        LAST_LOG_MS.store(now_ms, Ordering::Relaxed);
        true
    } else {
        false
    }
}

/// Helper to get current time in milliseconds
#[inline]
pub fn now_ms() -> u64 {
    stem::monotonic_ns() / 1_000_000
}
