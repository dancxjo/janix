use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;

use crate::log::{self, Level};
use crate::machine::{ArchTrap, CurrentArch, TrapFrame};
use crate::memory::space::AddressSpace;
use crate::watch;
use abi::types::WakeReason;
use graph::store;
use graph::symbols::sym;

pub mod percpu;
pub mod run_queue;
pub mod task;
pub mod thread_group;

use percpu::PerCpu;
use run_queue::RunQueue;
pub use task::BlockReason;
pub use task::{Task, TaskId, TaskState};
use thread_group::ThreadGroup;

pub(crate) static SCHEDULER: Mutex<Option<Scheduler>> = Mutex::new(None);

pub fn with_sched<F, R>(f: F) -> R
where
    F: FnOnce(&mut Scheduler) -> R,
{
    let irq_token = crate::machine::irq_disable();
    let res = {
        let mut guard = SCHEDULER.lock();
        let sched = guard.as_mut().expect("SCHEDULER not initialized");
        f(sched)
    };
    crate::machine::irq_restore(irq_token);
    res
}

pub struct Scheduler {
    pub(crate) tasks: Vec<Task>,
    pub(crate) run_queue: RunQueue,
    pub(crate) cpu: PerCpu,
    pub(crate) next_id: u64,
    pub(crate) thread_groups: BTreeMap<u64, ThreadGroup>,
    pub(crate) next_group_id: u64,
}

impl Scheduler {
    fn new(run_queue_thing: abi::ids::ThingId, cpu_thing: abi::ids::ThingId) -> Self {
        Self {
            tasks: Vec::new(),
            run_queue: RunQueue::new(run_queue_thing),
            cpu: PerCpu::new(0, cpu_thing, run_queue_thing),
            next_id: 1,
            thread_groups: BTreeMap::new(),
            next_group_id: 1,
        }
    }

    /// Primary spawn entry point. 
    /// This method is DEPRECATED in favor of prepare_spawn + commit_task
    /// when called from contexts where GRAPH_STORE might be locked.
    pub(crate) fn spawn(
        &mut self,
        _name: &'static str,
        as_opt: Option<Arc<AddressSpace>>,
    ) -> TaskId {
        log::klog(Level::Info, "SCHED", &alloc::format!("spawn: name={} (legacy)", _name));
        
        let (task_thing_bits, stack_ptr, address_space) = prepare_spawn_internal(_name, as_opt);
        let task_thing = abi::ids::ThingId(task_thing_bits);

        let id = TaskId(self.next_id);
        self.next_id += 1;

        let mut task = Task::new(id, task_thing, stack_ptr, address_space);

        // Eager SIMD enablement
        let simd = crate::machine::simd();
        if simd.save_policy() == abi::cpu::SimdSavePolicy::Eager {
            task.simd_used = true;
            task.simd_state = Some(alloc::vec![0u8; simd.required_size()]);
        }

        task.state = TaskState::Ready;
        self.commit_task(task)
    }

    /// Internal helper to finalize task spawning.
    /// MUST be called with SCHEDULER lock held.
    pub(crate) fn commit_task(&mut self, task: Task) -> TaskId {
        let tid = task.id;
        let thing = task.thing;
        
        self.tasks.push(task);
        self.run_queue.push_back(tid, thing);
        
        tid
    }
}

