use crate::handles::ProcessRef;
use abi::ThingId;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub enum WorkItem {
    UpdateLayout(ThingId),
    RebuildInputTree,
    SyncProcessState(ProcessRef),
    GraphCleanup,
    SpawnProgram(ThingId, Option<ThingId>),
}

struct WorkQueue {
    high_priority: Vec<WorkItem>,
    normal_priority: Vec<WorkItem>,
}

impl WorkQueue {
    const fn new() -> Self {
        Self {
            high_priority: Vec::new(),
            normal_priority: Vec::new(),
        }
    }
}

static mut GLOBAL_QUEUE: Option<WorkQueue> = None;

pub fn init() {
    unsafe {
        GLOBAL_QUEUE = Some(WorkQueue::new());
    }
}

pub fn push_high(item: WorkItem) {
    unsafe {
        let ptr = &raw mut GLOBAL_QUEUE;
        if let Some(queue) = (*ptr).as_mut() {
            queue.high_priority.push(item);
        }
    }
}

pub fn push_normal(item: WorkItem) {
    unsafe {
        let ptr = &raw mut GLOBAL_QUEUE;
        if let Some(queue) = (*ptr).as_mut() {
            queue.normal_priority.push(item);
        }
    }
}

pub fn pop() -> Option<WorkItem> {
    unsafe {
        let ptr = &raw mut GLOBAL_QUEUE;
        if let Some(queue) = (*ptr).as_mut() {
            if let Some(item) = queue.high_priority.pop() {
                return Some(item);
            }
            if let Some(item) = queue.normal_priority.pop() {
                return Some(item);
            }
        }
        None
    }
}
