extern crate alloc;

use alloc::string::String;

pub struct ProgramImageData {
    pub identifier: String,
    pub module_index: u64,
    pub base_phys: u64,
    pub size: u64,
}

pub struct LoadedElfProgram {
    pub entry_point: u64,
    pub user_stack_top: u64,
    pub address_space_token: u64,
    pub heap_base: u64,
    pub heap_limit: u64,
}

fn log_milestone(message: &str) {
    kernel::log::log_message(message);
}

pub fn load_program(image: &ProgramImageData) -> Result<LoadedElfProgram, &'static str> {
    #[cfg(target_arch = "x86_64")]
    {
        x86_64::load_program(image)
    }
    #[cfg(target_arch = "aarch64")]
    {
        aarch64::load_program(image)
    }
    #[cfg(all(not(target_arch = "x86_64"), not(target_arch = "aarch64")))]
    {
        let _ = image;
        Err("ELF loader not implemented for this architecture")
    }
}

#[cfg(target_arch = "x86_64")]
mod x86_64 {
    use super::{log_milestone, LoadedElfProgram, ProgramImageData};
    use crate::boot_model::HHDM_REQUEST;
    use abi::{USER_HEAP_END, USER_HEAP_START};
    use alloc::format;
    use alloc::vec::Vec;
    use core::ptr;
    use kernel::memory;
    use x86_64::registers::control::Cr3;
    use x86_64::structures::paging::mapper::MapToError;
    use x86_64::structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageSize, PageTable, PageTableFlags,
        PhysFrame, Size4KiB,
    };
    use x86_64::{PhysAddr, VirtAddr};

    const PT_LOAD: u32 = 1;
    const PF_X: u32 = 1;
    const PF_W: u32 = 2;
    const STACK_SIZE: u64 = 32 * 1024 * 1024;
    const USER_STACK_TOP: u64 = 0x0000_7fff_ffff_f000;

    pub fn load_program(image: &ProgramImageData) -> Result<LoadedElfProgram, &'static str> {
        let hhdm = HHDM_REQUEST
            .get_response()
            .ok_or("HHDM response unavailable for ELF loader")?
            .offset();
        if image.size == 0 {
            return Err("ProgramImage has zero length");
        }
        let module_slice = unsafe {
            core::slice::from_raw_parts((image.base_phys + hhdm) as *const u8, image.size as usize)
        };
        let elf = ElfFile::parse(module_slice)?;
        let total_segments = elf.program_headers.len();
        let loadable_segments = elf
            .program_headers
            .iter()
            .filter(|seg| seg.p_type == PT_LOAD && seg.p_memsz != 0)
            .count();
        log_milestone(&format!(
            "elf: parsed '{}' entry={:#x} segments={}/{}",
            image.identifier, elf.entry_point, loadable_segments, total_segments
        ));
        let mut space = AddressSpace::new(hhdm)?;
        let mut frame_alloc = KernelFrameAllocator;
        let mut mapped_segments = 0usize;
        let mut skipped_segments = 0usize;
        for segment in elf.program_headers.iter() {
            if segment.p_type != PT_LOAD || segment.p_memsz == 0 {
                skipped_segments += 1;
                continue;
            }
            map_segment(&mut space, segment, module_slice, &mut frame_alloc, hhdm)?;
            mapped_segments += 1;
        }
        log_milestone(&format!(
            "elf: mapped segments loadable={} skipped={}",
            mapped_segments, skipped_segments
        ));
        let stack_top = map_stack(&mut space, &mut frame_alloc, hhdm)?;
        let (heap_base, heap_limit) = map_user_heap(&mut space, &mut frame_alloc, hhdm)?;
        log_milestone(&format!(
            "elf: stack+heap ready stack_top={:#x} heap={:#x}-{:#x}",
            stack_top, heap_base, heap_limit
        ));
        Ok(LoadedElfProgram {
            entry_point: elf.entry_point,
            user_stack_top: stack_top,
            address_space_token: space.pml4_phys,
            heap_base,
            heap_limit,
        })
    }

    struct ElfFile {
        pub entry_point: u64,
        pub program_headers: Vec<ProgramHeader>,
    }

    impl ElfFile {
        fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
            if bytes.len() < ELF_HEADER_SIZE {
                return Err("ELF image too small");
            }
            if &bytes[0..4] != b"\x7FELF" {
                return Err("Invalid ELF magic");
            }
            if bytes[4] != 2 {
                return Err("ELF is not 64-bit");
            }
            if bytes[5] != 1 {
                return Err("ELF is not little-endian");
            }
            let entry_point = read_u64(bytes, 24);
            let ph_offset = read_u64(bytes, 32) as usize;
            let ph_entry_size = read_u16(bytes, 54) as usize;
            let ph_num = read_u16(bytes, 56) as usize;
            if ph_entry_size == 0 {
                return Err("ELF has zero-sized program headers");
            }
            let mut headers = Vec::new();
            for i in 0..ph_num {
                let start = ph_offset
                    .checked_add(i * ph_entry_size)
                    .ok_or("Program header overflow")?;
                let end = start
                    .checked_add(ph_entry_size)
                    .ok_or("Program header overflow")?;
                if end > bytes.len() {
                    return Err("Program header out of bounds");
                }
                headers.push(ProgramHeader::parse(&bytes[start..end])?);
            }
            Ok(Self {
                entry_point,
                program_headers: headers,
            })
        }
    }

    const ELF_HEADER_SIZE: usize = 64;

    fn read_u16(bytes: &[u8], offset: usize) -> u16 {
        let data = [bytes[offset], bytes[offset + 1]];
        u16::from_le_bytes(data)
    }

    fn read_u32(bytes: &[u8], offset: usize) -> u32 {
        let data = [
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ];
        u32::from_le_bytes(data)
    }

    fn read_u64(bytes: &[u8], offset: usize) -> u64 {
        let mut data = [0u8; 8];
        data.copy_from_slice(&bytes[offset..offset + 8]);
        u64::from_le_bytes(data)
    }

    #[derive(Clone)]
    struct ProgramHeader {
        pub p_type: u32,
        pub p_flags: u32,
        pub p_offset: u64,
        pub p_vaddr: u64,
        pub p_memsz: u64,
        pub p_filesz: u64,
        pub p_align: u64,
    }

    impl ProgramHeader {
        fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
            if bytes.len() < 56 {
                return Err("Program header too small");
            }
            Ok(Self {
                p_type: read_u32(bytes, 0),
                p_flags: read_u32(bytes, 4),
                p_offset: read_u64(bytes, 8),
                p_vaddr: read_u64(bytes, 16),
                p_memsz: read_u64(bytes, 40),
                p_filesz: read_u64(bytes, 32),
                p_align: read_u64(bytes, 48),
            })
        }
    }

    struct KernelFrameAllocator;

    unsafe impl FrameAllocator<Size4KiB> for KernelFrameAllocator {
        fn allocate_frame(&mut self) -> Option<PhysFrame> {
            memory::allocate_frame().map(|frame| {
                PhysFrame::from_start_address(PhysAddr::new(frame.start_address))
                    .expect("Invalid frame start address")
            })
        }
    }

    struct AddressSpace {
        pml4_phys: u64,
        pml4_table: &'static mut PageTable,
        hhdm_offset: u64,
    }

    impl AddressSpace {
        fn new(hhdm_offset: u64) -> Result<Self, &'static str> {
            let frame = memory::allocate_frame().ok_or("Out of frames for PML4")?;
            let pml4_phys = frame.start_address;
            let table_ptr = (pml4_phys + hhdm_offset) as *mut PageTable;
            let table = unsafe { &mut *table_ptr };
            *table = PageTable::new();
            clone_kernel_space(table, hhdm_offset)?;
            Ok(Self {
                pml4_phys,
                pml4_table: table,
                hhdm_offset,
            })
        }
    }

    fn clone_kernel_space(table: &mut PageTable, hhdm_offset: u64) -> Result<(), &'static str> {
        let current = Cr3::read();
        let current_ptr = (current.0.start_address().as_u64() + hhdm_offset) as *const PageTable;
        let current_table = unsafe { &*current_ptr };
        for i in 256..512 {
            table[i] = current_table[i].clone();
        }
        Ok(())
    }

    fn mapper<'a, 'b>(space: &'a mut AddressSpace) -> OffsetPageTable<'b>
    where
        'a: 'b,
    {
        unsafe { OffsetPageTable::new(space.pml4_table, VirtAddr::new(space.hhdm_offset)) }
    }

    fn map_segment(
        space: &mut AddressSpace,
        segment: &ProgramHeader,
        image: &[u8],
        frame_alloc: &mut KernelFrameAllocator,
        hhdm_offset: u64,
    ) -> Result<(), &'static str> {
        let start = align_down(segment.p_vaddr);
        let end = align_up(segment.p_vaddr + segment.p_memsz);

        if end <= start {
            return Err("Invalid segment size");
        }
        let flags = segment_flags(segment.p_flags);
        let mut mapper = mapper(space);
        for page_addr in (start..end).step_by(Size4KiB::SIZE as usize) {
            let page = Page::<Size4KiB>::containing_address(VirtAddr::new(page_addr));
            let frame = frame_alloc
                .allocate_frame()
                .ok_or("Out of frames mapping segment")?;
            unsafe {
                mapper
                    .map_to(page, frame, flags, frame_alloc)
                    .map_err(map_err_to_str)?
                    .flush();
            }
            zero_frame(frame.start_address().as_u64(), hhdm_offset);
            copy_segment_bytes(
                frame.start_address().as_u64(),
                page_addr,
                segment,
                image,
                hhdm_offset,
            );
        }
        Ok(())
    }

    fn map_stack(
        space: &mut AddressSpace,
        frame_alloc: &mut KernelFrameAllocator,
        hhdm_offset: u64,
    ) -> Result<u64, &'static str> {
        let stack_bottom = USER_STACK_TOP
            .checked_sub(STACK_SIZE)
            .ok_or("Stack underflow")?;
        let mut mapper = mapper(space);
        let mut addr = stack_bottom;
        while addr < USER_STACK_TOP {
            let page = Page::<Size4KiB>::containing_address(VirtAddr::new(addr));
            let frame = frame_alloc
                .allocate_frame()
                .ok_or("Out of frames mapping stack")?;
            unsafe {
                mapper
                    .map_to(
                        page,
                        frame,
                        PageTableFlags::PRESENT
                            | PageTableFlags::WRITABLE
                            | PageTableFlags::USER_ACCESSIBLE,
                        frame_alloc,
                    )
                    .map_err(map_err_to_str)?
                    .flush();
            }
            zero_frame(frame.start_address().as_u64(), hhdm_offset);
            addr += Size4KiB::SIZE as u64;
        }
        Ok(USER_STACK_TOP)
    }

    fn map_user_heap(
        space: &mut AddressSpace,
        frame_alloc: &mut KernelFrameAllocator,
        hhdm_offset: u64,
    ) -> Result<(u64, u64), &'static str> {
        let heap_start = USER_HEAP_START as u64;
        let heap_end = USER_HEAP_END as u64;
        let map_end = heap_start + (16 * Size4KiB::SIZE as u64);

        let mut mapper = mapper(space);
        let mut addr = heap_start;
        while addr < map_end {
            let page = Page::<Size4KiB>::containing_address(VirtAddr::new(addr));
            let frame = frame_alloc
                .allocate_frame()
                .ok_or("Out of frames mapping heap")?;
            unsafe {
                mapper
                    .map_to(
                        page,
                        frame,
                        PageTableFlags::PRESENT
                            | PageTableFlags::WRITABLE
                            | PageTableFlags::USER_ACCESSIBLE,
                        frame_alloc,
                    )
                    .map_err(map_err_to_str)?
                    .flush();
            }
            zero_frame(frame.start_address().as_u64(), hhdm_offset);
            addr += Size4KiB::SIZE as u64;
        }
        Ok((heap_start, heap_end))
    }

    fn align_down(addr: u64) -> u64 {
        addr & !(Size4KiB::SIZE as u64 - 1)
    }

    fn align_up(addr: u64) -> u64 {
        if addr % Size4KiB::SIZE as u64 == 0 {
            addr
        } else {
            (addr & !(Size4KiB::SIZE as u64 - 1)) + Size4KiB::SIZE as u64
        }
    }

    fn segment_flags(flags: u32) -> PageTableFlags {
        let mut out = PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE;
        if flags & PF_W != 0 {
            out |= PageTableFlags::WRITABLE;
        }
        if flags & PF_X == 0 {
            out |= PageTableFlags::NO_EXECUTE;
        }
        out
    }

    fn zero_frame(phys: u64, hhdm: u64) {
        unsafe {
            ptr::write_bytes((phys + hhdm) as *mut u8, 0, Size4KiB::SIZE as usize);
        }
    }

    fn copy_segment_bytes(
        frame_phys: u64,
        page_addr: u64,
        segment: &ProgramHeader,
        image: &[u8],
        hhdm: u64,
    ) {
        let page_start = page_addr;
        let page_end = page_start + Size4KiB::SIZE as u64;
        let seg_start = segment.p_vaddr;
        let file_end = segment.p_vaddr + segment.p_filesz;
        let copy_start = page_start.max(seg_start);
        let copy_end = page_end.min(file_end);
        if copy_end <= copy_start {
            return;
        }
        let src_offset = segment
            .p_offset
            .checked_add(copy_start - seg_start)
            .expect("Segment offset overflow") as usize;
        let len = (copy_end - copy_start) as usize;
        if src_offset + len > image.len() {
            return;
        }
        let dest_offset = copy_start - page_start;
        let dest_ptr = (frame_phys + hhdm + dest_offset) as *mut u8;
        unsafe {
            ptr::copy_nonoverlapping(image.as_ptr().add(src_offset), dest_ptr, len);
        }
    }

    fn map_err_to_str(err: MapToError<Size4KiB>) -> &'static str {
        match err {
            MapToError::FrameAllocationFailed => "Frame allocation failed",
            MapToError::ParentEntryHugePage => "Encountered huge page while mapping",
            MapToError::PageAlreadyMapped(_) => "Page already mapped",
        }
    }
}

