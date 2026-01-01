//! Scheduler
//!
//! Provides thread management and scheduling for the kernel.
//! This is a minimal stub with the correct "future shape".

use alloc::collections::VecDeque;
use alloc::vec::Vec;
use spin::Mutex;

use crate::arch::Arch;
use crate::arch::ARCH;
use crate::log::{self, Level};
use crate::{graph, symbols};
use abi::ids::SymbolId;

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
    _current: Option<ThreadId>,
    /// Next thread ID
    next_id: u64,
    /// Initialized flag
    _initialized: bool,
}

impl Scheduler {
    fn new() -> Self {
        Self {
            threads: Vec::new(),
            run_queue: VecDeque::new(),
            _current: None,
            next_id: 1,
            _initialized: false,
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

/// Seed the scheduler ontology
fn seed_scheduler() {
    let kind_thing = symbols::well_known(b"kind.Thing");
    let pred_contains = symbols::well_known(b"predicate.contains");
    let pred_state = symbols::sym_pred_state();

    // 1. Find Scheduler Place
    let sched_name = symbols::sym_scheduler();
    let sched_id = graph::find_thing_by_name(sched_name).expect("Scheduler place should be seeded by kernel");
    log::klog(Level::Info, "SCHED", "scheduler place found");

    // 2. Create Runqueue (Thing)
    let runqueue_name = symbols::sym_runqueue_default();
    let runqueue_id = graph::thing_create(kind_thing, SymbolId::INVALID, 1);
    graph::thing_register_name(runqueue_id, runqueue_name);

    // Relate: scheduler contains runqueue
    graph::relationship_create(sched_id, runqueue_id, pred_contains);

    // 3. Create Task (Sprout)
    let task_name = symbols::sym_task_sprout();
    let task_id = graph::thing_create(kind_thing, SymbolId::INVALID, 1);
    graph::thing_register_name(task_id, task_name);
    log::klog(Level::Info, "SCHED", "task created: thing.task.sprout");

    // Relate: runqueue contains task
    graph::relationship_create(runqueue_id, task_id, pred_contains);

    // 4. Create State (Running)
    let state_running_name = symbols::sym_state_running();
    let state_running_id = graph::thing_create(kind_thing, SymbolId::INVALID, 1);
    graph::thing_register_name(state_running_id, state_running_name);

    // Relate: task has state running
    graph::relationship_create(task_id, state_running_id, pred_state);
    log::klog(Level::Info, "SCHED", "task state set: running");
}

/// Initialize the scheduler
pub fn init() {
    let mut sched = Scheduler::new();
    sched._initialized = true;
    *SCHEDULER.lock() = Some(sched);

    log::klog(Level::Info, "KERNEL", "scheduler init");
    
    // Seed the scheduler ontology
    seed_scheduler();
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
pub fn run() -> ! {
    log::klog(Level::Info, "KERNEL", "scheduler running");
    log::klog(Level::Info, "HELLO", "scheduler alive");

    // For now, just idle forever
    // In the future, this will pick threads from the run queue
    loop {
        // Check if there are any runnable threads
        let has_runnable = {
            let guard = SCHEDULER.lock();
            guard
                .as_ref()
                .map(|s| !s.run_queue.is_empty())
                .unwrap_or(false)
        };

        if has_runnable {
            // TODO: Context switch to next thread
            // For now, just log and idle
        }

        // Idle until next interrupt
        ARCH.idle();
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
