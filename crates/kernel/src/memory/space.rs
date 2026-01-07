use crate::machine::AddressSpace as ArchAddressSpace;
use crate::memory::bytespace::Bytespace;
use crate::memory::id::next_thing_id;
use crate::memory::journal;
use crate::memory::map::{MapPerms, MapResult};
use abi::ids::ThingId;
use alloc::vec::Vec;

use spin::Mutex;

#[derive(Clone, Copy, Debug)]
pub struct UserMapping {
    start: u64,
    end: u64,
    perms: MapPerms,
}

#[derive(Debug)]
pub struct UserMappings {
    entries: Mutex<Vec<UserMapping>>,
}

impl UserMappings {
    pub fn new() -> Self {
        Self {
            entries: Mutex::new(Vec::new()),
        }
    }

    pub fn record(&self, start: u64, len: usize, perms: MapPerms) {
        if len == 0 {
            return;
        }
        let Some(end) = start.checked_add(len as u64) else {
            return;
        };
        let mapping = UserMapping { start, end, perms };
        self.entries.lock().push(mapping);
    }

    pub fn permits(&self, start: u64, len: usize, required: MapPerms) -> bool {
        if len == 0 {
            return true;
        }
        let Some(end) = start.checked_add(len as u64) else {
            return false;
        };

        let entries = self.entries.lock();
        let mut cursor = start;
        while cursor < end {
            let mut covered = false;
            for entry in entries.iter() {
                if entry.start <= cursor && entry.end > cursor && entry.perms.contains(required) {
                    cursor = core::cmp::min(end, entry.end);
                    covered = true;
                    break;
                }
            }
            if !covered {
                return false;
            }
        }
        true
    }
}

pub struct AddressSpace {
    pub id: ThingId,
    pub arch: Mutex<ArchAddressSpace>,
    user_mappings: UserMappings,
}

impl AddressSpace {
    pub fn new() -> MapResult<Self> {
        let id = next_thing_id();
        Ok(Self {
            id,
            arch: Mutex::new(ArchAddressSpace::new()?),
            user_mappings: UserMappings::new(),
        })
    }

    pub fn new_kernel_share() -> MapResult<Self> {
        let id = next_thing_id();

        #[cfg(all(target_arch = "x86_64", not(test)))]
        let arch = {
            use x86_64::registers::control::Cr3;
            let (frame, _) = Cr3::read();
            ArchAddressSpace::from_existing(frame.start_address().as_u64())
        };

        #[cfg(test)]
        let arch = ArchAddressSpace::new()?;

        #[cfg(target_arch = "aarch64")]
        let arch = ArchAddressSpace::new()?; // Fallback for aarch64 pending impl

        #[cfg(any(target_arch = "riscv64", target_arch = "loongarch64"))]
        let arch = ArchAddressSpace::new()?;

        Ok(Self {
            id,
            arch: Mutex::new(arch),
            user_mappings: UserMappings::new(),
        })
    }

    pub fn activate(&self) {
        self.arch.lock().activate();
    }

    pub fn user_mappings(&self) -> &UserMappings {
        &self.user_mappings
    }

    pub fn user_range_end(&self) -> u64 {
        self.arch.lock().user_range_end()
    }

    pub fn probe_user_range(&self, start: u64, len: usize, perms: MapPerms) -> bool {
        self.arch.lock().probe_user_range(start, len, perms)
    }

    pub fn map(&self, virt: u64, phys: u64, len: usize, perms: MapPerms) -> MapResult<()> {
        self.arch.lock().map(virt, phys, len, perms)?;
        journal::emit_map(self.id, virt, phys, len, perms);
        if perms.contains(MapPerms::USER) {
            self.user_mappings.record(virt, len, perms);
        }
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
