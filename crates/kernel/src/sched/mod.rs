//! Scheduler
//!
//! Provides task management and scheduling for the kernel.
//! This is a minimal stub with the correct "future shape".

use alloc::collections::VecDeque;
use alloc::vec::Vec;
use spin::Mutex;

use crate::machine::abi::setup_new_task_stack;
// use crate::arch::Context; <-- Removed
use crate::log::{self, Level};
use crate::{graph, symbols, machine::Context, machine::machine};
use abi::ids::SymbolId;

/// Task identifier
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TaskId(pub u64);

/// Task state
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TaskState {
    /// Ready to run
    Ready,
    /// Currently running
    Running,
    /// Blocked on something
    Blocked,
    /// Terminated
    Dead,
}

/// Task control block
pub struct Task {
    /// Task identifier
    pub id: TaskId,
    /// Current state
    pub state: TaskState,
    /// Task name (for debugging)
    pub name: &'static str,
    
    /// Architecture context (saved stack pointer)
    pub ctx: Context,
    /// Kernel stack
    pub kstack: Vec<u8>,
    
    // Memory regions
    pub image_base: u64,
    pub image_size: u64,
    pub stack_base: u64,
    pub stack_size: u64,
    pub heap_base: u64,
    pub heap_size: u64,
    pub heap_brk: u64,
}

/// Global scheduler state
static SCHEDULER: Mutex<Option<Scheduler>> = Mutex::new(None);

struct Scheduler {
    /// All tasks
    tasks: Vec<Task>,
    /// Run queue (ready tasks)
    run_queue: VecDeque<TaskId>,
    /// Currently running task
    current: Option<TaskId>,
    /// Next task ID
    next_id: u64,
}

impl Scheduler {
    fn new() -> Self {
        Self {
            tasks: Vec::new(),
            run_queue: VecDeque::new(),
            current: None,
            next_id: 1,
        }
    }

    fn spawn(&mut self, name: &'static str) -> TaskId {
        let id = TaskId(self.next_id);
        self.next_id += 1;

        let task = Task {
            id,
            state: TaskState::Ready,
            name,
            ctx: Context::default(),
            kstack: alloc::vec![0u8; 16 * 1024], // 16KB kernel stack
            image_base: 0,
            image_size: 0,
            stack_base: 0,
            stack_size: 0,
            heap_base: 0,
            heap_size: 0,
            heap_brk: 0,
        };

        self.tasks.push(task);
        self.run_queue.push_back(id);

        id
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
    let sched = Scheduler::new();
    *SCHEDULER.lock() = Some(sched);

    log::klog(Level::Info, "KERNEL", "scheduler init");
    
    // Seed the scheduler ontology
    seed_scheduler();
}

/// Spawn a new kernel task
pub fn spawn_kernel_task(name: &'static str) -> TaskId {
    let mut guard = SCHEDULER.lock();
    match guard.as_mut() {
        Some(sched) => sched.spawn(name),
        None => TaskId(0),
    }
}

/// Access the current task mutably
pub fn with_current_task<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&mut Task) -> R,
{
    let mut guard = SCHEDULER.lock();
    if let Some(sched) = guard.as_mut() {
        if let Some(current_id) = sched.current {
            // Find the task
            if let Some(task) = sched.tasks.iter_mut().find(|t| t.id == current_id) {
                return Some(f(task));
            }
        } else if !sched.tasks.is_empty() {
             // Fallback for single-task phase (boot): verify if this is safe
             // For Task 1 (single task), we might want to just pick the first task if current is None
             // or ensure current is set.
             // But actually, `spawn_module` creates a task but doesn't set it as current until `run`?
             // Or we just hack it for now: if only one task, it's current.
             if sched.tasks.len() == 1 {
                 return Some(f(&mut sched.tasks[0]));
             }
        }
    }
    None
}

/// Configure task memory regions
pub fn configure_task_memory(id: TaskId, image: (u64,u64), stack: (u64,u64), heap: (u64,u64,u64)) {
    let mut guard = SCHEDULER.lock();
    if let Some(sched) = guard.as_mut() {
        if let Some(task) = sched.tasks.iter_mut().find(|t| t.id == id) {
            task.image_base = image.0;
            task.image_size = image.1;
            task.stack_base = stack.0;
            task.stack_size = stack.1;
            task.heap_base = heap.0;
            task.heap_size = heap.1;
            task.heap_brk = heap.2;
        }
    }
}

/// Configure task entry point (trampoline)
pub fn configure_task_context(id: TaskId, entry: u64) {
    let mut guard = SCHEDULER.lock();
    if let Some(sched) = guard.as_mut() {
        if let Some(task) = sched.tasks.iter_mut().find(|t| t.id == id) {
            let stack_size = task.kstack.len();
            let stack_ptr = task.kstack.as_mut_ptr();

            unsafe {
                let stack_top = stack_ptr.add(stack_size);
                
                // Assume `crate::syscall::dispatch` address is constant at link time.
                let dispatch = crate::syscall::dispatch as *const () as u64;
                
                setup_new_task_stack(
                    stack_top,
                    entry,
                    dispatch,
                    &mut task.ctx
                );
            }

        }
    }
}

/// Manually set the current task (for boot sequence)
pub fn set_current_task(id: TaskId) {
    let mut guard = SCHEDULER.lock();
    if let Some(sched) = guard.as_mut() {
        sched.current = Some(id);
    }
}

