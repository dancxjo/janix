//! Scheduler
//!
//! Provides thread management and scheduling for the kernel.
//! This is a minimal stub with the correct "future shape".

use alloc::collections::VecDeque;
use alloc::vec::Vec;
use spin::Mutex;

use crate::log::{self, Level};
use crate::machine::Machine;
use crate::symbols;

/// Thread identifier
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ThreadId(pub u64);

/// Thread state
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ThreadState {
    /// Ready to run
    Ready,
    /// Currently running
    Running,
    /// Blocked on something
    Blocked,
    /// Terminated
    Dead,
}

/// Thread control block
pub struct Thread {
    /// Thread identifier
    pub id: ThreadId,
    /// Current state
    pub state: ThreadState,
    /// Thread name (for debugging)
    pub name: &'static str,
    // Future: saved context, stack pointer, etc.
}

/// Global scheduler state
static SCHEDULER: Mutex<Option<Scheduler>> = Mutex::new(None);

struct Scheduler {
    /// All threads
    threads: Vec<Thread>,
    /// Run queue (ready threads)
    run_queue: VecDeque<ThreadId>,
    /// Currently running thread
    current: Option<ThreadId>,
    /// Next thread ID
    next_id: u64,
    /// Initialized flag
    initialized: bool,
}

impl Scheduler {
    fn new() -> Self {
        Self {
            threads: Vec::new(),
            run_queue: VecDeque::new(),
            current: None,
            next_id: 1,
            initialized: false,
        }
    }
    
    fn spawn(&mut self, name: &'static str) -> ThreadId {
        let id = ThreadId(self.next_id);
        self.next_id += 1;
        
        let thread = Thread {
            id,
            state: ThreadState::Ready,
            name,
        };
        
        self.threads.push(thread);
        self.run_queue.push_back(id);
        
        id
    }
    
    fn thread_count(&self) -> usize {
        self.threads.len()
    }
}

/// Initialize the scheduler
pub fn init(machine: &'static dyn Machine) {
    let mut sched = Scheduler::new();
    sched.initialized = true;
    *SCHEDULER.lock() = Some(sched);
    
    log::klog(Level::Info, "KERNEL", "scheduler init");
}

/// Spawn a new kernel thread
pub fn spawn_kernel_thread(name: &'static str) -> ThreadId {
    let mut guard = SCHEDULER.lock();
    match guard.as_mut() {
        Some(sched) => sched.spawn(name),
        None => ThreadId(0),
    }
}

/// Get the number of threads
pub fn thread_count() -> usize {
    let guard = SCHEDULER.lock();
    guard.as_ref().map(|s| s.thread_count()).unwrap_or(0)
}

/// Run the scheduler loop (never returns)
pub fn run(machine: &'static dyn Machine) -> ! {
    log::klog(Level::Info, "KERNEL", "scheduler running");
    
    // For now, just idle forever
    // In the future, this will pick threads from the run queue
    loop {
        // Check if there are any runnable threads
        let has_runnable = {
            let guard = SCHEDULER.lock();
            guard.as_ref().map(|s| !s.run_queue.is_empty()).unwrap_or(false)
        };
        
        if has_runnable {
            // TODO: Context switch to next thread
            // For now, just log and idle
        }
        
        // Idle until next interrupt
        machine.arch().idle();
    }
}

/// Yield the current thread
pub fn yield_current() {
    // TODO: Implement yield
}

/// Exit the current thread
pub fn exit_current() {
    // TODO: Implement exit
}
