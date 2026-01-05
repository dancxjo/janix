use abi::machine::MMIO_MAP;
use abi::syscall::nr::SYS_MACHINE;
use thing_std::*;

pub struct XhciController {
    base: u64,
}

impl XhciController {
    pub unsafe fn new(bar0: u64) -> Result<Self, i32> {
        // Map 64KB
        let len = 64 * 1024;
        let flags = 0xF; // RW | DEVICE | UNCACHED

        // Remove bits 0-3 from BAR0 to get phys address (PCI BARs use lower bits for type)
        let phys = bar0 & !0xF;

        let res = thing_std::syscall(SYS_MACHINE, MMIO_MAP, phys, len, flags, 0, 0);

        if res.status != 0 {
            return Err(res.status as i32);
        }

        let base = res.val0;
        log_info(&alloc::format!(
            "xHCI: Mapped BAR0 {:#x} -> {:#x}",
            phys,
            base
        ));

        Ok(Self { base })
    }

    pub fn init(&mut self) {
        log_info("xHCI: Initializing...");

        // Host Controller Capability Protocol (HCCPARAMS1) is at base + 0x10 usually
        unsafe {
            let cap_len = (self.read32(0) & 0xFF) as u64;
            log_info(&alloc::format!("xHCI: CAPLENGTH = {}", cap_len));

            let op_base = self.base + cap_len;

            // USBCMD is at op_base + 0
            // USBSTS is at op_base + 4
            let sts = self.read32_offset(cap_len + 4);
            log_info(&alloc::format!("xHCI: USBSTS = {:#x}", sts));

            // Reset (USBCMD bit 1)
            let mut cmd = self.read32_offset(cap_len);
            cmd |= 2;
            self.write32_offset(cap_len, cmd);

            // Wait for reset to clear (bit 1 becomes 0)
            let mut timeout = 1000;
            while (self.read32_offset(cap_len) & 2) != 0 && timeout > 0 {
                thing_std::process::sched_yield();
                timeout -= 1;
            }

            if timeout == 0 {
                log_info("xHCI: Reset timed out");
                return;
            }

            log_info(
                "xHCI: Reset pending/complete (controller might need more setup for Run/Stop)",
            );
        }
    }

    unsafe fn read32(&self, offset: u64) -> u32 {
        ((self.base + offset) as *const u32).read_volatile()
    }

    unsafe fn read32_offset(&self, offset: u64) -> u32 {
        ((self.base + offset) as *const u32).read_volatile()
    }

    unsafe fn write32_offset(&self, offset: u64, val: u32) {
        ((self.base + offset) as *mut u32).write_volatile(val)
    }
}
