use abi::syscall_defs::SymbolId;
use abi::wire::graph::{WatchEvent, WatchEventKind, WatchFlags, WatchId, WatchSpec, WatchSpecTag};
use abi::{ProcessId, ThingId, ThreadId};
use alloc::collections::VecDeque;
use hashbrown::HashMap;
use spin::Mutex;

use crate::sched;

pub struct Watch {
    pub id: WatchId,
    pub owner_pid: ProcessId,
    pub spec: WatchSpec,
    pub queue: VecDeque<WatchEvent>,
    pub dropped_count: u64,
    pub waiting_thread: Option<ThreadId>,
}

pub struct WatchRegistry {
    pub watches: HashMap<WatchId, Watch>,
    pub next_id: u64,
}

impl WatchRegistry {
    pub fn new() -> Self {
        Self {
            watches: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn create_watch(&mut self, owner_pid: ProcessId, spec: WatchSpec) -> WatchId {
        let id = WatchId(self.next_id);
        self.next_id += 1;
        
        let watch = Watch {
            id,
            owner_pid,
            spec,
            queue: VecDeque::with_capacity(64), // Fixed cap for v1
            dropped_count: 0,
            waiting_thread: None,
        };
        
        self.watches.insert(id, watch);
        id
    }

    pub fn close_watch(&mut self, id: WatchId, caller_pid: ProcessId) -> bool {
        if let Some(watch) = self.watches.get(&id) {
            if watch.owner_pid != caller_pid {
                return false;
            }
        } else {
            return false;
        }

        if let Some(mut watch) = self.watches.remove(&id) {
            // Wake any waiter
            if let Some(tid) = watch.waiting_thread.take() {
                sched::with_scheduler(|s| s.wake_thread(tid));
            }
            true
        } else {
            false
        }
    }

    pub fn close_all_for_process(&mut self, pid: ProcessId) {
        let mut to_remove = alloc::vec::Vec::new();
        for (id, watch) in &self.watches {
            if watch.owner_pid == pid {
                to_remove.push(*id);
            }
        }
        for id in to_remove {
             if let Some(mut watch) = self.watches.remove(&id) {
                if let Some(tid) = watch.waiting_thread.take() {
                    sched::with_scheduler(|s| s.wake_thread(tid));
                }
             }
        }
    }
}

static REGISTRY_VAL: Mutex<Option<WatchRegistry>> = Mutex::new(None);

pub fn lock_registry() -> spin::MutexGuard<'static, Option<WatchRegistry>> {
    let mut guard = REGISTRY_VAL.lock();
    if guard.is_none() {
        *guard = Some(WatchRegistry::new());
    }
    guard
}

// --- Enqueue Logic ---

fn enqueue_event(watch: &mut Watch, event: WatchEvent) {
    if watch.queue.len() >= 64 {
        watch.queue.pop_front(); // Drop oldest
        watch.dropped_count += 1;
    }
    watch.queue.push_back(event);

    if let Some(tid) = watch.waiting_thread.take() {
        sched::with_scheduler(|s| s.wake_thread(tid));
    }
}

// --- Matching Logic ---

pub fn emit_link_added(src: ThingId, pred: SymbolId, dst: ThingId) {
    let mut guard = lock_registry();
    let reg = guard.as_mut().unwrap();
    for watch in reg.watches.values_mut() {
        if watch.spec.tag == WatchSpecTag::Link 
           && watch.spec.flags.contains(WatchFlags::LINK_ADDED) 
           && watch.spec.thing == src 
           && watch.spec.key == pred 
        {
            enqueue_event(watch, WatchEvent {
                kind: WatchEventKind::LinkAdded,
                src_or_thing: src,
                pred_or_key: pred,
                dst_or_aux: dst.0,
            });
        }
    }
}

pub fn emit_link_removed(src: ThingId, pred: SymbolId, dst: ThingId) {
    let mut guard = lock_registry();
    let reg = guard.as_mut().unwrap();
    for watch in reg.watches.values_mut() {
        if watch.spec.tag == WatchSpecTag::Link 
           && watch.spec.flags.contains(WatchFlags::LINK_REMOVED) 
           && watch.spec.thing == src 
           && watch.spec.key == pred 
        {
             enqueue_event(watch, WatchEvent {
                kind: WatchEventKind::LinkRemoved,
                src_or_thing: src,
                pred_or_key: pred,
                dst_or_aux: dst.0, // We include dst for removed too in v1
            });
        }
    }
}

pub fn emit_prop_set(thing: ThingId, key: SymbolId) {
    let mut guard = lock_registry();
    let reg = guard.as_mut().unwrap();
    for watch in reg.watches.values_mut() {
        if watch.spec.tag == WatchSpecTag::Prop 
           && watch.spec.flags.contains(WatchFlags::PROP_SET) 
           && watch.spec.thing == thing 
           && watch.spec.key == key 
        {
             enqueue_event(watch, WatchEvent {
                kind: WatchEventKind::PropSet,
                src_or_thing: thing,
                pred_or_key: key,
                dst_or_aux: 0, 
            });
        }
    }
}

pub fn emit_prop_del(thing: ThingId, key: SymbolId) {
     let mut guard = lock_registry();
     let reg = guard.as_mut().unwrap();
    for watch in reg.watches.values_mut() {
         if watch.spec.tag == WatchSpecTag::Prop 
           && watch.spec.flags.contains(WatchFlags::PROP_DEL) 
           && watch.spec.thing == thing 
           && watch.spec.key == key 
        {
             enqueue_event(watch, WatchEvent {
                kind: WatchEventKind::PropDel,
                src_or_thing: thing,
                pred_or_key: key,
                dst_or_aux: 0, 
            });
        }
    }
}
