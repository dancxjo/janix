use x86_64::instructions::port::Port;

#[derive(Debug, Clone, Copy)]
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
            let mut addr_port = Port::<u32>::new(0xCF8);
            let mut data_port = Port::<u32>::new(0xCFC);
            addr_port.write(addr);
            data_port.read()
        }
    }

    pub fn vendor_id(&self) -> u16 {
        (self.read(0x00) & 0xFFFF) as u16
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

    pub fn bar0(&self) -> u64 {
        // Handle 64-bit BARs if needed, but for now assume 32-bit or lower half
        let low = self.read(0x10);
        let type_mask = 0x6; // Bit 1-2: 00=32, 10=64
        let is_64 = (low & type_mask) == 0x4;
        let base_addr = (low & 0xFFFFFFF0) as u64;
        
        if is_64 {
            let high = self.read(0x14);
            base_addr | ((high as u64) << 32)
        } else {
            base_addr
        }
    }

    pub fn interrupt_line(&self) -> u8 {
        (self.read(0x3C) & 0xFF) as u8
    }
}

pub fn scan_xhci() -> Option<PciAddress> {
    for bus in 0..255 {
        for dev in 0..32 {
            let addr = PciAddress {
                bus,
                device: dev,
                function: 0,
            };
            if addr.vendor_id() == 0xFFFF {
                continue;
            }

            // Class 0x0C (Serial Bus), Subclass 0x03 (USB), ProgIF 0x30 (xHCI)
            if addr.class_code() == 0x0C && addr.subclass() == 0x03 && addr.prog_if() == 0x30 {
                return Some(addr);
            }
        }
    }
    None
}
