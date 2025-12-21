/// Map platform-specific MMIO or PCI regions needed during boot.
/// Returns `true` when any mappings were emitted.
pub fn map_boot_device_regions() -> bool {
    #[cfg(target_arch = "aarch64")]
    {
        unsafe {
            super::aarch64::paging::map_device_region(0x3f000000, 0x01000000);
            super::aarch64::paging::map_device_region(0x10000000, 0x2effffff);
            // PL011 UART
            super::aarch64::paging::map_device_region(0x09000000, 0x1000);
        }
        return true;
    }

    #[cfg(target_arch = "x86_64")]
    {
        unsafe {
            // Map Local APIC (default base 0xFEE00000)
            super::x86_64::paging::map_device_region(0xfee00000, 0x1000);
        }
        return true;
    }

    #[allow(unreachable_code)]
    false
}

/// Initialize architecture-specific tables such as the GDT.
pub fn init_arch_tables() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        super::gdt::init();
        super::current::trap::init();
        super::current::fpu::init();
        return true;
    }

    #[allow(unreachable_code)]
    false
}
