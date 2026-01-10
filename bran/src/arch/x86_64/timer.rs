
use core::arch::asm;
use super::paging::{phys_to_virt, map_mmio_page};
use crate::runtime::LimineRuntimeData;

// MSRs
const IA32_APIC_BASE: u32 = 0x1B;
const MSR_GS_BASE: u32 = 0xC0000101;
// LAPIC Offsets
const LAPIC_ID: u32 = 0x020;
const LAPIC_EOI: u32 = 0x0B0;
const LAPIC_SVR: u32 = 0x0F0;
const LAPIC_TIMER: u32 = 0x320;
const LAPIC_TIMER_INIT: u32 = 0x380;
const LAPIC_TIMER_CUR: u32 = 0x390;
const LAPIC_TIMER_DIV: u32 = 0x3E0;

// LAPIC Base (default)
static mut LAPIC_BASE: u64 = 0xFEE00000;

unsafe fn rdmsr(msr: u32) -> u64 {
    let (low, high): (u32, u32);
    unsafe { asm!("rdmsr", in("ecx") msr, out("eax") low, out("edx") high, options(nomem, nostack, preserves_flags)) };
    ((high as u64) << 32) | (low as u64)
}

unsafe fn lapic_read(reg: u32) -> u32 {
    unsafe {
        let addr = (LAPIC_BASE + reg as u64) as *const u32;
        addr.read_volatile()
    }
}

unsafe fn lapic_write(reg: u32, val: u32) {
    unsafe {
        let addr = (LAPIC_BASE + reg as u64) as *mut u32;
        addr.write_volatile(val);
    }
}

pub unsafe fn init() {
    unsafe {
        kernel::kinfo!("timer: init start");
        // 1. Get LAPIC Base from MSR
        let apic_base_msr = rdmsr(IA32_APIC_BASE);
        let phys_base = apic_base_msr & 0xFFFFF000;
        
        // Map LAPIC if needed (it is MMIO, so we need to ensure it is mapped uncached and present)
    // Common LAPIC base is 0xFEE00000.
    // Common LAPIC base is 0xFEE00000.
    let lapic_phys = 0xFEE00000;
    // Map to Kernel Region (High Half, inside existing P3[510] range)
    // 0xffffffff80000000 is kernel base. We add 256MB offset to be safe.
    let lapic_virt = 0xffffffff8f000000;
    
    map_mmio_page(lapic_virt, lapic_phys);
    super::paging::tlb_flush_all();
             
    if phys_base != lapic_phys {
        kernel::kinfo!("timer: warning LAPIC phys {:#x} != standard {:#x}", phys_base, lapic_phys);
    }
    
    LAPIC_BASE = lapic_virt;
    kernel::kinfo!("timer: LAPIC_BASE set to {:#x} (mapped)", lapic_virt);

        // 2. Enable LAPIC
        lapic_write(LAPIC_SVR, 0x100 | 0xFF); 
        kernel::kinfo!("timer: LAPIC enabled");

        // 3. Init Timer
        lapic_write(LAPIC_TIMER_DIV, 0x3);
        lapic_write(LAPIC_TIMER, 0x20020);
        lapic_write(LAPIC_TIMER_INIT, 10_000_000);
        kernel::kinfo!("timer: initialized");
    }
}

pub unsafe fn ack() {
    unsafe { lapic_write(LAPIC_EOI, 0); }
}
