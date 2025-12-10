#![no_std]

use userland::prelude::*;

pub fn run<S: Sys>(sys: &mut S) -> ! {
    println(sys, "thread_dashboard: run() reached");

    let start = sys.time_monotonic_ns();

    // Show 10 snapshots, one per "tick".
    for tick in 0..10 {
        let now = sys.time_monotonic_ns();
        let elapsed_ms = (now - start) / 1_000_000;

        log_dynamic(
            sys,
            format!(
                "=== Thread dashboard tick {} ({} ms since start) ===",
                tick, elapsed_ms
            ),
        );

        let threads: Vec<ThreadThing> = list_things_by_kind::<S, ThreadThing>(sys);

        if threads.is_empty() {
            log_dynamic(sys, "  (no ThreadInfo Things found)".into());
        } else {
            for t in threads.iter() {
                log_dynamic(
                    sys,
                    format!(
                        "  tid={:<4} state={:<10} priority={} runtime_ns={}",
                        t.tid, t.state, t.priority, t.runtime_ns,
                    ),
                );
            }
        }

        sys.yield_now();
    }

    sys.exit_thread()
}
