//! aarch64 machine backend.
//!
//! Provides console output through PL011 and minimal MMIO mappings without
//! assuming the HHDM covers device space.

use core::arch::asm;
use core::arch::global_asm;

global_asm!(include_str!("switch.S"));
global_asm!(include_str!("vectors.S"));
use core::sync::atomic::{AtomicU64, Ordering};

mod exception;
mod exception;
mod serial;
pub mod gic;
pub mod timer;
pub mod abi;

// TrapFrame alias for generic Scheduler usage
pub type TrapFrame = exception::ExceptionContext;
use serial::Serial;
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
static mut MMIO_L3: [PageTable; 512] = [const { PageTable::new() }; 512];
static mut BOOT_L0: PageTable = PageTable::new();

pub struct ArchMachine {
    serial: Serial,
    hhdm_offset: AtomicU64,
    kernel_phys_base: AtomicU64,
    kernel_virt_base: AtomicU64,
    uart_base: AtomicU64,
}

static ARCH_MACHINE_IMPL: ArchMachine = ArchMachine::new();
pub static ARCH_MACHINE: &'static dyn Machine = &ARCH_MACHINE_IMPL;

impl ArchMachine {
    pub const fn new() -> Self {
        Self {
            serial: Serial::new(),
            hhdm_offset: AtomicU64::new(0),
            kernel_phys_base: AtomicU64::new(0),
            kernel_virt_base: AtomicU64::new(0),
            uart_base: AtomicU64::new(0),
        }
    }

    pub fn init_machine(&self, info: crate::machine::PreBootInfo) {
        self.hhdm_offset.store(info.hhdm_offset, Ordering::Relaxed);
        self.kernel_phys_base.store(info.kernel_phys_base, Ordering::Relaxed);
        self.kernel_virt_base.store(info.kernel_virt_base, Ordering::Relaxed);
        
        // Install VBAR_EL1
        extern "C" {
             static aarch64_vectors: u8; // Symbol
        }
        unsafe {
             let vectors_addr = core::ptr::addr_of!(aarch64_vectors) as u64;
             asm!("msr vbar_el1, {}", in(reg) vectors_addr, options(nomem, preserves_flags));
             asm!("msr vbar_el1, {}", in(reg) vectors_addr, options(nomem, preserves_flags));
             
             // Initialize GIC and Timer
             gic::init();
             timer::init();
        }
    }

    fn hhdm_offset(&self) -> u64 {
        self.hhdm_offset.load(Ordering::Relaxed)
    }

    fn kernel_phys_base(&self) -> u64 {
        self.kernel_phys_base.load(Ordering::Relaxed)
    }

    fn kernel_virt_base(&self) -> u64 {
        self.kernel_virt_base.load(Ordering::Relaxed)
    }

    /// Translate HHDM-mapped virtual address to physical
    fn phys_to_virt(&self, phys: u64) -> u64 {
        phys.wrapping_add(self.hhdm_offset())
    }

    /// Translate HHDM virtual address back to physical
    #[allow(dead_code)]
    fn virt_to_phys(&self, virt: u64) -> u64 {
        virt.wrapping_sub(self.hhdm_offset())
    }

    /// Translate kernel virtual address (BSS/data segment) to physical
    fn kernel_virt_to_phys(&self, virt: u64) -> u64 {
        // kernel physical = kernel virtual - virt_base + phys_base
        virt.wrapping_sub(self.kernel_virt_base())
            .wrapping_add(self.kernel_phys_base())
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
        // table is a kernel BSS address, not an HHDM address
        let phys = self.kernel_virt_to_phys(table as *const _ as u64);
        (phys & !0xfff) | 0b11
    }

