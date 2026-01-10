#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct TrapFrame {
    pub regs: [u64; 32], // x0..x31
    pub sstatus: u64,
    pub sepc: u64,
    pub stval: u64,
    pub scause: u64,
}

impl TrapFrame {
    pub fn new_user(entry: u64, stack: u64) -> Self {
        let mut tf = Self::default();
        tf.set_user_entry(entry, stack);
        tf
    }

    pub fn set_user_entry(&mut self, entry: u64, stack: u64) {
        self.sepc = entry;
        self.regs[2] = stack; // x2 is SP
        // sstatus: SUM (bit 18)=1, SPP (bit 8)=0 (User), SPIE (bit 5)=1
        self.sstatus = (1 << 18) | (1 << 5);
    }
}
