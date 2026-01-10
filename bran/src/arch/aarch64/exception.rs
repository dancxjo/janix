use core::arch::global_asm;
use kernel::kinfo;

// Load the assembly vectors
global_asm!(include_str!("exception_entry.S"));

unsafe extern "C" {
    static mut _syscall_handler_addr: u64;
    fn exception_vector_table();
}

pub unsafe fn init() {
    kinfo!("aarch64: installing vector table");
    unsafe {
        let vbar = exception_vector_table as u64;
        core::arch::asm!("msr vbar_el1, {}", in(reg) vbar, options(nomem, nostack));
        // Reset syscall handler to 0 just in case
        _syscall_handler_addr = 0;
    }
}

pub unsafe fn register_syscall_handler(addr: u64) {
    unsafe {
        _syscall_handler_addr = addr;
        kinfo!("aarch64: registered syscall handler at {:#x}", addr);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kern_sync_handler(frame: *mut u64) {
    // frame points to x0...
    let elr: u64;
    let esr: u64;
    unsafe { 
        core::arch::asm!("mrs {}, elr_el1", out(reg) elr, options(nomem, nostack));
        core::arch::asm!("mrs {}, esr_el1", out(reg) esr, options(nomem, nostack));
    }
    
    kinfo!("*** SYNC EXCEPTION EL1 ***");
    kinfo!("ELR: {:#x} ESR: {:#x}", elr, esr);
    kinfo!("Frame: {:p}", frame);
    
    // Minimal dump
    super::hcf();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kern_irq_handler() {
    // 1. Check/Ack Timer
    let was_timer = unsafe { super::timer::check_and_ack() };
    
    if was_timer {
        kernel::task::set_need_resched(true);
    }
    
    // 2. Check preemption
    kernel::task::check_preemption();
}
