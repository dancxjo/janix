#![no_std]
#![no_main]

use standard::prelude::*;
use stem::{sys_spawn_module, sys_yield};

#[no_mangle]
pub fn main() -> i32 {
    println!("sprout: hello");

    // Spawn clock
    // For now we assume typical paths.
    // The previous implementation used "/boot/modules/clock"
    // We should make sure we know what the paths are.
    // The plan said:
    // /boot/modules/sprout
    // /boot/modules/rtc_cmos
    // /boot/modules/clock
    
    let modules = [
        "/boot/modules/rtc_cmos",
        "/boot/modules/clock",
    ];

    for mod_path in modules {
        print!("sprout: spawning {}... ", mod_path);
        let tid = unsafe { sys_spawn_module(mod_path) };
        if tid >= 0 {
            println!("ok tid={}", tid);
        } else {
            println!("failed err={}", tid);
        }
    }

    loop {
        // Heartbeat or just yield forever
        unsafe { sys_yield() };
    }
}
