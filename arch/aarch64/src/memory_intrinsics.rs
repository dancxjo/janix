//! Memory intrinsics for AArch64.
//!
//! These functions provide cache maintenance operations required for
//! self-modifying code and DMA coherency.

use core::arch::asm;

/// Flush the instruction cache for a given memory region.
///
/// This is required on AArch64 after writing code to memory that will be executed.
pub unsafe fn flush_icache(start: *const u8, len: usize) {
    let end = start.add(len);
    let mut addr = start;

    // Clean data cache by VA to PoU
    while addr < end {
        asm!("dc cvau, {}", in(reg) addr, options(nostack, preserves_flags));
        addr = addr.add(64); // Typical cache line size
    }

    // Data Synchronization Barrier
    asm!("dsb ish", options(nostack, preserves_flags));

    // Invalidate instruction cache by VA to PoU
    addr = start;
    while addr < end {
        asm!("ic ivau, {}", in(reg) addr, options(nostack, preserves_flags));
        addr = addr.add(64);
    }

    // Data Synchronization Barrier + Instruction Synchronization Barrier
    asm!("dsb ish", options(nostack, preserves_flags));
    asm!("isb", options(nostack, preserves_flags));
}

/// Flush the data cache for a given memory region.
///
/// This cleans and invalidates the data cache to Point of Coherency.
pub unsafe fn flush_dcache(start: *const u8, len: usize) {
    let end = start.add(len);
    let mut addr = start;

    while addr < end {
        asm!("dc civac, {}", in(reg) addr, options(nostack, preserves_flags));
        addr = addr.add(64); // Typical cache line size
    }

    asm!("dsb ish", options(nostack, preserves_flags));
}
