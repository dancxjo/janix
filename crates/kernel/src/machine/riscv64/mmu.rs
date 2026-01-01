use crate::memory::map::{MapPerms, MapResult};

#[derive(Clone, Copy)]
pub struct AddressSpace {
    pub satp: u64,
}

impl AddressSpace {
    pub fn new() -> Self {
        Self { satp: 0 } // Bare mode
    }

    pub fn activate(&self) {
        unsafe {
            core::arch::asm!("csrw satp, {}", in(reg) self.satp);
            core::arch::asm!("sfence.vma");
        }
    }

    pub fn map(&mut self, _virt: u64, _phys: u64, _len: usize, _perms: MapPerms) -> MapResult<()> {
        Ok(())
    }
}
