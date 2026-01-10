use kernel::{MapPerms, MapKind, FrameAllocatorHook};

#[derive(Clone, Copy, Default)]
pub struct X86_64AddressSpace(pub u64);

static mut HHDM_OFFSET: u64 = 0;

pub fn init(offset: u64) {
    unsafe { HHDM_OFFSET = offset };
}

pub fn active_address_space() -> X86_64AddressSpace {
    let cr3: u64;
    unsafe {
        core::arch::asm!("mov {}, cr3", out(reg) cr3);
    }
    X86_64AddressSpace(cr3)
}

pub fn make_user_address_space(active: X86_64AddressSpace, allocator: &dyn FrameAllocatorHook) -> X86_64AddressSpace {
    let phys = allocator.alloc_frame().expect("No frames for User PML4");
    let virt = phys + unsafe { HHDM_OFFSET };
    let ptr = virt as *mut u64;
    
    unsafe {
        let active_ptr = (active.0 + HHDM_OFFSET) as *const u64;
        core::ptr::copy_nonoverlapping(active_ptr.add(256), ptr.add(256), 256);
        core::ptr::write_bytes(ptr, 0, 256 * 8);
    }
    
    X86_64AddressSpace(phys)
}

pub fn map_page(
    aspace: X86_64AddressSpace,
    virt: u64,
    phys: u64,
    perms: MapPerms,
    kind: MapKind,
    allocator: &dyn FrameAllocatorHook
) -> Result<(), ()> {
    let mut flags = 1u64; // Present
    if perms.write { flags |= 1 << 1; } // R/W
    if perms.user { flags |= 1 << 2; } // U/S
    if !perms.exec { flags |= 1 << 63; } // NX
    
    if kind == MapKind::Device { flags |= 1 << 4; }

    let pml4 = (aspace.0 + unsafe { HHDM_OFFSET }) as *mut u64;
    
    let pdpt = ensure_table(pml4, (virt >> 39) & 0x1ff, allocator)?;
    let pd = ensure_table(pdpt, (virt >> 30) & 0x1ff, allocator)?;
    let pt = ensure_table(pd, (virt >> 21) & 0x1ff, allocator)?;
    
    let pte_idx = (virt >> 12) & 0x1ff;
    unsafe {
        *pt.add(pte_idx as usize) = (phys & !0xfff) | flags;
    }
    
    Ok(())
}

fn ensure_table(parent: *mut u64, index: u64, allocator: &dyn FrameAllocatorHook) -> Result<*mut u64, ()> {
    let entry = unsafe { *parent.add(index as usize) };
    if entry & 1 == 0 {
        let phys = allocator.alloc_frame().ok_or(())?;
        unsafe {
            let virt = phys + HHDM_OFFSET;
            core::ptr::write_bytes(virt as *mut u8, 0, 4096);
            *parent.add(index as usize) = phys | 7;
        }
        Ok((phys + unsafe { HHDM_OFFSET }) as *mut u64)
    } else {
        Ok(((entry & !0xfff) + unsafe { HHDM_OFFSET }) as *mut u64)
    }
}

pub fn unmap_page(_aspace: X86_64AddressSpace, _virt: u64) -> Result<Option<u64>, ()> {
    Ok(None)
}

pub fn translate(_aspace: X86_64AddressSpace, _virt: u64) -> Option<u64> {
    None
}

pub fn tlb_flush_page(virt: u64) {
    unsafe {
        core::arch::asm!("invlpg [{}]", in(reg) virt);
    }
}
