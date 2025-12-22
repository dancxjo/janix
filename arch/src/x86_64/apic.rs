use core::arch::asm;
use core::ptr::{read_volatile, write_volatile};
use kernel::memory;
use x86_64::registers::model_specific::Msr;

// MSRs
const IA32_APIC_BASE: u32 = 0x1B;
const IA32_TSC_DEADLINE: u32 = 0x6E0;

// Offsets
const APIC_EOI: usize = 0xB0;
const APIC_SIV: usize = 0xF0;
const APIC_LVT_TIMER: usize = 0x320;
const _APIC_INIT_COUNT: usize = 0x380;
const _APIC_CUR_COUNT: usize = 0x390;
const APIC_DIV_CONFIG: usize = 0x3E0;

static mut APIC_BASE: u64 = 0;
static mut HAS_TSC_DEADLINE: bool = false;

pub fn init() {
    unsafe {
        // Check for TSC Deadline support
        // CPUID.01H:ECX.TSC_Deadline[bit 24]
        let cpuid = core::arch::x86_64::__cpuid(1);
        HAS_TSC_DEADLINE = (cpuid.ecx & (1 << 24)) != 0;

        // Get APIC Base
        let mut msr = Msr::new(IA32_APIC_BASE);
        let base = msr.read();

        // Enable APIC if disabled (bit 11 of IA32_APIC_BASE)
        if base & (1 << 11) == 0 {
            msr.write(base | (1 << 11));
        }

        // Mask out flags to get physical address (bits 12-35 usually, but we mask lower 12)
        // x2APIC mode might be different, but assuming xAPIC for now as we use MMIO.
        let phys_addr = base & 0xFFFFF000;

        // Add HHDM offset to access via virtual memory
        let hhdm = memory::get_hhdm_offset();
        APIC_BASE = phys_addr + hhdm;

        // Spurious Interrupt Vector (enable APIC)
        // Set vector 0xFF, and bit 8 (APIC Software Enable)
        write_reg(APIC_SIV, 0x1FF);

        // Copy statics to local to avoid "shared reference to mutable static" error in println
        let base = APIC_BASE;
        let has_tsc = HAS_TSC_DEADLINE;
        kernel::println!(
            "LAPIC initialized. Base={:#x}, TSC-Deadline={}",
            base,
            has_tsc
        );
    }
}

pub fn eoi() {
    unsafe {
        write_reg(APIC_EOI, 0);
    }
}

pub fn set_timer_vector(vector: u8) {
    unsafe {
        if HAS_TSC_DEADLINE {
            // TSC-Deadline mode: Bit 18 set, Bit 17 clear.
            // Vector in bits 0-7.
            let value = (2 << 17) | (vector as u32);
            write_reg(APIC_LVT_TIMER, value);
        } else {
            // Fallback: One-Shot mode (Bits 17, 18 clear).
            // We'll use this if we implement fallback logic.
            let value = vector as u32;
            write_reg(APIC_LVT_TIMER, value);

            // Set Divider to 16 (bits 3:0 of 0x3E0 -> 0x3)
            // 0000: /2, 0001: /4, 0010: /8, 0011: /16
            write_reg(APIC_DIV_CONFIG, 0x3);
        }
    }
}

pub fn set_deadline_tsc(tsc: u64) {
    unsafe {
        if HAS_TSC_DEADLINE {
            // Write absolute TSC value to MSR
            let mut msr = Msr::new(IA32_TSC_DEADLINE);
            msr.write(tsc);
        } else {
            // Fallback: One-shot timer.
            // We need to calculate delta ticks.
            // This is difficult without calibration of APIC bus frequency.
            // For now, we simply ignore if not supported, or maybe fire immediately?
            // Ignoring is safer than hanging or panic.
            // Ideally we would log once.
        }
    }
}

unsafe fn write_reg(offset: usize, value: u32) {
    if APIC_BASE == 0 {
        return;
    }
    let addr = (APIC_BASE as usize + offset) as *mut u32;
    write_volatile(addr, value);
}

#[allow(dead_code)]
unsafe fn read_reg(offset: usize) -> u32 {
    if APIC_BASE == 0 {
        return 0;
    }
    let addr = (APIC_BASE as usize + offset) as *const u32;
    read_volatile(addr)
}
