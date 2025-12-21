#![no_std]
#![feature(allocator_api)]

extern crate alloc;

use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PciDevice {
    pub bus: u8,
    pub slot: u8,
    pub func: u8,
    pub class: u8,
    pub subclass: u8,
    pub prog_if: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub header_type: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum Bar {
    Memory32(u32),
    Memory64(u64),
    Io(u16),
}

/// High-level PCI interface used by drivers (e.g. USB)
pub trait Pci {
    fn scan(&self) -> Vec<PciDevice>;
    fn read_bar(&self, dev: PciDevice, index: u8) -> Bar;
}

/// Low-level PCI Config Space Access interface provided by Kernel/Arch
pub trait PciConfigAccess {
    fn read_u8(&self, bus: u8, slot: u8, func: u8, offset: u16) -> u8;
    fn read_u16(&self, bus: u8, slot: u8, func: u8, offset: u16) -> u16;
    fn read_u32(&self, bus: u8, slot: u8, func: u8, offset: u16) -> u32;
}

pub trait MmioMapper {
    /// Map a physical region to virtual memory.
    /// Returns a pointer to the start of the mapped region.
    ///
    /// # Safety
    /// Caller must ensure physical address is valid for MMIO.
    unsafe fn map_mmio(&self, phys: u64, size: u64) -> *mut u8;
}