    fn page_desc(&self, phys: u64, flags: MmioFlags) -> u64 {
        let mut desc = (phys & !0xfff) | 0b11;

        // AttrIndx=2 (Device), SH=inner shareable, AF=1, RW EL1, execute-never.
        // Limine sets MAIR indices 0/1 to Normal 0xFF, and 2..7 to 0x00 (Device).
        // So we must use index 2 for Device-nGnRnE.
        desc |= 2 << 2; // Original Device-nGnRnE assumption
        // desc |= 0 << 2; // Debug fallback: Use Index 0 (Normal) to avoid potential MAIR mismatch
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
        let current_ttbr1: u64;
        asm!("mrs {}, ttbr1_el1", out(reg) current_ttbr1, options(nomem, preserves_flags));

        let boot_l0_virt = core::ptr::addr_of_mut!(BOOT_L0);
        let boot_l0_phys = self.kernel_virt_to_phys(boot_l0_virt as u64);

        let _l0_phys = if (current_ttbr1 & !0xfff) != (boot_l0_phys & !0xfff) {
            // Need to switch to our own L0 table
            let old_l0_phys = current_ttbr1 & !0xfff;
            let old_l0_virt = self.phys_to_virt(old_l0_phys) as *const u64;
            
            // Copy entries
            core::ptr::copy_nonoverlapping(old_l0_virt, boot_l0_virt as *mut u64, 512);
            
            // Switch TTBR1
            // Use current flags but lower address bits mapped to new table
            let new_ttbr1_val = (current_ttbr1 & 0xFFFF000000000FFF) | (boot_l0_phys & 0x0000FFFFFFFFF000);
            
            asm!("msr ttbr1_el1, {}", in(reg) new_ttbr1_val, options(nomem, preserves_flags));
            asm!("isb; tlbi vmalle1; dsb ish; isb", options(nostack, preserves_flags));
            
            boot_l0_phys
        } else {
             current_ttbr1 & !0xfff
        };

        // Now we are using BOOT_L0 (or already were)
        let l0 = boot_l0_virt as *mut u64;
        
        let l0_index = ((virt >> 39) & 0x1ff) as usize;
        let l1_slot = l0.add(l0_index);
        if l1_slot.read() & 1 == 0 {
            l1_slot.write(self.table_desc(unsafe { &*core::ptr::addr_of!(MMIO_L1) }));
        }

        let l1 = self.table_from_desc(l1_slot.read());
        let l1_index = ((virt >> 30) & 0x1ff) as usize;
        let l2_slot = l1.add(l1_index);
        if l2_slot.read() & 1 == 0 {
            l2_slot.write(self.table_desc(unsafe { &*core::ptr::addr_of!(MMIO_L2) }));
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

extern "C" {
    fn aarch64_switch_to(old_sp: *mut u64, new_sp: *const u64);
    fn aarch64_task_entry_stub() -> !;
}

impl Machine for ArchMachine {
    fn init(&self, info: crate::machine::PreBootInfo) {
        self.init_machine(info);
    }

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

    fn irq_disable(&self) -> u64 {
        let flags: u64;
        unsafe {
            asm!("mrs {}, daif; msr daifset, #0xf", out(reg) flags, options(nomem, preserves_flags));
        }
        flags
    }

    fn irq_restore(&self, token: u64) {
        unsafe {
            asm!("msr daif, {}", in(reg) token, options(nomem, preserves_flags));
        }
    }

    fn halt(&self) -> ! {
        loop {
            unsafe { asm!("wfi"); }
        }
    }

    fn idle(&self) {
        unsafe { asm!("wfi"); }
    }

    fn switch_to(&self, old_ctx: &mut crate::machine::Context, new_ctx: &crate::machine::Context) {
        unsafe {
            // Context has `sp` as first field (u64).
            // Matches user's cast: &mut old_ctx.sp as *mut u64
            aarch64_switch_to(&mut old_ctx.sp as *mut u64, &new_ctx.sp as *const u64);
        }
    }

    fn task_entry_stub(&self) -> u64 {
        aarch64_task_entry_stub as usize as u64
    }
}
