use core::sync::atomic::{AtomicU64, Ordering};
use kernel::time::RealTimeClock;

pub struct Arm64Rtc {
    boot_offset_ns: AtomicU64,
}

impl Arm64Rtc {
    pub const fn new() -> Self {
        Arm64Rtc {
            boot_offset_ns: AtomicU64::new(0),
        }
    }

    pub fn set_offset_ns(&self, offset: u64) {
        self.boot_offset_ns.store(offset, Ordering::Relaxed);
    }
}

impl RealTimeClock for Arm64Rtc {
    fn init(&self) {
        if let Some(addr) = super::dtb::get_pl031_address() {
            unsafe {
                // Map the PL031 device region (4KB is sufficient)
                super::paging::map_device_region(addr, 4096);

                let virt = kernel::memory::phys_to_virt(addr);
                let ptr = virt as *const u32;
                // RTCDR (Data Register) is at offset 0x000
                let seconds = ptr.read_volatile();

                let now_ns = kernel::time::monotonic_now_ns();
                let rtc_ns = (seconds as u64) * 1_000_000_000;

                // Calculate offset: offset = rtc_ns - monotonic_now_ns
                // Since monotonic starts at 0, this effectively sets the base time.
                // We use saturating_sub just in case, though monotonic should be small.
                let offset = rtc_ns.saturating_sub(now_ns);

                self.boot_offset_ns.store(offset, Ordering::Relaxed);

                kernel::log("Initialized RTC from PL031");
            }
        } else {
            kernel::log("PL031 RTC not found in DTB");
        }
    }

    fn now_utc(&self) -> (u64, u32) {
        let now_ns = kernel::time::monotonic_now_ns();
        let off = self.boot_offset_ns.load(Ordering::Relaxed);
        let total = now_ns.saturating_add(off);
        let secs = total / 1_000_000_000;
        let nanos = (total % 1_000_000_000) as u32;
        (secs, nanos)
    }
}

static RTC: Arm64Rtc = Arm64Rtc::new();

pub fn init_arch_rtc() {
    RTC.init();
    kernel::time::register_rtc(&RTC);
}

pub fn read_boot_rtc_epoch_seconds() -> i64 {
    RTC.now_utc().0 as i64
}
