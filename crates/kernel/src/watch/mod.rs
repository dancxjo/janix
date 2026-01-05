//! Watch wait registry for kernel-visible waiters.
//!
//! Tracks which tasks are blocked on which WatchIds and produces wake lists
//! for the scheduler without allocating in interrupt context.

use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec::Vec;
use spin::Mutex;

use crate::sched::task::TaskId;
use abi::ids::{ThingId, WatchId};
use abi::syscall::err;
use abi::types::{
    WaitFlags, WakeReason, WatchEvent, WatchEventKind, WatchKind, MAX_WATCH_EVENTS,
};

struct WaitEntry {
    watches: Vec<(WatchId, bool)>, // bool = fired
    flags: WaitFlags,
    timeout_at: Option<u64>,
}

struct Registry {
    by_task: BTreeMap<TaskId, WaitEntry>,
    by_watch: BTreeMap<WatchId, Vec<TaskId>>,
}

impl Registry {
    const fn new() -> Self {
        Self {
            by_task: BTreeMap::new(),
            by_watch: BTreeMap::new(),
        }
    }

    fn remove_task_locked(&mut self, task: TaskId) {
        if let Some(entry) = self.by_task.remove(&task) {
            for (watch_id, _) in entry.watches.iter() {
                if let Some(waiters) = self.by_watch.get_mut(watch_id) {
                    waiters.retain(|t| *t != task);
                    if waiters.is_empty() {
                        self.by_watch.remove(watch_id);
                    }
                }
            }
        }
    }
}

static REGISTRY: Mutex<Registry> = Mutex::new(Registry::new());

/// Bounded list of tasks to wake with their reason.
pub struct WakeList {
    entries: [(TaskId, WakeReason); MAX_WATCH_EVENTS],
    len: usize,
}

impl WakeList {
    pub const fn new() -> Self {
        Self {
            entries: [(TaskId(0), WakeReason::timeout()); MAX_WATCH_EVENTS],
            len: 0,
        }
    }

    pub fn push(&mut self, task: TaskId, reason: WakeReason) {
        if self.len >= self.entries.len() {
            return;
        }
        self.entries[self.len] = (task, reason);
        self.len += 1;
    }

    pub fn iter(&self) -> impl Iterator<Item = (TaskId, WakeReason)> + '_ {
        self.entries[..self.len].iter().copied()
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// Register a task wait on a set of watches with optional timeout in ticks.
pub fn register_wait(task: TaskId, watches: &[WatchId], flags: WaitFlags, timeout_at: Option<u64>) {
    let mut guard = REGISTRY.lock();
    let registry = &mut *guard;

    registry.remove_task_locked(task);

    let mut entry = WaitEntry {
        watches: Vec::new(),
        flags,
        timeout_at,
    };

    entry.watches.reserve(watches.len());
    for &w in watches {
        entry.watches.push((w, false));
        registry.by_watch.entry(w).or_default().push(task);
    }

    registry.by_task.insert(task, entry);
}

/// Remove any outstanding wait registration for a task.
pub fn unregister_wait(task: TaskId) {
    let mut guard = REGISTRY.lock();
    guard.remove_task_locked(task);
}

/// Mark a watch as fired, filling the wake list for tasks that should resume.
pub fn fire_watch(id: WatchId, out: &mut WakeList) {
    let mut guard = REGISTRY.lock();
    let registry = &mut *guard;

    // Copy waiters into a bounded stack array to avoid heap work here.
    let mut waiters: [Option<TaskId>; MAX_WATCH_EVENTS] = [None; MAX_WATCH_EVENTS];
    let mut count = 0usize;
    {
        if let Some(list) = registry.by_watch.get(&id) {
            for &tid in list.iter() {
                if count < waiters.len() {
                    waiters[count] = Some(tid);
                    count += 1;
                }
            }
        }
    }

    for tid_opt in waiters.into_iter().flatten().take(count) {
        if let Some(entry) = registry.by_task.get_mut(&tid_opt) {
            for (watch_id, fired) in entry.watches.iter_mut() {
                if *watch_id == id {
                    *fired = true;
                }
            }
            let ready = if entry.flags.contains(WaitFlags::WAIT_ALL) {
                entry.watches.iter().all(|(_, fired)| *fired)
            } else {
                true
            };
            if ready {
                out.push(tid_opt, WakeReason::watch(id));
                registry.remove_task_locked(tid_opt);
            }
        }
    }
}

/// Scan for timed-out waits.
pub fn check_timeouts(now_ticks: u64, out: &mut WakeList) {
    let mut guard = REGISTRY.lock();
    let registry = &mut *guard;

    // Collect tasks that have expired without reallocating during the scan.
    let mut expired: [Option<TaskId>; MAX_WATCH_EVENTS] = [None; MAX_WATCH_EVENTS];
    let mut count = 0usize;

    for (task_id, entry) in registry.by_task.iter() {
        if let Some(deadline) = entry.timeout_at {
            if now_ticks >= deadline && count < expired.len() {
                expired[count] = Some(*task_id);
                count += 1;
            }
        }
    }

    for tid_opt in expired.into_iter().flatten().take(count) {
        out.push(tid_opt, WakeReason::timeout());
        registry.remove_task_locked(tid_opt);
    }
}

/// Convenience helper: fire a watch and wake any waiting tasks immediately.
pub fn fire_and_wake(id: WatchId) {
    let mut list = WakeList::new();
    fire_watch(id, &mut list);
    crate::sched::wake_from_watch_list(&list);
}

// === Watch event registry ===

struct WatchQueue {
    kind: WatchKind,
    subject: ThingId,
    queue: VecDeque<WatchEvent>,
}

struct WatchState {
    next_id: u64,
    watches: BTreeMap<WatchId, WatchQueue>,
    graph_membership: BTreeMap<ThingId, Vec<WatchId>>,
    thing_watchers: BTreeMap<ThingId, Vec<WatchId>>,
}

impl WatchState {
    const fn new() -> Self {
        Self {
            next_id: 1,
            watches: BTreeMap::new(),
            graph_membership: BTreeMap::new(),
            thing_watchers: BTreeMap::new(),
        }
    }

