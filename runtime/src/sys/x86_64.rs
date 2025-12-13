use abi::SyscallNumber;

#[inline(always)]
pub unsafe fn syscall_stub(
    num: SyscallNumber,
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
            "int 0x80",
            inlateout("rax") num as u64 => ret,
            in("rdi") arg0,
            in("rsi") arg1,
            in("rdx") arg2,
            in("rcx") arg3,
            in("r8") arg4,
            in("r9") arg5,
            options(nostack, preserves_flags),
        );
    }
    ret
}
