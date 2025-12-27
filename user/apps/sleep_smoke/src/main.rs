#![no_std]
#![no_main]

extern crate alloc;

use thing_std::{GraphClient, StdoutConsole, Console};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let g = GraphClient::new();
    let c = StdoutConsole;
    c.write_str("SLEEP SMOKE: Starting...\n");
    
    // 1. Monotonic Start
    let start = thing_std::time::monotonic_ns(&g).unwrap_or(0);
    c.write_str("Start: "); c.write_u64(start); c.write_str(" ns\n");
    
    // 2. Sleep 200ms
    c.write_str("Sleeping 200ms...\n");
    let target_wake = start + 200_000_000;
    
    // We expect to wake around target_wake (or slightly after)
    let waking_at = thing_std::time::sleep_ms(&g, 200).unwrap_or(0);
    
    c.write_str("Woke:  "); c.write_u64(waking_at); c.write_str("\n");
    
    // 3. Monotonic End (now)
    let end = thing_std::time::monotonic_ns(&g).unwrap_or(0);
    c.write_str("End:   "); c.write_u64(end); c.write_str("\n");
    
    let elapsed = if end >= start { end - start } else { 0 };
    c.write_str("Elapsed: "); c.write_u64(elapsed); c.write_str(" ns\n");
    
    // Validation
    // 200ms = 200,000,000 ns
    if elapsed >= 200_000_000 {
        c.write_str("PASS: Elapsed >= 200ms\n");
    } else {
        c.write_str("FAIL: Elapsed < 200ms\n");
    }
    
    // verify sleep_until
    c.write_str("Testing sleep_until...\n");
    let now = thing_std::time::monotonic_ns(&g).unwrap_or(0);
    let until = now + 100_000_000; // +100ms
    let _ = thing_std::time::sleep_until_ns(&g, until);
    let now2 = thing_std::time::monotonic_ns(&g).unwrap_or(0);
    let elapsed2 = now2 - now;
    c.write_str("Elapsed2: "); c.write_u64(elapsed2); c.write_str("\n");
    if elapsed2 >= 100_000_000 {
         c.write_str("PASS: sleep_until >= 100ms\n");
    } else {
         c.write_str("FAIL: sleep_until < 100ms\n");
    }
    
    loop { core::hint::spin_loop(); }
}
