//! LAPIC MMIO register offsets and bit definitions.

pub const LAPIC_ID: u32 = 0x020;
pub const LAPIC_VERSION: u32 = 0x030;
pub const LAPIC_TPR: u32 = 0x080;
pub const LAPIC_EOI: u32 = 0x0B0;
pub const LAPIC_SVR: u32 = 0x0F0;
pub const LAPIC_LVT_TIMER: u32 = 0x320;
pub const LAPIC_TIMER_INIT: u32 = 0x380;
pub const LAPIC_TIMER_CURRENT: u32 = 0x390;
pub const LAPIC_TIMER_DIV: u32 = 0x3E0;

// SVR bits
pub const SVR_APIC_ENABLE: u32 = 1 << 8;

// LVT Timer bits
pub const LVT_TIMER_ONESHOT: u32 = 0b00 << 17;
pub const LVT_TIMER_PERIODIC: u32 = 0b01 << 17;
pub const LVT_MASK: u32 = 1 << 16;

// Timer divider values
pub const TIMER_DIV_16: u32 = 0b0011;

// Interrupt vectors
pub const TIMER_VECTOR: u8 = 32;
pub const SPURIOUS_VECTOR: u8 = 0xFF;
