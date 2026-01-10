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

    // Pushed by ISR stub
    pub trap_num: u64,
    pub error_code: u64,

    // Pushed by CPU
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

impl TrapFrame {
    pub fn new_user(entry: usize, stack_top: usize) -> Self {
        Self {
            rip: entry as u64,
            cs: 0x23, // User Code (Ring 3) | RPL 3
            rflags: 0x202, // IF | Reserved
            rsp: stack_top as u64,
            ss: 0x1B, // User Data (Ring 3) | RPL 3
            ..Default::default()
        }
    }
}
