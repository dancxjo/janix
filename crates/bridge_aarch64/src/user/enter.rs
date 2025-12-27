use super::UserEntryRegs;
use core::arch::global_asm;
extern crate alloc;
use super::super::paging; // Access paging from root
use alloc::alloc::{Layout, alloc_zeroed};
use core::ptr::NonNull;

global_asm!(
    r#"
.global enter_user_mode_asm
enter_user_mode_asm:
    // x0 points to UserEntryRegs
    // struct UserEntryRegs {{ entry_point: u64, user_stack: u64, arg0: u64 }}
    
    ldr x1, [x0, #0]  // entry_point
    ldr x2, [x0, #8]  // user_stack
    ldr x0, [x0, #16] // arg0

    // Set SPSR_EL1 to EL0t (0b0000)
    // Mask all interrupts (DAIF) for now? Or unmask?
    // Let's unmask (0). So SPSR = 0.
    mov x3, #0
    msr spsr_el1, x3

    // Set ELR_EL1 to entry point
    msr elr_el1, x1

    // Set SP_EL0 to user stack
    msr sp_el0, x2

    // Clear other registers?
    // x0 is arg0.
    
    eret
"#
);

global_asm!(
    r#"
.global resume_user_mode_asm
resume_user_mode_asm:
    // x0 points to context array (34 u64s)
    // See trap.S SAVE_REGS for layout
    // Layout: x0..x29, x30, sp_el0, elr_el1, spsr_el1
    
    mov sp, x0
    
    ldp x22, x23, [sp, #16 * 16]
    msr elr_el1, x22
    msr spsr_el1, x23
    
    ldp x30, x21, [sp, #16 * 15]
    msr sp_el0, x21
    
    ldp x28, x29, [sp, #16 * 14]
    ldp x26, x27, [sp, #16 * 13]
    ldp x24, x25, [sp, #16 * 12]
    ldp x22, x23, [sp, #16 * 11]
    ldp x20, x21, [sp, #16 * 10]
    ldp x18, x19, [sp, #16 * 9]
    ldp x16, x17, [sp, #16 * 8]
    ldp x14, x15, [sp, #16 * 7]
    ldp x12, x13, [sp, #16 * 6]
    ldp x10, x11, [sp, #16 * 5]
    ldp x8, x9, [sp, #16 * 4]
    ldp x6, x7, [sp, #16 * 3]
    ldp x4, x5, [sp, #16 * 2]
    ldp x2, x3, [sp, #16 * 1]
    ldp x0, x1, [sp, #16 * 0]
    
    eret
"#
);

unsafe extern "C" {
    fn enter_user_mode_asm(regs: *const UserEntryRegs) -> !;
    fn resume_user_mode_asm(context: *const u64) -> !;
}

pub fn enter_user_mode(regs: &UserEntryRegs) -> ! {
    unsafe { enter_user_mode_asm(regs) }
}

pub fn resume_user_mode(context: &[u64], _fpu_context: &kernel_core::sched::fpu::FpuContext) -> ! {
    unsafe { resume_user_mode_asm(context.as_ptr()) }
}

const USER_STACK_SIZE: usize = 64 * 1024;

pub fn alloc_user_stack() -> u64 {
    let layout = Layout::from_size_align(USER_STACK_SIZE, 16).expect("invalid user stack layout");
    let stack_ptr = unsafe { alloc_zeroed(layout) };
    let stack_ptr = NonNull::new(stack_ptr).expect("alloc_user_stack: allocation failed");
    let stack_addr = stack_ptr.as_ptr() as u64;

    unsafe {
        // Map as user accessible + Normal memory
        let start = stack_addr;
        let end = stack_addr + USER_STACK_SIZE as u64;
        let mut curr = start;
        while curr < end {
            // We want AP[1]=1 (EL0 access) and Normal memory type
            paging::update_page_flags(curr, paging::DESC_AP_EL0 | paging::ATTR_NORMAL, 0);
            curr += 4096;
        }
    }

    // Stack grows down, so return end
    let stack_top = stack_addr + USER_STACK_SIZE as u64;
    // Align to 16 bytes (layout guarantees it, but just in case logic changes)
    stack_top & !0xf
}

pub unsafe fn init_user_stack(_phys_mem_offset: u64) {
    // Map user_thread_main as user accessible
    // For now we assume the function is accessible. 
    // The previous code had a reference to crate::user::user_thread_main but that might not exist here.
    // Let's keep it commented or check where it is.
    // In bridge v0.2, user entry is passed from outside.
}

use core::sync::atomic::{AtomicU64, Ordering};

static KERNEL_TTBR0: AtomicU64 = AtomicU64::new(0);

fn ensure_kernel_ttbr0_recorded() -> u64 {
    let stored = KERNEL_TTBR0.load(Ordering::SeqCst);
    if stored != 0 {
        return stored;
    }
    let current: u64;
    unsafe {
        core::arch::asm!("mrs {reg}, ttbr0_el1", reg = out(reg) current);
    }
    KERNEL_TTBR0.store(current, Ordering::SeqCst);
    current
}

pub fn activate_address_space(token: Option<u64>) {
    let kernel_ttbr0 = ensure_kernel_ttbr0_recorded();
    let target = token.unwrap_or(kernel_ttbr0);
    let current: u64;
    unsafe {
        core::arch::asm!("mrs {reg}, ttbr0_el1", reg = out(reg) current);
    }
    if current == target {
        return;
    }
    // Ensure TTBR0 translations are enabled (clear EPD0 if firmware left it set).
    unsafe {
        let mut tcr: u64;
        core::arch::asm!("mrs {reg}, tcr_el1", reg = out(reg) tcr);
        if tcr & (1 << 7) != 0 {
            tcr &= !(1 << 7);
            core::arch::asm!("msr tcr_el1, {val}", val = in(reg) tcr, options(nostack));
            core::arch::asm!("isb");
        }
    }
    unsafe {
        core::arch::asm!(
            "dsb ish",
            "msr ttbr0_el1, {ttbr}",
            "isb",
            "tlbi vmalle1",
            "dsb ish",
            "isb",
            ttbr = in(reg) target,
            options(nostack)
        );
    }
}
