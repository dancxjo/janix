#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    log_info("GRAPH SMOKE: PANIC!");
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start(syscall_ptr: u64) -> ! {
    thing_std::init(syscall_ptr);
    log_info("GRAPH SMOKE: Starting...");

    // 1. Get our Task ID (from boot grant context? Only place.user is granted).
    // We assume we know our user place? Or we query "place.permissions"?
    // For now, let's try to create a Thing under the Root (should fail?) or under our User Place.
    // We don't have a way to find our User Place ID easily yet.
    // BUT! `sys_thing_create` validates capability.
    // If we have a Global Create Cap (boot grant), we can create anywhere? NO.
    // Boot grant targetted `place.user.<task_id>`.
    
    // How do we find `place.user.<task_id>`?
    // We need `sys_find_by_name`.
    // I stubbed `sys_find_by_name` in thing_std!
    // So `graph_smoke` can't find its place.
    
    // Immediate Fix: Add back SYS_FIND or `SYS_GET_PROCESS_INFO` equivalent.
    // OR: Assume a hardcoded "My User Place" ID passed in args?
    // OR: Iterate root children and look for "place.user..."
    
    log_info("GRAPH SMOKE: Warning - Cannot find user place yet. Trying arbitrary ID 100 for parent which will likely fail.");
    
    // Try to create under Root (ID 2). Should Fail (No CAP).
    // let kind_thing = symbol_intern("kind.thing");
    // let root = PlaceId(2);
    // let t = thing_create_under(kind_thing, root);
    // log_info("Created under root?");
    
    sys_exit(0);
}
