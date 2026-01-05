//! aarch64 machine backend.
//!
//! Provides console output through PL011 and minimal MMIO mappings without
//! assuming the HHDM covers device space.

use core::arch::asm;
use core::arch::global_asm;

global_asm!(include_str!("switch.S"));
global_asm!(include_str!("vectors.S"));

use core::sync::atomic::{AtomicU64, Ordering};

pub mod abi;
pub mod exception;

pub mod context;
pub mod gic;
pub mod mmu;
pub mod percpu;
mod serial;
pub mod timer;
pub mod usb;
pub mod simd;
pub use mmu::AddressSpace;
use percpu::ArchPerCpu;

// TrapFrame alias for generic Scheduler usage
pub type TrapFrame = exception::ExceptionContext;
use crate::machine::{BootColor, Machine, MmioFlags, MmioMapping, MmioRange};
use serial::Serial;

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

#[no_mangle]
pub extern "C" fn task_dispatch(_dispatch_ptr: u64, entry: extern "C" fn()) {
    entry();
}

pub struct ArchMachine {
    serial: Serial,
    hhdm_offset: AtomicU64,
    kernel_phys_base: AtomicU64,
    kernel_virt_base: AtomicU64,
    uart_base: AtomicU64,
}

pub static mut PERCPU_BSP: Option<ArchPerCpu> = None;

#[repr(C, align(16))]
pub struct ExceptionStack([u8; 16384]);

#[no_mangle]
pub static mut BSP_EXCEPTION_STACK: ExceptionStack = ExceptionStack([0; 16384]);

