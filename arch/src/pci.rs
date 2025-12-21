#[cfg(target_arch = "aarch64")]
const AARCH64_PCI_ECAM_BASE: u64 = 0x3f000000;

macro_rules! pci_log {
    ($($arg:tt)*) => {
        {
            #[cfg(feature = "arch_pci_in_kernel")]
            {
                crate::println!($($arg)*);
            }
            #[cfg(not(feature = "arch_pci_in_kernel"))]
            {
                kernel::println!($($arg)*);
            }
        }
    };
}

pub fn read_config_u32(bus: u8, slot: u8, func: u8, offset: u16, hhdm_offset: u64) -> Option<u32> {
    #[cfg(target_arch = "aarch64")]
    {
        let phys = AARCH64_PCI_ECAM_BASE
            + ((bus as u64) << 20)
            + ((slot as u64) << 15)
            + ((func as u64) << 12)
            + (offset as u64);
        let virt = phys.wrapping_add(hhdm_offset);
        // (page-table dump removed; not available when this file is included into kernel crate)
        pci_log!(
            "arch::pci read_config_u32: bus={} slot={} func={} offset={:#x} phys={:#x} virt={:#x} hhdm={:#x}",
            bus,
            slot,
            func,
            offset,
            phys,
            virt,
            hhdm_offset,
        );
        Some(unsafe { (virt as *const u32).read_volatile() })
    }

    #[cfg(target_arch = "x86_64")]
    {
        let _ = hhdm_offset;
        use x86_64::instructions::port::Port;
        let address = 0x80000000 | ((bus as u32) << 16) | ((slot as u32) << 11) | ((func as u32) << 8) | ((offset as u32) & 0xFC);
        unsafe {
            let mut config_address_port = Port::<u32>::new(0xCF8);
            let mut config_data_port = Port::<u32>::new(0xCFC);
            config_address_port.write(address);
            Some(config_data_port.read())
        }
    }

    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    {
        let _ = (bus, slot, func, offset, hhdm_offset);
        None
    }
}

pub fn read_config_u16(bus: u8, slot: u8, func: u8, offset: u16, hhdm_offset: u64) -> Option<u16> {
    #[cfg(target_arch = "aarch64")]
    {
        let phys = AARCH64_PCI_ECAM_BASE
            + ((bus as u64) << 20)
            + ((slot as u64) << 15)
            + ((func as u64) << 12)
            + (offset as u64);
        let virt = phys.wrapping_add(hhdm_offset);
        // (page-table dump removed; not available when this file is included into kernel crate)
        pci_log!(
            "arch::pci read_config_u16: bus={} slot={} func={} offset={:#x} phys={:#x} virt={:#x} hhdm={:#x}",
            bus,
            slot,
            func,
            offset,
            phys,
            virt,
            hhdm_offset,
        );
        Some(unsafe { (virt as *const u16).read_volatile() })
    }

    #[cfg(target_arch = "x86_64")]
    {
        let val = read_config_u32(bus, slot, func, offset & !3, hhdm_offset)?;
        let shift = (offset & 3) * 8;
        Some((val >> shift) as u16)
    }

    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    {
        let _ = (bus, slot, func, offset, hhdm_offset);
        None
    }
}

pub fn read_config_u8(bus: u8, slot: u8, func: u8, offset: u16, hhdm_offset: u64) -> Option<u8> {
    #[cfg(target_arch = "aarch64")]
    {
        let phys = AARCH64_PCI_ECAM_BASE
            + ((bus as u64) << 20)
            + ((slot as u64) << 15)
            + ((func as u64) << 12)
            + (offset as u64);
        let virt = phys.wrapping_add(hhdm_offset);
        // (page-table dump removed; not available when this file is included into kernel crate)
        pci_log!(
            "arch::pci read_config_u8: bus={} slot={} func={} offset={:#x} phys={:#x} virt={:#x} hhdm={:#x}",
            bus,
            slot,
            func,
            offset,
            phys,
            virt,
            hhdm_offset,
        );
        Some(unsafe { (virt as *const u8).read_volatile() })
    }

    #[cfg(target_arch = "x86_64")]
    {
        let val = read_config_u32(bus, slot, func, offset & !3, hhdm_offset)?;
        let shift = (offset & 3) * 8;
        Some((val >> shift) as u8)
    }

    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    {
        let _ = (bus, slot, func, offset, hhdm_offset);
        None
    }
}
