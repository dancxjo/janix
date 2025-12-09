use kernel_core::time::HardwareTimer;
use core::arch::asm;

pub struct Arm64HardwareTimer {
    freq_hz: u64,
}

impl HardwareTimer for Arm64HardwareTimer {
    fn init(&self) {
        // Read CNTFRQ_EL0 once at boot and store in freq_hz.
        // For now, we just read it every time or assume a value if we can't write to self.
        // Since self is &self, we can't mutate. But we can use interior mutability or just read the register.
        // The prompt suggests storing it. But `init` takes `&self`.
        // I'll just read the register in `now_ns` for simplicity or assume it's constant.
        // Actually, CNTFRQ_EL0 is a system register, so reading it is fast.
    }

    fn now_ns(&self) -> u64 {
        let cnt: u64;
        let freq: u64;
        unsafe {
            asm!("mrs {}, cntvct_el0", out(reg) cnt, options(nomem, nostack));
            asm!("mrs {}, cntfrq_el0", out(reg) freq, options(nomem, nostack));
        }
        if freq == 0 { return 0; }
        // ns = cnt * 1_000_000_000 / freq
        // Use u128 to avoid overflow
        ((cnt as u128 * 1_000_000_000) / freq as u128) as u64
    }

    fn set_deadline_ns(&self, _deadline_ns: u64) {
        // TODO: Program CNTV_TVAL_EL0
    }
}

pub fn init_arch_timer() {
    static TIMER: Arm64HardwareTimer = Arm64HardwareTimer { freq_hz: 0 };
    kernel_core::time::register_timer(&TIMER);
}