pub fn init() {
    crate::serial::write(b"SCHED: init start\n");
    // Seed Graph (scheduler.main, cpu.0, run_queue.0)
    let (_sched_thing, cpu_thing, rq_thing) = store::with_store(|s| {
        crate::serial::write(b"SCHED: with_store interior\n");
        let graph_tasks = s
            .find_by_name(sym::GRAPH_TASKS)
            .expect("graph.tasks missing");
        crate::serial::write(b"SCHED: found graph.tasks\n");

        // scheduler.main
        let sched = s.create_thing(sym::KIND_SCHEDULER).expect("create sched");
        s.register_name(sched, sym::SCHEDULER_MAIN);
        s.create_relationship(sym::PRED_CONTAINS, graph_tasks, sched)
            .ok();

        // cpu.0
        let cpu = s.create_thing(sym::KIND_CPU).expect("create cpu");
        // s.register_name(cpu, \"cpu.0\"); // Need symbol
        s.create_relationship(sym::PRED_CONTAINS, graph_tasks, cpu)
            .ok();

        // run_queue.0
        let rq = s.create_thing(sym::KIND_RUN_QUEUE).expect("create rq");
        s.create_relationship(sym::PRED_CONTAINS, sched, rq).ok();

        (sched, cpu, rq)
    });
    crate::serial::write(b"SCHED: with_store done\n");

    let sched = Scheduler::new(rq_thing, cpu_thing);
    crate::serial::write(b"SCHED: scheduler struct created\n");
    *SCHEDULER.lock() = Some(sched);
    crate::serial::write(b"SCHED: scheduler initialized\n");
}

pub fn run() -> ! {
    log::klog(Level::Info, "SCHED", "entering loop");

    // Enable interrupts
    crate::serial::write(b"SCHED: calling irq_enable...\n");
    crate::machine::machine().irq_enable();

    let mut _last_irq_check = 0;
    loop {
        // Probe A2: Check IRQ progress
        #[cfg(target_arch = "aarch64")]
        {
            let irq_hits = crate::machine::aarch64::exception::IRQ_COUNT.load(Ordering::Relaxed);
            if irq_hits != _last_irq_check {
                _last_irq_check = irq_hits;
            }
        }

        crate::machine::idle();
    }
}

pub fn yield_current() {
    core::hint::spin_loop();
}

// Sprout helpers
pub fn mark_as_init(_id: TaskId) {
    // Mark in graph?
}

pub fn configure_task_memory(
    id: TaskId,
    _img: (u64, u64),
    _stack: (u64, u64),
    heap: (u64, u64, u64),
) {
    with_sched(|sched| {
        if let Some(t) = sched.tasks.iter_mut().find(|t| t.id == id) {
            t.heap_base = heap.0;
            t.heap_size = heap.1;
            t.heap_brk = heap.2;
        }
    });
}

pub fn configure_task_context(id: TaskId, entry: u64, user_stack: u64) {
    use crate::machine::{ArchTask, CpuMode, CurrentArch, TaskContext};

    with_sched(|sched| {
        if let Some(task) = sched.tasks.iter_mut().find(|t| t.id == id) {
            // Use Kernel Stack Top implicitly allocated by spawn
            // TrapFrame is built on kernel stack, user_stack is stored in RSP field for user mode
            let kernel_stack_top = task.stack_ptr & !0xf;

            crate::log::klog(
                crate::log::Level::Info,
                "SCHED",
                &alloc::format!(
                    "configure_ctx: k_stack={:#x} u_stack={:#x} entry={:#x}",
                    kernel_stack_top,
                    user_stack,
                    entry
                ),
            );

            // Initialize task context using arch-generic trait
            // stack_top = kernel stack (where TrapFrame is built)
            // arg0 = user stack (stored in RSP field for user mode return)
            let mut ctx = TaskContext::default();
            CurrentArch::init_task_context(
                &mut ctx,
                entry,
                kernel_stack_top, // TrapFrame built on kernel stack
                CpuMode::User,
                user_stack, // User stack passed via arg0 for RSP field
            );

            task.stack_ptr = ctx.sp;

            crate::log::klog(
                crate::log::Level::Info,
                "SCHED",
                &alloc::format!("configure_ctx: finalized sp={:#x}", task.stack_ptr),
            );
        }
    });
}