    fn deliver(&mut self, watch_id: WatchId, event: WatchEvent) {
        if let Some(entry) = self.watches.get_mut(&watch_id) {
            if entry.queue.len() >= MAX_WATCH_EVENTS {
                entry.queue.pop_front();
            }
            entry.queue.push_back(event);
            drop(entry);
            fire_and_wake(watch_id);
        }
    }
}

static WATCH_STATE: Mutex<WatchState> = Mutex::new(WatchState::new());

pub fn create_watch(kind: WatchKind, target: ThingId) -> WatchId {
    let mut guard = WATCH_STATE.lock();
    let id = WatchId(guard.next_id);
    guard.next_id += 1;

    guard.watches.insert(
        id,
        WatchQueue {
            kind,
            subject: target,
            queue: VecDeque::new(),
        },
    );

    match kind {
        WatchKind::GraphMembership => guard
            .graph_membership
            .entry(target)
            .or_default()
            .push(id),
        WatchKind::Thing => guard.thing_watchers.entry(target).or_default().push(id),
    }

    id
}

pub fn poll_watch(id: WatchId, out: &mut [WatchEvent]) -> Result<usize, i32> {
    let mut guard = WATCH_STATE.lock();
    let Some(queue) = guard.watches.get_mut(&id) else {
        return Err(err::ENOENT);
    };

    let mut count = 0usize;
    while count < out.len() {
        if let Some(ev) = queue.queue.pop_front() {
            out[count] = ev;
            count += 1;
        } else {
            break;
        }
    }
    Ok(count)
}

fn broadcast_graph_event(graph: ThingId, event: WatchEvent) {
    let mut guard = WATCH_STATE.lock();
    if let Some(watchers) = guard.graph_membership.get(&graph) {
        let ids = watchers.clone();
        for wid in ids {
            guard.deliver(wid, event);
        }
    }
}

fn broadcast_thing_event(thing: ThingId, event: WatchEvent) {
    let mut guard = WATCH_STATE.lock();
    if let Some(watchers) = guard.thing_watchers.get(&thing) {
        let ids = watchers.clone();
        for wid in ids {
            guard.deliver(wid, event);
        }
    }
}

pub fn graph_member_added(graph: ThingId, member: ThingId) {
    let ev = WatchEvent {
        kind: WatchEventKind::GraphMemberAdded,
        flags: 0,
        subject: graph,
        arg0: member,
    };
    broadcast_graph_event(graph, ev);
}

pub fn graph_member_removed(graph: ThingId, member: ThingId) {
    let ev = WatchEvent {
        kind: WatchEventKind::GraphMemberRemoved,
        flags: 0,
        subject: graph,
        arg0: member,
    };
    broadcast_graph_event(graph, ev);
}

pub fn thing_updated(id: ThingId) {
    let ev = WatchEvent {
        kind: WatchEventKind::ThingUpdated,
        flags: 0,
        subject: id,
        arg0: ThingId(0),
    };
    broadcast_thing_event(id, ev);
}

pub fn thing_deleted(id: ThingId) {
    let ev = WatchEvent {
        kind: WatchEventKind::ThingDeleted,
        flags: 0,
        subject: id,
        arg0: ThingId(0),
    };
    broadcast_thing_event(id, ev);
}
