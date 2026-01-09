//! Monotonic time utilities and conversion helpers.

use core::sync::atomic::{AtomicU64, Ordering};

/// A helper to enforce strict monotonicity on a raw hardware counter.
pub struct MonotonicClamp {
    last_value: AtomicU64,
}

impl MonotonicClamp {
    pub const fn new() -> Self {
        Self {
            last_value: AtomicU64::new(0),
        }
    }

    /// Clamps the raw value to be at least as large as the last value seen.
    pub fn clamp(&self, raw_value: u64) -> u64 {
        // Relaxed loading is fine because we're just checking against a potentially
        // stale value, and the CAS loop below handles the actual update synchronization.
        let mut last = self.last_value.load(Ordering::Relaxed);
        
        // Fast path: if we're clearly moving forward, just try to update.
        if raw_value > last {
            match self.last_value.compare_exchange(
                last,
                raw_value,
                Ordering::Relaxed, // Success: new value visible eventually is fine
                Ordering::Relaxed  // Failure: we get a fresher value to retry/check
            ) {
                Ok(_) => return raw_value,
                Err(fresh_last) => last = fresh_last,
            }
        }
        
        // If we get here, either raw_value <= last initially, or we raced and someone
        // else updated the value. We need to ensure we return the max.
        
        // Re-check monotonicity against the potentially fresher last value
        if raw_value <= last {
            return last;
        }

        // We raced, but our value is still arguably "newer" than what we saw,
        // but wait, if CAS failed, `last` is now the value that was in the atomic.
        // If `raw_value` is still greater than that new `last`, we should try to update again?
        // Or simpler: just use fetch_max, but AtomicU64::fetch_max is not available in all rust versions/targets easily?
        // Actually, fetch_max IS standard since 1.38. Let's use that if possible.
        // But for no_std freestanding definition it's safer to just do the loop.
        
        // Let's rely on a simple loop for correctness.
        // We already loaded `last`.
        loop {
            if raw_value <= last {
                return last;
            }
            match self.last_value.compare_exchange_weak(
                last,
                raw_value,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return raw_value,
                Err(new_last) => last = new_last,
            }
        }
    }
}

/// Convert ticks to nanoseconds, avoiding overflow.
pub fn ticks_to_ns(ticks: u64, freq_hz: u64) -> u64 {
    if freq_hz == 0 {
        return 0;
    }
    // (ticks * 1_000_000_000) / freq_hz
    let ticks = ticks as u128;
    let freq = freq_hz as u128;
    let ns = (ticks.saturating_mul(1_000_000_000)) / freq;
    
    if ns > u64::MAX as u128 {
        u64::MAX
    } else {
        ns as u64
    }
}

/// Convert nanoseconds to ticks, avoiding overflow.
pub fn ns_to_ticks(ns: u64, freq_hz: u64) -> u64 {
    if freq_hz == 0 {
        return 0;
    }
    // (ns * freq_hz) / 1_000_000_000
    let ns = ns as u128;
    let freq = freq_hz as u128;
    let ticks = (ns.saturating_mul(freq)) / 1_000_000_000;

    if ticks > u64::MAX as u128 {
        u64::MAX
    } else {
        ticks as u64
    }
}

/// Spin-wait for a duration roughly equal to `ns` nanoseconds.
///
/// If `freq_hz` is 0, this returns immediately.
/// The precision is limited by `mono_ticks` resolution and overhead.
pub fn busy_wait_ns(ns: u64, ticks_fn: impl Fn() -> u64, freq_hz: u64) {
    if freq_hz == 0 {
        return;
    }
    
    let start_ticks = ticks_fn();
    let target_ticks = ns_to_ticks(ns, freq_hz);
    
    loop {
        let current_ticks = ticks_fn();
        let elapsed = current_ticks.saturating_sub(start_ticks);
        if elapsed >= target_ticks {
            break;
        }
        // Hint to the CPU that we are spinning
        core::hint::spin_loop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monotonic_clamp() {
        let clamp = MonotonicClamp::new();
        assert_eq!(clamp.clamp(10), 10);
        assert_eq!(clamp.clamp(20), 20);
        assert_eq!(clamp.clamp(15), 20); // Should clamp to last max
        assert_eq!(clamp.clamp(25), 25);
    }

    #[test]
    fn test_conversion_helpers() {
        let freq = 1_000_000_000; // 1 GHz
        assert_eq!(ticks_to_ns(1000, freq), 1000);
        assert_eq!(ns_to_ticks(1000, freq), 1000);

        let freq = 100_000_000; // 100 MHz
        assert_eq!(ticks_to_ns(100, freq), 1000);
        assert_eq!(ns_to_ticks(1000, freq), 100);
    }

    #[test]
    fn test_overflow_protection() {
        let freq = 1_000_000_000;
        let large_ticks = u64::MAX;
        // Should saturate, not panic
        let ns = ticks_to_ns(large_ticks, freq);
        assert!(ns > 0);
        
        // Large ns to ticks
        let ticks = ns_to_ticks(u64::MAX, freq);
        assert_eq!(ticks, u64::MAX); // Adjusted expectation locally? 
        // 1GHz: u64::MAX ticks = u64::MAX ns approx.
        // Actually (u64::MAX * 1e9) / 1e9 = u64::MAX.
        
        // Try with low freq, high ticks
        let freq = 1;
        // ticks_to_ns: u64::MAX * 1e9 / 1 -> saturates
        assert_eq!(ticks_to_ns(u64::MAX, freq), u64::MAX);
    }
}
