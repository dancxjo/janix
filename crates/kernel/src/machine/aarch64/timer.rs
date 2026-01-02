use core::arch::asm;
use super::gic;

// Timer CTL bits
const CNTP_CTL_ENABLE: u64 = 1;

// Interrupt ID for CNTV_NS (Virtual Non-Secure)
// On QEMU virt: 27
pub const TIMER_IRQ: u32 = 27; 

pub unsafe fn init() {
    // 2. Enable IRQ in GIC
    // GICv2: Interrupts must be Configured as Group 1 (Non-Secure) for EL1
    gic::set_priority(TIMER_IRQ, 0); // Highest priority
    gic::set_group1(TIMER_IRQ);      // Group 1
    gic::enable_irq(TIMER_IRQ);
    
    // 3. Configure Timer
    let freq: u64;
    asm!("mrs {}, cntfrq_el0", out(reg) freq);
    crate::serial::write(b"TIMER: FREQ=");
    crate::serial::write_hex(freq);
    crate::serial::write(b"\n");
    
    // Set interval (e.g. 100Hz = freq / 100)
    let interval = freq / 100;
    
    asm!("msr cntv_tval_el0, {}", in(reg) interval);
    
    // Enable + Unmask
    asm!("msr cntv_ctl_el0, {}", in(reg) CNTP_CTL_ENABLE);
}

pub unsafe fn ack() {
    let freq: u64;
    asm!("mrs {}, cntfrq_el0", out(reg) freq);
    let interval = freq / 100;
    asm!("msr cntv_tval_el0, {}", in(reg) interval);
    
    gic::eoi(TIMER_IRQ);
}
