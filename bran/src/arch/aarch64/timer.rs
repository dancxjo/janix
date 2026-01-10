use super::gic;
use core::arch::asm;
use kernel::kinfo;

// Timer CTL bits
const CNTP_CTL_ENABLE: u64 = 1;

// Interrupt ID for CNTV_NS (Virtual Non-Secure)
// On QEMU virt: 27
pub const TIMER_IRQ: u32 = 27;

pub unsafe fn init() {
    kinfo!("timer: init virtual timer");

    // 1. Init GIC (Assuming default Virt addresses)
    // We should map these if we had paging, but for now we try physical.
    unsafe { gic::init(gic::GICD_PHYS, gic::GICC_PHYS); }

    // 2. Enable IRQ in GIC
    unsafe {
        gic::set_priority(TIMER_IRQ, 0x80); 
        gic::enable_irq(TIMER_IRQ);
    }

    // 3. Configure Timer
    let freq: u64;
    unsafe { asm!("mrs {}, cntfrq_el0", out(reg) freq, options(nomem, nostack)); }
    kinfo!("timer: freq {} Hz", freq);

    // Set interval (e.g. 100Hz = freq / 100)
    let interval = freq / 100;

    unsafe {
        asm!("msr cntv_tval_el0, {}", in(reg) interval, options(nomem, nostack));
        // Enable + Unmask -> 1
        asm!("msr cntv_ctl_el0, {}", in(reg) CNTP_CTL_ENABLE, options(nomem, nostack));
    }
}

/// Returns true if the timer was the source of the interrupt
pub unsafe fn check_and_ack() -> bool {
    // Read IAR from GIC
    let irq = unsafe { gic::ack_irq() };
    let id = irq & 0x3FF; 
    
    if id == TIMER_IRQ {
        // Ack Timer HW (reload)
        let freq: u64;
        unsafe { asm!("mrs {}, cntfrq_el0", out(reg) freq, options(nomem, nostack)); }
        let interval = freq / 100;
        unsafe { 
             asm!("msr cntv_tval_el0, {}", in(reg) interval, options(nomem, nostack));
             // Ensure enabled
             asm!("msr cntv_ctl_el0, {}", in(reg) CNTP_CTL_ENABLE, options(nomem, nostack));
        }
        
        // EOI GIC
        unsafe { gic::eoi(irq); }
        return true;
    } else if id < 1020 {
        // Other Valid IRQ -> EOI but return false
        // We don't handle other IRQs yet, but we must EOI to prevent loop
        unsafe { gic::eoi(irq); }
        return false;
    }
    
    // Spurious (1023) -> Just ignore
    false
}
