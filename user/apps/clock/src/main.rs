#![no_std]
#![no_main]

extern crate alloc;

use thing_std::{GraphClient, StdoutConsole, Console};

// Structs provided by thing_std::time


fn format_hms(system_ns: u64) -> (u64, u64, u64) {
    // seconds since epoch-ish
    let total = system_ns / 1_000_000_000;
    let s = total % 60;
    let m = (total / 60) % 60;
    let h = (total / 3600) % 24;
    (h, m, s)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let g = GraphClient::new();
    let c = StdoutConsole;
    c.write_str("CLOCK: Starting...\n");
    
    loop {
        match thing_std::time::time_now(&g) {
            Ok(t) => {
                let (h, m, s) = format_hms(t.system_ns);
                
                c.write_str("clock: ");
                c.write_2d(h);
                c.write_str(":");
                c.write_2d(m);
                c.write_str(":");
                c.write_2d(s);
                c.write_str("\n");
            }
            Err(_) => {
                 c.write_str("clock: time.now failed\n");
            }
        }

        // Sleep 1 second
        // We use sleep_ms to demonstrate sleep.
        let _ = thing_std::time::sleep_ms(&g, 1000);
    }
}
