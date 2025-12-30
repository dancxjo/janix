use crate::bridge::Bridge;
use kernel::bridge::CpuBridge;

pub struct EarlyUart;

impl core::fmt::Write for EarlyUart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bridge = Bridge;
        bridge.log(s);
        Ok(())
    }
}

pub fn log_heap_init(phys_start: u64, virt_start: u64, size: u64) {
    crate::bootlog!("Heap Init:");
    crate::bootlog!("  Phys: 0x{:x}", phys_start);
    crate::bootlog!("  Virt: 0x{:x}", virt_start);
    crate::bootlog!("  Size: 0x{:x}", size);
}
