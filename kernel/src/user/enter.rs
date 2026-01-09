//! User entry points

use crate::trap::x86_64::TrapFrame;

/// Enter user mode using SYSRETQ.
/// 
/// # Safety
/// This function is inherently unsafe as it jumps to user code and switches privilege levels.
/// It assumes `tf` points to a valid `TrapFrame` with valid user state.
/// This function terminates the kernel thread (diverges).
#[unsafe(no_mangle)]
pub unsafe fn enter_user_sysret(tf: &TrapFrame) -> ! {
    unsafe {
        core::arch::asm!(
            "cli", // Disable interrupts
            
            // Point RSP to the TrapFrame logic (we treat tf ref as stack ptr)
            "mov rsp, {tf}",
            
            // Restore GPRs (popping from tf)
            // Order matches TrapFrame definition (r15 at offset 0)
            "pop r15",
            "pop r14",
            "pop r13",
            "pop r12",
            "pop r11",
            "pop r10",
            "pop r9",
            "pop r8",
            "pop rsi",
            "pop rdi",
            "pop rbp",
            "pop rdx",
            "pop rcx",
            "pop rbx",
            "pop rax",
            
            // Usage:
            // RSP now points to user_rip (offset 120)
            // [rsp] = user_rip (RCX for sysret)
            // [rsp+8] = user_rsp (New RSP)
            // [rsp+16] = user_rflags (R11 for sysret)
            
            "mov rcx, [rsp]",      // Load User RIP
            "mov r11, [rsp + 16]", // Load User RFLAGS
            "mov rsp, [rsp + 8]",  // Load User RSP (Switches stack! Kernel stack lost!)
            
            "swapgs",              // Switch to user GS
            "sysretq",             // Jump to user mode
            
            tf = in(reg) tf,
            options(noreturn)
        );
    }
}
