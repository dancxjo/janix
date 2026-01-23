//! Scheduler helpers for Blossom's repaint loop.
//!
//! Blossom keeps per-view repaint rates capped to avoid thrash while
//! still guaranteeing forward progress.

/// Return true if enough time has elapsed to allow a tick at the target rate.
pub fn should_tick(now_ns: u64, last_ns: u64, hz: u64) -> bool {
    if hz == 0 {
        return true;
    }
    let period = 1_000_000_000u64 / hz;
    now_ns.saturating_sub(last_ns) >= period
}

/// Clamp a work budget to avoid runaway processing.
pub fn budget_exhausted(start_ns: u64, now_ns: u64, max_ns: u64) -> bool {
    now_ns.saturating_sub(start_ns) >= max_ns
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;

    #[test]
    fn tick_respects_rate() {
        let hz = 60;
        assert!(should_tick(1_000_000_000, 0, hz));
        assert!(!should_tick(10_000_000, 0, hz));
    }
}
