//! IA32_APIC_BASE MSR helpers.

use core::arch::asm;

pub const IA32_APIC_BASE: u32 = 0x1B;
pub const APIC_BASE_ENABLE: u64 = 1 << 11;
pub const APIC_BASE_ADDR_MASK: u64 = 0x000F_FFFF_FFFF_F000;

#[inline]
pub fn read_apic_base() -> u64 {
    let low: u32;
    let high: u32;
    unsafe {
        asm!(
            "rdmsr",
            in("ecx") IA32_APIC_BASE,
            out("eax") low,
            out("edx") high,
            options(nomem, nostack)
        );
    }
    ((high as u64) << 32) | (low as u64)
}

#[inline]
pub unsafe fn write_apic_base(value: u64) {
    let low = value as u32;
    let high = (value >> 32) as u32;
    asm!(
        "wrmsr",
        in("ecx") IA32_APIC_BASE,
        in("eax") low,
        in("edx") high,
        options(nomem, nostack)
    );
}

#[inline]
pub fn apic_base_address() -> u64 {
    read_apic_base() & APIC_BASE_ADDR_MASK
}

pub unsafe fn enable_xapic() {
    let current = read_apic_base();
    write_apic_base(current | APIC_BASE_ENABLE);
}