pub fn exit_current_task(_code: i32) -> ! {
    if let Some(task) = current_task_handle() {
        crate::watch::unregister_wait(task);
    }

    crate::machine::irq_disable();
    let next_sp = {
        let mut guard = SCHEDULER.lock();
        let sched = guard.as_mut().expect("sched not init");

        let prev = sched.cpu.current_task;
        if let Some(task) = sched.tasks.iter_mut().find(|t| t.id == prev) {
            task.state = TaskState::Dead;
        }

        if let Some(next) = sched.run_queue.pop_front() {
            sched.cpu.current_task = next;
            let t = sched.tasks.iter_mut().find(|t| t.id == next).unwrap();

            if t.simd_used {
                if let Some(state) = &t.simd_state {
                    crate::machine::simd().restore(state);
                }
            }

            t.state = TaskState::Running;
            t.first_run = false;

            crate::machine::machine().set_kernel_stack(t.stack_top);
            let new_sp = t.stack_ptr;
            t.address_space.activate();

            Some(new_sp)
        } else {
            sched.cpu.current_task = TaskId(0);
            None
        }
    };

    match next_sp {
        Some(sp) => unsafe {
            CurrentArch::return_from_trap(sp as *const TrapFrame);
        },
        None => {
            // No runnable tasks left; restore interrupts and halt.
            crate::machine::irq_enable();
            crate::machine::halt()
        }
    }
}

pub fn with_current_task<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&mut Task) -> R,
{
    let irq_token = crate::machine::irq_disable();
    let res = {
        let mut guard = SCHEDULER.lock();
        if let Some(sched) = guard.as_mut() {
            let curr = sched.cpu.current_task;
            if curr.0 == 0 {
                None
            } else {
                let t = sched.tasks.iter_mut().find(|t| t.id == curr).expect("current task not found");
                Some(f(t))
            }
        } else {
            None
        }
    };
    crate::machine::irq_restore(irq_token);
    res
}

pub fn with_task<F, R>(id: TaskId, f: F) -> Option<R>
where
    F: FnOnce(&mut Task) -> R,
{
    let irq_token = crate::machine::irq_disable();
    let res = {
        let mut guard = SCHEDULER.lock();
        if let Some(sched) = guard.as_mut() {
            let t = sched.tasks.iter_mut().find(|t| t.id == id)?;
            Some(f(t))
        } else {
            None
        }
    };
    crate::machine::irq_restore(irq_token);
    res
}

pub fn current_task_id() -> Option<abi::ids::ThingId> {
    // Disable interrupts to prevent deadlock with Timer ISR which also locks SCHEDULER
    let irq_token = crate::machine::irq_disable();
    let res = {
        let guard = SCHEDULER.lock();
        if let Some(sched) = guard.as_ref() {
            let tid = sched.cpu.current_task;
            if tid.0 != 0 {
                sched.tasks.iter().find(|t| t.id == tid).map(|t| t.thing)
            } else {
                None
            }
        } else {
            None
        }
    };
    crate::machine::irq_restore(irq_token);
    res
}

pub fn current_task_handle() -> Option<TaskId> {
    let irq_token = crate::machine::irq_disable();
    let res = {
        let guard = SCHEDULER.lock();
        guard.as_ref().and_then(|sched| {
            let tid = sched.cpu.current_task;
            if tid.0 != 0 {
                Some(tid)
            } else {
                None
            }
        })
    };
    crate::machine::irq_restore(irq_token);
    res
}

/// Internal implementation helper.
fn prepare_spawn_internal(
    _name: &'static str,
    as_opt: Option<Arc<AddressSpace>>,
) -> (u128, u64, Arc<AddressSpace>) {
    // 1. Allocate stack
    let stack_size = 64 * 1024; // 64KB
    let stack = alloc::vec![0u8; stack_size];
    let stack_ptr = stack.as_ptr() as u64 + stack_size as u64; // Top
    core::mem::forget(stack); // Leak for now

    // 2. Graph reflection (can deadlock if SCHEDULER lock is already held!)
    let task_thing = store::with_store(|s| {
        let t = s.create_thing(sym::KIND_TASK).expect("create task");
        if let Some(graph_tasks) = s.find_by_name(sym::GRAPH_TASKS) {
            let _ = s.create_relationship(sym::PRED_CONTAINS, graph_tasks, t);
        }
        t
    });

    let address_space = as_opt.unwrap_or_else(|| Arc::new(AddressSpace::new().expect("failed create AS")));
    
    (task_thing.0, stack_ptr, address_space)
}

