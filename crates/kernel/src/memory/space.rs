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

    /// Record a user-visible mapping, coalescing overlapping or adjacent regions.
    ///
    /// Overlaps are normalized so that the recorded permissions are authoritative
    /// for the range being inserted.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// ```rust,ignore
    /// use kernel::memory::map::MapPerms;
    /// use kernel::memory::space::UserMappings;
    ///
    /// let mappings = UserMappings::new();
    /// mappings.record(0x1000, 0x1000, MapPerms::READ | MapPerms::USER);
    /// mappings.record(0x2000, 0x1000, MapPerms::READ | MapPerms::USER);
    ///
    /// assert!(mappings.permits(0x1000, 0x2000, MapPerms::READ | MapPerms::USER));
    /// ```
    pub fn record(&self, start: u64, len: usize, perms: MapPerms) {
        if len == 0 {
            return;
        }
        let Some(end) = start.checked_add(len as u64) else {
            return;
        };
        let mut entries = self.entries.lock();
        entries.sort_by_key(|entry| entry.start);

        let mut updated = Vec::new();
        let mut trailing = Vec::new();

        for entry in entries.iter() {
            if entry.end <= start {
                updated.push(*entry);
                continue;
            }

            if entry.start >= end {
                trailing.push(*entry);
                continue;
            }

            if entry.start < start {
                updated.push(UserMapping {
                    start: entry.start,
                    end: start,
                    perms: entry.perms,
                });
            }

            if entry.end > end {
                trailing.push(UserMapping {
                    start: end,
                    end: entry.end,
                    perms: entry.perms,
                });
            }
        }

        updated.push(UserMapping { start, end, perms });
        updated.extend(trailing);
        Self::normalize_entries(&mut updated);
        *entries = updated;
    }

    /// Remove a previously recorded user mapping range.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kernel::memory::map::MapPerms;
    /// use kernel::memory::space::UserMappings;
    ///
    /// let mappings = UserMappings::new();
    /// mappings.record(0x1000, 0x2000, MapPerms::READ | MapPerms::USER);
    /// mappings.unrecord(0x1800, 0x800);
    ///
    /// assert!(!mappings.permits(0x1800, 0x800, MapPerms::READ | MapPerms::USER));
    /// ```
    pub fn unrecord(&self, start: u64, len: usize) {
        if len == 0 {
            return;
        }
        let Some(end) = start.checked_add(len as u64) else {
            return;
        };

        let mut entries = self.entries.lock();
        entries.sort_by_key(|entry| entry.start);

        let mut updated = Vec::new();
        for entry in entries.iter() {
            if entry.end <= start || entry.start >= end {
                updated.push(*entry);
                continue;
            }

            if entry.start < start {
                updated.push(UserMapping {
                    start: entry.start,
                    end: start,
                    perms: entry.perms,
                });
            }

            if entry.end > end {
                updated.push(UserMapping {
                    start: end,
                    end: entry.end,
                    perms: entry.perms,
                });
            }
        }

        Self::normalize_entries(&mut updated);
        *entries = updated;
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
        for entry in entries.iter() {
            if entry.end <= cursor {
                continue;
            }
            if entry.start > cursor {
                return false;
            }
            if !entry.perms.contains(required) {
                return false;
            }
            cursor = core::cmp::min(end, entry.end);
            if cursor >= end {
                return true;
            }
        }
        false
    }

    fn normalize_entries(entries: &mut Vec<UserMapping>) {
        entries.sort_by_key(|entry| entry.start);
        let mut normalized: Vec<UserMapping> = Vec::with_capacity(entries.len());
        for entry in entries.drain(..) {
            if entry.start >= entry.end {
                continue;
            }
            if let Some(last) = normalized.last_mut() {
                if last.perms == entry.perms && last.end >= entry.start {
                    last.end = core::cmp::max(last.end, entry.end);
                    continue;
                }
                debug_assert!(last.end <= entry.start);
            }
            normalized.push(entry);
        }
        *entries = normalized;
    }

    #[cfg(test)]
    fn entries_len(&self) -> usize {
        self.entries.lock().len()
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

    pub fn unmap(&self, virt: u64, len: usize) -> MapResult<()> {
        if len == 0 {
            return Ok(());
        }
        self.arch.lock().unmap(virt, len)?;
        journal::emit_unmap(self.id, virt, len);

        let user_end = self.user_range_end();
        let Some(end) = virt.checked_add(len as u64) else {
            return Err(crate::memory::map::MapError::InvalidAddress);
        };
        if virt < user_end {
            let user_end = core::cmp::min(end, user_end);
            let user_len = user_end.saturating_sub(virt) as usize;
            self.user_mappings.unrecord(virt, user_len);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use abi::syscall::err;

    #[test]
    fn record_coalesces_adjacent_regions() {
        let mappings = UserMappings::new();
        mappings.record(0x1000, 0x1000, MapPerms::READ | MapPerms::USER);
        mappings.record(0x2000, 0x1000, MapPerms::READ | MapPerms::USER);

        assert_eq!(mappings.entries_len(), 1);
        assert!(mappings.permits(0x1000, 0x2000, MapPerms::READ | MapPerms::USER));
    }

    #[test]
    fn record_overlapping_regions_split_by_perms() {
        let mappings = UserMappings::new();
        mappings.record(0x0, 0x3000, MapPerms::READ | MapPerms::USER);
        mappings.record(0x1000, 0x1000, MapPerms::WRITE | MapPerms::USER);

        assert!(mappings.permits(0x0, 0x1000, MapPerms::READ | MapPerms::USER));
        assert!(mappings.permits(0x1000, 0x1000, MapPerms::WRITE | MapPerms::USER));
        assert!(!mappings.permits(0x1000, 0x1000, MapPerms::READ | MapPerms::USER));
        assert!(!mappings.permits(0x0, 0x3000, MapPerms::READ | MapPerms::USER));
    }

    #[test]
    fn unrecord_removes_coverage() {
        let mappings = UserMappings::new();
        mappings.record(0x0, 0x3000, MapPerms::READ | MapPerms::USER);
        mappings.unrecord(0x1000, 0x1000);

        assert!(mappings.permits(0x0, 0x1000, MapPerms::READ | MapPerms::USER));
        assert!(!mappings.permits(0x1000, 0x1000, MapPerms::READ | MapPerms::USER));
        assert!(mappings.permits(0x2000, 0x1000, MapPerms::READ | MapPerms::USER));
        assert!(!mappings.permits(0x0, 0x3000, MapPerms::READ | MapPerms::USER));
    }

    #[test]
    fn permits_spans_multiple_intervals() {
        let mappings = UserMappings::new();
        mappings.record(0x0, 0x1000, MapPerms::READ | MapPerms::USER);
        mappings.record(0x1000, 0x1000, MapPerms::READ | MapPerms::USER);
        mappings.record(0x2000, 0x1000, MapPerms::READ | MapPerms::USER);

        assert!(mappings.permits(0x0, 0x3000, MapPerms::READ | MapPerms::USER));
    }
}
