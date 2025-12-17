#![no_std]
#![no_main]

extern crate alloc;
use alloc::format;
use thing_os::prelude::*;
use thing_os::ui::{create_window, set_window_text};

#[thing_os::main]
fn main() {
    println!("window_demo: starting");

    // Open a simple window
    let handle = create_window("Window Demo").expect("Failed to create window");
    
    println!("window_demo: window created");

    let mut counter = 0_u64;
    
    // Main Event Loop
    loop {
        let text = format!("Hello from windowed world!\nCounter: {}", counter);
        set_window_text(handle, &text);
        
        counter = counter.wrapping_add(1);
        
        sleep_ms(1000);
    }
}