/// Public API: Prepare a task for spawning without holding the scheduler lock.
pub fn prepare_spawn(
    name: &'static str,
    as_opt: Option<Arc<AddressSpace>>,
) -> (u128, u64, Arc<AddressSpace>) {
    prepare_spawn_internal(name, as_opt)
}

pub fn spawn_kernel_task(name: &'static str, entry: extern "C" fn()) -> TaskId {
    use crate::machine::{ArchTask, CpuMode, CurrentArch, TaskContext};

    // 1. Prepare (No SCHEDULER lock)
    let k_as = Arc::new(AddressSpace::new_kernel_share().expect("failed share kernel AS"));
    let (task_thing_bits, stack_ptr, address_space) = prepare_spawn_internal(name, Some(k_as));
    let task_thing = abi::ids::ThingId(task_thing_bits);

    // 2. Commit (SCHEDULER lock)
    let irq_token = crate::machine::irq_disable();
    let res = {
        let mut guard = SCHEDULER.lock();
        let sched = guard.as_mut().expect("sched not init");

        let id = TaskId(sched.next_id);
        sched.next_id += 1;

        let mut task = Task::new(id, task_thing, stack_ptr, address_space);

        // Eager SIMD enablement
        let simd = crate::machine::simd();
        if simd.save_policy() == abi::cpu::SimdSavePolicy::Eager {
            task.simd_used = true;
            task.simd_state = Some(alloc::vec![0u8; simd.required_size()]);
        }

        // Finalize task context
        let stack_top = task.stack_ptr & !0xf; // 16-byte align
        let mut ctx = TaskContext::default();
        CurrentArch::init_task_context(
            &mut ctx,
            entry as usize as u64,
            stack_top,
            CpuMode::Kernel,
            0, // arg0
        );
        task.stack_ptr = ctx.sp;
        task.state = TaskState::Ready;

        sched.commit_task(task)
    };
    
    crate::machine::irq_restore(irq_token);
    res
}

pub fn spawn_empty(name: &'static str) -> TaskId {
    // 1. Prepare (No SCHEDULER lock)
    let (task_thing_bits, stack_ptr, address_space) = prepare_spawn_internal(name, None);
    let task_thing = abi::ids::ThingId(task_thing_bits);

    // 2. Commit (SCHEDULER lock)
    let irq_token = crate::machine::irq_disable();
    let res = {
        let mut guard = SCHEDULER.lock();
        let sched = guard.as_mut().expect("sched not init");
        
        let id = TaskId(sched.next_id);
        sched.next_id += 1;
        
        let mut task = Task::new(id, task_thing, stack_ptr, address_space);
        
        // Eager SIMD enablement
        let simd = crate::machine::simd();
        if simd.save_policy() == abi::cpu::SimdSavePolicy::Eager {
            task.simd_used = true;
            task.simd_state = Some(alloc::vec![0u8; simd.required_size()]);
        }
        
        task.state = TaskState::Ready;
        sched.commit_task(task)
    };
    crate::machine::irq_restore(irq_token);
    res
}

pub static TIMER_TICKS: AtomicU64 = AtomicU64::new(0);

/// Counter for tick log suppression - only print first N switch messages
static TICK_LOG_COUNT: AtomicU64 = AtomicU64::new(0);

/// Maximum number of tick switch messages to print before going quiet
const TICK_LOG_LIMIT: u64 = 5;

