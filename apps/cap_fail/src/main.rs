#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    log_info("CAP FAIL: PANIC!");
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start(syscall_ptr: u64) -> ! {
    thing_std::init(syscall_ptr);
    log_info("CAP FAIL: Starting...");

    let root_id = get_root_place();
    log_info("CAP FAIL: Trying to create under ROOT (should fail)...");
    
    // We expect this to fail because we only have caps for place.user.<me>
    let valid_kind = symbol_intern("kind.thing");
    let id = thing_create_under(valid_kind, root_id);
    
    if id.0 == 0 {
        log_info("CAP FAIL: Success! Creation denied (ID=0).");
    } else {
        log_info("CAP FAIL: ERROR! Creation SUCCEEDED (should have failed).");
    }

    sys_exit(0);
}
