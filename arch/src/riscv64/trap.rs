use core::arch::global_asm;

global_asm!(include_str!("trap.S"));

#[unsafe(no_mangle)]
pub extern "C" fn trap_handler(tf: &TrapFrame, scause: u64, stval: u64, sepc: u64, sstatus: u64) {
    kernel_core::println!("EXCEPTION: RISC-V Trap");
    kernel_core::println!(
        "scause: {:#x}, stval: {:#x}, sepc: {:#x}, sstatus: {:#x}",
        scause,
        stval,
        sepc,
        sstatus
    );
    kernel_core::println!("{:#?}", tf);
    loop {}
}

#[repr(C)]
#[derive(Debug)]
pub struct TrapFrame {
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
    pub x31: u64,
}

pub fn init() {
    unsafe extern "C" {
        static trap_vector: u8;
    }
    unsafe {
        core::arch::asm!(
            "csrw stvec, {}",
            in(reg) &trap_vector,
        );
    }
}
