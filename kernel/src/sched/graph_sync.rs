use crate::graph::{self, Graph};
use crate::graph_kinds;
use crate::sched::types::{Thread, ThreadState, SleepEntry};
use crate::sched::MAX_THREADS;
use abi::{ThreadId, ThingId};
use thing_models::PropValue;
use heapless::Vec;
use alloc::string::String;

pub struct RebuiltCache {
    pub run_queue: Vec<ThreadId, MAX_THREADS>,
    pub sleep_queue: Vec<SleepEntry, MAX_THREADS>,
    pub current: Option<ThreadId>,
}

pub fn rebuild_cache_from_graph(threads: &mut [Option<Thread>; MAX_THREADS]) -> RebuiltCache {
    let mut run_queue: Vec<ThreadId, MAX_THREADS> = Vec::new();
    let mut sleep_queue: Vec<SleepEntry, MAX_THREADS> = Vec::new();
    let mut current = None;

    // Iterate over local thread slots
    for slot in threads.iter_mut() {
        if let Some(thread) = slot {
            if let Some(thing_id) = thread.thing_id {
                // Sync properties from graph

                // State
                if let Some(val) = graph::get_prop(thing_id, "state") {
                    if let PropValue::Str(s) = val {
                         if let Some(st) = ThreadState::from_str(&s) {
                             thread.state = st;
                         }
                    }
                }

                // Priority
                if let Some(val) = graph::get_prop(thing_id, "priority") {
                    if let PropValue::U64(p) = val {
                        thread.priority = p;
                    }
                }

                // Sleep until
                if let Some(val) = graph::get_prop(thing_id, "sleep_until_ns") {
                    if let PropValue::U64(t) = val {
                        thread.sleep_until_ns = t;
                    }
                }
            }

            // Rebuild queues based on state
            match thread.state {
                ThreadState::Runnable | ThreadState::New => {
                    if run_queue.push(thread.id).is_err() {
                        crate::log("Run queue full during rebuild");
                    }
                }
                ThreadState::Sleeping => {
                    if sleep_queue.push(SleepEntry {
                        thread_id: thread.id,
                        wake_at_ns: thread.sleep_until_ns,
                    }).is_err() {
                        crate::log("Sleep queue full during rebuild");
                    }
                }
                ThreadState::Running => {
                    current = Some(thread.id);
                }
                _ => {}
            }
        }
    }

    // Sort run_queue to ensure fairness (Round Robin / Priority)
    // Primary: Priority (Descending)
    // Secondary: Last Run Start (Ascending) - approximate LRU/FIFO
    run_queue.sort_unstable_by(|a, b| {
        let idx_a = (a.0 - 1) as usize;
        let idx_b = (b.0 - 1) as usize;
        let thread_a = threads[idx_a].as_ref().unwrap();
        let thread_b = threads[idx_b].as_ref().unwrap();

        let prio_cmp = thread_b.priority.cmp(&thread_a.priority); // Descending
        if prio_cmp != core::cmp::Ordering::Equal {
            return prio_cmp;
        }
        thread_a.last_run_start_ns.cmp(&thread_b.last_run_start_ns) // Ascending
    });

    RebuiltCache {
        run_queue,
        sleep_queue,
        current,
    }
}

pub fn commit_decision_to_graph(
    old_tid: Option<ThreadId>,
    new_tid: Option<ThreadId>,
    old_state: ThreadState,
    new_state: ThreadState,
    threads: &[Option<Thread>; MAX_THREADS],
    runtime_ns: u64,
    now_ns: u64,
    cpu_thing_id: Option<ThingId>,
) {
    // 1. Update old thread
    if let Some(tid) = old_tid {
        let idx = (tid.0 - 1) as usize;
        if let Some(thread) = threads.get(idx).and_then(|t| t.as_ref()) {
            if let Some(thing_id) = thread.thing_id {
                let mut props = alloc::vec![
                     (crate::symbols::intern("state"), PropValue::Str(String::from(old_state.as_str()))),
                ];

                if runtime_ns > 0 {
                    props.push((crate::symbols::intern("runtime_ns"), PropValue::U64(thread.total_run_ns + runtime_ns)));
                }

                // If it was sleeping, ensure sleep_until is set (it should be set in thread struct by now)
                if old_state == ThreadState::Sleeping {
                    props.push((crate::symbols::intern("sleep_until_ns"), PropValue::U64(thread.sleep_until_ns)));
                } else if thread.state == ThreadState::Sleeping && old_state != ThreadState::Sleeping {
                    // Transitioned OUT of sleeping? No, old_state is what it IS NOW (after switch).
                    // If old_state != Sleeping, we clear sleep_until.
                     props.push((crate::symbols::intern("sleep_until_ns"), PropValue::U64(0)));
                }

                let _ = graph::update_thing(thing_id, props);

                // Remove runs_on link
                if let Some(cpu) = cpu_thing_id {
                     let _ = graph::remove_link(thing_id, graph_kinds::LINK_RUNS_ON, cpu);
                } else {
                     // Try to find it dynamically if not provided
                     let mut runs_on = [None; 1];
                     graph::neighbors(thing_id, graph_kinds::LINK_RUNS_ON, &mut runs_on);
                     if let Some(cpu_thing) = runs_on[0] {
                          let _ = graph::remove_link(thing_id, graph_kinds::LINK_RUNS_ON, cpu_thing);
                     }
                }
            }
        }
    }

    // 2. Update new thread
    if let Some(tid) = new_tid {
        let idx = (tid.0 - 1) as usize;
        if let Some(thread) = threads.get(idx).and_then(|t| t.as_ref()) {
             if let Some(thing_id) = thread.thing_id {
                 let props = alloc::vec![
                     (crate::symbols::intern("state"), PropValue::Str(String::from(new_state.as_str()))),
                     (crate::symbols::intern("last_started_ns"), PropValue::U64(now_ns)),
                 ];
                 let _ = graph::update_thing(thing_id, props);

                 // Add runs_on link
                 if let Some(cpu) = cpu_thing_id {
                      let _ = graph::add_link(thing_id, graph_kinds::LINK_RUNS_ON, cpu);
                 }
             }
        }
    }
}

pub fn commit_thread_wake(thread: &Thread) {
    if let Some(thing_id) = thread.thing_id {
        let props = alloc::vec![
            (crate::symbols::intern("state"), PropValue::Str(String::from(ThreadState::Runnable.as_str()))),
            (crate::symbols::intern("sleep_until_ns"), PropValue::U64(0)),
        ];
        let _ = graph::update_thing(thing_id, props);
    }
}
