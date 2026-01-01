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
    log_info("BLOOM: starting desktop seeding");

    let kind_place = symbol_intern("kind.Place");
    let kind_thing = symbol_intern("kind.Thing");
    let pred_contains = symbol_intern("predicate.contains");

    let bloom_sym = symbol_intern("thing.bloom");
    let desktop_sym = symbol_intern("place.desktop");
    let display_sym = symbol_intern("thing.display_primary");
    let pointer_sym = symbol_intern("thing.pointer");
    let wallpaper_sym = symbol_intern("thing.wallpaper_sky");

    let root_id = get_root_place();

    // Create Bloom Thing
    let bloom_id = thing_create(kind_thing, SymbolId::INVALID, 1);
    // Note: symbols are already registered in kernel for v0.3 demo, 
    // but we can't easily register names from userland yet without a dedicated syscall 
    // if thing_std doesn't support it.
    // For now, let's just create and link.
    log_info("BLOOM: bloom thing created");

    // Create Desktop Place
    let desktop_id = thing_create(kind_place, SymbolId::INVALID, 1);
    log_info("BLOOM: desktop place created");

    // Ensure they are in root
    relationship_create(root_id, bloom_id, pred_contains);
    relationship_create(root_id, desktop_id, pred_contains);

    // Create desktop facts
    let display_id = thing_create(kind_thing, SymbolId::INVALID, 1);
    let pointer_id = thing_create(kind_thing, SymbolId::INVALID, 1);
    let wallpaper_id = thing_create(kind_thing, SymbolId::INVALID, 1);

    relationship_create(desktop_id, display_id, pred_contains);
    relationship_create(desktop_id, pointer_id, pred_contains);
    relationship_create(desktop_id, wallpaper_id, pred_contains);

    log_info("BLOOM: desktop contains display/pointer/wallpaper");

    // Verification log
    let count = contained_in(desktop_id);
    if count == 3 {
        log_info("BLOOM: desktop contains expected count (3)");
    } else {
        log_info("BLOOM: desktop containment check FAILED");
    }

    loop {}
}
