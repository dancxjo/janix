use core::arch::asm;
use kernel::time::HardwareTimer;

pub struct X86HardwareTimer;

impl HardwareTimer for X86HardwareTimer {
    fn init(&self) {
        // TODO: Calibrate TSC or initialize HPET
    }

    fn now_ns(&self) -> u64 {
        let tsc: u64;
        unsafe {
            asm!(
                "rdtsc",
                "shl rdx, 32",
                "or rax, rdx",
                out("rax") tsc,
                out("rdx") _,
                options(nomem, nostack)
            );
        }
        // Assume 2GHz for now (0.5 ns per cycle)
        tsc / 2
    }

    fn set_deadline_ns(&self, _deadline_ns: u64) {
        // TODO: Program LAPIC timer
    }
}

pub fn init_arch_timer() {
    static TIMER: X86HardwareTimer = X86HardwareTimer;
    kernel::time::register_timer(&TIMER);
}
