use crate::sched::task::TaskId;
use alloc::collections::VecDeque;

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

    pub fn push_back(&mut self, task_id: TaskId, _task_thing: ThingId) {
        self.queue.push_back(task_id);
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
