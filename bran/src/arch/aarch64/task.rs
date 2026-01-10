use core::arch::asm;
use kernel::UserTaskSpec;
use super::paging::AArch64AddressSpace;

#[derive(Copy, Clone, Default)]
pub struct AArch64Context(pub [u64; 13]); // x19-x29, lr, sp

#[unsafe(no_mangle)]
pub unsafe extern "C" fn context_switch(old: *mut u64, new: *const u64) {
    unsafe {
        asm!(
            // Save old
            "stp x19, x20, [x0, #0]",
            "stp x21, x22, [x0, #16]",
            "stp x23, x24, [x0, #32]",
            "stp x25, x26, [x0, #48]",
            "stp x27, x28, [x0, #64]",
            "stp x29, lr, [x0, #80]",
            "mov x9, sp",
            "str x9, [x0, #96]",

            // Load new
            "ldp x19, x20, [x1, #0]",
            "ldp x21, x22, [x1, #16]",
            "ldp x23, x24, [x1, #32]",
            "ldp x25, x26, [x1, #48]",
            "ldp x27, x28, [x1, #64]",
            "ldp x29, lr, [x1, #80]",
            "ldr x9, [x1, #96]",
            "mov sp, x9",

            "ret",
            options(noreturn)
        );
    }
}

pub fn init_kernel_context(
    entry: extern "C" fn(usize) -> !,
    stack_top: u64,
    arg: usize,
) -> AArch64Context {
    let mut ctx = AArch64Context::default();
    
    ctx.0[11] = trampoline as *const () as u64; // lr
    ctx.0[12] = stack_top;         // sp
    
    // x19 = entry, x20 = arg
    ctx.0[0] = entry as *const () as u64;
    ctx.0[1] = arg as u64;
    
    ctx
}

pub fn init_user_context(_spec: UserTaskSpec<AArch64AddressSpace>, _kstack_top: u64) -> AArch64Context {
    AArch64Context::default() 
}

#[unsafe(no_mangle)]
unsafe extern "C" fn trampoline() -> ! {
    unsafe {
        asm!(
            "mov x0, x20",
            "blr x19",
            "hlt #0",
            options(noreturn)
        );
    }
}

pub unsafe fn switch(from: &mut AArch64Context, to: &AArch64Context) {
    unsafe { context_switch(from.0.as_mut_ptr(), to.0.as_ptr()) };
}
