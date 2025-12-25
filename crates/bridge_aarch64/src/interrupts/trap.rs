use core::arch::global_asm;
extern crate alloc;

global_asm!(include_str!("trap.S"));

#[repr(C)]
#[derive(Debug)]
pub struct TrapFrame {
    pub x0: u64,
    pub x1: u64,
    pub x2: u64,
    pub x3: u64,
    pub x4: u64,
    pub x5: u64,
    pub x6: u64,
    pub x7: u64,
    pub x8: u64,
    pub x9: u64,
    pub x10: u64,
    pub x11: u64,
    pub x12: u64,
    pub x13: u64,
    pub x14: u64,
    pub x15: u64,
    pub x16: u64,
    pub x17: u64,
    pub x18: u64,
    pub x19: u64,
    pub x20: u64,
    pub x21: u64,
    pub x22: u64,
    pub x23: u64,
    pub x24: u64,
    pub x25: u64,
    pub x26: u64,
    pub x27: u64,
    pub x28: u64,
    pub x29: u64,
    pub x30: u64,
    pub sp_el0: u64,
    pub elr: u64,
    pub spsr: u64,
}

#[unsafe(no_mangle)]
pub extern "C" fn syscall_handler_rust(_tf: &mut TrapFrame) -> u64 {
    // Stub
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn invalid_exception(_tf: &TrapFrame, _kind: usize, _source: usize) {
    // Stub
    loop {
        core::hint::spin_loop();
    }
}
