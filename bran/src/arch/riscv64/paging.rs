use kernel::{MapPerms, MapKind, FrameAllocatorHook};

#[derive(Clone, Copy, Default)]
pub struct RISCV64AddressSpace(pub u64);

static mut HHDM_OFFSET: u64 = 0;

pub fn init(offset: u64) {
    unsafe { HHDM_OFFSET = offset };
}

pub fn active_address_space() -> RISCV64AddressSpace {
    let satp: u64;
    unsafe {
        core::arch::asm!("csrr {}, satp", out(reg) satp);
    }
    RISCV64AddressSpace(satp)
}

pub fn make_user_address_space(_active: RISCV64AddressSpace, allocator: &dyn FrameAllocatorHook) -> RISCV64AddressSpace {
    let phys = allocator.alloc_frame().expect("No frames for User SATP");
    let virt = phys + unsafe { HHDM_OFFSET };
    unsafe { core::ptr::write_bytes(virt as *mut u8, 0, 4096); }
    // Mode Sv39 = 8
    RISCV64AddressSpace((8 << 60) | (phys >> 12))
}

pub fn map_page(
    aspace: RISCV64AddressSpace,
    virt: u64,
    phys: u64,
    perms: MapPerms,
    kind: MapKind,
    allocator: &dyn FrameAllocatorHook
) -> Result<(), ()> {
    let mut bits = 1u64; // Valid
    if perms.read { bits |= 1 << 1; }
    if perms.write { bits |= 1 << 2; }
    if perms.exec { bits |= 1 << 3; }
    if perms.user { bits |= 1 << 4; }
    bits |= (1 << 6) | (1 << 7); // Accessed + Dirty (for simplicity)

    let root_phys = (aspace.0 & 0x0000_0FFF_FFFF_FFFF) << 12;
    let l2 = (root_phys + unsafe { HHDM_OFFSET }) as *mut u64;
    let l1 = ensure_table(l2, (virt >> 30) & 0x1ff, allocator)?;
    let l0 = ensure_table(l1, (virt >> 21) & 0x1ff, allocator)?;
    
    let pte_idx = (virt >> 12) & 0x1ff;
    unsafe {
        *l0.add(pte_idx as usize) = (phys >> 2) | bits;
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
            *parent.add(index as usize) = (phys >> 2) | 1;
        }
        Ok((phys + unsafe { HHDM_OFFSET }) as *mut u64)
    } else {
        Ok((((entry >> 10) << 12) + unsafe { HHDM_OFFSET }) as *mut u64)
    }
}

pub fn unmap_page(_aspace: RISCV64AddressSpace, _virt: u64) -> Result<Option<u64>, ()> { Ok(None) }
pub fn translate(_aspace: RISCV64AddressSpace, _virt: u64) -> Option<u64> { None }
pub fn tlb_flush_page(virt: u64) {
    unsafe { core::arch::asm!("sfence.vma {}, x0", in(reg) virt); }
}
