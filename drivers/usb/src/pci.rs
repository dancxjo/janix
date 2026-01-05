pub mod sys {
    #[cfg(target_arch = "x86_64")]
    pub unsafe fn conf_read(bus: u8, dev: u8, func: u8, offset: u8) -> u32 {
        use abi::machine::{PORT_READ, PORT_WRITE};
        use abi::syscall::nr::SYS_MACHINE;
        use thing_std::syscall;

        let addr = 0x80000000
            | ((bus as u32) << 16)
            | ((dev as u32) << 11)
            | ((func as u32) << 8)
            | ((offset as u32) & 0xFC);

        syscall(SYS_MACHINE, PORT_WRITE, 0xCF8, addr as u64, 4, 0, 0);
        syscall(SYS_MACHINE, PORT_READ, 0xCFC, 4, 0, 0, 0).val0 as u32
    }

    #[cfg(target_arch = "aarch64")]
    static mut ECAM_BASE: u64 = 0;

    #[cfg(target_arch = "aarch64")]
    pub unsafe fn init_ecam() {
        if ECAM_BASE != 0 {
            return;
        }

        use abi::machine::MMIO_MAP;
        use abi::syscall::nr::SYS_MACHINE;
        use thing_std::{log_info, syscall};

        // QEMU Virt ECAM: 0x1000_0000, Size 256MB
        let phys = 0x1000_0000;
        let len = 0x1000_0000;
        let flags = 0xF; // RW | DEVICE | UNCACHED

        let res = syscall(SYS_MACHINE, MMIO_MAP, phys, len, flags, 0, 0);
        if res.status == 0 {
            ECAM_BASE = res.val0;
            log_info(&::alloc::format!("PCI: ECAM mapped at {:#x}", ECAM_BASE));
        } else {
            log_info("PCI: Failed to map ECAM!");
        }
    }

    #[cfg(target_arch = "aarch64")]
    pub unsafe fn conf_read(bus: u8, dev: u8, func: u8, offset: u8) -> u32 {
        if ECAM_BASE == 0 {
            init_ecam();
        }
        if ECAM_BASE == 0 {
            return 0xFFFF_FFFF;
        }

        let addr = ECAM_BASE
            + ((bus as u64) << 20)
            + ((dev as u64) << 15)
            + ((func as u64) << 12)
            + (offset as u64);
        
        (addr as *const u32).read_volatile()
    }
}

pub struct PciAddress {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
}

impl PciAddress {
    fn read(&self, offset: u8) -> u32 {
        unsafe {
            sys::conf_read(self.bus, self.device, self.function, offset)
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
    for bus in 0..2 {
        // Optimization: Checking all 255 buses on QEMU is slow and usually unnecessary for simple devices.
        // QEMU Virt usually puts devices on Bus 0.
        // We can increase this range if needed.
        for dev in 0..32 {
            let addr = PciAddress {
                bus,
                device: dev,
                function: 0,
            };
            if addr.vendor_id() == 0xFFFF {
                continue;
            }

            // Checks...
            // Check xHCI: Class 0x0C, Subclass 0x03, ProgIF 0x30
            if addr.class_code() == 0x0C && addr.subclass() == 0x03 && addr.prog_if() == 0x30 {
                return Some(addr);
            }
        }
    }
    None
}
