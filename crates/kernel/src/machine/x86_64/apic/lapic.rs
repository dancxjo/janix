//! Local APIC implementation.

use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

use super::msr;
use super::registers::*;
use crate::interrupt::{InterruptController, TimerMode};

struct CalibrationResult {
    ticks_per_ms: u32,
    frequency_hz: u32,
}

/// Fixed virtual address for LAPIC MMIO mapping.
/// This is in the kernel's reserved space and is mapped explicitly.
const LAPIC_VIRT_BASE: u64 = 0xFFFF_FFFF_FEC0_0000;

pub struct Lapic {
    base_virt: AtomicU64,
    timer_freq_hz: AtomicU32,
    ticks_per_ms: AtomicU32,
    initialized: AtomicU32,
}

impl Lapic {
    pub const fn new() -> Self {
        Self {
            base_virt: AtomicU64::new(0),
            timer_freq_hz: AtomicU32::new(0),
            ticks_per_ms: AtomicU32::new(0),
            initialized: AtomicU32::new(0),
        }
    }

    #[inline]
    fn read(&self, offset: u32) -> u32 {
        let addr = self.base_virt.load(Ordering::Relaxed) + offset as u64;
        unsafe { read_volatile(addr as *const u32) }
    }

    #[inline]
    fn write(&self, offset: u32, value: u32) {
        let addr = self.base_virt.load(Ordering::Relaxed) + offset as u64;
        unsafe { write_volatile(addr as *mut u32, value) }
    }

    pub fn id(&self) -> u32 {
        if self.base_virt.load(Ordering::Relaxed) == 0 {
            return 0; // Not initialized yet
        }
        (self.read(LAPIC_ID) >> 24) & 0xFF
    }

    #[inline]
    pub fn send_eoi(&self) {
        if self.base_virt.load(Ordering::Relaxed) != 0 {
            self.write(LAPIC_EOI, 0);
        }
    }

    /// Map the LAPIC MMIO region into kernel virtual address space.
    unsafe fn map_lapic_mmio(&self, phys_base: u64, hhdm_offset: u64) -> u64 {
        use x86_64::registers::control::Cr3;
        use x86_64::structures::paging::{PageTable, PageTableFlags, PhysFrame, Size4KiB};
        use x86_64::PhysAddr;

        // Get current page table
        let (cr3_frame, _) = Cr3::read();
        let pml4_phys = cr3_frame.start_address().as_u64();
        let pml4_virt = pml4_phys + hhdm_offset;
        let pml4 = &mut *(pml4_virt as *mut PageTable);

        let virt = LAPIC_VIRT_BASE;
        let page_flags = PageTableFlags::PRESENT
            | PageTableFlags::WRITABLE
            | PageTableFlags::NO_EXECUTE
            | PageTableFlags::WRITE_THROUGH
            | PageTableFlags::NO_CACHE;

        // Walk/create page tables for LAPIC_VIRT_BASE
        // Index calculations for 0xFFFF_FFFF_FEC0_0000
        let p4_idx = ((virt >> 39) & 0x1FF) as usize; // 511
        let p3_idx = ((virt >> 30) & 0x1FF) as usize; // 511
        let p2_idx = ((virt >> 21) & 0x1FF) as usize; // 503
        let p1_idx = ((virt >> 12) & 0x1FF) as usize; // 0

        // Get or create PDP
        let pdp = if pml4[p4_idx].flags().contains(PageTableFlags::PRESENT) {
            let pdp_phys = pml4[p4_idx].addr().as_u64();
            &mut *((pdp_phys + hhdm_offset) as *mut PageTable)
        } else {
            crate::serial::write(b"LAPIC: Creating P3 table\n");
            let new_table = alloc::boxed::Box::leak(alloc::boxed::Box::new(PageTable::new()));
            let new_virt = new_table as *mut _ as u64;
            let new_phys = crate::machine::machine().virt_to_phys(new_virt);
            pml4[p4_idx].set_addr(
                PhysAddr::new(new_phys),
                PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
            );
            new_table
        };

        // Get or create PD
        let pd = if pdp[p3_idx].flags().contains(PageTableFlags::PRESENT) {
            let pd_phys = pdp[p3_idx].addr().as_u64();
            &mut *((pd_phys + hhdm_offset) as *mut PageTable)
        } else {
            crate::serial::write(b"LAPIC: Creating P2 table\n");
            let new_table = alloc::boxed::Box::leak(alloc::boxed::Box::new(PageTable::new()));
            let new_virt = new_table as *mut _ as u64;
            let new_phys = crate::machine::machine().virt_to_phys(new_virt);
            pdp[p3_idx].set_addr(
                PhysAddr::new(new_phys),
                PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
            );
            new_table
        };

        // Get or create PT
        let pt = if pd[p2_idx].flags().contains(PageTableFlags::PRESENT) {
            let pt_phys = pd[p2_idx].addr().as_u64();
            &mut *((pt_phys + hhdm_offset) as *mut PageTable)
        } else {
            crate::serial::write(b"LAPIC: Creating P1 table\n");
            let new_table = alloc::boxed::Box::leak(alloc::boxed::Box::new(PageTable::new()));
            let new_virt = new_table as *mut _ as u64;
            let new_phys = crate::machine::machine().virt_to_phys(new_virt);
            pd[p2_idx].set_addr(
                PhysAddr::new(new_phys),
                PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
            );
            new_table
        };

        // Map the LAPIC page
        pt[p1_idx].set_addr(PhysAddr::new(phys_base), page_flags);

        // Flush TLB for this address
        x86_64::instructions::tlb::flush(x86_64::VirtAddr::new(virt));

        crate::serial::write(b"LAPIC: Mapped MMIO at 0x");
        crate::serial::write_hex(virt);
        crate::serial::write(b"\n");

        virt
    }

