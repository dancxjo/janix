use core::sync::atomic::{AtomicU64, Ordering};
use kernel_core::time::RealTimeClock;

pub struct RiscvRtc {
    boot_offset_ns: AtomicU64,
}

impl RiscvRtc {
    pub const fn new() -> Self {
        RiscvRtc {
            boot_offset_ns: AtomicU64::new(0),
        }
    }

    pub fn set_offset_ns(&self, offset: u64) {
        self.boot_offset_ns.store(offset, Ordering::Relaxed);
    }
}

impl RealTimeClock for RiscvRtc {
    fn init(&self) {
        // TODO: Read RTC from firmware/bootloader if available.
    }

    fn now_utc(&self) -> (u64, u32) {
        let now_ns = kernel_core::time::monotonic_now_ns();
        let off = self.boot_offset_ns.load(Ordering::Relaxed);
        let total = now_ns.saturating_add(off);
        let secs = total / 1_000_000_000;
        let nanos = (total % 1_000_000_000) as u32;
        (secs, nanos)
    }
}

pub fn init_arch_rtc() {
    static RTC: RiscvRtc = RiscvRtc::new();
    RTC.init();
    kernel_core::time::register_rtc(&RTC);
}
