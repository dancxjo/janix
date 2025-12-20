use core::arch::asm;
use kernel::time::HardwareTimer;

pub struct RiscvHardwareTimer;

impl HardwareTimer for RiscvHardwareTimer {
    fn init(&self) {
        // Enable Supervisor Timer Interrupt (STIE) in sie CSR.
        // Bit 5 is STIE.
        unsafe {
            asm!(
                "csrs sie, {}",
                in(reg) 1 << 5,
                options(nostack)
            );
        }
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

    fn set_deadline_ns(&self, deadline_ns: u64) {
        // Convert deadline_ns to cycles.
        // Since now_ns = cycles * 100, then cycles = deadline_ns / 100.
        let cycles = deadline_ns / 100;

        // Program the timer using SBI.
        super::sbi::set_timer(cycles);
    }
}

pub fn init_arch_timer() {
    static TIMER: RiscvHardwareTimer = RiscvHardwareTimer;
    kernel::time::register_timer(&TIMER);
    TIMER.init();
}
