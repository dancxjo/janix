use crate::memory::phys_to_virt;
use alloc::vec::Vec;

// QEMU virt default for aarch64
#[cfg(target_arch = "aarch64")]
pub const PCI_ECAM_BASE: u64 = 0x3f000000;

// For x86_64, we might use IO ports, but let's stick to aarch64 for now as requested.
#[cfg(not(target_arch = "aarch64"))]
pub const PCI_ECAM_BASE: u64 = 0; // Placeholder

#[derive(Debug, Clone)]
pub struct PciDevice {
    pub bus: u8,
    pub slot: u8,
    pub func: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class: u8,
    pub subclass: u8,
    pub prog_if: u8,
    pub header_type: u8,
}

fn read_config_u32(bus: u8, slot: u8, func: u8, offset: u16) -> u32 {
    #[cfg(target_arch = "aarch64")]
    {
        let phys = PCI_ECAM_BASE
            + ((bus as u64) << 20)
            + ((slot as u64) << 15)
            + ((func as u64) << 12)
            + (offset as u64);
        let virt = phys_to_virt(phys);
        unsafe { (virt as *const u32).read_volatile() }
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let _ = (bus, slot, func, offset);
        0
    }
}

fn read_config_u16(bus: u8, slot: u8, func: u8, offset: u16) -> u16 {
    #[cfg(target_arch = "aarch64")]
    {
        let phys = PCI_ECAM_BASE
            + ((bus as u64) << 20)
            + ((slot as u64) << 15)
            + ((func as u64) << 12)
            + (offset as u64);
        let virt = phys_to_virt(phys);
        unsafe { (virt as *const u16).read_volatile() }
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let _ = (bus, slot, func, offset);
        0
    }
}

fn read_config_u8(bus: u8, slot: u8, func: u8, offset: u16) -> u8 {
    #[cfg(target_arch = "aarch64")]
    {
        let phys = PCI_ECAM_BASE
            + ((bus as u64) << 20)
            + ((slot as u64) << 15)
            + ((func as u64) << 12)
            + (offset as u64);
        let virt = phys_to_virt(phys);
        unsafe { (virt as *const u8).read_volatile() }
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let _ = (bus, slot, func, offset);
        0
    }
}


pub fn read_bar(bus: u8, slot: u8, func: u8, bar_index: u8) -> u32 {
    read_config_u32(bus, slot, func, 0x10 + (bar_index as u16) * 4)
}

pub fn scan_pci() -> Vec<PciDevice> {
    let mut devices = Vec::new();
    
    #[cfg(target_arch = "aarch64")]
    for bus in 0..=0 { // Just scan bus 0 for now
        for slot in 0..32 {
            let vendor_id = read_config_u16(bus, slot, 0, 0);
            if vendor_id == 0xFFFF { continue; }
            
            let header_type = read_config_u8(bus, slot, 0, 0x0E);
            let func_count = if header_type & 0x80 != 0 { 8 } else { 1 };
            
            for func in 0..func_count {
                let vendor_id = read_config_u16(bus, slot, func, 0);
                if vendor_id == 0xFFFF { continue; }
                
                let device_id = read_config_u16(bus, slot, func, 2);
                let class = read_config_u8(bus, slot, func, 0x0B);
                let subclass = read_config_u8(bus, slot, func, 0x0A);
                let prog_if = read_config_u8(bus, slot, func, 0x09);
                
                devices.push(PciDevice {
                    bus, slot, func, vendor_id, device_id, class, subclass, prog_if, header_type
                });
            }
        }
    }
    devices
}
