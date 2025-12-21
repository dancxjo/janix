use core::arch::asm;
use core::sync::atomic::{AtomicU64, Ordering};
use kernel::time::HardwareTimer;

pub struct Arm64HardwareTimer {
    freq_hz: AtomicU64,
}

impl HardwareTimer for Arm64HardwareTimer {
    fn init(&self) {
        let freq: u64;
        unsafe {
            asm!("mrs {}, cntfrq_el0", out(reg) freq, options(nomem, nostack));
        }
        self.freq_hz.store(freq, Ordering::Relaxed);
    }

    fn now_ns(&self) -> u64 {
        let cnt: u64;
        unsafe {
            asm!("mrs {}, cntvct_el0", out(reg) cnt, options(nomem, nostack));
        }

        let freq = self.freq_hz.load(Ordering::Relaxed);
        let freq = if freq == 0 {
            let freq_reg: u64;
             unsafe {
                asm!("mrs {}, cntfrq_el0", out(reg) freq_reg, options(nomem, nostack));
            }
            freq_reg
        } else {
            freq
        };

        if freq == 0 {
            return 0;
        }

        ((cnt as u128 * 1_000_000_000) / freq as u128) as u64
    }

    fn set_deadline_ns(&self, deadline_ns: u64) {
        let now = self.now_ns();

        // Calculate ticks to wait.
        // If deadline is in the past, we want to fire immediately.
        // Writing 0 to TVAL triggers the interrupt immediately because the condition is (TVAL <= 0).
        let ticks: u64 = if deadline_ns <= now {
             0
        } else {
             let freq = self.freq_hz.load(Ordering::Relaxed);
             // If freq is 0 (should not happen if init called), try read again
             let freq = if freq == 0 {
                 let freq_reg: u64;
                 unsafe { asm!("mrs {}, cntfrq_el0", out(reg) freq_reg, options(nomem, nostack)); }
                 freq_reg
             } else {
                 freq
             };

             if freq == 0 { return; } // Can't do anything without frequency

             let delta_ns = deadline_ns - now;
             ((delta_ns as u128 * freq as u128) / 1_000_000_000) as u64
        };

        // CNTV_TVAL_EL0 is a 32-bit signed down counter.
        // Max positive value is i32::MAX.
        // If ticks > i32::MAX, we cap it at i32::MAX.
        // This effectively sleeps for the maximum possible duration supported by the 32-bit counter.
        // The interrupt will fire "early" relative to the true deadline,
        // and the kernel's timer loop will re-evaluate and sleep again.
        let tval = if ticks > i32::MAX as u64 {
            i32::MAX as u64
        } else {
            ticks
        };

        unsafe {
            asm!("msr cntv_tval_el0, {}", in(reg) tval, options(nomem, nostack));
            // Enable timer and unmask interrupt (ENABLE=1, IMASK=0, ISTATUS is read-only)
            // bit 0: ENABLE
            // bit 1: IMASK (1 = masked, 0 = unmasked)
            asm!("msr cntv_ctl_el0, {}", in(reg) 1u64, options(nomem, nostack));
        }
    }
}

pub fn init_arch_timer() {
    static TIMER: Arm64HardwareTimer = Arm64HardwareTimer { freq_hz: AtomicU64::new(0) };
    kernel::time::register_timer(&TIMER);
    TIMER.init();
}