pub fn tick(current_sp: u64) -> u64 {
    let mut guard = SCHEDULER.lock();
    let sched = guard.as_mut();
    if sched.is_none() {
        return 0;
    }
    let sched = sched.unwrap();

    if sched.cpu.in_switch || sched.cpu.preempt_disabled > 0 {
        return 0; // Defer if already switching or preemption disabled
    }

    // Set in_switch guard
    sched.cpu.in_switch = true;

    // 1. Inc timer ticks
    let ticks = TIMER_TICKS.fetch_add(1, Ordering::Relaxed) + 1;
    if ticks == 1 {
        crate::serial::write(b"SCHED: first tick!\n");
    }

    // NOTE: Cannot call graph operations from tick() - would deadlock on spinlock!
    // Mouse pointer state is published directly by the driver when queried.

    // 1.5 handle wait timeouts
    let mut wake_list = watch::WakeList::new();
    watch::check_timeouts(ticks, &mut wake_list);
    apply_wake_list(sched, &wake_list);

    let prev_task = sched.cpu.current_task;

    // Default: Round Robin
    // 1. Save current SP to current task (if we have one)
    if prev_task.0 != 0 {
        if let Some(t) = sched.tasks.iter_mut().find(|t| t.id == prev_task) {
            // Only save SP if this task has executed at least once.
            // first_run tasks have prepared contexts that must not be overwritten.
            if !t.first_run {
                t.stack_ptr = current_sp;

                // SIMD Save
                if t.simd_used {
                    let simd = crate::machine::simd();
                    if t.simd_state.is_none() {
                        t.simd_state = Some(alloc::vec![0u8; simd.required_size()]);
                    }
                    if let Some(buf) = t.simd_state.as_mut() {
                        simd.save(buf);
                    }
                }
            }
            match t.state {
                TaskState::Blocked(_) | TaskState::Dead => {}
                _ => {
                    t.state = TaskState::Ready;
                    // Enqueue
                    let thing = t.thing;
                    sched.run_queue.push_back(prev_task, thing);
                }
            }
        }
    }

    // 2. Pick next
    if let Some(next) = sched.run_queue.pop_front() {
        sched.cpu.current_task = next;
        let t = sched.tasks.iter_mut().find(|t| t.id == next).unwrap();

        // Only log the first few context switches to avoid flooding logs
        let log_count = TICK_LOG_COUNT.fetch_add(1, Ordering::Relaxed);
        if log_count < TICK_LOG_LIMIT {
            crate::serial::write(b"TICK: switch ");
            if prev_task.0 != 0 {
                crate::serial::write_num(prev_task.0);
            } else {
                crate::serial::write(b"IDLE");
            }
            crate::serial::write(b" -> ");
            crate::serial::write_num(next.0);
            crate::serial::write(b" sp=");
            crate::serial::write_hex(t.stack_ptr);

            if t.stack_ptr > t.stack_top {
                crate::serial::write(b" [STACK_OVERFLOW_DETECTED]");
            }
            if t.stack_ptr & 0xf != 0 {
                crate::serial::write(b" [SP_MISALIGN]");
            }
            crate::serial::write(b"\n");

            if log_count + 1 == TICK_LOG_LIMIT {
                crate::serial::write(b"TICK: (further switch logs suppressed)\n");
            }
        }

        // SIMD Restore
        if t.simd_used {
            if let Some(state) = &t.simd_state {
                crate::machine::simd().restore(state);
            }
        }

        t.state = TaskState::Running;
        t.first_run = false; // Mark as having started execution
                             // set_on_cpu requires graph lock, skipping for now to avoid deadlock in IRQ
                             // store::with_store(|s| t.set_on_cpu(s, sched.cpu.thing));

        // Set Kernel Stack for Syscall/Traps
        crate::machine::machine().set_kernel_stack(t.stack_top);

        let new_sp = t.stack_ptr;
        t.address_space.activate();

        // Clear in_switch before return
        sched.cpu.in_switch = false;
        new_sp
    } else {
        // Clear in_switch before return
        sched.cpu.in_switch = false;
        0
    }
}

