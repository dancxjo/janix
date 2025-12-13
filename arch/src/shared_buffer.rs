use abi::MapFlags;

/// Helper for architecture-specific frame allocation.
pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<u64>;
}

/// Trait describing something that can provide a physical address for mapping.
pub trait FrameDescriptor {
    fn start_address(&self) -> u64;
}

/// Map a slice of physical frames into the current address space at `vaddr`.
pub fn map_frames<F, T>(
    vaddr: u64,
    frames: &[T],
    flags: MapFlags,
    allocator: &mut F,
    hhdm_offset: u64,
) -> Result<(), &'static str>
where
    F: FrameAllocator,
    T: FrameDescriptor,
{
    #[cfg(target_arch = "x86_64")]
    {
        return x86_64::map_frames(vaddr, frames, flags, allocator, hhdm_offset);
    }

    #[cfg(target_arch = "aarch64")]
    {
        return aarch64::map_frames(vaddr, frames, flags, allocator, hhdm_offset);
    }

    #[allow(unreachable_code)]
    Err("SharedBuffer mapping not implemented for this architecture")
}

#[cfg(target_arch = "x86_64")]
mod x86_64 {
    use super::{FrameAllocator, FrameDescriptor};
    use abi::MapFlags;
    use x86_64::registers::control::Cr3;
    use x86_64::structures::paging::mapper::MapToError;
    use x86_64::structures::paging::{
        FrameAllocator as X86FrameAllocator, Mapper, OffsetPageTable, Page, PageTable,
        PageTableFlags, PhysFrame, Size4KiB,
    };
    use x86_64::{PhysAddr, VirtAddr};

    const PAGE_SIZE: u64 = 4096;

    struct TableFrameAllocator<'a, F> {
        allocator: &'a mut F,
        hhdm_offset: u64,
    }

    unsafe impl<'a, F: FrameAllocator> X86FrameAllocator<Size4KiB> for TableFrameAllocator<'a, F> {
        fn allocate_frame(&mut self) -> Option<PhysFrame> {
            let addr = self.allocator.allocate_frame()?;
            unsafe {
                core::ptr::write_bytes((addr + self.hhdm_offset) as *mut u8, 0, PAGE_SIZE as usize);
            }
            let phys = PhysAddr::new(addr);
            PhysFrame::from_start_address(phys).ok()
        }
    }

    pub fn map_frames<F, T>(
        vaddr: u64,
        frames: &[T],
        flags: MapFlags,
        allocator: &mut F,
        hhdm_offset: u64,
    ) -> Result<(), &'static str>
    where
        F: FrameAllocator,
        T: FrameDescriptor,
    {
        let current_cr3 = Cr3::read();
        let l4_phys = current_cr3.0.start_address().as_u64();
        let l4_ptr = (l4_phys + hhdm_offset) as *mut PageTable;
        let l4_table = unsafe { &mut *l4_ptr };
        let mut mapper = unsafe { OffsetPageTable::new(l4_table, VirtAddr::new(hhdm_offset)) };
        let mut table_alloc = TableFrameAllocator {
            allocator,
            hhdm_offset,
        };

        let mut page_addr = vaddr;
        for frame in frames {
            let phys = frame.start_address();
            if phys % PAGE_SIZE != 0 {
                return Err("SharedBuffer frame is not page aligned");
            }

            let page = Page::<Size4KiB>::containing_address(VirtAddr::new(page_addr));
            let phys_frame = PhysFrame::from_start_address(
                PhysAddr::try_new(phys).map_err(|_| "Invalid frame address")?,
            )
            .map_err(|_| "Invalid frame address")?;
            let mut page_flags = PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE;
            if flags.contains(MapFlags::WRITE) {
                page_flags |= PageTableFlags::WRITABLE;
            }
            if !flags.contains(MapFlags::EXECUTE) {
                page_flags |= PageTableFlags::NO_EXECUTE;
            }

            unsafe {
                match mapper.map_to(page, phys_frame, page_flags, &mut table_alloc) {
                    Ok(mapping) => mapping.flush(),
                    Err(MapToError::PageAlreadyMapped(_)) => {
                        if let Ok(flush) = mapper.update_flags(page, page_flags) {
                            flush.flush();
                        }
                    }
                    Err(_) => return Err("Failed to map shared buffer frame"),
                }
            }

            page_addr = page_addr.saturating_add(PAGE_SIZE);
        }

        Ok(())
    }
}

#[cfg(target_arch = "aarch64")]
mod aarch64 {
    use super::{FrameAllocator, FrameDescriptor};
    use abi::MapFlags;
    use core::ptr;

