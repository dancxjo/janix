//! Port I/O helpers for x86_64.
//!
//! These functions provide raw port I/O access.

use core::arch::asm;

/// Read a byte from a port.
///
/// # Safety
/// Port access is inherently unsafe.
pub unsafe fn read8(addr_or_port: u64) -> u8 {
    let port = addr_or_port as u16;
    let val: u8;
    asm!("in al, dx", out("al") val, in("dx") port, options(nomem, nostack, preserves_flags));
    val
}

/// Read a word from a port.
///
/// # Safety
/// Port access is inherently unsafe.
pub unsafe fn read16(addr_or_port: u64) -> u16 {
    let port = addr_or_port as u16;
    let val: u16;
    asm!("in ax, dx", out("ax") val, in("dx") port, options(nomem, nostack, preserves_flags));
    val
}

/// Read a dword from a port.
///
/// # Safety
/// Port access is inherently unsafe.
pub unsafe fn read32(addr_or_port: u64) -> u32 {
    let port = addr_or_port as u16;
    let val: u32;
    asm!("in eax, dx", out("eax") val, in("dx") port, options(nomem, nostack, preserves_flags));
    val
}

/// Write a byte to a port.
///
/// # Safety
/// Port access is inherently unsafe.
pub unsafe fn write8(addr_or_port: u64, v: u8) {
    let port = addr_or_port as u16;
    asm!("out dx, al", in("dx") port, in("al") v, options(nomem, nostack, preserves_flags));
}

/// Write a word to a port.
///
/// # Safety
/// Port access is inherently unsafe.
pub unsafe fn write16(addr_or_port: u64, v: u16) {
    let port = addr_or_port as u16;
    asm!("out dx, ax", in("dx") port, in("ax") v, options(nomem, nostack, preserves_flags));
}

/// Write a dword to a port.
///
/// # Safety
/// Port access is inherently unsafe.
pub unsafe fn write32(addr_or_port: u64, v: u32) {
    let port = addr_or_port as u16;
    asm!("out dx, eax", in("dx") port, in("eax") v, options(nomem, nostack, preserves_flags));
}
