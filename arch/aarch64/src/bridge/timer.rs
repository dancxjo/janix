use alloc::format;
use core::arch::asm;
use kernel::bridge::CpuBridge;

// ARM Generic Timer (EL1 Physical)
// CNTP_TVAL_EL1: Timer Value (Downcounter)
// CNTP_CTL_EL1: Control Register

const CNT_CTL_ENABLE: u64 = 1;
// const CNT_CTL_IMASK: u64 = 2; // 1 = Masked (Disabled IRQ), 0 = Unmasked

pub fn init() {
    unsafe {
        // Disable first
        asm!("msr cntp_ctl_el0, {}", in(reg) 0u64);

        // Read Frequency
        let frq: u64;
        asm!("mrs {}, cntfrq_el0", out(reg) frq);

        super::Bridge.log(alloc::format!("timer: frequency = {}\n", frq).as_str());

        // Set Interval (e.g. 100Hz = frq / 100)
        // If frq is 0 (should not happen on proper HW/QEMU), prevent div by zero
        if frq > 0 {
            let interval = frq / 100;
            asm!("msr cntp_tval_el0, {}", in(reg) interval);
            // Enable
            asm!("msr cntp_ctl_el0, {}", in(reg) CNT_CTL_ENABLE);
            super::Bridge.log("timer: enabled\n");
        } else {
            super::Bridge.log("timer: frequency is 0!\n");
        }
    }
}

pub fn next_match() {
    unsafe {
        // Read Frequency
        let frq: u64;
        asm!("mrs {}, cntfrq_el0", out(reg) frq);

        if frq > 0 {
            let interval = frq / 100;
            asm!("msr cntp_tval_el0, {}", in(reg) interval);
            // Ensure enabled
            asm!("msr cntp_ctl_el0, {}", in(reg) CNT_CTL_ENABLE);
        }
    }
}
