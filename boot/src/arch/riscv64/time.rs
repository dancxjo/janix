use kernel_core::time::HardwareTimer;
use core::arch::asm;

pub struct RiscvHardwareTimer;

impl HardwareTimer for RiscvHardwareTimer {
    fn init(&self) {
    }

    fn now_ns(&self) -> u64 {
        let cycles: u64;
        unsafe {
            asm!("rdtime {}", out(reg) cycles, options(nomem, nostack));
        }
        // Assume 10MHz for QEMU virt (100 ns per cycle)
        // TODO: Get actual frequency from device tree or config
        cycles * 100
    }

    fn set_deadline_ns(&self, _deadline_ns: u64) {
        // TODO: Program mtimecmp / stimecmp
    }
}

pub fn init_arch_timer() {
    static TIMER: RiscvHardwareTimer = RiscvHardwareTimer;
    kernel_core::time::register_timer(&TIMER);
}
