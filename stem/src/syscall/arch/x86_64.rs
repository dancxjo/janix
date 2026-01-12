use core::arch::asm;

/// Raw syscall entry point.
///
/// # Safety
/// This function executes a system call, which invokes the kernel. The caller must ensure
/// that the arguments satisfy the kernel's ABI for the specific syscall number `n`.
#[inline(always)]
pub unsafe fn raw_syscall6(
    n: u32,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> isize {
    let ret: isize;
    asm!(
        "syscall",
        inlateout("rax") n as usize => ret,
        in("rdi") a0,
        in("rsi") a1,
        in("rdx") a2,
        in("r10") a3, // RCX is destroyed by syscall, so Linux/SystemV uses R10 for 4th arg
        in("r8") a4,
        in("r9") a5,
        out("rcx") _, // Destroyed by syscall instruction
        out("r11") _, // Destroyed by syscall instruction
        options(nostack, preserves_flags)
    );
    ret
}
