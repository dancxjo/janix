#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TrapFrame {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub rbp: u64,
    pub rbx: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rax: u64,
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

// Stub for now. Logic needs kernel_core::sched access (Mutex) which we need to expose.
// Stub for now. Logic needs kernel_core::sched access (Mutex) which we need to expose.
pub fn timer_tick(frame: &mut TrapFrame) {
    unsafe {
        if let Some(hook) = crate::TICK_HOOK {
            hook(frame);
        }
    }
}
