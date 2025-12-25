use crate::sched::fpu::FpuContext;
use crate::sched::types::{ThreadState, TimeNs};
use abi::ids::{ProcessId, ThingId, ThreadId};
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct Process {
    pub id: ProcessId,
    pub name: String,
    pub thing_id: Option<ThingId>,
    // Add other fields as needed, e.g. memory map
}

// Update struct fields to use TimeNs aliases if preferred, or just remove import.
// For now, removing import is simpler if I used u64. But using types is better.

#[derive(Debug)]
pub struct Thread {
    pub id: ThreadId,
    pub process_id: ProcessId,
    pub name: String,
    pub state: ThreadState,
    pub priority: u64,
    pub entry_point: u64,
    pub user_arg: u64,
    pub user_stack_top: u64,
    pub context: [u64; 20],
    pub fpu_context: FpuContext,
    pub started: bool,
    pub thing_id: Option<ThingId>,
    pub sleep_event_id: Option<ThingId>,
    pub sleep_until_ns: TimeNs,
    pub last_run_start_ns: TimeNs,
    pub total_run_ns: TimeNs,
    pub pending_wake: bool,
}

pub struct Scheduler {
    pub threads: Vec<Option<Thread>>,
    pub processes: Vec<Option<Process>>,
    pub current: Option<ThreadId>,
    pub run_queue: Vec<ThreadId>,
    pub sleep_queue: Vec<SleepEntry>,
    pub graph_enabled: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct SleepEntry {
    pub thread_id: ThreadId,
    pub wake_at_ns: TimeNs,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            threads: Vec::new(),
            processes: Vec::new(),
            current: None,
            run_queue: Vec::new(),
            sleep_queue: Vec::new(),
            graph_enabled: false,
        }
    }

    // Adapt other methods as needed, stubbing graph logic
}
