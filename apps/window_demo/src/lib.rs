#![no_std]

extern crate alloc;

use alloc::format;
use userland::prelude::*;

pub fn run<S: Sys>(sys: &mut S) -> ! {
    println(sys, "window_demo: starting");
    let handle = match create_window(sys, "Demo", 1) {
        Some(h) => h,
        None => {
            println(sys, "window_demo: failed to create window");
            sys.exit_thread();
        }
    };

    let mut counter = 0_u64;
    loop {
        let text = format!("Hello from windowed world!\nCounter: {}", counter);
        set_window_text(sys, handle, &text);
        counter = counter.saturating_add(1);
        sys.sleep_for_ns(1_000_000_000);
    }
}
