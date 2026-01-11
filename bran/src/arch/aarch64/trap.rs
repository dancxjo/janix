#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct UserTrapFrame {
    // x0-x30
    pub regs: [u64; 31],
    
    // Exception context
    pub sp_el0: u64,
    pub elr_el1: u64,
    pub spsr_el1: u64,
    
    // Padding to 16-byte align if needed, but [u64; 34] is 272 bytes (16-aligned).
    // 31 + 3 = 34. 34 * 8 = 272. 272 % 16 == 0. Good.
}
