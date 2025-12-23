#![no_std]
#![no_main]

use thing_os::prelude::*;
use thing_os::ThreadThing;

#[thing_os::main]
fn main() {
    println!("debug_thread: starting");

    if !ensure_schema_exists_for::<ThreadThing>() {
        println!("debug_thread: Thread schema missing");
        return;
    }

    let start = Instant::now();
    let mut tick: u64 = 0;

    loop {
        let elapsed_ms = start.elapsed().as_nanos() / 1_000_000;
        println!(
            "debug_thread: === Dashboard tick {} ({} ms) ===",
            tick, elapsed_ms
        );

        let threads: Vec<ThreadThing> = list_things_by_kind();
        if threads.is_empty() {
            println!("debug_thread:   (no Thread Things found)");
        } else {
            for t in threads {
                println!(
                    "debug_thread:   tid={:<4} state={:<10} priority={} runtime_ns={}",
                    t.tid, t.state, t.priority, t.runtime_ns
                );
            }
        }

        tick = tick.wrapping_add(1);
        sleep_ms(1000);
    }
}
