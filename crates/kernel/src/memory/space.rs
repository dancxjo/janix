use crate::memory::map::{MapPerms, MapResult};
use crate::machine::AddressSpace as ArchAddressSpace;
use abi::ids::ThingId;

pub struct AddressSpace {
    pub id: ThingId,
    pub arch: ArchAddressSpace,
}

impl AddressSpace {
    pub fn new() -> MapResult<Self> {
        Ok(Self {
            id: ThingId(0), // TODO: Create in Graph
            arch: ArchAddressSpace::new(),
        })
    }

    pub fn activate(&self) {
        self.arch.activate();
    }

    pub fn map(&mut self, virt: u64, phys: u64, len: usize, perms: MapPerms) -> MapResult<()> {
        self.arch.map(virt, phys, len, perms)
    }
}
