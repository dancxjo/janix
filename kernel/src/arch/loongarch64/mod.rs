#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct TrapFrame {
    pub regs: [u64; 32], // $r0..$r31
    pub era: u64,        // PC
    pub prmd: u64,       // PSTATE / Flags (PLV, PIE)
    pub estat: u64,      // Exception Status
}

impl TrapFrame {
    pub fn new_user(entry: u64, stack: u64) -> Self {
        let mut tf = Self::default();
        tf.set_user_entry(entry, stack);
        tf
    }

    pub fn set_user_entry(&mut self, entry: u64, stack: u64) {
        self.era = entry;
        self.regs[3] = stack; // $r3 is SP
        // PRMD: PPLV (bits 0-1) = 3 (User), PIE (bit 2) = 1 (Interrupts Enable)
        self.prmd = 0x3 | 0x4; 
    }
}
