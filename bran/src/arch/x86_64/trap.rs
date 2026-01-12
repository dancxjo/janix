#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct UserTrapFrame {
    // General purpose registers pushed by `pushall` or similar
    // ABI order usually depends on push implementation.
    // Let's assume standard push order (last pushed is top of stack/first in struct).
    // If we push: rax, rbx, rcx, rdx, rsi, rdi, rbp, r8-r15
    // Then struct order is r15 first.
    pub r15: usize,
    pub r14: usize,
    pub r13: usize,
    pub r12: usize,
    pub r11: usize,
    pub r10: usize,
    pub r9: usize,
    pub r8: usize,
    pub rbp: usize,
    pub rdi: usize, // System V: RDI is arg0
    pub rsi: usize,
    pub rdx: usize,
    pub rcx: usize,
    pub rbx: usize,
    pub rax: usize,

    // Exception info
    pub error_code: usize,
    pub int_no: usize, // Often useful to distinguish trap type

    // Hardware frame via IRET
    pub rip: usize,
    pub cs: usize,
    pub rflags: usize,
    pub rsp: usize,
    pub ss: usize,
}
