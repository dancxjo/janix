#![no_std]
#![no_main]

extern crate alloc;

use thing_std::prelude::*;
use abi::wire::time::RtcSample;
use abi::syscall_defs::SYSCALL_RTC_READ;

#[no_mangle]
pub fn main() {
    let _ = run();
}

fn run() -> Result<(), ()> {
    thing_std::log!("RTC_X86: Starting...");
    
    // Create TimeSource Thing
    // We need Kind ID for TimeSource. Let's assume standard creation via Thing::new.
    // Wait, Kinds? "TimeSource" (1001 maybe?).
    // The task said: "1.1 New/standard Kinds: TimeSource, SystemTime"
    // Ideally I should define these Kinds in `models` or just create them dynamically by name if possible?
    // ThingOS v0.2 uses Kind Strings? No, KindIds.
    // I should check `models` for Kind definitions.
    // If not present, I might need to abuse existing ones or define new ones.
    
    // For now, I'll assume I can just operate on SystemTime if I find it.
    // But I need to create TimeSource.
    
    // Wait, "SystemTime is the canonical current time Thing". 
    // "On boot: create/find SystemTime Thing and a TimeSource Thing named like rtc_cmos."
    
    loop {
        // 1. Read RTC
        unsafe {
            // Check syscall wrapper in thing_std or use raw
            // thing_std usually wraps syscalls.
            // I'll assume I can use invoke_syscall directly from lib or I need to add it to thing_std?
            // Userland shouldn't do raw syscalls if possible.
            // I will implement a helper here.
        }
        
        // 2. Publish/Update SystemTime
        
        // Sleep 1s
        thing_std::time::sleep(core::time::Duration::from_secs(1));
    }
}
