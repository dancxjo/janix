use crate::sched::fpu::FpuContext;
use crate::sched::types::{ThreadState, TimeNs};
use abi::ids::{ProcessId, ThingId, ThreadId};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use hw::HardwareBridge;

#[derive(Debug)]
pub struct Process {
    pub id: ProcessId,
    pub name: String,
    pub thing_id: Option<ThingId>,
}

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

    pub fn spawn<B: HardwareBridge>(
        &mut self,
        bridge: &B,
        name: &str,
        entry: u64,
        stack_top: u64,
        arg: u64,
    ) {
        let pid = ProcessId(self.processes.len() as u64 + 1);
        let tid = ThreadId(self.threads.len() as u64 + 1);

        let process = Process {
            id: pid,
            name: name.to_string(),
            thing_id: None,
        };
        self.processes.push(Some(process));

        let context = bridge.init_thread_context(entry, stack_top, arg);

        let thread = Thread {
            id: tid,
            process_id: pid,
            name: name.to_string(),
            state: ThreadState::Runnable,
            priority: 1,
            entry_point: entry,
            user_arg: arg,
            user_stack_top: stack_top,
            context,
            fpu_context: FpuContext::default(),
            started: false,
            thing_id: None,
            sleep_event_id: None,
            sleep_until_ns: 0,
            last_run_start_ns: 0,
            total_run_ns: 0,
            pending_wake: false,
        };

        self.threads.push(Some(thread));
        self.run_queue.push(tid);
    }

    pub fn pick_next(&mut self) -> Option<ThreadId> {
        if self.run_queue.is_empty() {
            return None;
        }
        // Round robin: pop front, push back if still runnable
        // But for now simple: pop front
        let tid = self.run_queue.remove(0);
        // re-push to end if it's meant to run again? 
        // For cooperative or simple slice, we re-push when it yields.
        // But if pick_next implies we are switching to it, we don't push it back YET.
        // We push it back when we switch OUT of it.
        // But here we are just picking. 
        // Actually typical simple RR:
        // current = tid.
        
        Some(tid)
    }
}
