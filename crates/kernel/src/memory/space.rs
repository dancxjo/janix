use crate::machine::AddressSpace as ArchAddressSpace;
use crate::memory::bytespace::Bytespace;
use crate::memory::id::next_thing_id;
use crate::memory::journal;
use crate::memory::map::{MapPerms, MapResult};
use abi::ids::ThingId;

use spin::Mutex;

pub struct AddressSpace {
    pub id: ThingId,
    pub arch: Mutex<ArchAddressSpace>,
}

impl AddressSpace {
    pub fn new() -> MapResult<Self> {
        let id = next_thing_id();
        Ok(Self {
            id,
            arch: Mutex::new(ArchAddressSpace::new()?),
        })
    }

    pub fn new_kernel_share() -> MapResult<Self> {
        let id = next_thing_id();

        #[cfg(target_arch = "x86_64")]
        let arch = {
            use x86_64::registers::control::Cr3;
            let (frame, _) = Cr3::read();
            ArchAddressSpace::from_existing(frame.start_address().as_u64())
        };

        #[cfg(target_arch = "aarch64")]
        let arch = ArchAddressSpace::new()?; // Fallback for aarch64 pending impl

        #[cfg(any(target_arch = "riscv64", target_arch = "loongarch64"))]
        let arch = ArchAddressSpace::new()?;

        Ok(Self {
            id,
            arch: Mutex::new(arch),
        })
    }

    pub fn activate(&self) {
        self.arch.lock().activate();
    }

    pub fn map(&self, virt: u64, phys: u64, len: usize, perms: MapPerms) -> MapResult<()> {
        self.arch.lock().map(virt, phys, len, perms)?;
        journal::emit_map(self.id, virt, phys, len, perms);
        Ok(())
    }

    pub fn map_bytespace_shared(
        &self,
        virt: u64,
        backing: &Bytespace,
        offset: u64,
        len: usize,
        perms: MapPerms,
    ) -> MapResult<()> {
        // 1. Get phys
        let ptr = backing
            .backing_ptr()
            .ok_or(crate::memory::map::MapError::InvalidAddress)?;

        // 2. Resolve Physical Address
        let final_phys = if let Some(base) = backing.phys_base {
            base + offset
        } else {
            // Resolve from backing_ptr (Virtual RAM)
            let vaddr = ptr as u64 + offset;
            crate::machine::machine().virt_to_phys(vaddr)
        };

        self.map(virt, final_phys, len, perms)?;
        Ok(())
    }
}
