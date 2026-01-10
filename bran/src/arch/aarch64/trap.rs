use core::any::Any;
use kernel::boot::ArchTrapFrame;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TrapFrame {
    // 0..240
    pub x: [u64; 30],
    // 240
    pub x30: u64,
    // 248
    pub elr: u64,
    // 256
    pub spsr: u64,
    // 264
    pub sp_el0: u64,
}

// 272 bytes total
const _: () = assert!(core::mem::size_of::<TrapFrame>() == 272);

impl Default for TrapFrame {
    fn default() -> Self {
        Self {
            x: [0; 30],
            x30: 0,
            elr: 0,
            spsr: 0,
            sp_el0: 0,
        }
    }
}

impl ArchTrapFrame for TrapFrame {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }

    fn syscall_arg(&self, idx: usize) -> u64 {
        // x0..x7 are args
        if idx < 8 {
            if idx < 30 {
               self.x[idx]
            } else {
               0 // Should not happen given logic, but safety checks
            }
        } else {
            0 
        }
    }

    fn syscall_ret(&mut self, val: u64) {
        self.x[0] = val;
    }

    fn syscall_num(&self) -> u64 {
        // x8 is syscall number in Linux/ThingOS AArch64
        self.x[8]
    }

    fn set_user_stack(&mut self, stack: u64) {
        self.sp_el0 = stack;
    }

    fn user_stack(&self) -> u64 {
        self.sp_el0
    }

    fn set_user_ip(&mut self, ip: u64) {
        self.elr = ip;
    }

    fn user_ip(&self) -> u64 {
        self.elr
    }
}
