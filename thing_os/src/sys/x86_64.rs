

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
    let mut ret: u64;
    unsafe {
        core::arch::asm!(
            "syscall",
            "nop", // No-ops kept for padding/alignment if needed, though likely unnecessary
            "nop",
            "nop",
            "nop",
            inlateout("rax") num as u64 => ret,
            in("rdi") arg0,
            in("rsi") arg1,
            in("rdx") arg2,
            in("r10") arg3,
            in("r8") arg4,
            in("r9") arg5,
            lateout("rcx") _, // rcx is clobbered by syscall
            lateout("r11") _, // r11 is clobbered by syscall
            options(nostack),
        );
    }
    ret
}
