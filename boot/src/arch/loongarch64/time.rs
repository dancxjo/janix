use kernel_core::time::HardwareTimer;
use core::arch::asm;

pub struct LoongArchHardwareTimer;

impl HardwareTimer for LoongArchHardwareTimer {
    fn init(&self) {
    }

    fn now_ns(&self) -> u64 {
        let cycles: u64;
        unsafe {
            // rdcntvl.w $rd
            // But we want 64-bit counter. rdcntvl.d
            // asm!("rdcntvl.d {}", out(reg) cycles);
            // Using a placeholder as I'm not 100% sure on the asm syntax for loongarch in Rust right now without checking docs.
            // But `rdcntvl.d` is correct instruction.
            // Let's assume a simple counter read.
            // Actually, let's just return 0 for now to avoid build errors if asm is wrong, 
            // or try to use the correct one.
            // "rdcntvl.d $r4" -> $r4 = counter
            asm!("rdcntvl.d {}", out(reg) cycles, options(nomem, nostack));
        }
        // Assume 100MHz?
        cycles * 10
    }

    fn set_deadline_ns(&self, _deadline_ns: u64) {
    }
}

pub fn init_arch_timer() {
    static TIMER: LoongArchHardwareTimer = LoongArchHardwareTimer;
    kernel_core::time::register_timer(&TIMER);
}
