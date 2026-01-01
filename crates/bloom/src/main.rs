#![no_std]
#![no_main]

extern crate alloc;

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

    // 4. Watch place.desktop
    watch(bloom_id, desktop_id);
    log_info("BLOOM: watching desktop");

    // 5. Watch place.log
    let log_sym = symbol_intern("place.log");
    let log_id = thing_find_by_name(log_sym).expect("place.log not found");
    watch(bloom_id, log_id);
    log_info("BLOOM: watching place.log");

    // 6. Reactive Loop
    log_info("BLOOM: entering event loop");
    loop {
        let event_id = wait_event(bloom_id);
        let view = thing_std::event::decode_event(event_id);
        
        if let (Some(from), Some(to), Some(pred)) = (view.rel_from, view.rel_to, view.rel_predicate) {
            // Check for Log events
            let contains_sym = symbol_intern("predicate.contains");
            if from == log_id && pred == contains_sym {
                // Log entry added
                let entry_id = to;
                let mut buf = [0u8; 256];
                let len = thing_get_payload(entry_id, &mut buf);
                if len > 11 { // 1+8+2
                     let level_byte = buf[0];
                     // let sub_sym = u64::from_le_bytes(...);
                     // let msg_len = ...
                     // offset: 1(level) + 8(sub) + 2(len) = 11
                     
                     let msg_len = u16::from_le_bytes(buf[9..11].try_into().unwrap()) as usize;
                     if 11 + msg_len <= len {
                         let msg = core::str::from_utf8(&buf[11..11+msg_len]).unwrap_or("<invalid utf8>");
                         let level_str = match level_byte {
                             2 => "INFO", 3 => "WARN", 4 => "ERROR", _ => "LOG"
                         };
                         let out = alloc::format!("BLOOM: LOG[{}]: {}", level_str, msg);
                         console_write(&out);
                         console_write("\n");
                     } else {
                         let out = alloc::format!("BLOOM: LOG entry payload truncation: len={} msg_len={}", len, msg_len);
                         console_write(&out);
                         console_write("\n");
                     }
                } else {
                     let out = alloc::format!("BLOOM: LOG entry too small: len={}", len);
                     console_write(&out);
                     console_write("\n");
                }
            } else {
                let msg = alloc::format!("BLOOM: event relationship_created from={:?} to={:?} predicate={:?}", from, to, pred);
                console_write(&msg);
                console_write("\n");
            }
        } else {
            console_write("BLOOM: event received (opaque target)\n");
            if let Some(target) = view.target {
                let msg = alloc::format!("BLOOM: target={:?}", target);
                console_write(&msg);
                console_write("\n");
            }
        }
    }
}
