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
    let mut x0 = arg0;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") num,
            inlateout("x0") x0,
            in("x1") arg1,
            in("x2") arg2,
            in("x3") arg3,
            in("x4") arg4,
            in("x5") arg5,
            clobber_abi("C"),
            options(nostack),
        );
    }
    x0
}
