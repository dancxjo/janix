#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    log_info("CLOCK: PANIC!");
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start(syscall_ptr: u64) -> ! {
    thing_std::init(syscall_ptr);
    log_info("CLOCK: Starting...");

    // 1. Create Surface Bytespace
    let width = 320;
    let height = 200;
    let size = width * height * 4;
    let bs_id = bytespace_create(size as u64).expect("CLOCK: Failed to create bytespace");
    log_info("CLOCK: Bytespace created");

    // 2. Map it
    let vaddr = 0x4000_0000;
    space_map(bs_id, vaddr, 0, size as u64).expect("CLOCK: Failed to map");
    let buffer = vaddr as *mut u32;

    // 3. Create Graph Things
    // surface.clock (backed by bs_id)
    // window.clock (targets surface.clock)
    
    // We need to intern kinds first or use helper constants if available in ThingStd? 
    // ThingStd doesn't have the new symbols yet. We will intern them.
    let kind_surface = symbol_intern("kind.surface");
    let kind_window = symbol_intern("kind.window");
    let pred_backs = symbol_intern("predicate.backs");
    let pred_targets = symbol_intern("predicate.targets");
    let pred_primary = symbol_intern("predicate.primary"); // Not needed for app window
    
    // Create Surface
    let surf = thing_create_under(kind_surface, get_root_place()); // Use root or user place? Root for now.
    thing_register_name(surf, "surface.clock");
    
    // Link Backs
    relationship_create(surf, bs_id, pred_backs);
    
    // Create Window
    let win = thing_create_under(kind_window, get_root_place());
    thing_register_name(win, "window.clock");
    
    // Link Targets
    relationship_create(win, surf, pred_targets);
    
    // Add to place.windows?
    if let Some(place_wins) = thing_find("place.windows") {
        let pred_contains = symbol_intern("predicate.contains");
        relationship_create(place_wins, win, pred_contains);
    }
    
    log_info("CLOCK: Registered window.clock and surface.clock");

    // Drawing Loop
    let mut frame = 0;
    loop {
        let color = frame as u32; // changing color
        unsafe {
            for i in 0..(width*height) {
                *buffer.add(i as usize) = 0xFF000000 | color;
            }
        }
        
        if frame % 100 == 0 {
            log_info("CLOCK: Drawing...");
        }
        
        frame += 1;
        // yield
         for _ in 0..10000 {}
    }
}
