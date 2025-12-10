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
}

pub fn load_program(image: &ProgramImageData) -> Result<LoadedElfProgram, &'static str> {
    #[cfg(target_arch = "x86_64")]
    {
        x86_64::load_program(image)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = image;
        Err("ELF loader not implemented for this architecture")
    }
}

#[cfg(target_arch = "x86_64")]
mod x86_64 {
    use super::{LoadedElfProgram, ProgramImageData};
    use crate::boot_model::HHDM_REQUEST;
    use alloc::vec::Vec;
    use core::ptr;
    use kernel_core::memory;
    use kernel_core::log;
    use x86_64::registers::control::Cr3;
    use x86_64::structures::paging::mapper::MapToError;
    use x86_64::structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame,
        Size4KiB, PageSize,
    };
    use x86_64::{PhysAddr, VirtAddr};

    const PT_LOAD: u32 = 1;
    const PF_X: u32 = 1;
    const PF_W: u32 = 2;
    const STACK_SIZE: u64 = 64 * 1024;
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
            core::slice::from_raw_parts(
                (image.base_phys + hhdm) as *const u8,
                image.size as usize,
            )
        };
        let elf = ElfFile::parse(module_slice)?;
        log("ELF loader: parsed header");
        let mut space = AddressSpace::new(hhdm)?;
        let mut frame_alloc = KernelFrameAllocator;
        for segment in elf.program_headers.iter() {
            if segment.p_type != PT_LOAD || segment.p_memsz == 0 {
                continue;
            }
            map_segment(
                &mut space,
                segment,
                module_slice,
                &mut frame_alloc,
                hhdm,
            )?;
        }
        let stack_top = map_stack(&mut space, &mut frame_alloc, hhdm)?;
        Ok(LoadedElfProgram {
            entry_point: elf.entry_point,
            user_stack_top: stack_top,
            address_space_token: space.pml4_phys,
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
        let data = [bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]];
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
        let current_ptr =
            (current.0.start_address().as_u64() + hhdm_offset) as *const PageTable;
        let current_table = unsafe { &*current_ptr };
        for i in 256..512 {
            table[i] = current_table[i].clone();
        }
        Ok(())
    }

    fn mapper<'a, 'b>(
        space: &'a mut AddressSpace,
    ) -> OffsetPageTable<'b>
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