#[no_mangle]
pub static mut exception_stack_top: u64 = 0; // Will be set in init_machine

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
        // 1. Set constants first!
        self.hhdm_offset.store(info.hhdm_offset, Ordering::Relaxed);
        self.kernel_phys_base
            .store(info.kernel_phys_base, Ordering::Relaxed);
        self.kernel_virt_base
            .store(info.kernel_virt_base, Ordering::Relaxed);

        // 2. Map early console (UART)
        // On QEMU virt, UART is at 0x09000000
        let uart_phys = 0x0900_0000;
        let uart_map = self
            .map_mmio(
                MmioRange {
                    phys: uart_phys,
                    len: 0x1000,
                },
                MmioFlags::READ | MmioFlags::WRITE | MmioFlags::DEVICE,
            )
            .expect("UART map fail");

        self.serial.init(uart_map.virt);
        self.uart_base.store(uart_map.virt, Ordering::Relaxed);

        // 3. Install VBAR_EL1
        extern "C" {
            static aarch64_vectors: u8; // Symbol
        }
        unsafe {
            let vectors_addr = core::ptr::addr_of!(aarch64_vectors) as u64;
            asm!("msr vbar_el1, {}", in(reg) vectors_addr, options(nomem, preserves_flags));
            asm!("isb", options(nomem, preserves_flags));
        }

        // 4. Initialize Hardware
        unsafe {
            // Initialize GIC (Map Disributor and CPU Interface)
            let flags = MmioFlags::READ | MmioFlags::WRITE | MmioFlags::DEVICE;
            let gicd_map = self
                .map_mmio(
                    MmioRange {
                        phys: gic::GICD_PHYS,
                        len: 0x1000,
                    },
                    flags,
                )
                .expect("GICD map fail");
            let gicc_map = self
                .map_mmio(
                    MmioRange {
                        phys: gic::GICC_PHYS,
                        len: 0x1000,
                    },
                    flags,
                )
                .expect("GICC map fail");

            gic::init(gicd_map.virt, gicc_map.virt);

            // Initialize PerCpu
            let cpu_thing = ::abi::ids::ThingId(0); // TODO: Real ID
            let rq_thing = ::abi::ids::ThingId(0);
            PERCPU_BSP = Some(ArchPerCpu::new(0, cpu_thing, rq_thing));

            let stack_top = (&raw const BSP_EXCEPTION_STACK as u64) + 16384;
            exception_stack_top = stack_top;

            #[allow(static_mut_refs)]
            if let Some(ref mut pc) = PERCPU_BSP {
                pc.core.exception_stack_ptr = stack_top;
                crate::serial::write(b"INIT: EXC_STACK set to ");
                crate::serial::write_hex(stack_top);
                crate::serial::write(b"\n");
                percpu::init_percpu(pc);
                let t: u64;
                core::arch::asm!("mrs {}, tpidr_el1", out(reg) t);
                crate::serial::write(b"INIT: TPIDR_EL1 set to ");
                crate::serial::write_hex(t);
                crate::serial::write(b"\n");
            }

            // Initialize Timer
            timer::init();

            // Enable xHCI IRQ (SPI 0 = IRQ 32)
            gic::enable_irq(32);
            gic::set_priority(32, 0xF0);
            gic::set_group1(32);
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

    pub fn kernel_root_table(&self) -> Option<*mut u64> {
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

        if flags.contains(MmioFlags::DEVICE) {
            // AttrIndx=2 (Device-nGnRnE)
            desc |= 2 << 2;
        } else {
            // AttrIndx=0 (Normal Writeback)
            desc |= 0 << 2;
            // Mark as Inner Shareable explicitly (bit 8,9 -> 11)
            desc |= 0b11 << 8;
        }

        desc |= 1 << 10; // AF=1

        if !flags.contains(MmioFlags::READ) && !flags.contains(MmioFlags::WRITE) {
            // Default to readable if unspecified? No, strict.
            // But existing code did: "Default to readable if neither flag set"
            desc |= 0 << 6;
        }

        // Execute Permissions
        desc |= 1 << 53; // PXN (Privileged Execute-Never) - Default to true for now
        desc |= 1 << 54; // UXN (Unprivileged Execute-Never)

        // TODO: If we want Exec, we need MmioFlags::EXEC.
        // For now, Heap is NX.

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

            if self.hhdm_offset() != 0 {
                let old_l0_virt = self.phys_to_virt(old_l0_phys) as *const u64;
                // Copy entries
                core::ptr::copy_nonoverlapping(old_l0_virt, boot_l0_virt as *mut u64, 512);

                // Switch TTBR1
                let new_ttbr1_val = (current_ttbr1 & 0x0000_0000_0000_0FFF) | boot_l0_phys;
                asm!("msr ttbr1_el1, {}", in(reg) new_ttbr1_val, options(nomem, preserves_flags));
                asm!(
                    "isb; tlbi vmalle1; dsb ish; isb",
                    options(nostack, preserves_flags)
                );
                boot_l0_phys
            } else {
                current_ttbr1 & !0xfff
            }
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

    unsafe fn map_range(
        &self,
        virt_base: u64,
        phys_base: u64,
        len: usize,
        flags: MmioFlags,
    ) -> bool {
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

        extern "C" {
            static aarch64_vectors: u8;
        }
        let vectors_addr = core::ptr::addr_of!(aarch64_vectors) as u64;
        crate::serial::write(b"VBAR=");
        crate::serial::write_hex(vectors_addr);
        crate::serial::write(b" ");

        let daif: u64;
        let el: u64;
        unsafe {
            asm!("mrs {}, daif", out(reg) daif, options(nomem, preserves_flags));
            asm!("mrs {}, CurrentEL", out(reg) el, options(nomem, preserves_flags));
        }
        crate::serial::write(b"DAIF=");
        crate::serial::write_hex(daif);
        crate::serial::write(b" EL=");
        crate::serial::write_num(el >> 2);
        crate::serial::write(b"\n");

        unsafe {
            asm!("msr vbar_el1, {}", in(reg) vectors_addr, options(nomem, preserves_flags));
        }

        Some(mapping.virt)
    }

    /// Explicitly map a kernel virtual region to its linear physical backing.
    /// Used for demand-paging the kernel heap/BSS.
    pub unsafe fn map_kernel_region(&self, virt: u64, len: usize, flags: MmioFlags) -> bool {
        let phys = self.kernel_virt_to_phys(virt);
        // Ensure flags do not include DEVICE unless explicitly requested (they shouldn't for RAM)
        // Map it.
        self.map_range(virt, phys, len, flags)
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

    fn set_boot_color(&self, fb: &crate::boot::FramebufferInfo, color: BootColor) {
        let bytes_per_pixel = (fb.bpp / 8) as usize;
        if bytes_per_pixel < 4 || fb.width == 0 || fb.height == 0 || fb.pitch == 0 {
            return;
        }
        let min_pitch = fb.width as usize * bytes_per_pixel;
        if (fb.pitch as usize) < min_pitch {
            return;
        }

        let pack = |component: u8, size: u8, shift: u8| -> u32 {
            if size == 0 || size > 24 {
                return 0;
            }
            let mask = (1u32 << size) - 1;
            let scaled = (component as u32 * mask + 127) / 255;
            scaled << shift
        };

        let pixel = pack(color.red, fb.red_mask_size, fb.red_mask_shift)
            | pack(color.green, fb.green_mask_size, fb.green_mask_shift)
            | pack(color.blue, fb.blue_mask_size, fb.blue_mask_shift);

        let fb_base = fb.addr + self.hhdm_offset();

        let width = fb.width as usize;
        let height = fb.height as usize;
        let pitch = fb.pitch as usize;
        let row_len_bytes = width * bytes_per_pixel;

        unsafe {
            if pitch == row_len_bytes {
                let buf = core::slice::from_raw_parts_mut(fb_base as *mut u32, width * height);
                buf.fill(pixel);
            } else {
                for row in 0..height {
                    let row_ptr = (fb_base + (row * pitch) as u64) as *mut u32;
                    let row_buf = core::slice::from_raw_parts_mut(row_ptr, width);
                    row_buf.fill(pixel);
                }
            }
        }
    }

    fn mmio_map(&self, range: MmioRange, flags: MmioFlags) -> Option<MmioMapping> {
        self.map_mmio(range, flags)
    }

    fn monotonic_now(&self) -> u64 {
        let cnt: u64;
        let freq: u64;
        unsafe {
            asm!("mrs {}, cntvct_el0", out(reg) cnt);
            asm!("mrs {}, cntfrq_el0", out(reg) freq);
        }
        // (cnt * 1_000_000_000) / freq
        ((cnt as u128 * 1_000_000_000) / freq as u128) as u64
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

    fn irq_enable(&self) {
        unsafe {
            asm!("msr daifclr, #2", options(nomem, preserves_flags));
            let daif: u64;
            asm!("mrs {}, daif", out(reg) daif, options(nomem, preserves_flags));
            crate::serial::write(b"IRQ ENABLED: DAIF=");
            crate::serial::write_hex(daif);
            crate::serial::write(b"\n");
        }
    }

    fn halt(&self) -> ! {
        loop {
            unsafe {
                asm!("wfi");
            }
        }
    }

    fn idle(&self) {
        // crate::serial::write(b"IDLE\n");
        unsafe {
            core::arch::asm!("nop");
        }
    }

    fn switch_to(&self, old_ctx: &mut crate::machine::Context, new_ctx: &crate::machine::Context) {
        unsafe {
            // Context has `sp` as first field (u64).
            // Matches user's cast: &mut old_ctx.sp as *mut u64
            aarch64_switch_to(&mut old_ctx.sp as *mut u64, &new_ctx.sp as *const u64);
        }
    }

    fn task_entry_stub(&self) -> u64 {
        aarch64_task_entry_stub as *const () as usize as u64
    }

    fn virt_to_phys(&self, virt: u64) -> u64 {
        if virt >= self.hhdm_offset() && self.hhdm_offset() != 0 {
            virt.wrapping_sub(self.hhdm_offset())
        } else if virt >= self.kernel_virt_base() && self.kernel_virt_base() != 0 {
            self.kernel_virt_to_phys(virt)
        } else {
            virt // Fallback
        }
    }


    fn simd(&self) -> &'static dyn crate::machine::Simd {
        &simd::AARCH64_SIMD
    }

    fn set_kernel_stack(&self, top: u64) {
        let pc = percpu::get_local();
        pc.kernel_stack_top = top;
    }
}

impl ArchMachine {
    fn simd_impl(&self) -> &'static dyn crate::machine::Simd {
        &simd::AARCH64_SIMD
    }
}
