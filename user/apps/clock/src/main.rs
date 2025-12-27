#![no_std]
#![no_main]

extern crate alloc;

use thing_std::{GraphClient, StdoutConsole, Console};

#[derive(serde::Serialize)]
pub struct TimeNowReq {}

#[derive(serde::Deserialize)]
pub struct TimeNowResp {
    pub system_ns: u64,
    pub monotonic_ns: u64,
}

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
    let mut out = [0u8; 256];
    
    let mut last_print_ns: u64 = 0;
    // let mut last_err_ns: u64 = 0; // Unused for now

    loop {
        // We poll efficiently-ish. 
        // In V1, we'd sleep. In V0, we spin but only print on specific delta.
        match g.call::<TimeNowReq, TimeNowResp>("time.now", &TimeNowReq {}, &mut out) {
            Ok(t) => {
                if t.monotonic_ns >= last_print_ns + 1_000_000_000 {
                    let (h, m, s) = format_hms(t.system_ns);
                    
                    c.write_str("clock: ");
                    c.write_2d(h);
                    c.write_str(":");
                    c.write_2d(m);
                    c.write_str(":");
                    c.write_2d(s);
                    c.write_str("\n");

                    last_print_ns = t.monotonic_ns;
                }
            }
            Err(_) => {
                // Determine time? We can't... so just monotonic guess or spin count?
                // We just rely on spin count for error backoff if syscall fails entirely.
                 c.write_str("clock: time.now failed\n");
                 for _ in 0..10_000_000 { core::hint::spin_loop(); }
            }
        }

        // Yield/Sleep placeholder
        // Check frequently so we hit the second boundary close to correct
        for _ in 0..100_000 { core::hint::spin_loop(); }
    }
}
