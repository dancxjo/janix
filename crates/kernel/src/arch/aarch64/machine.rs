//! aarch64 machine backend.
//!
//! Provides console output through PL011 and minimal MMIO mappings without
//! assuming the HHDM covers device space.

use core::arch::asm;
use core::sync::atomic::{AtomicU64, Ordering};

use super::serial::Serial;
use crate::machine::{Machine, MmioFlags, MmioMapping, MmioRange};

const UART_PHYS: u64 = 0x0900_0000;
const UART_LEN: usize = 0x1000;
const PAGE_SIZE: u64 = 0x1000;
const MMIO_BASE: u64 = 0xffff_ffc0_0000_0000;

#[repr(C, align(4096))]
struct PageTable {
    entries: [u64; 512],
}

impl PageTable {
    const fn new() -> Self {
        Self { entries: [0; 512] }
    }
}

static mut MMIO_L1: PageTable = PageTable::new();
static mut MMIO_L2: PageTable = PageTable::new();
static mut MMIO_L3: [PageTable; 512] = [PageTable::new(); 512];

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

    fn kernel_root_table(&self) -> Option<*mut u64> {
        let ttbr1: u64;
        unsafe {
            asm!("mrs {}, ttbr1_el1", out(reg) ttbr1, options(nomem, preserves_flags));
        }

        if ttbr1 == 0 {
            return None;
        }

        let phys = ttbr1 & !0xfff;
        let virt = if self.hhdm_offset() != 0 {
            self.phys_to_virt(phys)
        } else {
            phys
        };

        Some(virt as *mut u64)
    }

    fn table_from_desc(&self, desc: u64) -> *mut u64 {
        let phys = desc & 0x0000_FFFF_FFFFF000;
        self.phys_to_virt(phys) as *mut u64
    }

    fn table_desc(&self, table: &PageTable) -> u64 {
        let phys = self.virt_to_phys(table as *const _ as u64);
        (phys & !0xfff) | 0b11
    }

    fn page_desc(&self, phys: u64, flags: MmioFlags) -> u64 {
        let mut desc = (phys & !0xfff) | 0b11;

        // AttrIndx=0 (Device), SH=inner shareable, AF=1, RW EL1, execute-never.
        desc |= 0 << 2;
        desc |= 0b11 << 8;
        desc |= 1 << 10;
        desc |= 1 << 53; // PXN
        desc |= 1 << 54; // UXN

        if !flags.contains(MmioFlags::READ) && !flags.contains(MmioFlags::WRITE) {
            // Default to readable if neither flag set to avoid accidental faults.
            desc |= 0 << 6;
        }

        desc
    }

    unsafe fn ensure_mmio_tables(&self, virt: u64) -> Option<*mut u64> {
        let l0 = self.kernel_root_table()?;
        let l0_index = ((virt >> 39) & 0x1ff) as usize;
        let l1_slot = l0.add(l0_index);
        if l1_slot.read() & 1 == 0 {
            l1_slot.write(self.table_desc(&MMIO_L1));
        }

        let l1 = self.table_from_desc(l1_slot.read());
        let l1_index = ((virt >> 30) & 0x1ff) as usize;
        let l2_slot = l1.add(l1_index);
        if l2_slot.read() & 1 == 0 {
            l2_slot.write(self.table_desc(&MMIO_L2));
        }

        let l2 = self.table_from_desc(l2_slot.read());
        let l2_index = ((virt >> 21) & 0x1ff) as usize;
        let l3_slot = l2.add(l2_index);
        if l3_slot.read() & 1 == 0 {
            l3_slot.write(self.table_desc(unsafe { &MMIO_L3[l2_index] }));
        }

        let l3 = self.table_from_desc(l3_slot.read());
        Some(l3)
    }

    unsafe fn map_range(&self, virt_base: u64, phys_base: u64, len: usize, flags: MmioFlags) -> bool {
        let l3 = match self.ensure_mmio_tables(virt_base) {
            Some(t) => t,
            None => return false,
        };

        let pages = (len as u64 + PAGE_SIZE - 1) / PAGE_SIZE;
        for i in 0..pages {
            let virt = virt_base + i * PAGE_SIZE;
            let phys = phys_base + i * PAGE_SIZE;
            let entry = l3.add(((virt >> 12) & 0x1ff) as usize);
            entry.write(self.page_desc(phys, flags));
        }

        // Ensure page table writes are visible before use.
        asm!("dsb ishst; dsb ish; isb", options(nostack, preserves_flags));

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
