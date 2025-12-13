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
            "syscall 0",
            inlateout("$a7") num as u64 => ret,
            inlateout("$a0") arg0 => ret,
            in("$a1") arg1,
            in("$a2") arg2,
            in("$a3") arg3,
            in("$a4") arg4,
            in("$a5") arg5,
            options(nostack, preserves_flags),
        );
    }
    ret
}