    pub unsafe fn init_with_hhdm(&self, hhdm_offset: u64) {
        msr::enable_xapic();

        let phys_base = msr::apic_base_address();

        crate::serial::write(b"LAPIC: phys=0x");
        crate::serial::write_hex(phys_base);
        crate::serial::write(b"\n");

        // Map LAPIC MMIO explicitly (HHDM doesn't cover device memory)
        let virt_base = self.map_lapic_mmio(phys_base, hhdm_offset);
        self.base_virt.store(virt_base, Ordering::Relaxed);

        // Enable APIC via SVR
        let svr = SPURIOUS_VECTOR as u32 | SVR_APIC_ENABLE;
        self.write(LAPIC_SVR, svr);

        // Calibrate timer
        let cal = self.calibrate_timer();
        self.timer_freq_hz
            .store(cal.frequency_hz, Ordering::Relaxed);
        self.ticks_per_ms.store(cal.ticks_per_ms, Ordering::Relaxed);

        crate::serial::write(b"LAPIC: timer freq=");
        crate::serial::write_hex(cal.frequency_hz as u64);
        crate::serial::write(b"Hz\n");

        // Set up periodic timer at ~100Hz
        self.setup_periodic_timer(100);
        self.initialized.store(1, Ordering::Release);

        crate::serial::write(b"LAPIC: enabled, ID=");
        crate::serial::write_hex(self.id() as u64);
        crate::serial::write(b"\n");
    }

    fn calibrate_timer(&self) -> CalibrationResult {
        use x86_64::instructions::port::Port;

        const PIT_FREQ: u32 = 1_193_182;
        const CALIBRATION_MS: u32 = 10;
        const PIT_DIVISOR: u16 = ((PIT_FREQ * CALIBRATION_MS) / 1000) as u16;

        self.write(LAPIC_TIMER_DIV, TIMER_DIV_16);
        self.write(
            LAPIC_LVT_TIMER,
            LVT_MASK | LVT_TIMER_ONESHOT | TIMER_VECTOR as u32,
        );
        self.write(LAPIC_TIMER_INIT, 0xFFFF_FFFF);

        unsafe {
            let mut cmd = Port::<u8>::new(0x43);
            let mut ch2 = Port::<u8>::new(0x42);
            let mut gate = Port::<u8>::new(0x61);

            let g = gate.read();
            gate.write((g & 0xFC) | 0x01);

            cmd.write(0b10110000);
            ch2.write((PIT_DIVISOR & 0xFF) as u8);
            ch2.write((PIT_DIVISOR >> 8) as u8);

            loop {
                if (gate.read() & 0x20) != 0 {
                    break;
                }
            }

            gate.write(g & 0xFC);
        }

        let end_count = self.read(LAPIC_TIMER_CURRENT);
        let elapsed = 0xFFFF_FFFF_u32.wrapping_sub(end_count);
        let ticks_per_ms = elapsed / CALIBRATION_MS;
        let frequency_hz = ticks_per_ms * 1000;

        CalibrationResult {
            ticks_per_ms,
            frequency_hz,
        }
    }

    fn setup_periodic_timer(&self, freq_hz: u32) {
        let ticks_per_ms = self.ticks_per_ms.load(Ordering::Relaxed);
        if ticks_per_ms == 0 {
            self.write(LAPIC_TIMER_DIV, TIMER_DIV_16);
            self.write(LAPIC_TIMER_INIT, 10_000_000);
            self.write(LAPIC_LVT_TIMER, LVT_TIMER_PERIODIC | TIMER_VECTOR as u32);
            return;
        }

        let initial_count = (ticks_per_ms as u64 * 1000 / freq_hz as u64) as u32;
        self.write(LAPIC_TIMER_DIV, TIMER_DIV_16);
        self.write(LAPIC_TIMER_INIT, initial_count);
        self.write(LAPIC_LVT_TIMER, LVT_TIMER_PERIODIC | TIMER_VECTOR as u32);
    }
}

impl InterruptController for Lapic {
    fn init(&self) {}

    fn enable(&self) {
        let lvt = self.read(LAPIC_LVT_TIMER);
        self.write(LAPIC_LVT_TIMER, lvt & !LVT_MASK);
    }

    fn disable(&self) {
        let lvt = self.read(LAPIC_LVT_TIMER);
        self.write(LAPIC_LVT_TIMER, lvt | LVT_MASK);
    }

    fn eoi(&self) {
        self.send_eoi();
    }

    fn set_timer(&self, mode: TimerMode, ticks: u32) {
        let mode_bits = match mode {
            TimerMode::OneShot => LVT_TIMER_ONESHOT,
            TimerMode::Periodic => LVT_TIMER_PERIODIC,
        };
        self.write(LAPIC_TIMER_INIT, ticks);
        self.write(LAPIC_LVT_TIMER, mode_bits | TIMER_VECTOR as u32);
    }

    fn timer_frequency_hz(&self) -> u32 {
        self.timer_freq_hz.load(Ordering::Relaxed)
    }

    fn local_id(&self) -> u32 {
        self.id()
    }
}

unsafe impl Send for Lapic {}
unsafe impl Sync for Lapic {}

pub static LAPIC: Lapic = Lapic::new();
