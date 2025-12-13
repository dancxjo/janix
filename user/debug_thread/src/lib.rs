#![no_std]

extern crate alloc;

use thing_os::prelude::*;

pub fn run<S: Sys>(sys: &mut S) -> ! {
    println(sys, "debug_thread: run() reached");

    let start = sys.time_monotonic_ns();

    // Run continuously, showing a snapshot every second.
    let mut tick: u64 = 0;
    loop {
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

        tick = tick.saturating_add(1);
        sys.sleep_for_ns(1_000_000_000);
    }
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
    use abi::{KernelRequest, KernelResponse, ThingId};
    use alloc::{
        string::{String, ToString},
        vec::Vec,
    };
    use core::cell::RefCell;
    use runtime::Sys;
    use thing_os::{ThreadThing, doc_helpers::DocSys};

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

    struct CaptureLogSys {
        inner: DocSys,
        logs: RefCell<Vec<String>>,
    }

    impl CaptureLogSys {
        fn with_responses(responses: Vec<KernelResponse>) -> Self {
            Self {
                inner: DocSys::with_responses(responses),
                logs: RefCell::new(Vec::new()),
            }
        }

        fn logs(&self) -> Vec<String> {
            self.logs.borrow().clone()
        }
    }

    impl Sys for CaptureLogSys {
        fn syscall(&self, request: KernelRequest) -> KernelResponse {
            if let KernelRequest::Log { message } = request {
                self.logs.borrow_mut().push(message.to_string());
            }
            self.inner.syscall(request)
        }

        fn time_now_ns(&mut self) -> u64 {
            self.inner.time_now_ns()
        }

        fn time_monotonic_ns(&mut self) -> u64 {
            self.inner.time_monotonic_ns()
        }

        fn time_system_ns(&mut self) -> u64 {
            self.inner.time_system_ns()
        }

        fn sleep_for_ns(&mut self, delta_ns: u64) {
            self.inner.sleep_for_ns(delta_ns)
        }

        fn sleep_until_ns(&mut self, deadline_ns: u64) {
            self.inner.sleep_until_ns(deadline_ns)
        }

        fn yield_now(&mut self) {
            self.inner.yield_now()
        }

        fn exit_thread(&mut self) -> ! {
            self.inner.exit_thread();
        }
    }

    #[test]
    fn logs_each_thread_line() {
        let mut responses = Vec::new();
        responses.push(KernelResponse::Success { data: None });
        responses.push(KernelResponse::Success { data: None });
        let mut sys = CaptureLogSys::with_responses(responses);
        let mut threads = Vec::new();
        threads.push(thread(2, 10, "Running"));
        threads.push(thread(3, 20, "Sleeping"));
        log_thread_snapshot(&mut sys, &threads);

        let log_messages = sys.logs();
        assert_eq!(log_messages.len(), 2);
        assert!(
            log_messages
                .iter()
                .any(|message| { message.contains("tid=10") && message.contains("Running") })
        );
        assert!(
            log_messages
                .iter()
                .any(|message| { message.contains("tid=20") && message.contains("Sleeping") })
        );
    }

    #[test]
    fn reports_missing_threads_gracefully() {
        let mut responses = Vec::new();
        responses.push(KernelResponse::Success { data: None });
        let mut sys = DocSys::with_responses(responses);
        log_thread_snapshot(&mut sys, &[]);
        let requests = sys.requests.borrow();
        assert!(requests.iter().any(|request| match request {
            KernelRequest::Log { message } => message.contains("(no ThreadInfo Things found)"),
            _ => false,
        }));
    }
}
