use crate::bridge::CpuBridge;
use core::ptr::NonNull;

#[repr(C, packed)]
pub struct HpetTable {
    header: [u8; 36], // SdtHeader
    event_timer_block_id: u32,
    address: GenericAddress,
    hpet_number: u8,
    min_clock_tick: u16,
    page_protection: u8,
}

#[repr(C, packed)]
struct GenericAddress {
    address_space: u8,
    bit_width: u8,
    bit_offset: u8,
    access_size: u8,
    address: u64,
}

// Offsets
const REG_GCID: usize = 0x00;
const REG_GEN_CONF: usize = 0x10;
const REG_GEN_INT_STATUS: usize = 0x20;
const REG_MAIN_COUNTER: usize = 0xF0;
const REG_T0_CONF: usize = 0x100;
const REG_T0_COMP: usize = 0x108;

// Bit Flags
const GEN_CONF_ENABLE_CNF: u64 = 1 << 0;
const GEN_CONF_LEGACY_RT_CNF: u64 = 1 << 1;

const TN_CONF_INT_TYPE_CNF: u64 = 1 << 1;
const TN_CONF_INT_ENB_CNF: u64 = 1 << 2;
const TN_CONF_TYPE_CNF: u64 = 1 << 3; // 0=one-shot, 1=periodic
const TN_CONF_VAL_SET_CNF: u64 = 1 << 6;
const TN_CONF_32MODE_CNF: u64 = 1 << 8;

static mut HPET_BASE: u64 = 0;
static mut CLK_PERIOD_FS: u64 = 0; // Femtoseconds per tick

pub unsafe fn init_table(bridge: &impl CpuBridge, ptr: u64, hhdm: u64) {
    // Note: acpi was moved to crate::platform::acpi
    let table = &*(crate::platform::acpi::to_virt(ptr, hhdm).as_ptr() as *const HpetTable);

    bridge.log("ACPI: Found HPET\n");

    if table.address.address_space != 0 {
        bridge.log("HPET: Not system memory, ignoring\n");
        return;
    }

    let phys_base = table.address.address;
    bridge.log("HPET: Base Found\n");

    // Verify we can access the registers
    let virt_base = crate::platform::acpi::to_virt(phys_base, hhdm);
    HPET_BASE = virt_base.as_ptr() as u64;

    let regs = HPET_BASE as *mut u8;

    let gcid = core::ptr::read_volatile(regs.add(REG_GCID) as *const u64);
    let period_fs = gcid >> 32;
    let num_timers = ((gcid >> 8) & 0x1F) + 1;

    bridge.log("HPET: Timers Found\n");

    // Validate period: must be <= 0x05F5E100 (100ns in femtoseconds)
    if period_fs == 0 || period_fs > 0x05F5E100 {
        bridge.log("HPET: Invalid period, aborting\n");
        HPET_BASE = 0;
        return;
    }

    CLK_PERIOD_FS = period_fs;

    bridge.log("HPET: Period Valid\n");

    // Disable HPET before config
    let mut config = core::ptr::read_volatile(regs.add(REG_GEN_CONF) as *const u64);
    config &= !GEN_CONF_ENABLE_CNF;
    core::ptr::write_volatile(regs.add(REG_GEN_CONF) as *mut u64, config);

    // Reset Main Counter
    core::ptr::write_volatile(regs.add(REG_MAIN_COUNTER) as *mut u64, 0);

    // Enable (we leave legacy route OFF until requested)
    config |= GEN_CONF_ENABLE_CNF;
    core::ptr::write_volatile(regs.add(REG_GEN_CONF) as *mut u64, config);

    bridge.log("HPET: Enabled\n");
}

pub fn read_ns() -> u64 {
    unsafe {
        if HPET_BASE == 0 {
            return 0;
        }
        let regs = HPET_BASE as *const u8;
        let tick = core::ptr::read_volatile(regs.add(REG_MAIN_COUNTER) as *const u64);

        let fs = tick as u128 * CLK_PERIOD_FS as u128;
        (fs / 1_000_000) as u64
    }
}

pub fn read_ticks() -> u64 {
    unsafe {
        if HPET_BASE == 0 {
            return 0;
        }
        let regs = HPET_BASE as *const u8;
        core::ptr::read_volatile(regs.add(REG_MAIN_COUNTER) as *const u64)
    }
}

pub fn ns_to_ticks(ns: u64) -> u64 {
    unsafe {
        if CLK_PERIOD_FS == 0 {
            return 0;
        }
        let fs = ns as u128 * 1_000_000;
        (fs / CLK_PERIOD_FS as u128) as u64
    }
}

pub fn ticks_per_second() -> u64 {
    unsafe {
        if CLK_PERIOD_FS == 0 {
            return 0;
        }
        // 1 second = 1e15 femtoseconds.
        1_000_000_000_000_000u64 / CLK_PERIOD_FS
    }
}

pub unsafe fn enable_legacy_mode() {
    if HPET_BASE == 0 {
        return;
    }
    let regs = HPET_BASE as *mut u8;

    // Set Legacy Replacement Route bit
    let mut config = core::ptr::read_volatile(regs.add(REG_GEN_CONF) as *const u64);
    config |= GEN_CONF_LEGACY_RT_CNF;
    core::ptr::write_volatile(regs.add(REG_GEN_CONF) as *mut u64, config);

    // Configure Timer 0
    // Edge triggered (TYPE=0), Interrupt Enabled (INT_ENB=1)
    let t0_conf_ptr = regs.add(REG_T0_CONF) as *mut u64;
    let mut t0_conf = core::ptr::read_volatile(t0_conf_ptr);

    t0_conf &= !TN_CONF_TYPE_CNF; // One-shot (periodic=0)
    t0_conf &= !TN_CONF_INT_TYPE_CNF; // Edge Triggered (Bit 1 = 0)
    t0_conf |= TN_CONF_INT_ENB_CNF; // Enable Ints

    core::ptr::write_volatile(t0_conf_ptr, t0_conf);
}

pub unsafe fn program_oneshot(deadline_ns: u64) {
    if HPET_BASE == 0 {
        return;
    }
    let regs = HPET_BASE as *mut u8;

    let now_ns = read_ns();
    // Min delta to avoid immediate retrigger issues or missed ticks?
    const MIN_DELTA_NS: u64 = 500_000; // 500us

    // Ensure deadline is in the future
    let effective_deadline = if deadline_ns < now_ns + MIN_DELTA_NS {
        now_ns + MIN_DELTA_NS
    } else {
        deadline_ns
    };

    let target_ticks = ns_to_ticks(effective_deadline);

    // DEBUG removed as no bridge access

    core::ptr::write_volatile(regs.add(REG_T0_COMP) as *mut u64, target_ticks);
}

pub unsafe fn ack_interrupt() {
    if HPET_BASE == 0 {
        return;
    }
    let regs = HPET_BASE as *mut u8;
    // Clear status bit 0 (T0_INT_STS)
    // "Write 1 to clear"
    let status = core::ptr::read_volatile(regs.add(REG_GEN_INT_STATUS) as *const u64);
    if status & 1 != 0 {
        core::ptr::write_volatile(regs.add(REG_GEN_INT_STATUS) as *mut u64, 1);
    }
}
