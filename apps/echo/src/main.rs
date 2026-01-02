#![no_std]
#![no_main]

extern crate alloc;
use thing_std::{thing_find, symbol_intern, relationships_from_into, sys_exit, console_write, sched_yield};
use thing_std::abi::ids::ThingId;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    thing_std::console_write("echo: panic!\n");
    thing_std::sys_exit(1);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    main();
    sys_exit(0);
}

fn main() {
    console_write("echo: starting...\n");
    
    // 1. Find place.input (we assume inputd created it or we need to find it)
    // Since inputd creates a new one each time in my placeholder code, we have a sync problem.
    // Ideally, `place.input` should be stable.
    // For this demo, let's verify if `thing_find("place.input")` works if inputd registered it.
    // If not, we might need to rely on a shared root scan.
    
    // Wait for place.input?
    let mut place_input = None;
    while place_input.is_none() {
        // console_write("echo: looking for place.input...\n");
        // In the inputd code, I didn't successfully register the name globaly because I doubted the syscall.
        // Let's modify inputd to register the name if possible, or use a known strategy.
        // Strategy: Scan `place.root` for `place` kind things?
        // Too complex for simple demo.
        
        // Let's rely on finding by name.
        if let Some(id) = thing_find("place.input") {
            place_input = Some(id);
        } else {
             sched_yield();
        }
    }
    let place_input = place_input.unwrap();
    console_write("echo: found place.input\n");
    
    let sym_text_event = symbol_intern("text_event");
    let mut seen_events: alloc::vec::Vec<ThingId> = alloc::vec::Vec::new();
    
    loop {
        // Poll for new text events in place.input
        // Query: place.input --[contains]--> ?
        let mut rels = [ThingId(0); 32];
        let n = relationships_from_into(place_input, &mut rels);
        
        // This gives *Relationships*. We can't verify predicates easily yet.
        // But we can check if we've seen this relationship ID before?
        // Or we need `get_relationship(rel_id)`?
        // thing_std is missing `get_relationship`.
        
        // Panic/Halt: We are missing graph read tools in thing_std to do this cleanly.
        // But wait, `sys_rel_get_from` fills a buffer. What format?
        // abi says: `sys_rel_get_from` fills buffer with... what?
        // Usually `(rel_id, predicate, target)`.
        // `thing_std::relationships_from_into` treats it as array of `ThingId`.
        // The current `thing_std` implementation: `let buf_len = out.len() * 16;`
        // 16 bytes = 128 bit ThingId.
        // So it likely returns just Relationship IDs.
        
        // Without `get_relationship`, we are blind.
        // We need to implement `get_relationship` in thing_std or similar.
        // Or assume simple polling of *Things* if we have `things_in(place)`.
        
        // Alternative: Use `inputd` to print!
        // `echo` is redundant if `inputd` prints "text event".
        // But loop said "demo app receives typed text".
        
        // I will implement `get_relationship` or equivalent in thing_std to fix this.
        sched_yield();
    }
}
