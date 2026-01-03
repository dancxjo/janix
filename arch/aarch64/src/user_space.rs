use crate::paging;
use core::arch::asm;
use core::cmp::{max, min};
use kernel::arch::user_space::{UserPageFlags, UserSpace};

pub struct Aarch64UserSpace;

// From arch/aarch64/src/paging.rs re-exports
use crate::paging::{
    PTE_AF, PTE_AP_RW_EL0, PTE_ATTR_DEVICE, PTE_ATTR_NORMAL, PTE_PAGE, PTE_SH_INNER, PTE_UXN,
    PTE_VALID,
};

fn flags_to_pte(flags: UserPageFlags) -> u64 {
    let mut out = PTE_VALID | PTE_PAGE | PTE_AF | PTE_SH_INNER | PTE_AP_RW_EL0 | PTE_ATTR_NORMAL;
    if flags.device {
        out = PTE_VALID
            | PTE_PAGE
            | PTE_AF
            | PTE_SH_INNER
            | PTE_AP_RW_EL0
            | PTE_ATTR_DEVICE
            | PTE_UXN;
    } else if !flags.executable {
        out |= PTE_UXN;
    }
    out
}

impl UserSpace for Aarch64UserSpace {
    type Root = u64;

    unsafe fn create_root() -> Self::Root {
        paging::create_user_root().expect("failed to allocate user root")
    }

    unsafe fn activate_root(root: &Self::Root) {
        asm!("msr ttbr0_el1, {}", in(reg) *root);
        asm!("isb");
    }

    unsafe fn alloc_frame_4k() -> u64 {
        paging::allocate_frame().map(|(phys, _)| phys).unwrap_or(0)
    }

    unsafe fn map_4k(root: &mut Self::Root, vaddr: u64, paddr: u64, flags: UserPageFlags) {
        paging::map_page_at_root(*root, paddr, vaddr, flags_to_pte(flags));
    }

    unsafe fn write_bytes(root: &mut Self::Root, vaddr: u64, bytes: &[u8]) {
        if bytes.is_empty() {
            return;
        }

        let start = vaddr;
        let end = vaddr + bytes.len() as u64;
        let start_page = start & !0xFFF;
        let end_page = (end - 1) & !0xFFF;

        let map_flags = flags_to_pte(UserPageFlags {
            writable: true,
            executable: false,
            device: false,
            user: true,
        });

        // We use the HHDM offset from the UART module as it holds the system-wide HHDM offset.
        let hhdm = crate::bridge::uart::hhdm_offset();

        let mut page = start_page;
        while page <= end_page {
            if paging::translate(*root, page).is_none() {
                let phys = Self::alloc_frame_4k();
                if phys == 0 {
                    return;
                }
                paging::map_page_at_root(*root, phys, page, map_flags);
            }

            if let Some(phys_page) = paging::translate(*root, page) {
                let page_start = page;
                let overlap_start = max(page_start, start);
                let overlap_end = min(page_start + 4096, end);

                if overlap_end > overlap_start {
                    let page_offset = overlap_start - page_start;
                    let buf_offset = overlap_start - start;
                    let len = overlap_end - overlap_start;

                    let dest = (phys_page + page_offset + hhdm) as *mut u8;
                    let src = bytes.as_ptr().add(buf_offset as usize);
                    core::ptr::copy_nonoverlapping(src, dest, len as usize);

                    // Clean D-cache to PoU for the written range
                    // Iterate over cache lines (assume 64 bytes is safe minimum)
                    let start_addr = dest as u64;
                    let end_addr = start_addr + len as u64;
                    let mut addr = start_addr & !63;
                    while addr < end_addr {
                        asm!("dc cvau, {}", in(reg) addr);
                        addr += 64;
                    }
                    asm!("dsb ish");
                }
            }

            if page == end_page {
                break;
            }
            page += 4096;
        }
    }

    unsafe fn sync_icache(_vaddr: u64, _len: usize) {
        // Invalidate entire I-cache to be safe and avoid needing VA mapping
        asm!("ic ialluis"); // Inner Shareable
        asm!("dsb ish");
        asm!("isb");
    }
}
