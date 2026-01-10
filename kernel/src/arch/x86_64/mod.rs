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

    pub user_rip: u64,
    pub user_rsp: u64,
    pub user_rflags: u64,
}

impl TrapFrame {
    pub fn new_user(entry: u64, stack: u64) -> Self {
        let mut tf = Self::default();
        tf.set_user_entry(entry, stack);
        tf
    }

    pub fn set_user_entry(&mut self, entry: u64, stack: u64) {
        self.user_rip = entry;
        self.user_rsp = stack;
        self.user_rflags = 0x202; // IF | Reserved
    }
}
