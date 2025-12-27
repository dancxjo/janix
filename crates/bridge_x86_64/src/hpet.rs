use core::ptr::NonNull;
use crate::Bridge;
use hw::HardwareBridge;

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

static mut HPET_BASE: u64 = 0;
static mut CLK_PERIOD_FS: u64 = 0; // Femtoseconds per tick

pub unsafe fn init_table(ptr: u64) {
    let bridge = Bridge;
    let table = &*(ptr as *const HpetTable);
    
    bridge.log("ACPI: Found HPET\n");
    
    if table.address.address_space != 0 {
        bridge.log("HPET: Not system memory, ignoring\n");
        return;
    }
    
    let phys_base = table.address.address;
    bridge.log("HPET: Base=");
    crate::print_hex(phys_base);
    bridge.log("\n");

    // Verify we can access the registers
    let virt_base = crate::acpi::to_virt(phys_base);
    
    // HPET Registers
    // 0x00: GCID (Capabilities)
    // 0x10: CONFIG
    // 0xF0: MAIN_COUNTER
    let regs = virt_base.as_ptr() as *mut u8;
    
    let gcid = core::ptr::read_volatile(regs.add(0x00) as *const u64);
    let period_fs = gcid >> 32;
    
    // Validate period: must be <= 0x05F5E100 (100ns in femtoseconds)
    if period_fs == 0 || period_fs > 0x05F5E100 {
        bridge.log("HPET: Invalid period, aborting\n");
        return;
    }
    
    CLK_PERIOD_FS = period_fs;
    
    bridge.log("HPET: Period=");
    crate::print_u64(period_fs);
    bridge.log("fs\n");
    
    // Enable (CONFIG bit 0)
    let mut config = core::ptr::read_volatile(regs.add(0x10) as *const u64);
    config |= 1; // Enable
    // If Legacy Replacement needed, bit 1. For now keep PIC/PIT separate.
    core::ptr::write_volatile(regs.add(0x10) as *mut u64, config);
    
    HPET_BASE = virt_base.as_ptr() as u64;
    bridge.log("HPET: Enabled\n");
}

pub fn read_ns() -> u64 {
    unsafe {
        if HPET_BASE == 0 { return 0; }
        let regs = HPET_BASE as *const u8;
        // Main Counter is at 0xF0
        let tick = core::ptr::read_volatile(regs.add(0xF0) as *const u64);
        
        let fs = tick as u128 * CLK_PERIOD_FS as u128;
        (fs / 1_000_000) as u64
    }
}
