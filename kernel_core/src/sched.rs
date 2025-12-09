#![no_std]

use abi::{ProcessId, ThreadId};
use heapless::Vec;
use spin::Mutex;

pub const MAX_THREADS: usize = 32;
pub const MAX_PROCESSES: usize = 16;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ThreadState {
    New,
    Runnable,
    Running,
    Waiting,
    Terminated,
}

#[derive(Copy, Clone, Debug)]
pub struct Thread {
    pub id: ThreadId,
    pub process_id: ProcessId,
    pub state: ThreadState,
    pub name: &'static str,
    pub user_entry: Option<extern "C" fn(u64) -> !>,
    pub user_arg: u64,
    pub user_stack_top: u64,
    pub context: [u64; 20],
    pub started: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct Process {
    pub id: ProcessId,
    pub name: &'static str,
}

pub struct Scheduler {
    // Simple round-robin run queue
    run_queue: Vec<ThreadId, MAX_THREADS>,
    threads: [Option<Thread>; MAX_THREADS],
    processes: [Option<Process>; MAX_PROCESSES],
    current: Option<ThreadId>,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            run_queue: Vec::new(),
            threads: [const { None }; MAX_THREADS],
            processes: [const { None }; MAX_PROCESSES],
            current: None,
        }
    }

    pub fn add_process(&mut self, name: &'static str) -> ProcessId {
        for (i, slot) in self.processes.iter_mut().enumerate() {
            if slot.is_none() {
                let pid = ProcessId(i as u64);
                *slot = Some(Process { id: pid, name });
                return pid;
            }
        }
        panic!("Max processes reached");
    }

    pub fn add_thread(
        &mut self,
        process_id: ProcessId,
        name: &'static str,
        entry: extern "C" fn(u64) -> !,
        arg: u64,
        stack_top: u64,
    ) -> ThreadId {
        for (i, slot) in self.threads.iter_mut().enumerate() {
            if slot.is_none() {
                let tid = ThreadId(i as u64);
                *slot = Some(Thread {
                    id: tid,
                    process_id,
                    state: ThreadState::New,
                    name,
                    user_entry: Some(entry),
                    user_arg: arg,
                    user_stack_top: stack_top,
                    context: [0; 20],
                    started: false,
                });
                // Add to run queue
                if self.run_queue.push(tid).is_err() {
                    panic!("Run queue full");
                }
                return tid;
            }
        }
        panic!("Max threads reached");
    }

    pub fn mark_yield(&mut self, tid: ThreadId) {
        if let Some(thread) = &mut self.threads[tid.0 as usize] {
            if thread.state == ThreadState::Running {
                thread.state = ThreadState::Runnable;
                if self.run_queue.push(tid).is_err() {
                    // Should not happen if we manage queue correctly
                    panic!("Run queue full on yield");
                }
            }
        }
    }

    pub fn mark_terminated(&mut self, tid: ThreadId) {
        if let Some(thread) = &mut self.threads[tid.0 as usize] {
            thread.state = ThreadState::Terminated;
        }
    }

    pub fn next_runnable(&mut self) -> Option<ThreadId> {
        // round-robin: pop front
        if !self.run_queue.is_empty() {
            return Some(self.run_queue.remove(0));
        }
        None
    }

    pub fn set_current(&mut self, tid: ThreadId) {
        self.current = Some(tid);
        if let Some(thread) = &mut self.threads[tid.0 as usize] {
            thread.state = ThreadState::Running;
        }
    }

    pub fn current_id(&self) -> Option<ThreadId> {
        self.current
    }

    pub fn thread_mut(&mut self, tid: ThreadId) -> Option<&mut Thread> {
        self.threads[tid.0 as usize].as_mut()
    }

    pub fn all_done(&self) -> bool {
        for thread in self.threads.iter().flatten() {
            if thread.state != ThreadState::Terminated {
                return false;
            }
        }
        true
    }
}

pub static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());

pub fn yield_current_thread() {
    let mut sched = SCHEDULER.lock();
    if let Some(tid) = sched.current {
        sched.mark_yield(tid);
        sched.current = None;
    }
}

pub fn exit_current_thread() {
    let mut sched = SCHEDULER.lock();
    if let Some(tid) = sched.current {
        sched.mark_terminated(tid);
        sched.current = None;
    }
}
