use bridge_aarch64::Bridge;
use kernel::bridge::HardwareBridge;

// Early print macro that doesn't rely on global logger or allocator
#[macro_export]
macro_rules! bootlog {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let mut sink = $crate::early_log::EarlyUart;
        let _ = writeln!(sink, $($arg)*);
    }};
}

pub struct EarlyUart;

impl core::fmt::Write for EarlyUart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bridge = Bridge;
        bridge.log(s);
        Ok(())
    }
}

pub fn log_heap_init(phys_start: u64, virt_start: u64, size: u64) {
    bootlog!("Heap Init:");
    bootlog!("  Phys: 0x{:x}", phys_start);
    bootlog!("  Virt: 0x{:x}", virt_start);
    bootlog!("  Size: 0x{:x}", size);
}
