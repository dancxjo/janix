#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;
use thing_std::cap::{CapOp, Cap, CapScope};
use models::{RtcDevice, HardwareInfo, Thing}; // Added Thing trait
// use graph::store;
// use graph::symbols::{self, sym};
use core::ptr::{read_volatile};

extern crate abi; // Needed for abi::machine::MMIO_MAP constants if not re-exported

// PL031 Registers
const PL031_DR: usize = 0x000;

pub fn main() {
    log_info("RTC-PL031: Starting...");

    // 1. Find our hardware resource
    // Logic changed: cannot iterate by kind easily without helper.
    // Rely on "hw.rtc0" name.
    let hw_thing = thing_std::graph::thing_find("hw.rtc0");
    
    if hw_thing.is_none() {
        log_info("RTC-PL031: No hardware found. Exiting.");
        return;
    }
    let hw_id = hw_thing.unwrap();

    // Get body and decode
    let body_tuple = thing_std::graph::thing_get_body(hw_id);
    if body_tuple.is_none() {
        log_info("RTC-PL031: Hardware has no body.");
        return;
    }
    let (body, _) = body_tuple.unwrap();
    
    let info = HardwareInfo::decode_full(&body).expect("failed to decode HardwareInfo");
    // Verify name == "pl031"?
    let sym_pl031 = thing_std::graph::symbol_intern("pl031");
    if info.name != sym_pl031 {
        log_info("RTC-PL031: Hardware name mismatch.");
        // continue?
    }

    log_info("RTC-PL031: Hardware found.");

    // 2. Map MMIO
    // Create a bytespace for the mapping (optional, but sys_machine::MMIO_MAP usually returns a bare pointer in this OS context 
    // or maybe it populates a VMA? 
    // Wait, sys_machine::MMIO_MAP in aarch64 kernel:
    // fn sys_mmio_map(...)
    // uses `vm::map_physical_region`.
    // It returns virtual address in `val0`.
    // It doesn't need a bytespace Argument.
    
    let flags = 1 | 2 | 4; // DEVICE | UNCACHED | READ
    let phys = info.start;
    let len = info.end - info.start;
    
    // We need to use thing_std::syscall.
    // SYS_MACHINE = 100.
    // MMIO_MAP = 1.
    // Need to verify constants.
    // abi::syscall::nr::SYS_MACHINE is 100.
    // abi::machine::MMIO_MAP is 1.
    
    let res = unsafe {
        thing_std::syscall(
            abi::syscall::nr::SYS_MACHINE,
            abi::machine::MMIO_MAP, // Sub-operation
            phys,
            len,
            flags as u64,
            0, 0
        )
    };
    
    if res.status != 0 {
         log_info("RTC-PL031: MMIO Map failed.");
         return;
    }
    
    let base_ptr = res.val0 as *const u8;
    log_info("RTC-PL031: MMIO Mapped.");

    // 3. Publish device.rtc0
    let kind_rtc = thing_std::graph::symbol_intern("kind.RtcDevice");
    let rtc_thing = thing_std::graph::thing_create(kind_rtc, ThingId(0));
    thing_std::graph::thing_register_name(rtc_thing, "device.rtc0");

    let device_place = thing_std::graph::thing_find("place.devices");
    if let Some(dp) = device_place {
        let pred_contains = thing_std::graph::symbol_intern("pred.contains");
        thing_std::graph::relationship_create(pred_contains, dp, rtc_thing);
    }

    // Read initial time
    let seconds = unsafe { read_volatile(base_ptr.add(PL031_DR) as *const u32) };
    let mono = thing_std::monotonic_now();

    let payload = RtcDevice {
        source: sym_pl031,
        accuracy_ns: 1_000_000_000,
        base_seconds: seconds as u64,
        base_mono_ns: mono,
        flags: 0,
        _pad: 0,
    };
    let _ = thing_std::graph::thing_set_body(rtc_thing, &payload.encode_full());

    log_info(&alloc::format!("RTC-PL031: Active. Base Seconds: {}", seconds));

    loop {
        // Keep alive
        thing_std::time::sleep_ms(10000);
    }
}
