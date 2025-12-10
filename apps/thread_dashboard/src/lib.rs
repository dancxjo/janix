#![no_std]

extern crate alloc;

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
            format_args!(
                "=== Thread dashboard tick {} ({} ms since start) ===",
                tick, elapsed_ms
            ),
        );

        let threads: Vec<ThreadThing> = list_things_by_kind::<S, ThreadThing>(sys);
        log_thread_snapshot(sys, &threads);

        sys.yield_now();
    }

    sys.exit_thread()
}

fn log_thread_snapshot<S: Sys>(sys: &mut S, threads: &[ThreadThing]) {
    if threads.is_empty() {
        println(sys, "  (no ThreadInfo Things found)");
        return;
    }

    for t in threads {
        log_dynamic(
            sys,
            format_args!(
                "  tid={:<4} state={:<10} priority={} runtime_ns={}",
                t.tid, t.state, t.priority, t.runtime_ns,
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::log_thread_snapshot;
    use abi::{KernelRequest, ThingId};
    use alloc::string::ToString;
    use alloc::vec::Vec;
    use userland_std::{ThreadThing, doc_helpers::DocSys};

    fn thread(id: u64, tid: u64, state: &str) -> ThreadThing {
        ThreadThing {
            id: ThingId(id),
            tid,
            state: state.to_string(),
            priority: 1,
            runtime_ns: 123,
            last_started_ns: 0,
        }
    }

    #[test]
    fn logs_each_thread_line() {
        let mut sys = DocSys::with_responses(Vec::new());
        let mut threads = Vec::new();
        threads.push(thread(2, 10, "Running"));
        threads.push(thread(3, 20, "Sleeping"));
        log_thread_snapshot(&mut sys, &threads);

        let requests = sys.requests.borrow();
        assert!(requests.iter().any(|request| match request {
            KernelRequest::Log { message } =>
                message.contains("tid=10") && message.contains("Running"),
            _ => false,
        }));
        assert!(requests.iter().any(|request| match request {
            KernelRequest::Log { message } =>
                message.contains("tid=20") && message.contains("Sleeping"),
            _ => false,
        }));
    }

    #[test]
    fn reports_missing_threads_gracefully() {
        let mut sys = DocSys::with_responses(Vec::new());
        log_thread_snapshot(&mut sys, &[]);
        let requests = sys.requests.borrow();
        assert!(requests.iter().any(|request| match request {
            KernelRequest::Log { message } => message.contains("(no ThreadInfo Things found)"),
            _ => false,
        }));
    }
}
