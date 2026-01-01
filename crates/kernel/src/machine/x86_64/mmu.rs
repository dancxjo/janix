use crate::memory::map::{MapPerms, MapResult, MapError};
use x86_64::registers::control::{Cr3, Cr3Flags};
use x86_64::{PhysAddr, VirtAddr};
use x86_64::structures::paging::{PageTable, PageTableFlags, PhysFrame, Size4KiB};

pub struct AddressSpace {
    pub pml4_table: u64, // Physical address of PML4
}

impl AddressSpace {
    pub fn new() -> Self {
        let (frame, _) = x86_64::registers::control::Cr3::read();
        Self { pml4_table: frame.start_address().as_u64() }
    }

    pub fn activate(&self) {
        let frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(self.pml4_table));
        unsafe {
            Cr3::write(frame, Cr3Flags::empty());
        }
    }

    pub fn map(&mut self, virt: u64, phys: u64, len: usize, perms: MapPerms) -> MapResult<()> {
        // TODO: Walk page table and map
        Ok(())
    }
}
