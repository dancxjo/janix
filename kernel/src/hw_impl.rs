use hw::{PciConfigAccess, MmioMapper};
use crate::memory::phys_to_virt;

pub struct KernelPciConfigAccess;

#[cfg(target_arch = "x86_64")]
mod pci_impl {
    use x86_64::instructions::port::Port;

    const PCI_CONFIG_ADDRESS: u16 = 0xCF8;
    const PCI_CONFIG_DATA: u16 = 0xCFC;

    unsafe fn write_addr(bus: u8, slot: u8, func: u8, offset: u16) {
        let addr = 0x80000000
            | ((bus as u32) << 16)
            | ((slot as u32) << 11)
            | ((func as u32) << 8)
            | (offset as u32 & 0xFC);
        let mut port = Port::<u32>::new(PCI_CONFIG_ADDRESS);
        unsafe { port.write(addr); }
    }

    pub fn read_u8(bus: u8, slot: u8, func: u8, offset: u16) -> u8 {
        unsafe {
            write_addr(bus, slot, func, offset);
            let mut port = Port::<u8>::new(PCI_CONFIG_DATA + (offset & 3));
            port.read()
        }
    }

    pub fn read_u16(bus: u8, slot: u8, func: u8, offset: u16) -> u16 {
        unsafe {
            write_addr(bus, slot, func, offset);
            let mut port = Port::<u16>::new(PCI_CONFIG_DATA + (offset & 2));
            port.read()
        }
    }

    pub fn read_u32(bus: u8, slot: u8, func: u8, offset: u16) -> u32 {
        unsafe {
            write_addr(bus, slot, func, offset);
            let mut port = Port::<u32>::new(PCI_CONFIG_DATA);
            port.read()
        }
    }
}

#[cfg(not(target_arch = "x86_64"))]
mod pci_impl {
    pub fn read_u8(_bus: u8, _slot: u8, _func: u8, _offset: u16) -> u8 { 0xFF }
    pub fn read_u16(_bus: u8, _slot: u8, _func: u8, _offset: u16) -> u16 { 0xFFFF }
    pub fn read_u32(_bus: u8, _slot: u8, _func: u8, _offset: u16) -> u32 { 0xFFFFFFFF }
}

impl PciConfigAccess for KernelPciConfigAccess {
    fn read_u8(&self, bus: u8, slot: u8, func: u8, offset: u16) -> u8 {
        pci_impl::read_u8(bus, slot, func, offset)
    }

    fn read_u16(&self, bus: u8, slot: u8, func: u8, offset: u16) -> u16 {
        pci_impl::read_u16(bus, slot, func, offset)
    }

    fn read_u32(&self, bus: u8, slot: u8, func: u8, offset: u16) -> u32 {
        pci_impl::read_u32(bus, slot, func, offset)
    }
}

pub struct KernelMmio;

impl MmioMapper for KernelMmio {
    unsafe fn map_mmio(&self, phys: u64, _size: u64) -> *mut u8 {
        // In this kernel, phys_to_virt maps to a direct map region (HHDM).
        // It returns a u64 virtual address.
        let virt = phys_to_virt(phys);
        virt as *mut u8
    }
}