fn apply_wake_list(sched: &mut Scheduler, wakes: &watch::WakeList) {
    for (task_id, reason) in wakes.iter() {
        wake_task_locked(sched, task_id, reason);
    }
}

fn wake_task_locked(sched: &mut Scheduler, task_id: TaskId, reason: WakeReason) {
    if let Some(task) = sched.tasks.iter_mut().find(|t| t.id == task_id) {
        task.wake_reason = Some(reason);
        if matches!(task.state, TaskState::Blocked(_)) {
            task.state = TaskState::Ready;
            let thing = task.thing;
            sched.run_queue.push_back(task_id, thing);
        }
    }
}

pub fn wake_task(task_id: TaskId, reason: WakeReason) {
    let irq_token = crate::machine::irq_disable();
    {
        let mut guard = SCHEDULER.lock();
        if let Some(sched) = guard.as_mut() {
            wake_task_locked(sched, task_id, reason);
        }
    }
    crate::machine::irq_restore(irq_token);
}

pub fn wake_from_watch_list(list: &watch::WakeList) {
    if list.is_empty() {
        return;
    }
    let irq_token = crate::machine::irq_disable();
    {
        let mut guard = SCHEDULER.lock();
        if let Some(sched) = guard.as_mut() {
            apply_wake_list(sched, list);
        }
    }
    crate::machine::irq_restore(irq_token);
}

pub fn block_current(reason: BlockReason) -> Option<TaskId> {
    let irq_token = crate::machine::irq_disable();
    let res = {
        let mut guard = SCHEDULER.lock();
        let sched = guard.as_mut()?;
        let curr = sched.cpu.current_task;
        if curr.0 == 0 {
            None
        } else {
            if let Some(task) = sched.tasks.iter_mut().find(|t| t.id == curr) {
                task.state = TaskState::Blocked(reason);
                task.wake_reason = None;
            }
            Some(curr)
        }
    };
    crate::machine::irq_restore(irq_token);
    res
}

pub fn take_wake_reason() -> Option<WakeReason> {
    with_current_task(|t| t.wake_reason.take()).flatten()
}

/// Configure task context while holding the scheduler lock.
/// This is used by spawn_kernel_module to set up the task context atomically
/// with spawning, preventing the task from being scheduled before it's ready.
pub fn configure_task_context_locked(
    sched: &mut Scheduler,
    id: TaskId,
    entry: u64,
    user_stack: u64,
) {
    use crate::machine::{ArchTask, CpuMode, CurrentArch, TaskContext};

    if let Some(task) = sched.tasks.iter_mut().find(|t| t.id == id) {
        // Use Kernel Stack Top implicitly allocated by spawn
        // TrapFrame is built on kernel stack, user_stack is stored in RSP field for user mode
        let kernel_stack_top = task.stack_ptr & !0xf;

        crate::log::klog(
            crate::log::Level::Info,
            "SCHED",
            &alloc::format!(
                "configure_ctx_locked: k_stack={:#x} u_stack={:#x} entry={:#x}",
                kernel_stack_top,
                user_stack,
                entry
            ),
        );

        // Initialize task context using arch-generic trait
        // stack_top = kernel stack (where TrapFrame is built)
        // arg0 = user stack (stored in RSP field for user mode return)
        let mut ctx = TaskContext::default();
        CurrentArch::init_task_context(
            &mut ctx,
            entry,
            kernel_stack_top, // TrapFrame built on kernel stack
            CpuMode::User,
            user_stack, // User stack passed via arg0 for RSP field
        );

        task.stack_ptr = ctx.sp;

        crate::log::klog(
            crate::log::Level::Info,
            "SCHED",
            &alloc::format!("configure_ctx_locked: finalized sp={:#x}", task.stack_ptr),
        );
    }
}
