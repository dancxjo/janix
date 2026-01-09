//! TrapFrame for x86_64
//!
//! Represents the state saved on the kernel stack during a syscall or exception.

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct TrapFrame {
    // General purpose registers
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rbx: u64,
    pub rax: u64,

    // User-mode "return" state (SYSRETQ expects these in specific regs/order if using stack)
    // But for our manual SYSRETQ, we pop these off and load them into regs manually.
    //
    // On syscall entry:
    // RCX = user RIP
    // R11 = user RFLAGS
    // RSP = user RSP (we save this manually)
    
    pub user_rip: u64,
    pub user_rsp: u64,
    pub user_rflags: u64,
}
