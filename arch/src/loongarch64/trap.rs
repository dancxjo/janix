use core::arch::global_asm;

global_asm!(include_str!("trap.S"));

#[unsafe(no_mangle)]
pub extern "C" fn trap_handler(tf: &TrapFrame, estat: u64, era: u64, badv: u64) {
    kernel::println!("EXCEPTIO: Decipula LoongArch64");
    kernel::println!("estat: {:#x}, era: {:#x}, badv: {:#x}", estat, era, badv);
    kernel::println!("{:#?}", tf);
    loop {}
}

#[repr(C)]
#[derive(Debug)]
pub struct TrapFrame {
    pub r1: u64,
    pub r2: u64,
    pub r3: u64,
    pub r4: u64,
    pub r5: u64,
    pub r6: u64,
    pub r7: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub r16: u64,
    pub r17: u64,
    pub r18: u64,
    pub r19: u64,
    pub r20: u64,
    pub r21: u64,
    pub r22: u64,
    pub r23: u64,
    pub r24: u64,
    pub r25: u64,
    pub r26: u64,
    pub r27: u64,
    pub r28: u64,
    pub r29: u64,
    pub r30: u64,
    pub r31: u64,
}

pub fn init() {
    unsafe extern "C" {
        static trap_vector: u8;
    }
    unsafe {
        core::arch::asm!(
            "csrwr {}, 0xc", // EENTRY = 0xc
            in(reg) &trap_vector,
        );
    }
}
