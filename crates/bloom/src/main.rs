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
    let pred_provides = symbol_intern("predicate.provides");

    // 1. Find Bloom identity from handoff
    let handoff_sym = symbol_intern("thing.handoff.bloom");
    let handoff_id = thing_find_by_name(handoff_sym).expect("Bloom handoff Thing not found");
    
    let mut bloom_id_bytes = [0u8; 16];
    thing_get_payload(handoff_id, &mut bloom_id_bytes);
    let bloom_id = abi::ids::ThingId(u128::from_le_bytes(bloom_id_bytes));
    log_info("BLOOM: acquired identity from handoff");

    let root_id = get_root_place();

    // 2. Create Desktop Place
    let desktop_sym = symbol_intern("place.desktop");
    let desktop_id = thing_create_named(desktop_sym, kind_place, SymbolId::INVALID);
    log_info("BLOOM: desktop place created");

    // 3. Publish provides relationship
    relationship_create(bloom_id, desktop_id, pred_provides);
    log_info("BLOOM: published provides relationship");

    // Ensure desktop is in root
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
