use abi::ids::{SymbolId, ThingId};
use graph::store;
use graph::symbols::sym;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TaskId(pub u64);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TaskState {
    New,
    Ready,
    Running,
    Yielded,
    Blocked(BlockReason),
    Dead,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlockReason {
    WatchWait,
    Timeout,
    Other,
}

impl TaskState {
    fn to_symbol(&self) -> SymbolId {
        match self {
            TaskState::New => sym::TASK_STATE_NEW,
            TaskState::Ready => sym::TASK_STATE_READY,
            TaskState::Running => sym::TASK_STATE_RUNNING,
            TaskState::Yielded => sym::TASK_STATE_YIELDED,
            TaskState::Blocked(BlockReason::WatchWait) => sym::TASK_STATE_BLOCKED_WATCH,
            TaskState::Blocked(BlockReason::Timeout) => sym::TASK_STATE_BLOCKED_TIMEOUT,
            TaskState::Blocked(BlockReason::Other) => sym::TASK_STATE_BLOCKED,
            TaskState::Dead => sym::TASK_STATE_DEAD,
        }
    }
}

use crate::memory::space::AddressSpace;
use alloc::sync::Arc;

pub struct Task {
    pub id: TaskId,
    pub thing: ThingId,
    pub state: TaskState,
    pub stack_ptr: u64,
    pub stack_top: u64,
    pub address_space: Arc<AddressSpace>,
    pub heap_base: u64,
    pub heap_size: u64,
    pub heap_brk: u64,
    pub wake_reason: Option<abi::types::WakeReason>,
    pub first_run: bool, // true if this task has prepared context, false if it has saved context
}

impl Task {
    pub fn new(
        id: TaskId,
        thing: ThingId,
        stack_ptr: u64,
        address_space: Arc<AddressSpace>,
    ) -> Self {
        Self {
            id,
            thing,
            state: TaskState::New,
            stack_ptr,
            stack_top: stack_ptr, // Initially same as ptr (empty stack)
            address_space,
            heap_base: 0x9000_0000,
            heap_size: 0,
            heap_brk: 0x9000_0000,
            wake_reason: None,
            first_run: true, // Starts with prepared context
        }
    }

    pub fn set_state(&mut self, place: &mut store::PlaceStore, new_state: TaskState) {
        if self.state == new_state {
            return;
        }
        self.state = new_state;

        let state_kind = new_state.to_symbol();
        if let Ok(_state_thing) = place.create_thing(sym::KIND_THING) {
            if let Ok(st) = place.create_thing(state_kind) {
                let _ = place.create_relationship(sym::PRED_STATE, self.thing, st);
            }
        }
    }

    pub fn set_on_cpu(&self, place: &mut store::PlaceStore, cpu_thing: ThingId) {
        let _ = place.create_relationship(sym::PRED_ON_CPU, self.thing, cpu_thing);
    }
}
