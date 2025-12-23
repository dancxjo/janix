#![no_std]
#![no_main]

use thing_os::prelude::*;
use thing_os::ThreadThing;

const DASHBOARD_INTERVAL_MS: u64 = 5_000;

#[thing_os::main]
fn main() {
    println!("taskman: starting");

    if !ensure_schema_exists_for::<ThreadThing>() {
        println!("taskman: Thread schema missing");
        return;
    }

    let start = Instant::now();
    let mut tick: u64 = 0;

    loop {
        let elapsed_ms = start.elapsed().as_nanos() / 1_000_000;
        println!(
            "taskman: === Dashboard tick {} ({} ms) ===",
            tick, elapsed_ms
        );

        let threads: Vec<ThreadThing> = list_things_by_kind();
        if threads.is_empty() {
            println!("taskman:   (no Thread Things found)");
        } else {
            for t in threads {
                println!(
                    "taskman:   tid={:<4} state={:<10} priority={} runtime_ns={} last_started_ns={} sleep_until_ns={}",
                    t.tid, t.state, t.priority, t.runtime_ns, t.last_started_ns, t.sleep_until_ns
                );
            }
        }

        tick = tick.wrapping_add(1);
        sleep_ms(DASHBOARD_INTERVAL_MS);
    }
}
