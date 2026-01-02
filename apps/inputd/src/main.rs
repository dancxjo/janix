#![no_std]
#![no_main]

extern crate alloc;

use thing_std::{abi, input_read, thing_create, thing_find, relationship_create, symbol_intern, thing_create_under};
use thing_std::abi::ids::{ThingId, SymbolId};
use alloc::vec::Vec;
use core::time::Duration;

mod scancodes;
use scancodes::{Parser, KeyEvent, KeyCode, Modifiers};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    thing_std::console_write("inputd: panic!\n");
    thing_std::sys_exit(1);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    thing_std::init(0); // Initialize if needed, though usually automatic via crt?
    // Actually thing_std::init is manual in current environment?
    // We rely on simple main for now if we had a runtime, but here we are raw _start.
    // For now, let's just run logic.
    
    main();
    thing_std::sys_exit(0);
}

fn main() {
    thing_std::console_write("inputd: starting...\n");
    
    // 1. Setup Graph
    // Ensure place.input exists
    let place_root = thing_find("place.root").unwrap_or(ThingId(2)); // Fallback
    
    // We should look for place.input or create it?
    // Usually created by boot or we create it.
    let input_place_name = symbol_intern("place.input");
    // let input_place = thing_find("place.input").unwrap_or_else(|| {
    //      let id = thing_create_under(symbol_intern("place"), place_root);
    //      thing_std::thing_register_name(id, "place.input");
    //      id
    // });
    // Assuming it exists or we just create our own scope for now.
    // Let's create a local place for this session if not found.
    let input_place = thing_create_under(symbol_intern("place"), place_root);
    // We can't register global names easily yet if not privileged store?
    // The plan said "place.input contains x".
    // For now, let's just use this place.
    
    thing_std::console_write("inputd: place.input created/found\n");

    let mut parser = Parser::new();
    let mut mods = Modifiers::default();
    let mut buf = [0u8; 32];
    
    // Intern symbols
    let sym_key_event = symbol_intern("key_event");
    let sym_text_event = symbol_intern("text_event");
    let sym_contains = symbol_intern("contains");
    let sym_focused = symbol_intern("focused");
    let sym_target = symbol_intern("target");
    let sym_scancode = symbol_intern("scancode");
    let sym_unicode = symbol_intern("unicode"); // value.u32 (codepoint)
    let sym_modifiers = symbol_intern("modifiers"); // value.u32 (flags)
    
    loop {
        let n = input_read(&mut buf);
        if n == 0 {
            // Sleep / Yield
            thing_std::sched_yield();
            continue;
        }

        for i in 0..n {
            let byte = buf[i];
            
            // Check for raw debug
            // thing_std::log_info(&alloc::format!("inputd: byte {:02x}", byte));
            
            if let Some(event) = parser.parse(byte, &mods) {
                // Update Modifiers
                match event.code {
                    KeyCode::LeftShift | KeyCode::RightShift => mods.shift = event.pressed,
                    KeyCode::LeftCtrl | KeyCode::RightCtrl => mods.ctrl = event.pressed,
                    KeyCode::LeftAlt | KeyCode::RightAlt => mods.alt = event.pressed,
                    _ => {}
                }
                
                // 1. Create Key Event Thing
                let key_thing = thing_create_under(sym_key_event, input_place);
                
                // Set payload or relationship?
                // Plan: predicates.
                // sys_rel_create or logic?
                // We don't have value predicates fully handy in syscalls yet (SYS_VAL_CREATE?).
                // We'll just create the Thing for now to show flow.
                // Ideally: key_event --[scancode]--> value.u32(code)
                
                // 2. Focused Window Logic
                // Find what is focused.
                // Query: place.input --[focused]--> ?
                // For now, let's look for *any* relationship "focused" from input_place.
                // We need `relationships_from`.
                
                let mut rels = [ThingId(0); 16];
                let count = thing_std::relationships_from_into(input_place, &mut rels);
                // We actually get ThingIds of *Relationships* or Targets?
                // SYS_REL_GET_FROM returns relationships. We need to inspect them.
                // But we can't inspect relationship contents (predicate/target) easily with current thing_std wrapper?
                // We need `relationship_get`? Not implemented in `thing_std`.
                // Wait, `relationships_from` usually returns IDs of relationships.
                // We need to read them.
                
                // WORKAROUND: Focus is tricky without rich graph read.
                // For Acceptance, we can just broadcast or assume a fixed target for "Echo".
                // Or better: Use a hardcoded focus target if found.
                // Let's create "text_event" always for now.
                
                if event.pressed {
                     if let Some(ch) = scancodes::to_char(event.code, mods.shift) {
                          // Create Text Event
                          let text_thing = thing_create_under(sym_text_event, input_place);
                          
                          // If we had focus, we'd link text_thing --[target]--> focused_window
                          // For logic, let's try to link to *something* so Echo sees it.
                          // Echo will look for text_event.
                          // If Echo claims to be "window.echo", we can target it.
                          // But we don't know window.echo ID.
                          
                          // Simply creating `text_event` in `place.input` is enough for Echo 
                          // to find it if Echo watches `place.input --[contains]--> text_event`.
                          
                          thing_std::console_write("inputd: text event\n");
                     }
                }
            }
        }
    }
}
