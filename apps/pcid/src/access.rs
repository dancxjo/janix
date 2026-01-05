use thing_std::sys_pci_cfg_read32;

pub trait ConfigAccess {
    fn read8(&self, seg: u16, bus: u8, dev: u8, fun: u8, off: u16) -> u8;
    fn read16(&self, seg: u16, bus: u8, dev: u8, fun: u8, off: u16) -> u16;
    fn read32(&self, seg: u16, bus: u8, dev: u8, fun: u8, off: u16) -> u32;
}

pub struct LegacyIoAccess;

impl ConfigAccess for LegacyIoAccess {
    fn read8(&self, seg: u16, bus: u8, dev: u8, fun: u8, off: u16) -> u8 {
        unsafe { (sys_pci_cfg_read32(seg, bus, dev, fun, off & !3) >> ((off & 3) * 8)) as u8 }
    }

    fn read16(&self, seg: u16, bus: u8, dev: u8, fun: u8, off: u16) -> u16 {
        unsafe { (sys_pci_cfg_read32(seg, bus, dev, fun, off & !3) >> ((off & 3) * 8)) as u16 }
    }

    fn read32(&self, seg: u16, bus: u8, dev: u8, fun: u8, off: u16) -> u32 {
        unsafe { sys_pci_cfg_read32(seg, bus, dev, fun, off) }
    }
}

pub struct EcamAccess {
    pub base_addr: *mut u8,
    pub bus_start: u8,
    pub bus_end: u8,
}

impl ConfigAccess for EcamAccess {
    fn read8(&self, _seg: u16, bus: u8, dev: u8, fun: u8, off: u16) -> u8 {
        if bus < self.bus_start || bus > self.bus_end { return 0xFF; }
        let offset = ((bus as usize) << 20) | ((dev as usize) << 15) | ((fun as usize) << 12) | (off as usize);
        unsafe { core::ptr::read_volatile(self.base_addr.add(offset)) }
    }

    fn read16(&self, _seg: u16, bus: u8, dev: u8, fun: u8, off: u16) -> u16 {
        if bus < self.bus_start || bus > self.bus_end { return 0xFFFF; }
        let offset = ((bus as usize) << 20) | ((dev as usize) << 15) | ((fun as usize) << 12) | (off as usize);
        unsafe { core::ptr::read_volatile(self.base_addr.add(offset) as *const u16) }
    }

    fn read32(&self, _seg: u16, bus: u8, dev: u8, fun: u8, off: u16) -> u32 {
        if bus < self.bus_start || bus > self.bus_end { return 0xFFFF_FFFF; }
        let offset = ((bus as usize) << 20) | ((dev as usize) << 15) | ((fun as usize) << 12) | (off as usize);
        unsafe { core::ptr::read_volatile(self.base_addr.add(offset) as *const u32) }
    }
}
