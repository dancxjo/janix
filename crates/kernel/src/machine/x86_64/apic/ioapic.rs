//! IO-APIC module for x86_64.
//!
//! The IO-APIC routes legacy ISA interrupts (like PS/2 keyboard and mouse)
//! to the Local APIC on modern x86_64 systems.

use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{AtomicU64, Ordering};

/// Standard IO-APIC physical base address (can be overridden by ACPI MADT).
const IOAPIC_PHYS_BASE: u64 = 0xFEC00000;

/// Fixed virtual address for IO-APIC MMIO mapping.
/// This is adjacent to the LAPIC mapping (0xFFFF_FFFF_FEC0_0000).
const IOAPIC_VIRT_BASE: u64 = 0xFFFF_FFFF_FEC0_1000;

/// IO-APIC register offsets (indirect access via IOREGSEL/IOWIN).
const IOREGSEL: u32 = 0x00;
const IOWIN: u32 = 0x10;

/// IO-APIC registers.
const IOAPIC_VER: u32 = 0x01;
const IOAPIC_REDTBL_BASE: u32 = 0x10;

/// Redirection table entry flags.
const REDTBL_MASKED: u64 = 1 << 16;

pub struct IoApic {
    base_virt: AtomicU64,
}

impl IoApic {
    pub const fn new() -> Self {
        Self {
            base_virt: AtomicU64::new(0),
        }
    }

    /// Read an IO-APIC register (indirect access).
    #[inline]
    fn read(&self, reg: u32) -> u32 {
        let base = self.base_virt.load(Ordering::Relaxed);
        if base == 0 {
            return 0;
        }
        unsafe {
            write_volatile((base + IOREGSEL as u64) as *mut u32, reg);
            read_volatile((base + IOWIN as u64) as *const u32)
        }
    }

    /// Write to an IO-APIC register (indirect access).
    #[inline]
    fn write(&self, reg: u32, value: u32) {
        let base = self.base_virt.load(Ordering::Relaxed);
        if base == 0 {
            return;
        }
        unsafe {
            write_volatile((base + IOREGSEL as u64) as *mut u32, reg);
            write_volatile((base + IOWIN as u64) as *mut u32, value);
        }
    }

    /// Read a 64-bit redirection table entry.
    fn read_redtbl(&self, irq: u8) -> u64 {
        let reg = IOAPIC_REDTBL_BASE + (irq as u32 * 2);
        let lo = self.read(reg) as u64;
        let hi = self.read(reg + 1) as u64;
        lo | (hi << 32)
    }

    /// Write a 64-bit redirection table entry.
    fn write_redtbl(&self, irq: u8, entry: u64) {
        let reg = IOAPIC_REDTBL_BASE + (irq as u32 * 2);
        self.write(reg, entry as u32);
        self.write(reg + 1, (entry >> 32) as u32);
    }

    /// Map the IO-APIC MMIO region into kernel virtual address space.
    unsafe fn map_ioapic_mmio(&self, hhdm_offset: u64) -> u64 {
        use x86_64::registers::control::Cr3;
        use x86_64::structures::paging::{PageTable, PageTableFlags};
        use x86_64::PhysAddr;

        // Get current page table
        let (cr3_frame, _) = Cr3::read();
        let pml4_phys = cr3_frame.start_address().as_u64();
        let pml4_virt = pml4_phys + hhdm_offset;
        let pml4 = &mut *(pml4_virt as *mut PageTable);

        let virt = IOAPIC_VIRT_BASE;
        let page_flags = PageTableFlags::PRESENT
            | PageTableFlags::WRITABLE
            | PageTableFlags::NO_EXECUTE
            | PageTableFlags::WRITE_THROUGH
            | PageTableFlags::NO_CACHE;

        // Walk/create page tables for IOAPIC_VIRT_BASE
        let p4_idx = ((virt >> 39) & 0x1FF) as usize;
        let p3_idx = ((virt >> 30) & 0x1FF) as usize;
        let p2_idx = ((virt >> 21) & 0x1FF) as usize;
        let p1_idx = ((virt >> 12) & 0x1FF) as usize;

        // Get or create PDP
        let pdp = if pml4[p4_idx].flags().contains(PageTableFlags::PRESENT) {
            let pdp_phys = pml4[p4_idx].addr().as_u64();
            &mut *((pdp_phys + hhdm_offset) as *mut PageTable)
        } else {
            crate::serial::write(b"IOAPIC: Creating P3 table\n");
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
            crate::serial::write(b"IOAPIC: Creating P2 table\n");
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
            crate::serial::write(b"IOAPIC: Creating P1 table\n");
            let new_table = alloc::boxed::Box::leak(alloc::boxed::Box::new(PageTable::new()));
            let new_virt = new_table as *mut _ as u64;
            let new_phys = crate::machine::machine().virt_to_phys(new_virt);
            pd[p2_idx].set_addr(
                PhysAddr::new(new_phys),
                PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
            );
            new_table
        };

        // Map the IO-APIC page
        pt[p1_idx].set_addr(PhysAddr::new(IOAPIC_PHYS_BASE), page_flags);

        // Flush TLB for this address
        x86_64::instructions::tlb::flush(x86_64::VirtAddr::new(virt));

        crate::serial::write(b"IOAPIC: Mapped MMIO at 0x");
        crate::serial::write_hex(virt);
        crate::serial::write(b"\n");

        virt
    }

    /// Initialize the IO-APIC with the given HHDM offset.
    pub unsafe fn init(&self, hhdm_offset: u64) {
        // Map IO-APIC MMIO region explicitly (like LAPIC)
        let virt_base = self.map_ioapic_mmio(hhdm_offset);
        self.base_virt.store(virt_base, Ordering::Relaxed);

        let version = self.read(IOAPIC_VER);
        let max_redir = ((version >> 16) & 0xFF) as u8;

        crate::serial::write(b"IOAPIC: version=0x");
        crate::serial::write_hex((version & 0xFF) as u64);
        crate::serial::write(b" max_irq=");
        crate::serial::write_hex(max_redir as u64);
        crate::serial::write(b"\n");

        // Mask all IRQs initially
        for irq in 0..=max_redir {
            let entry = self.read_redtbl(irq);
            self.write_redtbl(irq, entry | REDTBL_MASKED);
        }
    }

    /// Route an ISA IRQ to the Local APIC with a specific vector.
    pub fn route_irq(&self, irq: u8, vector: u8, lapic_id: u8) {
        // Build redirection entry:
        // - Vector in bits 0-7
        // - Delivery mode: Fixed (000)
        // - Destination mode: Physical (bit 11 = 0)
        // - Polarity: Active high (bit 13 = 0) for ISA
        // - Trigger: Edge (bit 15 = 0) for ISA
        // - Mask: Unmasked (bit 16 = 0)
        // - Destination: LAPIC ID in bits 56-63
        let entry: u64 = (vector as u64) | ((lapic_id as u64) << 56);

        self.write_redtbl(irq, entry);

        crate::serial::write(b"IOAPIC: IRQ ");
        crate::serial::write_hex(irq as u64);
        crate::serial::write(b" -> vector ");
        crate::serial::write_hex(vector as u64);
        crate::serial::write(b"\n");
    }
}

unsafe impl Send for IoApic {}
unsafe impl Sync for IoApic {}

pub static IOAPIC: IoApic = IoApic::new();
