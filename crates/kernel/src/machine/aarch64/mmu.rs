use crate::memory::map::{MapPerms, MapResult};
use core::arch::asm;

#[derive(Clone, Copy)]
pub struct AddressSpace {
    pub ttbr0: u64,
}

impl AddressSpace {
    pub fn new() -> Self {
        // TODO: Alloc page table
        Self { ttbr0: 0 } 
    }

    pub fn activate(&self) {
        unsafe {
            asm!("msr ttbr0_el1, {}", in(reg) self.ttbr0);
            asm!("isb");
            asm!("tlbi vmalle1");
            asm!("dsb sy");
            asm!("isb");
        }
    }

    pub fn map(&mut self, _virt: u64, _phys: u64, _len: usize, _perms: MapPerms) -> MapResult<()> {
        Ok(())
    }
}
