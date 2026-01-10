use kernel::{MapPerms, MapKind, FrameAllocatorHook};

#[derive(Clone, Copy, Default)]
pub struct AArch64AddressSpace(pub u64);

static mut HHDM_OFFSET: u64 = 0;

pub fn init(offset: u64) {
    unsafe { HHDM_OFFSET = offset };
}

pub fn active_address_space() -> AArch64AddressSpace {
    let ttbr0: u64;
    unsafe {
        core::arch::asm!("mrs {}, ttbr0_el1", out(reg) ttbr0);
    }
    AArch64AddressSpace(ttbr0)
}

pub fn make_user_address_space(active: AArch64AddressSpace, allocator: &dyn FrameAllocatorHook) -> AArch64AddressSpace {
    let phys = allocator.alloc_frame().expect("No frames for User TTBR0");
    let virt = phys + unsafe { HHDM_OFFSET };
    let ptr = virt as *mut u64;
    
    unsafe {
        // AArch64 typically split TTBR0 (user) and TTBR1 (kernel).
        // So User AS is just a fresh Table.
        core::ptr::write_bytes(ptr, 0, 4096);
    }
    
    AArch64AddressSpace(phys)
}

pub fn map_page(
    aspace: AArch64AddressSpace,
    virt: u64,
    phys: u64,
    perms: MapPerms,
    kind: MapKind,
    allocator: &dyn FrameAllocatorHook
) -> Result<(), ()> {
    // AArch64 attributes (4KB pages, MAIR index)
    // 0: Normal, 1: Device
    let mut attr = 0u64;
    if kind == MapKind::Device { attr = 1; }
    
    let mut desc = (phys & 0x0000_FFFF_FFFF_F000) | 0x3 | (attr << 2) | (1 << 10); // Valid + Page + AF
    
    if !perms.write { desc |= 1 << 7; } // AP[2] = 1 (Read-only)
    if perms.user { desc |= 1 << 6; } // AP[1] = 1 (User)
    
    if !perms.exec { desc |= (1 << 54) | (1 << 53); } // UXN + PXN

    let l0 = (aspace.0 + unsafe { HHDM_OFFSET }) as *mut u64;
    let l1 = ensure_table(l0, (virt >> 39) & 0x1ff, allocator)?;
    let l2 = ensure_table(l1, (virt >> 30) & 0x1ff, allocator)?;
    let l3 = ensure_table(l2, (virt >> 21) & 0x1ff, allocator)?;
    
    let l3_idx = (virt >> 12) & 0x1ff;
    unsafe {
        *l3.add(l3_idx as usize) = desc;
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
            *parent.add(index as usize) = phys | 0x3; // Table descriptor
        }
        Ok((phys + unsafe { HHDM_OFFSET }) as *mut u64)
    } else {
        Ok(((entry & 0x0000_FFFF_FFFF_F000) + unsafe { HHDM_OFFSET }) as *mut u64)
    }
}

pub fn unmap_page(_aspace: AArch64AddressSpace, _virt: u64) -> Result<Option<u64>, ()> { Ok(None) }
pub fn translate(_aspace: AArch64AddressSpace, _virt: u64) -> Option<u64> { None }
pub fn tlb_flush_page(virt: u64) {
    unsafe {
        core::arch::asm!("tlbi vaae1is, {}", in(reg) virt >> 12);
        core::arch::asm!("dsb ish", "isb");
    }
}
