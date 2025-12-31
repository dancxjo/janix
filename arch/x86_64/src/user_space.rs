use kernel::arch::user_space::{UserPageFlags, UserSpace};
use crate::bridge::{UserRoot, HHDM_OFFSET};
use core::sync::atomic::Ordering;
use x86_64::structures::paging::{
    mapper::{Mapper, TranslateError},
    FrameAllocator, OffsetPageTable, Page, PageTableFlags, PhysFrame, Size2MiB, Size4KiB,
    Translate,
};
use x86_64::{PhysAddr, VirtAddr};
use alloc::alloc::{alloc_zeroed, Layout};

pub struct X64UserSpace;

fn page_flags_from_user(flags: UserPageFlags) -> PageTableFlags {
    let mut out = PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE;
    if flags.writable {
        out |= PageTableFlags::WRITABLE;
    }
    if !flags.executable {
        out |= PageTableFlags::NO_EXECUTE;
    }
    if flags.device {
        out |= PageTableFlags::NO_CACHE | PageTableFlags::WRITE_THROUGH;
    }
    out
}

fn table_allocator(
    hhdm_offset: VirtAddr,
) -> impl FrameAllocator<Size4KiB> {
    struct HeapFrameAllocator {
        hhdm_offset: VirtAddr,
    }

    unsafe impl FrameAllocator<Size4KiB> for HeapFrameAllocator {
        fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
            let layout = unsafe { Layout::from_size_align_unchecked(4096, 4096) };
            let ptr = unsafe { alloc_zeroed(layout) };
            if ptr.is_null() {
                return None;
            }

            // We need to translate the virtual address of the allocated page to physical.
            // Since we are in the kernel, we can use the kernel's page table (CR3).
            // Or simplified: subtract HHDM? No, heap is not necessarily HHDM backed in all allocators,
            // but in this system it seems kernel heap is likely HHDM or we can translate.
            // The existing bridge code reads CR3.

            use x86_64::registers::control::Cr3;
            let (l4_frame, _) = Cr3::read();
            let virt_l4 = self.hhdm_offset + l4_frame.start_address().as_u64();
            let pml4_ptr: *mut x86_64::structures::paging::PageTable = virt_l4.as_mut_ptr();
            let mut mapper =
                unsafe { OffsetPageTable::new(&mut *pml4_ptr, self.hhdm_offset) };

            mapper
                .translate_addr(VirtAddr::new(ptr as u64))
                .map(|p| PhysFrame::containing_address(p))
        }
    }

    HeapFrameAllocator { hhdm_offset }
}

fn mapper_from_root<'a>(
    root: &'a mut UserRoot,
) -> OffsetPageTable<'a> {
    unsafe { OffsetPageTable::new(&mut *root.pml4, root.hhdm_offset) }
}

impl UserSpace for X64UserSpace {
    type Root = UserRoot;

    unsafe fn create_root() -> Self::Root {
        use x86_64::registers::control::Cr3;
        let (frame, _) = Cr3::read();
        let hhdm_offset = VirtAddr::new(HHDM_OFFSET.load(Ordering::Relaxed));
        let virt_l4 = hhdm_offset + frame.start_address().as_u64();
        let pml4 = virt_l4.as_mut_ptr();

        UserRoot {
            pml4,
            hhdm_offset,
            cr3_frame: frame,
        }
    }

    unsafe fn activate_root(root: &Self::Root) {
        use x86_64::registers::control::{Cr3, Cr3Flags};
        Cr3::write(root.cr3_frame, Cr3Flags::empty());
    }

    unsafe fn alloc_frame_4k() -> u64 {
        let layout = Layout::from_size_align_unchecked(4096, 4096);
        let ptr = alloc_zeroed(layout);
        if ptr.is_null() {
            return 0;
        }

        let hhdm_offset = VirtAddr::new(HHDM_OFFSET.load(Ordering::Relaxed));

        // Translate to physical
        use x86_64::registers::control::Cr3;
        let (l4_frame, _) = Cr3::read();
        let virt_l4 = hhdm_offset + l4_frame.start_address().as_u64();
        let pml4_ptr: *mut x86_64::structures::paging::PageTable = virt_l4.as_mut_ptr();
        let mut mapper = OffsetPageTable::new(&mut *pml4_ptr, hhdm_offset);

        mapper
            .translate_addr(VirtAddr::new(ptr as u64))
            .map(|p| PhysFrame::<Size4KiB>::containing_address(p).start_address().as_u64())
            .unwrap_or(0)
    }

