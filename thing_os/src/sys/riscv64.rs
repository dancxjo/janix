#[inline(always)]
pub unsafe fn syscall_stub(
    num: u64,
    arg0: u64,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
) -> u64 {
    let mut a0 = arg0;
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a7") num,
            inlateout("a0") a0,
            in("a1") arg1,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            clobber_abi("C"),
            options(nostack),
        );
    }
    a0
}
