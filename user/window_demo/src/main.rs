#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(target_os = "none")]
mod app {
    extern crate alloc;
    use thing_os::prelude::*;
    use examples_support::{log, sleep_ms, UserlandSys};
    use alloc::format;

    #[unsafe(no_mangle)]
    pub fn main() {
        let mut sys = examples_support::init();
        if let Err(e) = run(&mut sys) {
            log(&mut sys, "window_demo", "Error running app");
        }
    }

    fn run(sys: &mut UserlandSys) -> Result<(), ()> {
        log(sys, "window_demo", "starting");

        // Open a simple window
        let handle = create_window(sys, "Window Demo").ok_or(())?;
        
        log(sys, "window_demo", "window created");

        let mut counter = 0_u64;
        
        // Main Event Loop
        loop {
            let text = format!("Hello from windowed world!\nCounter: {}", counter);
            set_window_text(sys, handle, &text);
            
            counter = counter.wrapping_add(1);
            
            sleep_ms(sys, 1000);
        }
    }
}

#[cfg(not(target_os = "none"))]
fn main() {}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

