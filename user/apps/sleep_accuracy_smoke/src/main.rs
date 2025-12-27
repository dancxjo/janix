#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use thing_std::{GraphClient, StdoutConsole, Console};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let g = GraphClient::new();
    let c = StdoutConsole;
    c.write_str("SLEEP ACCURACY SMOKE: Starting...\n");

    let durations_ms = [1, 2, 5, 11, 17, 50, 123];
    let iterations = 5;

    for &ms in &durations_ms {
        c.write_str("Testing "); c.write_u64(ms); c.write_str("ms...\n");
        
        let mut min_error = u64::MAX;
        let mut max_error = 0;
        let mut total_error = 0;

        for _ in 0..iterations {
            let start = thing_std::time::monotonic_ns(&g).unwrap_or(0);
            let _ = thing_std::time::sleep_ms(&g, ms);
            let end = thing_std::time::monotonic_ns(&g).unwrap_or(0);
            
            let elapsed = end - start;
            let target_ns = ms * 1_000_000;
            
            let extra = if elapsed > target_ns { elapsed - target_ns } else { 0 };
            
            if extra < min_error { min_error = extra; }
            if extra > max_error { max_error = extra; }
            total_error += extra;
        }

        let avg_error = total_error / iterations as u64;

        c.write_str("  Min Err: "); c.write_u64(min_error / 1000); c.write_str(" us\n");
        c.write_str("  Avg Err: "); c.write_u64(avg_error / 1000); c.write_str(" us\n");
        c.write_str("  Max Err: "); c.write_u64(max_error / 1000); c.write_str(" us\n");
    }

    c.write_str("done\n");
    loop { core::hint::spin_loop(); }
}
