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

    // Get root place
    let root = get_root_place();
    log_info("SPROUT: acquired root place");

    // Create a demo Thing
    let kind_thing = symbol_intern("kind.Thing");
    let demo_thing = thing_create(kind_thing, SymbolId::INVALID, 1);
    log_info("SPROUT: created demo thing");

    // Relate it to root via contains
    let pred_contains = symbol_intern("predicate.contains");
    let rel = relationship_create(root, demo_thing, pred_contains);
    log_info("SPROUT: linked into place.root via contains");

    // Query containment
    let count = contained_in(root);
    // Note: Kernel previously had 1 (kernel itself), now should be 2.
    if count >= 2 {
        log_info("SPROUT: root contains expected count (>= 2)");
    } else {
        log_info("SPROUT: root contains UNEXPECTED count");
    }

    loop {}
}

