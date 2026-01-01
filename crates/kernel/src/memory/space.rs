use crate::memory::map::{MapPerms, MapResult};
use crate::machine::AddressSpace as ArchAddressSpace;
use abi::ids::ThingId;
use graph::store;
use graph::symbols::sym;
use crate::memory::bytespace::Bytespace;

use spin::Mutex;

pub struct AddressSpace {
    pub id: ThingId,
    pub arch: Mutex<ArchAddressSpace>,
}

impl AddressSpace {
    pub fn new() -> MapResult<Self> {
        let id = store::with_store(|s| {
            let t = s.create_thing(sym::KIND_ADDRESS_SPACE).expect("create AS");
            // Link to something?
            t
        });
        Ok(Self {
            id,
            arch: Mutex::new(ArchAddressSpace::new()),
        })
    }

    pub fn activate(&self) {
        self.arch.lock().activate();
    }

    pub fn map(&self, virt: u64, phys: u64, len: usize, perms: MapPerms) -> MapResult<()> {
        self.arch.lock().map(virt, phys, len, perms)
    }

    pub fn map_bytespace_shared(&self, virt: u64, backing: &Bytespace, offset: u64, len: usize, perms: MapPerms) -> MapResult<()> {
         // 1. Get phys
         let ptr = backing.backing_ptr().ok_or(crate::memory::map::MapError::InvalidAddress)?;
         
         // 2. Resolve Physical Address
         let final_phys = if let Some(base) = backing.phys_base {
             base + offset
         } else {
             // Resolve from backing_ptr (Virtual RAM)
             let vaddr = ptr as u64 + offset;
             crate::machine::machine().virt_to_phys(vaddr)
         };

        self.map(virt, final_phys, len, perms)?;

        // Metadata
        store::with_store(|s| {
             let m = s.create_thing(sym::KIND_MAPPING).unwrap();
             let _ = s.create_relationship(sym::PRED_MAPS, self.id, m);
             let _ = s.create_relationship(sym::PRED_BACKS, m, backing.id);
        });
        Ok(())
    }
}
