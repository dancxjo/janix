use crate::memory::map::{MapPerms, MapResult, MapError};
use x86_64::registers::control::{Cr3, Cr3Flags};
use x86_64::{PhysAddr, VirtAddr};
use x86_64::structures::paging::{PageTable, PageTableFlags, PhysFrame, Size4KiB};

pub struct AddressSpace {
    pub pml4_table: u64, // Physical address of PML4
}

impl AddressSpace {
    pub fn new() -> MapResult<Self> {
        // 1. Allocate new PML4
        let frame = allocate_frame()?;
        let new_table = unsafe { get_table_mut(frame.start_address().as_u64()) };
        
        // 2. Clear User Half (0..256) - Box::new(PageTable::new()) implies zeroed, but explicitly:
        // (It's already zeroed by PageTable constructor)
        
        // 3. Copy Kernel Half (256..512) from current active CR3
        let (current_frame, _) = Cr3::read();
        let current_table = unsafe { get_table_mut(current_frame.start_address().as_u64()) };
        
        for i in 256..512 {
            if current_table[i].flags().contains(PageTableFlags::PRESENT) {
                 crate::log::klog(crate::log::Level::Info, "MMU", &alloc::format!("copy kernel pml4[{}]", i));
                 new_table[i] = current_table[i].clone();
            }
        }
        
        Ok(Self { pml4_table: frame.start_address().as_u64() })
    }

    pub fn from_existing(pml4: u64) -> Self {
        Self { pml4_table: pml4 }
    }

    pub fn activate(&self) {
        let frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(self.pml4_table));
        unsafe {
            Cr3::write(frame, Cr3Flags::empty());
        }
    }

    pub fn map(&mut self, virt: u64, phys: u64, len: usize, perms: MapPerms) -> MapResult<()> {
        use x86_64::structures::paging::Page;
        crate::log::klog(crate::log::Level::Info, "MMU", &alloc::format!("mapping {:x}->{:x} len {:x}", virt, phys, len));
        
        let start = VirtAddr::new(virt);
        let end_addr = start + len as u64;
        let start_page = Page::<Size4KiB>::containing_address(start);
        let end_page = Page::<Size4KiB>::containing_address(end_addr - 1u64);

        let pml4 = unsafe { get_table_mut(self.pml4_table) };
        
        // Calculate physical address of the first page start
        // phys passed is for 'virt'. If virt is unaligned, phys is unaligned.
        // We need the page-aligned physical address.
        // phys_page_base = phys - (virt % 4096)
        let page_offset = virt % 4096;
        let start_phys = phys.checked_sub(page_offset).expect("phys addr underflow");
        
        for (i, page) in Page::range_inclusive(start_page, end_page).enumerate() {
            let frame_start = start_phys + (i as u64 * 4096);
            let frame_phys = PhysAddr::new(frame_start);
            let frame = PhysFrame::<Size4KiB>::from_start_address(frame_phys).map_err(|_| MapError::InvalidAddress)?;

            let p4_entry = &mut pml4[page.p4_index()];
            let pdp = ensure_table_entry(p4_entry)?;

            let p3_entry = &mut pdp[page.p3_index()];
            let pd = ensure_table_entry(p3_entry)?;

            let p2_entry = &mut pd[page.p2_index()];
            let pt = ensure_table_entry(p2_entry)?;

            let p1_entry = &mut pt[page.p1_index()];
            
            // Flags
            let mut flags = PageTableFlags::PRESENT;
            if perms.contains(MapPerms::WRITE) { flags |= PageTableFlags::WRITABLE; }
            if perms.contains(MapPerms::USER) { flags |= PageTableFlags::USER_ACCESSIBLE; }
            if !perms.contains(MapPerms::EXEC) { flags |= PageTableFlags::NO_EXECUTE; }

            p1_entry.set_addr(frame.start_address(), flags);
        }
        
        x86_64::instructions::tlb::flush_all();

        Ok(())
    }
}

// Helpers

unsafe fn get_table_mut(phys: u64) -> &'static mut PageTable {
    let offset = crate::boot::get_boot_ctx().hhdm_offset;
    let virt = phys + offset;
    &mut *(virt as *mut PageTable)
}

fn ensure_table_entry(entry: &mut x86_64::structures::paging::page_table::PageTableEntry) -> MapResult<&'static mut PageTable> {
    if !entry.flags().contains(PageTableFlags::PRESENT) {
        let frame = allocate_frame()?;
        // User Accessible must be set on higher levels to allow User access at bottom?
        // Yes. We set full permissions for intermediate tables to be permissive.
        entry.set_addr(frame.start_address(), PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE);
    }
    Ok(unsafe { get_table_mut(entry.addr().as_u64()) })
}

fn allocate_frame() -> MapResult<PhysFrame> {
    // Hack: Use Box leak to allocate a zeroed page table from Kernel Heap
    let frame_box = alloc::boxed::Box::new(PageTable::new());
    let leaked = alloc::boxed::Box::leak(frame_box);
    let virt = leaked as *mut _ as u64;
    let phys = crate::machine::machine().virt_to_phys(virt);
    PhysFrame::<Size4KiB>::from_start_address(PhysAddr::new(phys)).map_err(|_| MapError::Oom)
}
