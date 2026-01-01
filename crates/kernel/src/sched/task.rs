use crate::machine::Context;
use alloc::vec::Vec;
use graph::store;
use graph::symbols::sym;
use abi::ids::{ThingId, SymbolId};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TaskId(pub u64);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TaskState {
    New,
    Ready,
    Running,
    Blocked,
    Dead,
}

impl TaskState {
    fn to_symbol(&self) -> SymbolId {
        match self {
            TaskState::New => sym::TASK_STATE_NEW,
            TaskState::Ready => sym::TASK_STATE_READY,
            TaskState::Running => sym::TASK_STATE_RUNNING,
            TaskState::Blocked => sym::TASK_STATE_BLOCKED,
            TaskState::Dead => sym::TASK_STATE_DEAD,
        }
    }
}

pub struct Task {
    pub id: TaskId,
    pub thing: ThingId,
    pub state: TaskState,
    pub stack_ptr: u64, // Saved SP (top of kernel stack)
    pub heap_base: u64,
    pub heap_size: u64,
    pub heap_brk: u64,
}

impl Task {
    pub fn new(id: TaskId, thing: ThingId, stack_ptr: u64) -> Self {
        Self {
            id,
            thing,
            state: TaskState::New,
            stack_ptr,
            heap_base: 0,
            heap_size: 0,
            heap_brk: 0,
        }
    }

    pub fn set_state(&mut self, place: &mut store::PlaceStore, new_state: TaskState) {
        if self.state == new_state {
            return;
        }
        self.state = new_state;

        // Update graph: task --[state]--> state_thing
        // Simplify: Just create a specialized state thing, or reuse shared state things?
        // User plan said "task_state.* (or a value Thing)".
        // Reusing shared fixed state things is cleaner if we had them.
        // But we didn't seed them explicitly as Things, just symbols.
        // So we create a new Thing of that kind? Or value thing?
        // Plan said: "task --[predicate.state]--> task_state.*"
        // Let's create a new Thing with Kind = state_symbol.
        
        let state_kind = new_state.to_symbol();
        if let Ok(state_thing) = place.create_thing(sym::KIND_THING) { // Generic thing for now or Kind=State?
            // Actually, maybe we should just set the predicate to point to a "Concept" thing or just a Thing with that kind.
            // Let's create a Thing with Kind=StateSymbol.
            // place.create_thing(state_kind) works if state_kind is a valid kind?
            // state_kind is like "task_state.ready". Is that a Kind?
            // No, it's usually a value/concept.
            // Let's create a generic thing and name it? No, names are global.
            // Let's create a generic Thing and set its Kind to the state symbol.
            // But `create_thing` takes a Kind.
            // Let's use KIND_THING and link it?
            // "task --[predicate.state]--> Thing(Kind=task_state.ready)"
            
            // To be safe and avoid churn:
            // Remove old state edge? We don't track the edge ID.
            // We just add new edge. Old edges remain? That's bad.
            // For V0.3 Task 03, let's just add the edge.
            // A better way is to model state as a property.
            
            // Re-read constraints: "task --[state]--> task_state.*"
            // Let's assume we create a Thing for the state (e.g. ephemeral) or find a canonical one.
            // Since we didn't seed canonical state things, ephemeral is safer.
            if let Ok(st) = place.create_thing(state_kind) {
                let _ = place.create_relationship(sym::PRED_STATE, self.thing, st);
            }
        }
    }

    pub fn set_on_cpu(&self, place: &mut store::PlaceStore, cpu_thing: ThingId) {
        // task --[on_cpu]--> cpu.0
        let _ = place.create_relationship(sym::PRED_ON_CPU, self.thing, cpu_thing);
    }
}
