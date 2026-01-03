#![no_std]
#![no_main]

extern crate alloc;

use thing_std::{abi, console_write, surface_create, surface_draw, thing_find, input_read, sched_yield, sys_exit};
use thing_std::abi::ids::{ThingId};
use alloc::vec::Vec;
use scancodes::{Parser, Modifiers};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    console_write("logview: panic!\n");
    sys_exit(1);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    thing_std::init(0);
    main();
    sys_exit(0);
}

fn main() {
    console_write("logview: starting...\n");

    // 1. Create a surface to draw into (tiny for test)
    let w = 100;
    let h = 100;
    let surface = match surface_create(w, h, 0) {
        Ok(s) => s,
        Err(e) => {
            let mut msg = [0u8; 64];
            // Simple itoa or just print msg
            console_write("logview: failed to create surface err=");
            // we don't have itoa, just print something
            console_write("!\n");
            sys_exit(e);
            return;
        }
    };

    // 2. Find primary display to blit into? 
    // For now, our surface_draw might just blit to the physical display if we target it.
    // But the user said "Interactive Log Panel".
    
    let mut parser = Parser::new();
    let mods = Modifiers::default();
    let mut buf = [0u8; 32];
    let mut fb = alloc::vec![0u8; (w * h * 4) as usize];
    console_write("logview: loop reached\n");

    loop {
        // Clear background (Dark blue)
        for i in 0..((w * h) as usize) {
            fb[i * 4 + 0] = 0x22; // B
            fb[i * 4 + 1] = 0x11; // G
            fb[i * 4 + 2] = 0x00; // R
            fb[i * 4 + 3] = 0xFF; // A
        }

        // Draw some "Text" (Horizontal line for now)
        for x in 10..90 {
            let offset = (50 * w + x) * 4;
            fb[offset as usize + 0] = 0xFF;
            fb[offset as usize + 1] = 0xFF;
            fb[offset as usize + 2] = 0xFF;
        }

        // Blit
        surface_draw(surface, &fb, 0, 0, w as u32, h as u32);

        // Read input
        let n = input_read(&mut buf);
        if n > 0 {
            for i in 0..n {
                if let Some(event) = parser.parse(buf[i], &mods) {
                    if event.pressed {
                        console_write("logview: key pressed\n");
                    }
                }
            }
        }

        sched_yield();
    }
}
