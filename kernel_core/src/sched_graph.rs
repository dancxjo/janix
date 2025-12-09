use abi::{Thing, ThingId, PropValue};
use thing_macros::Thing;
use alloc::string::String;
use alloc::vec::Vec;
use crate::graph;
use crate::sched::ThreadState;

#[derive(Thing, Clone, Debug)]
pub struct Thread {
    pub name: String,
    pub state: String,
    pub last_run_ns: i64,
    pub total_run_ns: i64,
    pub process_thing_id: u64,
    pub scheduler_thing_id: u64,
}

#[derive(Thing, Clone, Debug)]
pub struct SleepEvent {
    pub wake_at_ns: i64,
    pub created_at_ns: i64,
    pub label: String,
    pub thread_thing_id: u64,
    pub scheduler_thing_id: u64,
}

pub struct SchedulerGraphMirror {
    scheduler_thing_id: ThingId,
}

impl SchedulerGraphMirror {
    pub fn new() -> Self {
        let mut scheduler_id = None;
        graph::iter_things(|thing| {
            if thing.kind == "Scheduler" {
                scheduler_id = Some(thing.id);
            }
        });

        let id = if let Some(id) = scheduler_id {
            id
        } else {
            graph::create_thing("Scheduler", &[]).expect("Failed to create Scheduler thing")
        };

        Self {
            scheduler_thing_id: id,
        }
    }

    pub fn register_thread(
        &mut self,
        process_thing_id: ThingId,
        name: &'static str,
    ) -> ThingId {
        let info = Thread {
            name: String::from(name),
            state: String::from("NEW"),
            last_run_ns: 0,
            total_run_ns: 0,
            process_thing_id: process_thing_id.0,
            scheduler_thing_id: self.scheduler_thing_id.0,
        };
        
        let _ = graph::register_schema(Thread::KIND, Thread::schema());

        let mut props = Vec::new();
        info.to_props(&mut props);
        
        graph::create_thing(Thread::KIND, &props).expect("Failed to create ThreadInfo")
    }

    pub fn update_thread_state(
        &mut self,
        thread_thing_id: ThingId,
        state: ThreadState,
        now_ns: u64,
    ) {
        let state_str = match state {
            ThreadState::New => "NEW",
            ThreadState::Runnable => "RUNNABLE",
            ThreadState::Running => "RUNNING",
            ThreadState::Waiting => "WAITING",
            ThreadState::Sleeping => "SLEEPING",
            ThreadState::Terminated => "TERMINATED",
        };

        let props = [
            ("state", PropValue::Str(String::from(state_str))),
            ("last_run_ns", PropValue::I64(now_ns as i64)),
        ];
        
        graph::update_thing(thread_thing_id, &props);
    }

    pub fn record_run_slice(
        &mut self,
        thread_thing_id: ThingId,
        run_start_ns: u64,
        run_end_ns: u64,
    ) {
        if let Some((_, props)) = graph::get_thing(thread_thing_id) {
            let mut info = Thread::from_props(thread_thing_id, props);
            let delta = run_end_ns.saturating_sub(run_start_ns);
            info.total_run_ns += delta as i64;
            
            let props = [
                ("total_run_ns", PropValue::I64(info.total_run_ns)),
            ];
            graph::update_thing(thread_thing_id, &props);
        }
    }

    pub fn create_sleep_event(
        &mut self,
        thread_thing_id: ThingId,
        wake_at_ns: u64,
        now_ns: u64,
    ) -> ThingId {
        let event = SleepEvent {
            wake_at_ns: wake_at_ns as i64,
            created_at_ns: now_ns as i64,
            label: String::from("sleep_for"),
            thread_thing_id: thread_thing_id.0,
            scheduler_thing_id: self.scheduler_thing_id.0,
        };

        let _ = graph::register_schema(SleepEvent::KIND, SleepEvent::schema());

        let mut props = Vec::new();
        event.to_props(&mut props);
        
        graph::create_thing(SleepEvent::KIND, &props).expect("Failed to create SleepEvent")
    }

    pub fn clear_sleep_event(&mut self, sleep_event_id: ThingId) {
        graph::delete_thing(sleep_event_id);
    }
}