#[cfg(target_arch = "aarch64")]
mod aarch64 {
    use super::{log_milestone, LoadedElfProgram, ProgramImageData};
    use crate::boot_model::HHDM_REQUEST;
    use abi::{USER_HEAP_END, USER_HEAP_START};
    use alloc::format;
    use alloc::vec::Vec;
    use core::ptr;
    use kernel::memory;

    const PT_LOAD: u32 = 1;
    const PF_X: u32 = 1;
    const PF_W: u32 = 2;
    const PAGE_SIZE: u64 = 4096;
    const STACK_SIZE: u64 = 32 * 1024 * 1024;
    const USER_STACK_TOP: u64 = 0x0000_0000_3fff_f000;

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

    pub fn load_program(image: &ProgramImageData) -> Result<LoadedElfProgram, &'static str> {
        let hhdm = HHDM_REQUEST
            .get_response()
            .ok_or("HHDM response unavailable for ELF loader")?
            .offset();
        if image.size == 0 {
            return Err("ProgramImage has zero length");
        }
        let module_slice = unsafe {
            core::slice::from_raw_parts((image.base_phys + hhdm) as *const u8, image.size as usize)
        };
        let elf = ElfFile::parse(module_slice)?;
        let total_segments = elf.program_headers.len();
        let loadable_segments = elf
            .program_headers
            .iter()
            .filter(|seg| seg.p_type == PT_LOAD && seg.p_memsz != 0)
            .count();
        log_milestone(&format!(
            "elf: parsed '{}' entry={:#x} segments={}/{}",
            image.identifier, elf.entry_point, loadable_segments, total_segments
        ));
        let mut space = AddressSpace::new(hhdm)?;
        let mut frame_alloc = FrameAlloc;

