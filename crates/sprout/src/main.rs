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

    let kind_thing = symbol_intern("kind.Thing");
    let pred_spawns = symbol_intern("predicate.spawns");

    // 1. Create Sprout identity
    let sprout_sym = symbol_intern("thing.sprout");
    let sprout_id = thing_create_named(sprout_sym, kind_thing, SymbolId::INVALID);

    // 2. Create Bloom identity ahead of time
    let bloom_sym = symbol_intern("thing.bloom");
    let bloom_id = thing_create_named(bloom_sym, kind_thing, SymbolId::INVALID);

    // 3. Publish spawns relationship
    relationship_create(sprout_id, bloom_id, pred_spawns);
    log_info("userland: SPROUT: published spawns relationship");

    // 4. Create handoff Thing for Bloom to find its ID
    let handoff_sym = symbol_intern("thing.handoff.bloom");
    let handoff_id = thing_create_named(handoff_sym, kind_thing, SymbolId::INVALID);
    
    // Store bloom_id in handoff payload (16 bytes)
    thing_set_payload(handoff_id, &bloom_id.0.to_le_bytes());

    // 5. Launch Bloom
    log_info("SPROUT: launching bloom");
    proc_spawn("bloom");

    log_info("SPROUT: world interaction demo complete (handoff to bloom)");
    loop {}
}