/// Run the scheduler loop (never returns)
pub fn run() -> ! {
    log::klog(Level::Info, "KERNEL", "scheduler running");

    crate::serial::write(b"SCHED: entering run loop\n");
    loop {
        // Simple Round Robin
        // Lock, check if current needs switching or if idle
        // crate::serial::write(b"SCHED: tick\n");
        let (_old_ptr, _new_ptr): (*mut Context, *const Context) = {
            let mut guard = SCHEDULER.lock();
            if let Some(sched) = guard.as_mut() {
                // If no current task, try to pick one
                if sched.current.is_none() {
                    if let Some(next) = sched.run_queue.pop_front() {
                        sched.current = Some(next);
                        // sched.tasks.find(next).state = Running; 
                        // Implement detail: we need mutable reference to update state
                        // We also need pointer for context switch if we were switching from something (but here we are starting)
                        // Actually boot sequence calls run() after setting up sprout.
                        // Sprout is already "configured" but not running?
                        // If we jump-start it here:
                        if let Some(task) = sched.tasks.iter_mut().find(|t| t.id == next) {
                            crate::serial::write(b"SCHED: switching to task\n");
                            task.state = TaskState::Running;
                            let new_ctx = &task.ctx as *const Context;
                            
                            // We need a dummy old context to save "scheduler loop" state?
                            // Or we just switch_to and never return to this precise point?
                            // Yes, the scheduler loop is "idle thread".
                            // We should have a Task for Idle? or just use stack local context?
                            // We'll create a dummy context on stack.
                            let mut idle_ctx = Context::default();
                            
                            // Drop lock before switch?
                            // No, pointers are derived from `sched`.
                            drop(guard);
                            
                            unsafe {
                                machine().switch_to(&mut idle_ctx, &*new_ctx);
                            }
                            // We returned! (Task yielded back to idle/scheduler?)
                            // Loop again.
                            continue;
                        }
                    }
                }
            }
            (core::ptr::null_mut(), core::ptr::null())
        };

        // Idle until next interrupt
        machine().idle();
    }
}

/// Yield the current task
pub fn yield_current() {
    // Disable interrupts to ensure atomicity of scheduling decision
    // (Single core assumption)
    let irq_state = machine().irq_disable();
    
    // We need to use raw pointers to avoid borrow checker issues with MutexGuard
    // while keeping the lock held or dropped safely.
    // Strategy:
    // 1. Lock.
    // 2. Pick next.
    // 3. If switch needed, get pointers, Updated states.
    // 4. Drop lock. (Safety: IRQs disabled, single core -> tasks Vec stable)
    // 5. switch_to.
    // 6. Re-enable IRQs.

    let switch_args = {
        let mut guard = SCHEDULER.lock();
        if let Some(sched) = guard.as_mut() {
            if let Some(current_id) = sched.current {
                // Round robin: push current to back of queue
                sched.run_queue.push_back(current_id);
                
                // Pop next
                if let Some(next_id) = sched.run_queue.pop_front() {
                    if next_id != current_id {
                        // Switch needed!
                        sched.current = Some(next_id);
                        
                        // We need mutable access to both old and new tasks.
                        // Since they are in the same Vec, we have to split borrow or use indices safe?
                        // Using indices to get pointers.
                        // Verify task existence.
                        let old_idx = sched.tasks.iter().position(|t| t.id == current_id);
                        let new_idx = sched.tasks.iter().position(|t| t.id == next_id);
                        
                        if let (Some(old_i), Some(new_i)) = (old_idx, new_idx) {
                             // Update states
                             sched.tasks[old_i].state = TaskState::Ready;
                             sched.tasks[new_i].state = TaskState::Running;
                             
                             let old_ptr = &mut sched.tasks[old_i].ctx as *mut Context;
                             let new_ptr = &sched.tasks[new_i].ctx as *const Context;
                             
                             Some((old_ptr, new_ptr))
                        } else {
                            None
                        }
                    } else {
                        // Same task, no switch
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    };

    if let Some((old_ptr, new_ptr)) = switch_args {
        // SAFETY: IRQs disabled, Scheduler lock dropped but we are single threaded and no one else modifies tasks.
        unsafe {
            machine().switch_to(&mut *old_ptr, &*new_ptr);
        }
    }
    
    machine().irq_restore(irq_state);
}

/// Trampoline for new task entry
///
/// Called by architecture-specific assembly stubs.
#[no_mangle]
pub extern "C" fn task_dispatch(dispatch_ptr: u64, entry: u64) -> ! {
    crate::serial::write(b"SCHED: dispatching...\n");
    // For Sprout (Ring 3), entry is the process entry point.
    // The dispatch_ptr is actually not used to call it directly for Ring 3?
    // Wait, if it's Ring 0 task, we call it. 
    // If it's Ring 3, we need to switch to user mode.
    // BUT current sprout spawning in boot.rs assumes it's just a kernel task running that code?
    // Yes, for now it runs in Ring 0 (kernel task). 
    // Future v0.3 plan involves separated userland. 
    // For now, we just jump to it.
    
    let f: extern "C" fn(u64) -> ! = unsafe { core::mem::transmute(entry) };
    
    // dispatch_ptr might be used if we needed to pass context, but here we just run.
    f(dispatch_ptr);
}

