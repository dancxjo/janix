use super::paging::AArch64AddressSpace;
use core::arch::{asm, naked_asm};
use kernel::UserTaskSpec;

#[derive(Copy, Clone, Default)]
pub struct AArch64Context(pub [u64; 13]); // x19-x29, lr, sp

/// Context switch between two tasks.
/// Saves callee-saved registers to `old` and restores from `new`.
#[unsafe(naked)]
pub unsafe extern "C" fn context_switch(_old: *mut u64, _new: *const u64) {
    naked_asm!(
        // Save old context (x0 = old)
        "stp x19, x20, [x0, #0]",
        "stp x21, x22, [x0, #16]",
        "stp x23, x24, [x0, #32]",
        "stp x25, x26, [x0, #48]",
        "stp x27, x28, [x0, #64]",
        "stp x29, x30, [x0, #80]", // x30 = lr
        "mov x9, sp",
        "str x9, [x0, #96]",
        // Load new context (x1 = new)
        "ldp x19, x20, [x1, #0]",
        "ldp x21, x22, [x1, #16]",
        "ldp x23, x24, [x1, #32]",
        "ldp x25, x26, [x1, #48]",
        "ldp x27, x28, [x1, #64]",
        "ldp x29, x30, [x1, #80]",
        "ldr x9, [x1, #96]",
        "mov sp, x9",
        "ret",
    );
}

pub fn init_kernel_context(
    entry: extern "C" fn(usize) -> !,
    stack_top: u64,
    arg: usize,
) -> AArch64Context {
    let mut ctx = AArch64Context::default();

    // When context_switch loads this context:
    // - lr (x30) should point to our trampoline
    // - sp should be the new stack
    // - We store entry in x19 and arg in x20 for the trampoline to use
    ctx.0[11] = trampoline as *const () as u64; // lr (position 11 = x30)
    ctx.0[12] = stack_top; // sp (position 12)
    ctx.0[0] = entry as *const () as u64; // x19 = entry
    ctx.0[1] = arg as u64; // x20 = arg

    ctx
}

pub fn init_user_context(
    _spec: UserTaskSpec<AArch64AddressSpace>,
    _kstack_top: u64,
) -> AArch64Context {
    AArch64Context::default()
}

/// Trampoline that sets up arguments and calls the thread entry point.
/// Called when a new thread is first scheduled via context_switch.
#[unsafe(naked)]
unsafe extern "C" fn trampoline() -> ! {
    naked_asm!(
        "mov x0, x20", // arg is in x20
        "br x19",      // entry is in x19, jump (not call since it's noreturn)
    );
}

pub unsafe fn switch(from: &mut AArch64Context, to: &AArch64Context) {
    unsafe { context_switch(from.0.as_mut_ptr(), to.0.as_ptr()) };
}