    unsafe fn map_4k(
        root: &mut Self::Root,
        vaddr: u64,
        paddr: u64,
        flags: UserPageFlags,
    ) {
        let hhdm = root.hhdm_offset;
        let mut mapper = mapper_from_root(root);
        let page = Page::<Size4KiB>::containing_address(VirtAddr::new(vaddr));
        let phys_frame = PhysFrame::containing_address(PhysAddr::new(paddr));
        let map_flags = page_flags_from_user(flags);
        let mut allocator = table_allocator(hhdm);

        // Handle existing mappings (huge pages splitting etc)
        match mapper.translate_page(page) {
            Ok(_) => {
                if let Ok(flush) = mapper.update_flags(page, map_flags) {
                    flush.flush();
                }
                return;
            }
            Err(TranslateError::ParentEntryHugePage) => {
                if let Ok((_phys, flush)) =
                    mapper.unmap(Page::<Size2MiB>::containing_address(VirtAddr::new(vaddr)))
                {
                    flush.flush();
                }
            }
            Err(TranslateError::PageNotMapped) => {}
            Err(_) => return,
        }

        if let Ok(flush) = mapper.map_to(page, phys_frame, map_flags, &mut allocator) {
            flush.flush();
        }
    }

    unsafe fn write_bytes(
        root: &mut Self::Root,
        vaddr: u64,
        bytes: &[u8],
    ) {
        if bytes.is_empty() {
            return;
        }

        use core::cmp::{max, min};

        // This function must temporarily map if needed, OR relies on the fact that
        // the user pages are already mapped?
        // The plan says: "implementation may temporarily map frames via HHDM/linear map."
        // In this implementation, since we have HHDM, if the user page is mapped to a physical frame,
        // we can access that physical frame via HHDM.

        let hhdm = root.hhdm_offset;
        let mut mapper = mapper_from_root(root);
        let start = VirtAddr::new(vaddr);
        let end = VirtAddr::new(vaddr + bytes.len() as u64);

        let start_page = Page::<Size4KiB>::containing_address(start);
        let end_page = Page::<Size4KiB>::containing_address(end - 1u64);

        // Note: The page MUST be mapped already for this function to work if we just translate.
        // But spawn logic calls `load_elf` which calls `write_bytes`.
        // `load_elf` does NOT map pages first. It expects `write_bytes` to handle it.
        // So we must allocate and map if not present.

        // Wait, `load_elf` calls `write`.
        // In `spawn.rs`:
        // load_elf(..., |vaddr, bytes, flags| { A::write_bytes(...) })
        // It does NOT pre-map.

        let mut allocator = table_allocator(hhdm);

        // Default flags for implicit mapping during load:
        // The prompt says: "write_bytes: ensure page exists + map writable + copy bytes using HHDM mapping."
        // We'll use User + RW.
        let map_flags = PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE | PageTableFlags::WRITABLE;

        for page in Page::range_inclusive(start_page, end_page) {
            let page_start_virt = page.start_address();
            let mut needs_alloc = true;

            // Check if mapped
            match mapper.translate_page(page) {
                Ok(_) => {
                     // Update flags to be writable if it wasn't?
                     // If it's already mapped, we assume we can write to it via HHDM alias,
                     // so user-side permissions don't strictly matter for kernel writing via HHDM.
                     // But we should ensure it is backed by RAM.
                     needs_alloc = false;
                }
                Err(TranslateError::ParentEntryHugePage) => {
                     // Split? Or just unmap and remap 4k?
                     if let Ok((_phys, flush)) = mapper.unmap(Page::<Size2MiB>::containing_address(page_start_virt)) {
                         flush.flush();
                     }
                }
                Err(TranslateError::PageNotMapped) => {}
                Err(_) => continue,
            }

            if needs_alloc {
                let phys = Self::alloc_frame_4k();
                if phys == 0 {
                    continue; // panic?
                }
                let frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(phys));
                if let Ok(flush) = mapper.map_to(page, frame, map_flags, &mut allocator) {
                    flush.flush();
                }
            }

            // Now copy
            // We need to find the physical address of the page we just ensured is there.
            if let x86_64::structures::paging::mapper::TranslateResult::Mapped { frame, offset, .. } =
                mapper.translate(page_start_virt)
            {
                let phys = frame.start_address() + offset;
                let frame_virt = hhdm + phys.as_u64();

                let overlap_start = max(page_start_virt, start);
                let overlap_end = min(page_start_virt + 4096u64, end);
                if overlap_end <= overlap_start {
                    continue;
                }

                let seg_offset = overlap_start - start;
                let page_offset = overlap_start - page_start_virt;
                let copy_len = overlap_end - overlap_start;

                let src_ptr = bytes.as_ptr().add(seg_offset as usize);
                let dest_ptr = (frame_virt.as_mut_ptr::<u8>()).add(page_offset as usize);
                core::ptr::copy_nonoverlapping(src_ptr, dest_ptr, copy_len as usize);
            }
        }
    }

    unsafe fn sync_icache(_vaddr: u64, _len: usize) {
        // x86_64 is coherent
    }
}