        let mut mapped_segments = 0usize;
        let mut skipped_segments = 0usize;
        for segment in elf.program_headers.iter() {
            if segment.p_type != PT_LOAD || segment.p_memsz == 0 {
                skipped_segments += 1;
                continue;
            }
            map_segment(&mut space, segment, module_slice, &mut frame_alloc, hhdm)?;
            mapped_segments += 1;
        }
        log_milestone(&format!(
            "elf: mapped segments loadable={} skipped={}",
            mapped_segments, skipped_segments
        ));

        let stack_top = map_stack(&mut space, &mut frame_alloc, hhdm)?;
        let (heap_base, heap_limit) = map_user_heap(&mut space, &mut frame_alloc, hhdm)?;
        log_milestone(&format!(
            "elf: stack+heap ready stack_top={:#x} heap={:#x}-{:#x}",
            stack_top, heap_base, heap_limit
        ));

        Ok(LoadedElfProgram {
            entry_point: elf.entry_point,
            user_stack_top: stack_top,
            address_space_token: space.ttbr0_phys,
            heap_base,
            heap_limit,
        })
    }

    struct ElfFile {
        pub entry_point: u64,
        pub program_headers: Vec<ProgramHeader>,
    }

    impl ElfFile {
        fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
            if bytes.len() < ELF_HEADER_SIZE {
                return Err("ELF image too small");
            }
            if &bytes[0..4] != b"\x7FELF" {
                return Err("Invalid ELF magic");
            }
            if bytes[4] != 2 {
                return Err("ELF is not 64-bit");
            }
            if bytes[5] != 1 {
                return Err("ELF is not little-endian");
            }
            let entry_point = read_u64(bytes, 24);
            let ph_offset = read_u64(bytes, 32) as usize;
            let ph_entry_size = read_u16(bytes, 54) as usize;
            let ph_num = read_u16(bytes, 56) as usize;
            if ph_entry_size == 0 {
                return Err("ELF has zero-sized program headers");
            }
            let mut headers = Vec::new();
            for i in 0..ph_num {
                let start = ph_offset
                    .checked_add(i * ph_entry_size)
                    .ok_or("Program header overflow")?;
                let end = start
                    .checked_add(ph_entry_size)
                    .ok_or("Program header overflow")?;
                if end > bytes.len() {
                    return Err("Program header out of bounds");
                }
                headers.push(ProgramHeader::parse(&bytes[start..end])?);
            }
            Ok(Self {
                entry_point,
                program_headers: headers,
            })
        }
    }

    const ELF_HEADER_SIZE: usize = 64;

    fn read_u16(bytes: &[u8], offset: usize) -> u16 {
        let data = [bytes[offset], bytes[offset + 1]];
        u16::from_le_bytes(data)
    }

    fn read_u32(bytes: &[u8], offset: usize) -> u32 {
        let data = [
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ];
        u32::from_le_bytes(data)
    }

    fn read_u64(bytes: &[u8], offset: usize) -> u64 {
        let mut data = [0u8; 8];
        data.copy_from_slice(&bytes[offset..offset + 8]);
        u64::from_le_bytes(data)
    }

    #[derive(Clone)]
    struct ProgramHeader {
        pub p_type: u32,
        pub p_flags: u32,
        pub p_offset: u64,
        pub p_vaddr: u64,
        pub p_memsz: u64,
        pub p_filesz: u64,
        pub p_align: u64,
    }

    impl ProgramHeader {
        fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
            if bytes.len() < 56 {
                return Err("Program header too small");
            }
            Ok(Self {
                p_type: read_u32(bytes, 0),
                p_flags: read_u32(bytes, 4),
                p_offset: read_u64(bytes, 8),
                p_vaddr: read_u64(bytes, 16),
                p_memsz: read_u64(bytes, 40),
                p_filesz: read_u64(bytes, 32),
                p_align: read_u64(bytes, 48),
            })
        }
    }

    struct FrameAlloc;

    impl FrameAlloc {
        fn allocate(&mut self) -> Option<u64> {
            memory::allocate_frame().map(|frame| frame.start_address)
        }
    }

    struct PageTable {
        entries: [u64; 512],
    }

    impl PageTable {
        #[allow(dead_code)]
        fn zero(&mut self) {
            for entry in self.entries.iter_mut() {
                *entry = 0;
            }
        }
    }

    struct AddressSpace {
        ttbr0_phys: u64,
        l0_table: &'static mut PageTable,
        hhdm_offset: u64,
    }

    impl AddressSpace {
        fn new(hhdm_offset: u64) -> Result<Self, &'static str> {
            let frame = memory::allocate_frame().ok_or("Out of frames for L0 table")?;
            let table_ptr = (frame.start_address + hhdm_offset) as *mut PageTable;
            unsafe {
                ptr::write_bytes(table_ptr as *mut u8, 0, core::mem::size_of::<PageTable>());
            }
            let table = unsafe { &mut *table_ptr };
            Ok(Self {
                ttbr0_phys: frame.start_address,
                l0_table: table,
                hhdm_offset,
            })
        }

        fn map_page(
            &mut self,
            virt: u64,
            phys: u64,
            writable: bool,
            executable: bool,
        ) -> Result<(), &'static str> {
            let l0_idx = ((virt >> 39) & 0x1ff) as usize;
            let l1_idx = ((virt >> 30) & 0x1ff) as usize;
            let l2_idx = ((virt >> 21) & 0x1ff) as usize;
            let l3_idx = ((virt >> 12) & 0x1ff) as usize;

            let l1_phys = ensure_table(self.l0_table, l0_idx, self.hhdm_offset)?;
            let l1_table = table_mut(l1_phys, self.hhdm_offset);

            let l2_phys = ensure_table(l1_table, l1_idx, self.hhdm_offset)?;
            let l2_table = table_mut(l2_phys, self.hhdm_offset);

            let l3_phys = ensure_table(l2_table, l2_idx, self.hhdm_offset)?;
            let l3_table = table_mut(l3_phys, self.hhdm_offset);

            if l3_table.entries[l3_idx] & DESC_VALID != 0 {
                return Err("Virtual page already mapped");
            }

            let mut desc =
                phys | DESC_VALID | DESC_TABLE_OR_PAGE | ATTR_INDEX0 | SH_INNER | AF | PXN;
            desc |= if writable { AP_RW_EL0 } else { AP_RO_EL0 };
            if !executable {
                desc |= UXN;
            }
            l3_table.entries[l3_idx] = desc;
            Ok(())
        }
    }

    fn ensure_table(
        parent: &mut PageTable,
        idx: usize,
        hhdm_offset: u64,
    ) -> Result<u64, &'static str> {
        let entry = parent.entries[idx];
        if entry & DESC_VALID == 0 {
            let frame = memory::allocate_frame().ok_or("Out of frames for page table")?;
            zero_frame(frame.start_address, hhdm_offset);
            parent.entries[idx] = frame.start_address | DESC_VALID | DESC_TABLE_OR_PAGE;
            Ok(frame.start_address)
        } else if entry & DESC_TABLE_OR_PAGE != 0 {
            Ok(entry & ADDR_MASK)
        } else {
            Err("Encountered block entry while walking page tables")
        }
    }

    fn table_mut(phys: u64, hhdm_offset: u64) -> &'static mut PageTable {
        let ptr = (phys + hhdm_offset) as *mut PageTable;
        let table = unsafe { &mut *ptr };
        table
    }

    fn map_segment(
        space: &mut AddressSpace,
        segment: &ProgramHeader,
        image: &[u8],
        frame_alloc: &mut FrameAlloc,
        hhdm: u64,
    ) -> Result<(), &'static str> {
        let start = align_down(segment.p_vaddr);
        let end = align_up(segment.p_vaddr + segment.p_memsz);
        if end <= start {
            return Err("Invalid segment size");
        }
        let writable = segment.p_flags & PF_W != 0;
        let executable = segment.p_flags & PF_X != 0;
        let mut addr = start;
        while addr < end {
            let frame = frame_alloc
                .allocate()
                .ok_or("Out of frames mapping segment")?;
            space.map_page(addr, frame, writable, executable)?;
            zero_frame(frame, hhdm);
            copy_segment_bytes(frame, addr, segment, image, hhdm);
            addr += PAGE_SIZE;
        }
        Ok(())
    }

    fn map_stack(
        space: &mut AddressSpace,
        frame_alloc: &mut FrameAlloc,
        hhdm: u64,
    ) -> Result<u64, &'static str> {
        let stack_bottom = USER_STACK_TOP
            .checked_sub(STACK_SIZE)
            .ok_or("Stack underflow")?;
        let mut addr = stack_bottom;
        while addr < USER_STACK_TOP {
            let frame = frame_alloc
                .allocate()
                .ok_or("Out of frames mapping stack")?;
            space.map_page(addr, frame, true, false)?;
            zero_frame(frame, hhdm);
            addr += PAGE_SIZE;
        }
        Ok(USER_STACK_TOP)
    }

    fn map_user_heap(
        space: &mut AddressSpace,
        frame_alloc: &mut FrameAlloc,
        hhdm: u64,
    ) -> Result<(u64, u64), &'static str> {
        let heap_start = USER_HEAP_START as u64;
        let heap_end = USER_HEAP_END as u64;
        let map_end = heap_start + (16 * PAGE_SIZE);

        let mut addr = heap_start;
        while addr < map_end {
            let frame = frame_alloc.allocate().ok_or("Out of frames mapping heap")?;
            space.map_page(addr, frame, true, false)?;
            zero_frame(frame, hhdm);
            addr += PAGE_SIZE;
        }
        Ok((heap_start, heap_end))
    }

    fn align_down(addr: u64) -> u64 {
        addr & !(PAGE_SIZE - 1)
    }

    fn align_up(addr: u64) -> u64 {
        if addr % PAGE_SIZE == 0 {
            addr
        } else {
            (addr & !(PAGE_SIZE - 1)) + PAGE_SIZE
        }
    }

    fn zero_frame(phys: u64, hhdm: u64) {
        unsafe {
            ptr::write_bytes((phys + hhdm) as *mut u8, 0, PAGE_SIZE as usize);
        }
    }

    fn copy_segment_bytes(
        frame_phys: u64,
        page_addr: u64,
        segment: &ProgramHeader,
        image: &[u8],
        hhdm: u64,
    ) {
        let page_start = page_addr;
        let page_end = page_start + PAGE_SIZE;
        let seg_start = segment.p_vaddr;
        let file_end = segment.p_vaddr + segment.p_filesz;
        let copy_start = page_start.max(seg_start);
        let copy_end = page_end.min(file_end);
        if copy_end <= copy_start {
            return;
        }
        let src_offset = segment
            .p_offset
            .checked_add(copy_start - seg_start)
            .expect("Segment offset overflow") as usize;
        let len = (copy_end - copy_start) as usize;
        if src_offset + len > image.len() {
            return;
        }
        let dest_offset = copy_start - page_start;
        let dest_ptr = (frame_phys + hhdm + dest_offset) as *mut u8;
        unsafe {
            ptr::copy_nonoverlapping(image.as_ptr().add(src_offset), dest_ptr, len);
        }
    }
}
