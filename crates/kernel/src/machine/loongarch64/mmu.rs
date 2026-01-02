use crate::memory::map::{MapPerms, MapResult};

#[derive(Clone, Copy)]
pub struct AddressSpace {
    pub pgd: u64,
}

impl AddressSpace {
    pub fn new() -> MapResult<Self> {
        Ok(Self { pgd: 0 })
    }

    pub fn activate(&self) {
        // Stub
    }

    pub fn map(&mut self, _virt: u64, _phys: u64, _len: usize, _perms: MapPerms) -> MapResult<()> {
        Ok(())
    }
}
