use super::UserEntryRegs;
use alloc::alloc::{alloc_zeroed, Layout};
use core::arch::global_asm;
use kernel_core::sched::fpu::FpuContext;

global_asm!(
    r#"
.global enter_user_mode_asm
enter_user_mode_asm:
    ldr x1, [x0, #0]  // entry_point
    ldr x2, [x0, #8]  // user_stack
    ldr x0, [x0, #16] // arg0

    mov x3, #0
    msr spsr_el1, x3
    msr elr_el1, x1
    msr sp_el0, x2
    
    eret
"#
);

global_asm!(
    r#"
.global resume_user_mode_asm
resume_user_mode_asm:
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

extern "C" {
    fn enter_user_mode_asm(regs: *const UserEntryRegs) -> !;
    fn resume_user_mode_asm(context: *const u64) -> !;
}

pub fn enter_user_mode(regs: &UserEntryRegs) -> ! {
    unsafe { enter_user_mode_asm(regs) }
}

pub fn resume_user_mode(context: &[u64], _fpu_context: &FpuContext) -> ! {
    unsafe { resume_user_mode_asm(context.as_ptr()) }
}

pub fn alloc_user_stack() -> u64 {
    const USER_STACK_SIZE: usize = 64 * 1024;
    let layout = Layout::from_size_align(USER_STACK_SIZE, 16).unwrap();
    let ptr = unsafe { alloc_zeroed(layout) };
    let addr = ptr as u64;
    // Stub flag updates
    addr + USER_STACK_SIZE as u64
}

pub unsafe fn init_user_stack(_phys_mem_offset: u64) {
    // Stub
}

pub fn activate_address_space(_token: Option<u64>) {
    // Stub
}
