use core::arch::asm;
use kernel::UserTaskSpec;
use super::paging::LoongArch64AddressSpace;

#[derive(Copy, Clone, Default)]
pub struct LoongArch64Context(pub [u64; 12]); // ra, sp, fp, s0-s8

#[unsafe(no_mangle)]
pub unsafe extern "C" fn context_switch(_old: *mut u64, _new: *const u64) {
    unsafe {
        asm!(
            "st.d $ra, $a0, 0",
            "st.d $sp, $a0, 8",
            "st.d $fp, $a0, 16",
            "st.d $s0, $a0, 24",
            "st.d $s1, $a0, 32",
            "st.d $s2, $a0, 40",
            "st.d $s3, $a0, 48",
            "st.d $s4, $a0, 56",
            "st.d $s5, $a0, 64",
            "st.d $s6, $a0, 72",
            "st.d $s7, $a0, 80",
            "st.d $s8, $a0, 88",

            "ld.d $ra, $a1, 0",
            "ld.d $sp, $a1, 8",
            "ld.d $fp, $a1, 16",
            "ld.d $s0, $a1, 24",
            "ld.d $s1, $a1, 32",
            "ld.d $s2, $a1, 40",
            "ld.d $s3, $a1, 48",
            "ld.d $s4, $a1, 56",
            "ld.d $s5, $a1, 64",
            "ld.d $s6, $a1, 72",
            "ld.d $s7, $a1, 80",
            "ld.d $s8, $a1, 88",
            "jirl $zero, $ra, 0",
            options(noreturn)
        );
    }
}

pub fn init_kernel_context(
    entry: extern "C" fn(usize) -> !,
    stack_top: u64,
    arg: usize,
) -> LoongArch64Context {
    let mut ctx = LoongArch64Context::default();
    ctx.0[0] = trampoline as *const () as u64; // ra
    ctx.0[1] = stack_top;         // sp
    ctx.0[3] = entry as *const () as u64; // s0
    ctx.0[4] = arg as u64;              // s1
    ctx
}

pub fn init_user_context(_spec: UserTaskSpec<LoongArch64AddressSpace>, _kstack_top: u64) -> LoongArch64Context {
    LoongArch64Context::default() 
}

#[unsafe(no_mangle)]
unsafe extern "C" fn trampoline() -> ! {
    unsafe {
        asm!(
            "or $a0, $s1, $zero",
            "jirl $zero, $s0, 0",
            "break 0",
            options(noreturn)
        );
    }
}

pub unsafe fn switch(from: &mut LoongArch64Context, to: &LoongArch64Context) {
    unsafe { context_switch(from.0.as_mut_ptr(), to.0.as_ptr()) };
}
