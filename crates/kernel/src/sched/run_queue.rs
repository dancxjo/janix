use alloc::collections::VecDeque;
use crate::sched::task::TaskId;
use graph::store::PlaceStore;
use graph::symbols::sym;
use abi::ids::ThingId;

pub struct RunQueue {
    queue: VecDeque<TaskId>,
    thing: ThingId, // run_queue.0
}

impl RunQueue {
    pub fn new(thing: ThingId) -> Self {
        Self {
            queue: VecDeque::new(),
            thing,
        }
    }

    pub fn push_back(&mut self, task_id: TaskId, task_thing: ThingId, place: &mut PlaceStore) {
        self.queue.push_back(task_id);
        // task --[in_run_queue]--> run_queue.0
        let _ = place.create_relationship(sym::PRED_IN_RUN_QUEUE, task_thing, self.thing);
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
