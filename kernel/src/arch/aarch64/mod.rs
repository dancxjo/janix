#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct TrapFrame {
    pub xs: [u64; 30], // x0..x29
    pub x30: u64,
    pub elr: u64,      // PC
    pub spsr: u64,     // PSTATE
    pub sp_el0: u64,   // User Stack Pointer
}

impl TrapFrame {
    pub fn new_user(entry: u64, stack: u64) -> Self {
        let mut tf = Self::default();
        tf.set_user_entry(entry, stack);
        tf
    }

    pub fn set_user_entry(&mut self, entry: u64, stack: u64) {
        self.elr = entry;
        self.sp_el0 = stack;
        self.spsr = 0; // EL0t, Interrupts unmasked? 
        // DAIF bits: 0 = unmasked.
        // Mode EL0t = 0x0000.
        // So 0 is fine?
    }
}
