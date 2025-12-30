use crate::bridge::HardwareBridge;
use crate::sched::fpu::FpuContext;
use crate::sched::types::{ThreadState, TimeNs};
use abi::ids::{ProcessId, ThingId, ThreadId};
use alloc::collections::VecDeque;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug)]
pub struct Process {
    pub id: ProcessId,
    pub name: String,
    pub thing_id: Option<ThingId>,
    pub heap_virt_start: u64,
    pub heap_virt_end: u64,
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, Default)]
pub struct ThreadContext<C>(pub C);

#[derive(Debug)]
pub struct Thread<C> {
    pub id: ThreadId,
    pub process_id: ProcessId,
    pub name: String,
    pub state: ThreadState,
    pub priority: u64,
    pub entry_point: u64,
    pub user_arg: u64,
    pub user_stack_top: u64,
    pub kernel_stack: Vec<u128>,
    pub kernel_stack_top: u64,
    pub context: ThreadContext<C>,
    pub fpu_context: FpuContext,
    pub started: bool,
    pub thing_id: Option<ThingId>,
    pub sleep_event_id: Option<ThingId>,
    pub sleep_until_ns: TimeNs,
    pub last_run_start_ns: TimeNs,
    pub total_run_ns: TimeNs,
    pub pending_wake: bool,
}

pub struct Scheduler<C> {
    pub threads: Vec<Option<Thread<C>>>,
    pub processes: Vec<Option<Process>>,
    pub current: Option<ThreadId>,
    pub run_queue: VecDeque<ThreadId>,
    pub sleep_queue: Vec<SleepEntry>,
    pub graph_enabled: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct SleepEntry {
    pub thread_id: ThreadId,
    pub wake_at_ns: TimeNs,
}

impl<C> Default for Scheduler<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C> Scheduler<C> {
    pub const fn new() -> Self {
        Self {
            threads: Vec::new(),
            processes: Vec::new(),
            current: None,
            run_queue: VecDeque::new(),
            sleep_queue: Vec::new(),
            graph_enabled: false,
        }
    }

    pub fn sleep_current_until(&mut self, wake_ns: TimeNs) {
        if let Some(tid) = self.current {
            if let Some(Some(thread)) = self.threads.get_mut(tid.0 as usize - 1) {
                thread.state = ThreadState::Sleeping;
                thread.sleep_until_ns = wake_ns;

                // Add to sleep queue
                let entry = SleepEntry {
                    thread_id: tid,
                    wake_at_ns: wake_ns,
                };

                // Optimized insertion: Maintain sorted order (descending by wake time).
                let idx = self.sleep_queue.partition_point(|x| x.wake_at_ns > wake_ns);
                self.sleep_queue.insert(idx, entry);
            }
        }
    }

    pub fn wake_sleepers(&mut self, now_ns: TimeNs) {
        // Queue is sorted descending by wake time (min at end).
        // Pop while last().wake_at_ns <= now_ns
        while let Some(last) = self.sleep_queue.last() {
            if last.wake_at_ns <= now_ns {
                let entry = self.sleep_queue.pop().unwrap();
                let tid = entry.thread_id;

                if let Some(Some(thread)) = self.threads.get_mut(tid.0 as usize - 1) {
                    if thread.state == ThreadState::Sleeping {
                        thread.state = ThreadState::Runnable;
                        thread.pending_wake = true; // flag if useful
                        self.run_queue.push_back(tid);
                    }
                }
            } else {
                break;
            }
        }
    }

    pub fn next_wakeup_deadline(&self) -> Option<TimeNs> {
        self.sleep_queue.last().map(|entry| entry.wake_at_ns)
    }

    pub fn spawn<B>(
        &mut self,
        bridge: &B,
        name: &str,
        entry: u64,
        stack_top: u64,
        arg: u64,
        heap_start: u64,
        heap_end: u64,
    ) where
        B: HardwareBridge<Context = C>,
    {
        let pid = ProcessId(self.processes.len() as u64 + 1);
        let tid = ThreadId(self.threads.len() as u64 + 1);

        let process = Process {
            id: pid,
            name: name.to_string(),
            thing_id: None,
            heap_virt_start: heap_start,
            heap_virt_end: heap_end,
        };
        self.processes.push(Some(process));

        // 16KB stack, 16-byte aligned. 16384 bytes / 16 bytes/u128 = 1024 u128s.
        let mut kernel_stack = alloc::vec![0u128; 1024];
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
        self.run_queue.push_back(tid);
    }

    pub fn pick_next(&mut self) -> Option<ThreadId> {
        self.run_queue.pop_front()
    }

    pub fn tick<B>(&mut self, _bridge: &B, current_context: &mut ThreadContext<C>)
    where
        B: HardwareBridge<Context = C>,
        C: Copy,
    {
        // 1. Save current context if we have a current thread
        if let Some(tid) = self.current {
            // Only save if it still exists (it might have exited/died, but we handle that elsewhere)
            // Ideally check state.
            if let Some(Some(thread)) = self.threads.get_mut(tid.0 as usize - 1) {
                thread.context = *current_context;
                _bridge.save_fpu(&mut thread.fpu_context.data);

                // If Running, user is preempted. Move to Runnable.
                // If Running, user is preempted. Move to Runnable.
                // If Sleeping, we LEAVE IT SLEEPING and do NOT push to run_queue.
                if thread.state == ThreadState::Running {
                    thread.state = ThreadState::Runnable;
                    self.run_queue.push_back(tid);
                    // _bridge.log("SCHED: Re-queued Running\n");
                } else if thread.state == ThreadState::Runnable {
                    self.run_queue.push_back(tid);
                    // _bridge.log("SCHED: Re-queued Runnable\n");
                } else {
                    // _bridge.log("SCHED: Thread not re-queued. dropped.\n");
                }
            }
        }

        // 2. Pick next
        if let Some(next_tid) = self.pick_next() {
            // _bridge.log("SCHED: Picked next\n");
            self.current = Some(next_tid);
            if let Some(Some(thread)) = self.threads.get_mut(next_tid.0 as usize - 1) {
                thread.state = ThreadState::Running;

                // _bridge.log("SCHED: Switch to ");
                // _bridge.log(&thread.name);
                // _bridge.log("\n");

                _bridge.set_kernel_stack(thread.kernel_stack_top);
                _bridge.restore_fpu(&thread.fpu_context.data);

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
            // We don't have explicit Idle context saved.
            // We can't switch to Idle Loop easily if we are in interrupt handler on top of App stack.

            // For now, if no threads, we define current = None.
            self.current = None;

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
