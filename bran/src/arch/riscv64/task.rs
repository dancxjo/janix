use super::paging::RISCV64AddressSpace;
use core::arch::asm;
use kernel::UserTaskSpec;

#[derive(Copy, Clone, Default)]
pub struct RISCV64Context(pub [u64; 14]); // ra, sp, s0-s11

#[unsafe(no_mangle)]
pub unsafe extern "C" fn context_switch(_old: *mut u64, _new: *const u64) {
    unsafe {
        asm!(
            "sd ra, 0(a0)",
            "sd sp, 8(a0)",
            "sd s0, 16(a0)",
            "sd s1, 24(a0)",
            "sd s2, 32(a0)",
            "sd s3, 40(a0)",
            "sd s4, 48(a0)",
            "sd s5, 56(a0)",
            "sd s6, 64(a0)",
            "sd s7, 72(a0)",
            "sd s8, 80(a0)",
            "sd s9, 88(a0)",
            "sd s10, 96(a0)",
            "sd s11, 104(a0)",
            "ld ra, 0(a1)",
            "ld sp, 8(a1)",
            "ld s0, 16(a1)",
            "ld s1, 24(a1)",
            "ld s2, 32(a1)",
            "ld s3, 40(a1)",
            "ld s4, 48(a1)",
            "ld s5, 56(a1)",
            "ld s6, 64(a1)",
            "ld s7, 72(a1)",
            "ld s8, 80(a1)",
            "ld s9, 88(a1)",
            "ld s10, 96(a1)",
            "ld s11, 104(a1)",
            "ret",
            options(noreturn)
        );
    }
}

pub fn init_kernel_context(
    entry: extern "C" fn(usize) -> !,
    stack_top: u64,
    arg: usize,
) -> RISCV64Context {
    let mut ctx = RISCV64Context::default();
    ctx.0[0] = trampoline as *const () as u64; // ra
    ctx.0[1] = stack_top; // sp
    ctx.0[2] = entry as *const () as u64; // s0
    ctx.0[3] = arg as u64; // s1
    ctx
}

pub fn init_user_context(
    _spec: UserTaskSpec<RISCV64AddressSpace>,
    _kstack_top: u64,
) -> RISCV64Context {
    RISCV64Context::default()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn trampoline() -> ! {
    unsafe {
        asm!("mv a0, s1", "jalr s0", "ebreak", options(noreturn));
    }
}

pub unsafe fn switch(from: &mut RISCV64Context, to: &RISCV64Context) {
    unsafe { context_switch(from.0.as_mut_ptr(), to.0.as_ptr()) };
}