    const PAGE_SIZE: u64 = 4096;
    const DESC_VALID: u64 = 1 << 0;
    const DESC_TABLE_OR_PAGE: u64 = 1 << 1;
    const ATTR_INDEX0: u64 = 0;
    const AP_RW_EL0: u64 = 0b01 << 6;
    const AP_RO_EL0: u64 = 0b11 << 6;
    const SH_INNER: u64 = 0b11 << 8;
    const AF: u64 = 1 << 10;
    const PXN: u64 = 1 << 53;
    const UXN: u64 = 1 << 54;
    const ADDR_MASK: u64 = 0x0000_FFFF_FFFF_F000;

    #[repr(C, align(4096))]
    struct PageTable {
        entries: [u64; 512],
    }

    pub fn map_frames<F, T>(
        vaddr: u64,
        frames: &[T],
        flags: MapFlags,
        allocator: &mut F,
        hhdm_offset: u64,
    ) -> Result<(), &'static str>
    where
        F: FrameAllocator,
        T: FrameDescriptor,
    {
        if vaddr & (PAGE_SIZE - 1) != 0 {
            return Err("SharedBuffer base must be page aligned");
        }

        let ttbr0 = current_ttbr0_phys();
        if ttbr0 == 0 {
            return Err("TTBR0_EL1 unavailable for mapping");
        }

        let writable = flags.contains(MapFlags::WRITE);
        let executable = flags.contains(MapFlags::EXECUTE);
        let mut addr = vaddr;
        for frame in frames {
            let phys = frame.start_address();
            map_page(
                ttbr0,
                hhdm_offset,
                addr,
                phys,
                writable,
                executable,
                allocator,
            )?;
            addr = addr.saturating_add(PAGE_SIZE);
        }

        unsafe {
            core::arch::asm!("dsb ishst", "isb", options(nostack, preserves_flags));
        }

        Ok(())
    }

    fn map_page<F>(
        l0_phys: u64,
        hhdm: u64,
        virt: u64,
        phys: u64,
        writable: bool,
        executable: bool,
        allocator: &mut F,
    ) -> Result<(), &'static str>
    where
        F: FrameAllocator,
    {
        if phys & (PAGE_SIZE - 1) != 0 {
            return Err("SharedBuffer frame is not page aligned");
        }

        let l0_table = table_mut(l0_phys, hhdm);
        let l0_idx = ((virt >> 39) & 0x1ff) as usize;
        let l1_phys = ensure_table(l0_table, l0_idx, hhdm, allocator)?;
        let l1_table = table_mut(l1_phys, hhdm);

        let l1_idx = ((virt >> 30) & 0x1ff) as usize;
        let l2_phys = ensure_table(l1_table, l1_idx, hhdm, allocator)?;
        let l2_table = table_mut(l2_phys, hhdm);

        let l2_idx = ((virt >> 21) & 0x1ff) as usize;
        let l3_phys = ensure_table(l2_table, l2_idx, hhdm, allocator)?;
        let l3_table = table_mut(l3_phys, hhdm);

        let l3_idx = ((virt >> 12) & 0x1ff) as usize;
        if l3_table.entries[l3_idx] & DESC_VALID != 0 {
            return Err("SharedBuffer VA already mapped");
        }

        let mut desc = phys | DESC_VALID | DESC_TABLE_OR_PAGE | ATTR_INDEX0 | SH_INNER | AF | PXN;
        desc |= if writable { AP_RW_EL0 } else { AP_RO_EL0 };
        if !executable {
            desc |= UXN;
        }
        l3_table.entries[l3_idx] = desc;
        Ok(())
    }

    fn ensure_table<F>(
        parent: &mut PageTable,
        idx: usize,
        hhdm: u64,
        allocator: &mut F,
    ) -> Result<u64, &'static str>
    where
        F: FrameAllocator,
    {
        let entry = parent.entries[idx];
        if entry & DESC_VALID == 0 {
            let frame = allocator
                .allocate_frame()
                .ok_or("Out of frames for page table")?;
            zero_frame(frame, hhdm);
            parent.entries[idx] = frame | DESC_VALID | DESC_TABLE_OR_PAGE;
            Ok(frame)
        } else if entry & DESC_TABLE_OR_PAGE != 0 {
            Ok(entry & ADDR_MASK)
        } else {
            Err("Encountered block entry while walking page tables")
        }
    }

    fn table_mut(phys: u64, hhdm: u64) -> &'static mut PageTable {
        let ptr = (phys + hhdm) as *mut PageTable;
        unsafe { &mut *ptr }
    }

    fn zero_frame(phys: u64, hhdm: u64) {
        unsafe {
            ptr::write_bytes((phys + hhdm) as *mut u8, 0, PAGE_SIZE as usize);
        }
    }

    fn current_ttbr0_phys() -> u64 {
        let ttbr: u64;
        unsafe {
            core::arch::asm!("mrs {val}, ttbr0_el1", val = out(reg) ttbr);
        }
        ttbr & ADDR_MASK
    }
}
