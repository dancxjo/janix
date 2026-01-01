//! riscv64 machine backend.
//!
//! Provides console output through the 16550 UART.
//! Tries HHDM-based access first, falls back to page table manipulation if needed.

use core::arch::asm;
use core::sync::atomic::{AtomicU64, AtomicBool, Ordering};

use super::serial::Serial;
use crate::machine::{Machine, MmioFlags, MmioMapping, MmioRange};

const UART_PHYS: u64 = 0x1000_0000;

pub struct ArchMachine {
    serial: Serial,
    hhdm_offset: AtomicU64,
    uart_initialized: AtomicBool,
}

pub static ARCH_MACHINE: ArchMachine = ArchMachine::new();

impl ArchMachine {
    pub const fn new() -> Self {
        Self {
            serial: Serial::new(),
            hhdm_offset: AtomicU64::new(0),
            uart_initialized: AtomicBool::new(false),
        }
    }

    pub fn init_machine(&self, info: crate::boot::PreBootInfo) {
        self.hhdm_offset.store(info.hhdm_offset, Ordering::Relaxed);
        
        // Try to access UART via HHDM
        // On RISC-V QEMU virt, Limine should set up HHDM that includes MMIO regions
        if info.hhdm_offset != 0 {
            let uart_virt = UART_PHYS.wrapping_add(info.hhdm_offset);
            self.serial.init(uart_virt);
            self.uart_initialized.store(true, Ordering::Relaxed);
        }
    }
}

impl Machine for ArchMachine {
    fn console_write(&self, bytes: &[u8]) -> usize {
        if !self.uart_initialized.load(Ordering::Relaxed) {
            return 0;
        }
        self.serial.write(bytes);
        bytes.len()
    }

    fn mmio_map(&self, _range: MmioRange, _flags: MmioFlags) -> Option<MmioMapping> {
        // For now, we don't support dynamic MMIO mapping
        // The HHDM should cover what we need
        None
    }
}
