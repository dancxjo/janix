use abi::GraphEvent;
use alloc::vec::Vec;
use spin::Mutex;
use crate::sched::types::Event;
use abi::wire::events::WireWatchSpec;
use abi::{ThingId, Predicate, syscall_defs::SymbolId};

type EventHandler = fn(&GraphEvent);

static LISTENERS: Mutex<Option<Vec<EventHandler>>> = Mutex::new(None);

pub fn init() {
    *LISTENERS.lock() = Some(Vec::new());
}

pub fn subscribe(handler: EventHandler) {
    LISTENERS
        .lock()
        .as_mut()
        .expect("Events not initialized")
        .push(handler);
}

pub fn dispatch_event(event: &GraphEvent) {
    let guard = LISTENERS.lock();
    if let Some(listeners) = guard.as_ref() {
        for handler in listeners.iter() {
            handler(event);
        }
    }
}

pub fn register_watch(pid: abi::ProcessId, spec: WireWatchSpec) -> Result<(), &'static str> {
    crate::sched::with_scheduler(|sched| {
        for slot in sched.processes.iter_mut() {
            if let Some(proc) = slot {
                if proc.id == pid {
                    proc.watches.push(spec);
                    return Ok(());
                }
            }
        }
        Err("Process not found")
    })
}

pub fn dispatch_custom_event(src: ThingId, src_kind: SymbolId, pred: Predicate, on: SymbolId, payload: &[u8]) {
    crate::sched::with_scheduler(|sched| {
        let mut pids_to_wake = Vec::new();

        for slot in sched.processes.iter_mut() {
             if let Some(proc) = slot {
                let mut matched = false;
                for watch in &proc.watches {
                    if watch.pred == pred && watch.on == on && watch.src_kind == src_kind {
                        if watch.src_thing.0 == 0 || watch.src_thing == src {
                            matched = true;
                            break;
                        }
                    }
                }

                if matched {
                    if proc.events.len() >= 64 {
                        proc.events.pop_front();
                    }
                    proc.events.push_back(Event {
                        src,
                        pred,
                        on,
                        payload: payload.to_vec(),
                    });
                    pids_to_wake.push(proc.id);
                }
             }
        }

        for pid in pids_to_wake {
            // Wake all threads of this process that are blocked
            // Ideally we only wake threads waiting for events, but waking all blocked is safe (they will check condition and sleep again)
            for i in 0..sched.threads.len() {
                if let Some(thread) = sched.threads[i].as_mut() {
                    if thread.process_id == pid && thread.state == crate::sched::ThreadState::Blocked {
                         thread.state = crate::sched::ThreadState::Runnable;
                         let tid = thread.id;
                         // Ignore error if run queue full
                         let _ = sched.cache.run_queue.push(tid);
                    }
                }
            }
        }
    })
}
