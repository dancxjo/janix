pub mod sys {
    use abi::machine::{PORT_READ, PORT_WRITE};
    use abi::syscall::nr::SYS_MACHINE;
    use thing_std::syscall;

    pub unsafe fn out32(port: u16, val: u32) {
        syscall(SYS_MACHINE, PORT_WRITE, port as u64, val as u64, 4, 0, 0);
    }

    pub unsafe fn in32(port: u16) -> u32 {
        syscall(SYS_MACHINE, PORT_READ, port as u64, 4, 0, 0, 0).val0 as u32
    }
}

pub struct PciAddress {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
}

impl PciAddress {
    fn read(&self, offset: u8) -> u32 {
        let addr = 0x80000000
            | ((self.bus as u32) << 16)
            | ((self.device as u32) << 11)
            | ((self.function as u32) << 8)
            | ((offset as u32) & 0xFC);
        unsafe {
            sys::out32(0xCF8, addr);
            sys::in32(0xCFC)
        }
    }

    pub fn vendor_id(&self) -> u16 {
        (self.read(0x00) & 0xFFFF) as u16
    }

    pub fn device_id(&self) -> u16 {
        (self.read(0x00) >> 16) as u16
    }
    
    pub fn class_code(&self) -> u8 {
        (self.read(0x08) >> 24) as u8
    }

    pub fn subclass(&self) -> u8 {
        (self.read(0x08) >> 16) as u8
    }
    
    pub fn prog_if(&self) -> u8 {
        (self.read(0x08) >> 8) as u8
    }

    pub fn bar0(&self) -> u32 {
        self.read(0x10)
    }
}

pub fn scan() -> Option<PciAddress> {
    for bus in 0..255 {
        for dev in 0..32 {
            let addr = PciAddress { bus, device: dev, function: 0 };
            if addr.vendor_id() == 0xFFFF { continue; }
            
            // Checks...
            // Check xHCI: Class 0x0C, Subclass 0x03, ProgIF 0x30
            if addr.class_code() == 0x0C && addr.subclass() == 0x03 && addr.prog_if() == 0x30 {
               return Some(addr);
            }
        }
    }
    None
}
