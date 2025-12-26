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

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, Default)]
pub struct ThreadContext(pub [u64; 20]);

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
    pub kernel_stack: Vec<u8>,
    pub kernel_stack_top: u64,
    pub context: ThreadContext,
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

        let mut kernel_stack = alloc::vec![0u8; 16384];
        let kernel_stack_top = kernel_stack.as_ptr() as u64 + 16384;
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
            kernel_stack,
            kernel_stack_top,
            context: ThreadContext(context),
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
        let tid = self.run_queue.remove(0);
        Some(tid)
    }

    pub fn tick<B: HardwareBridge>(&mut self, _bridge: &B, current_context: &mut ThreadContext) {
        // 1. Save current context if we have a current thread
        if let Some(tid) = self.current {
            // Only save if it still exists (it might have exited/died, but we handle that elsewhere)
            // Ideally check state.
            if let Some(Some(thread)) = self.threads.get_mut(tid.0 as usize - 1) {
                thread.context = *current_context;
                // If Running, user is preempted. Move to Runnable.
                if thread.state == ThreadState::Running || thread.state == ThreadState::Runnable {
                    thread.state = ThreadState::Runnable;
                    self.run_queue.push(tid);
                }
            }
        }

        // 2. Pick next
        if let Some(next_tid) = self.pick_next() {
            self.current = Some(next_tid);
            if let Some(Some(thread)) = self.threads.get_mut(next_tid.0 as usize - 1) {
                 thread.state = ThreadState::Running;
                 _bridge.set_kernel_stack(thread.kernel_stack_top);
                 
                 *current_context = thread.context;
            }
        } else {
             // Continue running current? 
             // If pick_next returned None, it means run_queue is empty.
             // If we just pushed current back to run_queue, pick_next shouldn't be None!
             // Unless current blocked or died.
             // If current blocked, we have no threads.
             // We must have an idle thread or just return (resume current context which is... kernel loop context?)
             // If we were in App, and App blocks. 
             // We are in Trap Handler.
             // We return to App... and App just spins? No, if we return to App execution it continues.
             // But if we want to run Idle Loop?
             // We don't have explicit Idle context saved.
             // We can't switch to Idle Loop easily if we are in interrupt handler on top of App stack.
             
             // For now, if no threads, we define current = None.
             self.current = None;
             
             // If we were running an App, and we set current=None, we return to App execution?
             // That's dangerous if we considered it "blocked".
             // But here we only handle RR preemption.
             // If thread blocked, it removed itself from run_queue beforehand.
             
             // If we return, we resume execution of whatever context is in `current_context`.
             // If it was an App, it keeps running.
             // This is acceptable for "Idle" behavior (spinning App).
             // But strictly we should switch to kernel idle loop.
             // We don't have kernel idle context saved.
             
             // Let's assume we maintain invariant: run_queue always has something if we have apps.
             // Or if empty, we assume we return to kernel idle loop (if we came from it).
        }
    }
}
