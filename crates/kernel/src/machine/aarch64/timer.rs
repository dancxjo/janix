use core::arch::asm;
use super::gic;

// Timer CTL bits
const CNTP_CTL_ENABLE: u64 = 1;

// Interrupt ID for CNTP_NS (Physical Non-Secure)
// On QEMU virt: 30
pub const TIMER_IRQ: u32 = 30; 

pub unsafe fn init() {
    // 1. Initialize GIC (if not already?)
    // Machine::init should call gic::init.
    
    // 2. Enable IRQ in GIC
    gic::enable_irq(TIMER_IRQ);
    
    // 3. Configure Timer
    // Frequency?
    let freq: u64;
    asm!("mrs {}, cntfrq_el0", out(reg) freq);
    
    // Set interval (e.g. 100Hz = freq / 100)
    let interval = freq / 100;
    
    asm!("msr cntp_tval_el0, {}", in(reg) interval);
    
    // Enable + Unmask
    // Write 1 (Enable=1, Imask=0)
    asm!("msr cntp_ctl_el0, {}", in(reg) CNTP_CTL_ENABLE);
    
    // 4. Unmask DAIF (Interrupts) on CPU?
    // This is done by `irq_enable()` in Machine trait.
}

pub unsafe fn ack() {
    // 1. Reset timer value (to clear internal status and schedule next)
    let freq: u64;
    asm!("mrs {}, cntfrq_el0", out(reg) freq);
    let interval = freq / 100;
    asm!("msr cntp_tval_el0, {}", in(reg) interval);
    
    // 2. Ack GIC?
    // GIC need EOI.
    // Usually handler reads IAR then writes EOIR.
    // Our wrapper might do it? 
    // Wait, the handler calls `tick`.
    // We should EOI in `tick` or here.
    // EOI typically requires the ID.
    // We assume we are in handler for TIMER_IRQ.
    // We write EOI(TIMER_IRQ) or we write matching EOI.
    gic::eoi(TIMER_IRQ);
}
