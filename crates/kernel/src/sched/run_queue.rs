use alloc::collections::VecDeque;
use crate::sched::task::TaskId;
use graph::store::PlaceStore;

use abi::ids::ThingId;

pub struct RunQueue {
    queue: VecDeque<TaskId>,
    pub _thing: ThingId, // run_queue.0
}

impl RunQueue {
    pub fn new(thing: ThingId) -> Self {
        Self {
            queue: VecDeque::new(),
            _thing: thing,
        }
    }

    pub fn push_back(&mut self, task_id: TaskId, _task_thing: ThingId, _place: &mut PlaceStore) {
        self.queue.push_back(task_id);
        // task --[in_run_queue]--> run_queue.0
        // FIXME: Allocating in IRQ (tick) causes panic/deadlock.
        // let _ = place.create_relationship(sym::PRED_IN_RUN_QUEUE, task_thing, self.thing);
    }

    pub fn pop_front(&mut self) -> Option<TaskId> {
        self.queue.pop_front()
    }
    
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
