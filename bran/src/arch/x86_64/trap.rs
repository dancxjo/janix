use kernel::boot::{ArchContext, ArchTrapFrame};
use core::any::Any;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TrapFrame {
    // General purpose registers pushed by `pushall` (reverse order of push)
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

    // Exception info
    pub trap_num: u64,
    pub error_code: u64,

    // Hardware checks (pushed by CPU or stub)
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

impl TrapFrame {
    pub fn new_user(rip: u64, rsp: u64) -> Self {
        let mut tf = Self::default();
        tf.rip = rip;
        tf.rsp = rsp;
        
        // Setup segment selectors for user mode.
        // CPL=3.
        tf.cs = super::gdt::USER_CODE64_SEL as u64;
        tf.ss = super::gdt::USER_DATA_SEL as u64;
        
        // RFLAGS: IF enabled (bit 9), Reserved (bit 1) always 1.
        tf.rflags = 0x202;
        tf
    }
}

impl ArchTrapFrame for TrapFrame {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }

    fn syscall_arg(&self, idx: usize) -> u64 {
        match idx {
            0 => self.rdi,
            1 => self.rsi,
            2 => self.rdx,
            3 => self.r10, // System V: RCX used for RIP, R10 for Arg4
            4 => self.r8,
            5 => self.r9,
            _ => 0,
        }
    }
    
    fn syscall_ret(&mut self, val: u64) {
        self.rax = val;
    }
    
    fn syscall_num(&self) -> u64 {
        self.rax
    }

    fn set_user_stack(&mut self, stack: u64) {
        self.rsp = stack;
    }
    fn user_stack(&self) -> u64 {
        self.rsp
    }
    fn set_user_ip(&mut self, ip: u64) {
        self.rip = ip;
    }
    fn user_ip(&self) -> u64 {
        self.rip
    }
    
    fn trap_num(&self) -> usize {
        self.trap_num as usize
    }
    fn error_code(&self) -> usize {
        self.error_code as usize
    }
}

#[derive(Debug, Clone, Default)]
pub struct Context {
    pub tf: TrapFrame,
    // FPU/SIMD state buffer could go here or be separate
}

impl ArchContext for Context {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

#[unsafe(no_mangle)]
pub extern "C" fn exception_dispatch_trampoline(tf: &mut TrapFrame) {
    kernel::trap::kernel_trap_handler(tf);
    
    // Check preemption if returning to user?
    // Only if we want exception returns to be preemption points.
    // Timer is the main one.
    // Yield (0x81) goes through here too.
    
    let is_user = (tf.cs & 3) == 3;
    if is_user {
         kernel::task::check_preemption();
    }
}

