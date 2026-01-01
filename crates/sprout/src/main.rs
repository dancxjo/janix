#![no_std]
#![no_main]

use core::panic::PanicInfo;
use thing_std::*;
use abi::ids::SymbolId;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start(syscall_ptr: u64) -> ! {
    thing_std::init(syscall_ptr);
    log_info("SPROUT: starting world interaction demo");

    // Sprout now launches Bloom
    log_info("SPROUT: launching bloom");
    proc_spawn("bloom");

    log_info("SPROUT: world interaction demo complete (handoff to bloom)");
    loop {}
}

