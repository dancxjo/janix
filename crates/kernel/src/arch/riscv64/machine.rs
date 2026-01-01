//! riscv64 machine backend.
//!
//! Provides console output through the 16550 UART and minimal MMIO mappings
//! without assuming HHDM coverage.

use core::arch::asm;
use core::sync::atomic::{AtomicU64, Ordering};

use super::serial::Serial;
use crate::machine::{Machine, MmioFlags, MmioMapping, MmioRange};

const UART_PHYS: u64 = 0x1000_0000;
const UART_LEN: usize = 0x1000;
const PAGE_SIZE: u64 = 0x1000;
const MMIO_BASE: u64 = 0xffff_ffc0_0000_0000;

#[repr(C, align(4096))]
struct PageTable {
    entries: [usize; 512],
}

impl PageTable {
    const fn new() -> Self {
        Self { entries: [0; 512] }
    }
}

static mut MMIO_L1: PageTable = PageTable::new();
static mut MMIO_L0: [PageTable; 512] = [PageTable::new(); 512];

pub struct ArchMachine {
    serial: Serial,
    hhdm_offset: AtomicU64,
    uart_base: AtomicU64,
}

pub static ARCH_MACHINE: ArchMachine = ArchMachine::new();

impl ArchMachine {
    pub const fn new() -> Self {
        Self {
            serial: Serial::new(),
            hhdm_offset: AtomicU64::new(0),
            uart_base: AtomicU64::new(0),
        }
    }

    pub fn init_machine(&self, hhdm_offset: u64) {
        self.hhdm_offset.store(hhdm_offset, Ordering::Relaxed);
    }

    fn hhdm_offset(&self) -> u64 {
        self.hhdm_offset.load(Ordering::Relaxed)
    }

    fn phys_to_virt(&self, phys: u64) -> u64 {
        phys.wrapping_add(self.hhdm_offset())
    }

    fn virt_to_phys(&self, virt: u64) -> u64 {
        virt.wrapping_sub(self.hhdm_offset())
    }

    fn kernel_root_table(&self) -> Option<*mut usize> {
        let satp: usize;
        unsafe {
            asm!("csrr {}, satp", out(reg) satp, options(nomem, preserves_flags));
        }

        let mode = satp >> 60;
        if mode != 8 {
            // Expect Sv39
            return None;
        }

        let ppn = satp & ((1usize << 44) - 1);
        let phys = (ppn as u64) << 12;
        let virt = if self.hhdm_offset() != 0 {
            self.phys_to_virt(phys)
        } else {
            phys
        };

        Some(virt as *mut usize)
    }

    fn table_pte(&self, table: &PageTable) -> usize {
        let phys = self.virt_to_phys(table as *const _ as u64);
        (((phys >> 12) as usize) << 10) | 0b001
    }

    fn leaf_pte(&self, phys: u64, flags: MmioFlags) -> usize {
        let mut pte = ((phys >> 12) as usize) << 10;
        pte |= 1; // V
        pte |= 1 << 5; // G
        pte |= 1 << 6; // A
        pte |= 1 << 7; // D

        if flags.contains(MmioFlags::READ) || flags.contains(MmioFlags::WRITE) {
            pte |= 1 << 1; // R
        }
        if flags.contains(MmioFlags::WRITE) {
            pte |= 1 << 2; // W
        }

        pte
    }

    unsafe fn ensure_mmio_tables(&self, virt: u64) -> Option<*mut usize> {
        let root = self.kernel_root_table()?;
        let vpn2 = ((virt >> 30) & 0x1ff) as usize;
        let l1_slot = root.add(vpn2);
        if l1_slot.read() & 1 == 0 {
            l1_slot.write(self.table_pte(&MMIO_L1));
        }

        let l1 = self.phys_to_virt(((l1_slot.read() >> 10) << 12) as u64) as *mut usize;
        let vpn1 = ((virt >> 21) & 0x1ff) as usize;
        let l0_slot = l1.add(vpn1);
        if l0_slot.read() & 1 == 0 {
            l0_slot.write(self.table_pte(unsafe { &MMIO_L0[vpn1] }));
        }

        let l0 = self.phys_to_virt(((l0_slot.read() >> 10) << 12) as u64) as *mut usize;
        Some(l0)
    }

    unsafe fn map_range(&self, virt_base: u64, phys_base: u64, len: usize, flags: MmioFlags) -> bool {
        let l0 = match self.ensure_mmio_tables(virt_base) {
            Some(t) => t,
            None => return false,
        };

        let pages = (len as u64 + PAGE_SIZE - 1) / PAGE_SIZE;
        for i in 0..pages {
            let virt = virt_base + i * PAGE_SIZE;
            let phys = phys_base + i * PAGE_SIZE;
            let entry = l0.add(((virt >> 12) & 0x1ff) as usize);
            entry.write(self.leaf_pte(phys, flags));
        }

        asm!("sfence.vma", options(nostack, preserves_flags));

        true
    }

    fn map_mmio(&self, range: MmioRange, flags: MmioFlags) -> Option<MmioMapping> {
        let aligned_phys = range.phys & !(PAGE_SIZE - 1);
        let offset = (range.phys - aligned_phys) as usize;
        let map_len = range.len + offset;
        let virt_base = MMIO_BASE + aligned_phys;

        let mapped = unsafe { self.map_range(virt_base, aligned_phys, map_len, flags) };
        if !mapped {
            return None;
        }

        Some(MmioMapping {
            virt: virt_base + offset as u64,
            len: range.len,
        })
    }

    fn ensure_uart(&self) -> Option<u64> {
        let current = self.uart_base.load(Ordering::Relaxed);
        if current != 0 {
            return Some(current);
        }

        let flags = MmioFlags::DEVICE | MmioFlags::UNCACHED | MmioFlags::READ | MmioFlags::WRITE;
        let mapping = self.map_mmio(
            MmioRange {
                phys: UART_PHYS,
                len: UART_LEN,
            },
            flags,
        )?;

        self.serial.init(mapping.virt);
        self.uart_base.store(mapping.virt, Ordering::Relaxed);

        Some(mapping.virt)
    }
}

impl Machine for ArchMachine {
    fn console_write(&self, bytes: &[u8]) -> usize {
        if self.ensure_uart().is_none() {
            return 0;
        }
        self.serial.write(bytes);
        bytes.len()
    }

    fn mmio_map(&self, range: MmioRange, flags: MmioFlags) -> Option<MmioMapping> {
        self.map_mmio(range, flags)
    }
}
