//! loongarch64 machine backend.

use super::serial::Serial;
use crate::machine::{Machine, MmioFlags, MmioMapping, MmioRange};

pub struct ArchMachine {
    serial: Serial,
}

pub static ARCH_MACHINE: ArchMachine = ArchMachine::new();

impl ArchMachine {
    pub const fn new() -> Self {
        Self {
            serial: Serial::new(),
        }
    }

    pub fn init_machine(&self, _hhdm_offset: u64) {}
}

impl Machine for ArchMachine {
    fn console_write(&self, bytes: &[u8]) -> usize {
        self.serial.write(bytes);
        bytes.len()
    }

    fn mmio_map(&self, _range: MmioRange, _flags: MmioFlags) -> Option<MmioMapping> {
        None
    }
}
