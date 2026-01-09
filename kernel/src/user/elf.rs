use crate::memory::paging::{AddressSpace, PageFlags};
use crate::memory::frame_alloc::FRAME_ALLOCATOR;
use crate::memory::paging::{phys_to_virt};
use core::ptr;

#[derive(Debug, Clone, Copy)]
pub struct UserImage {
    pub entry: u64,
    pub stack_top: u64,
}

/// Simple ELF64 loader.
/// 
/// Returns (entry_point, stack_top)
/// Simple ELF64 loader (Manual Safe Parsing).
/// 
/// Returns (entry_point, stack_top)
pub fn load_user_elf(bytes: &[u8], aspace: &mut AddressSpace) -> Result<UserImage, &'static str> {
    // 1. Validate Header
    if bytes.len() < 64 {
        return Err("ELF: too short");
    }
    // Debug magic
    if &bytes[0..4] != b"\x7fELF" {
        return Err("ELF: bad magic");
    }
    // Class (64-bit)
    if bytes[4] != 2 {
        return Err("ELF: not 64-bit");
    }
    // Endian (Little, 1) or Big (2). We require Little for x86/ARM/RISCV usually.
    if bytes[5] != 1 {
        return Err("ELF: not little-endian");
    }

    // Parse Entry Point (offset 0x18)
    let entry = u64::from_le_bytes(bytes[0x18..0x20].try_into().map_err(|_| "ELF: read entry")?);
    
    // Parse Program Headers
    // phoff at 0x20 (u64)
    let phoff = u64::from_le_bytes(bytes[0x20..0x28].try_into().map_err(|_| "ELF: read phoff")?);
    // phnum at 0x38 (u16)
    let phnum = u16::from_le_bytes(bytes[0x38..0x3a].try_into().map_err(|_| "ELF: read phnum")?) as usize;
    // phentsize at 0x36 (u16) 
    let phentsize = u16::from_le_bytes(bytes[0x36..0x38].try_into().map_err(|_| "ELF: read phentsize")?) as usize;

    if phentsize < 56 {
        return Err("ELF: phentsize too small");
    }

    let ph_start = phoff as usize;
    
    for i in 0..phnum {
        let offset = ph_start + (i * phentsize);
        if offset + 56 > bytes.len() {
            return Err("ELF: phdr out of bounds");
        }
        
        let phdr = &bytes[offset..offset+phentsize];
        
        // p_type (0x00, u32)
        let p_type = u32::from_le_bytes(phdr[0..4].try_into().unwrap());
        
        // PT_LOAD = 1
        if p_type == 1 {
            let p_flags = u32::from_le_bytes(phdr[4..8].try_into().unwrap());
            let p_offset = u64::from_le_bytes(phdr[8..16].try_into().unwrap());
            let p_vaddr = u64::from_le_bytes(phdr[16..24].try_into().unwrap());
            let p_filesz = u64::from_le_bytes(phdr[32..40].try_into().unwrap());
            let p_memsz = u64::from_le_bytes(phdr[40..48].try_into().unwrap());
            
            if p_memsz == 0 {
                continue;
            }

            let file_end = p_offset + p_filesz;
            if file_end as usize > bytes.len() {
                return Err("ELF: segment file truncated");
            }

            // Map segment
            // Align start down to page boundary
            let virt_start = p_vaddr & !0xFFF;
            let virt_end = (p_vaddr + p_memsz + 0xFFF) & !0xFFF;
            
            // Flags
            // PF_X = 1, PF_W = 2, PF_R = 4
            let mut flags = PageFlags::PRESENT | PageFlags::USER_ACCESSIBLE;
            if (p_flags & 2) != 0 {
                flags |= PageFlags::WRITABLE;
            }
            
            let mut curr = virt_start;
            while curr < virt_end {
                let frame = FRAME_ALLOCATOR.with_lock(|alloc| alloc.alloc().expect("nomem"));
                aspace.map_page(curr, frame, flags).expect("map failed");
                
                // Copy data if within file range
                // We need to zero page first for safety/BSS
                unsafe {
                    let dst = phys_to_virt(frame.0) as *mut u8;
                    ptr::write_bytes(dst, 0, 4096);
                    
                    // intersection of [curr, curr+4096) and [p_vaddr, p_vaddr + p_filesz)
                    let page_start = curr;
                    let page_end = curr + 4096;
                    
                    let data_start = p_vaddr;
                    let data_end = p_vaddr + p_filesz;
                    
                    let start = core::cmp::max(page_start, data_start);
                    let end = core::cmp::min(page_end, data_end);
                    
                    if start < end {
                         let len = (end - start) as usize;
                         let offset_in_page = (start - page_start) as usize;
                         let offset_in_file = (start - data_start) + p_offset;
                         
                         ptr::copy_nonoverlapping(
                             bytes.as_ptr().add(offset_in_file as usize),
                             dst.add(offset_in_page),
                             len
                         );
                    }
                }

                curr += 4096;
            }
        }
    }

    // Allocate stack (1 MiB)
    // Fixed at 0x7000_0000 for now (arbitrary user high mem)
    let stack_top = 0x7000_0000;
    let stack_size = 1024 * 1024;
    let stack_base = stack_top - stack_size;
    
    let mut curr = stack_base;
    while curr < stack_top {
        let frame = FRAME_ALLOCATOR.with_lock(|alloc| alloc.alloc().expect("stack nomem"));
        aspace.map_page(
            curr, 
            frame, 
            PageFlags::PRESENT | PageFlags::WRITABLE | PageFlags::USER_ACCESSIBLE
        ).expect("stack map failed");
        
        unsafe {
             let dst = phys_to_virt(frame.0) as *mut u8;
             ptr::write_bytes(dst, 0, 4096);
        }
        curr += 4096;
    }

    Ok(UserImage {
        entry,
        stack_top,
    })
}
