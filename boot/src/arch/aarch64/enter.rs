use super::super::UserEntryRegs;
use core::arch::global_asm;
extern crate alloc;
use alloc::boxed::Box;

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

pub fn resume_user_mode(context: &[u64]) -> ! {
    unsafe { resume_user_mode_asm(context.as_ptr()) }
}

pub fn alloc_user_stack() -> u64 {
    let stack = Box::new([0u8; 4096]);
    let stack_ptr = Box::leak(stack).as_mut_ptr();
    // Stack grows down, so return end
    let stack_addr = stack_ptr as u64 + 4096;
    // Align to 16 bytes
    stack_addr & !0xf
}

pub unsafe fn init_user_stack(_phys_mem_offset: u64) {
    // TODO: Map user code as accessible
}
