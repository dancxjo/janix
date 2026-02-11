pub mod loader;
pub mod scheduler;

pub use scheduler::Scheduler;

use crate::BootRuntime;
use crate::BootTasking;
use crate::simd::SimdState;
use abi::types::StackInfo;
use alloc::sync::Arc;
use spin::Mutex;

pub type TaskId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartupArg {
    None,
    BootRegistry,
    DeviceId(u64),
    Raw(usize),
}

impl StartupArg {
    pub fn to_raw(self) -> usize {
        match self {
            StartupArg::None => 0,
            StartupArg::BootRegistry => 0x600000,
            StartupArg::DeviceId(id) => id as usize,
            StartupArg::Raw(val) => val,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    Realtime = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Affinity {
    Any,
    Pinned(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Runnable,
    Running,
    Blocked,
    Dead,
}

pub struct Task<R: BootRuntime> {
    pub id: TaskId,
    pub state: TaskState,
    pub priority: TaskPriority,
    pub exit_code: Option<i32>,
    pub is_user: bool,
    pub wake_pending: bool,
    pub affinity: Affinity,

    pub kstack_base: *mut u8,
    pub kstack_size: usize,
    pub kstack_top: u64,

    pub ctx: <R::Tasking as BootTasking>::Context,
    pub aspace: <R::Tasking as BootTasking>::AddressSpace,

    pub simd: SimdState,

    pub stack_info: Option<StackInfo>,

    pub mappings: Arc<Mutex<crate::memory::mappings::MappingList>>,

    /// Remaining time slice in ticks before preemption
    pub timeslice_remaining: u32,
    pub last_cpu: Option<usize>,

    /// Short human-readable name (e.g. "bristle", "idle/0")
    pub name: [u8; 32],
    pub name_len: u8,
}

pub fn init<R: BootRuntime>() {
    scheduler::init::<R>();
}

pub fn spawn<R: BootRuntime>(
    entry: extern "C" fn(usize) -> !,
    arg: StartupArg,
    priority: TaskPriority,
    affinity: Affinity,
) -> TaskId {
    scheduler::spawn::<R>(entry, arg, priority, affinity)
}

pub fn spawn_with_priority<R: BootRuntime>(
    entry: extern "C" fn(usize) -> !,
    arg: StartupArg,
    priority: TaskPriority,
) -> TaskId {
    scheduler::spawn_with_priority::<R>(entry, arg, priority)
}

pub unsafe fn block_current_erased() {
    unsafe {
        scheduler::block_current_erased();
    }
}

pub unsafe fn wake_task_erased(tid: usize) {
    unsafe {
        scheduler::wake_task_erased(tid);
    }
}

pub fn yield_now<R: BootRuntime>() {
    scheduler::yield_now::<R>();
}

pub fn preempt_disable<R: BootRuntime>() {
    let lock = scheduler::SCHEDULER.lock();
    if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
        sched.preempt_disable();
    }
}

pub fn preempt_enable<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let irq = rt.irq_disable();

    let switch_params = {
        let lock = scheduler::SCHEDULER.lock();
        if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
            sched.preempt_enable()
        } else {
            None
        }
    };

    if let Some(switch) = switch_params {
        let cr3_before = rt.debug_active_aspace_root();

        rt.tasking().activate_address_space(switch.to_aspace);

        let cr3_after = rt.debug_active_aspace_root();
        scheduler::log_context_switch::<R>(&switch, cr3_before, cr3_after);

        unsafe {
            rt.tasking()
                .switch(&mut *switch.from_ctx, &*switch.to_ctx, switch.to_tid);
        }
    }

    rt.irq_restore(irq);
}

pub fn resched_if_needed<R: BootRuntime>() {
    // Similar to preempt_enable but without decrementing (conceptually checks "is resched needed?")
    // But actually, we just want to explicit check.
    // For now, let's just use yield_now if needed?
    // Actually, `schedule_point(PreemptTick)` logic inside `preempt_enable` handles the check.
    // So this might just be a no-op or a direct check.
    // Let's implement it as a check for `need_resched` and call `yield_now` (or equivalent) if true.
    // BUT we need to be careful about recursion.
    // For v0, let's leave it as a TODO or a simple "maybe yield".
    // "resched_if_needed at safe point" -> usually checks flags.

    let rt = crate::runtime::<R>();
    let irq = rt.irq_disable();

    let switch_params = {
        let lock = scheduler::SCHEDULER.lock();
        if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
            // We use PreemptTick reason to check need_resched flag inside schedule_point?
            // Wait, schedule_point(PreemptTick) checks if disabled.
            // If we are here, we are at a safe point, so we assume preemption is enabled (or we ignore depth?).
            // Actually, if we are at a safe point (e.g. syscall return), we should yield if need_resched is set.
            // But we can't access `need_resched` without the lock.
            // Let's rely on `sched.schedule_point(PreemptTick)` to return None if disabled,
            // OR we need a new reason `ReschedIfNeeded`?
            // For now, let's abuse `CooperativeYield` if we see the flag? No.
            // Let's allow `PreemptTick` to serve this purpose.

            // Wait, implementation of `schedule_point` for `PreemptTick`:
            // if disable_depth > 0 { need_resched = true; return None; }
            // else { prepare_yield() }

            // So calling it here works perfectly.
            sched.schedule_point(scheduler::ScheduleReason::PreemptTick)
        } else {
            None
        }
    };

    if let Some(switch) = switch_params {
        let cr3_before = rt.debug_active_aspace_root();

        rt.tasking().activate_address_space(switch.to_aspace);

        let cr3_after = rt.debug_active_aspace_root();
        scheduler::log_context_switch::<R>(&switch, cr3_before, cr3_after);

        unsafe {
            rt.tasking()
                .switch(&mut *switch.from_ctx, &*switch.to_ctx, switch.to_tid);
        }
    }

    rt.irq_restore(irq);
}

pub fn dump_stats<R: BootRuntime>() {
    scheduler::dump_stats::<R>();
}

/// Bootstrap a CPU for scheduling. Must be called before the first yield
/// on any CPU that doesn't already have a current task set (e.g., secondary CPUs).
fn bootstrap_cpu<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let lock = scheduler::SCHEDULER.lock();
    if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
        let cpu_idx = scheduler::current_cpu_index::<R>();

        if let Some(pc) = sched.per_cpu.get_mut(cpu_idx) {
            if pc.current.is_none() {
                // CPU hasn't been bootstrapped yet. Set current to idle task.
                if let Some(idle_id) = pc.idle_task {
                    pc.current = Some(idle_id);
                    rt.set_current_tid(idle_id);
                    crate::kinfo!(
                        "SMP: CPU {} bootstrapped with idle task {}",
                        cpu_idx,
                        idle_id
                    );

                    // Mark the idle task as running
                    if let Some(task) = sched.tasks.iter_mut().find(|t| t.id == idle_id) {
                        task.state = TaskState::Running;
                    }
                } else {
                    crate::kerror!("SMP: CPU {} has no idle task!", cpu_idx);
                }
            }
        }
    }

    rt.irq_restore(_irq);
}

pub fn run_scheduler<R: BootRuntime>() -> ! {
    // Bootstrap this CPU if needed (sets current task for secondary CPUs)
    bootstrap_cpu::<R>();

    // Enable interrupts so this CPU can be preempted or woken from idle (HLT)
    crate::runtime::<R>().irq_restore(crate::IrqState(1));

    loop {
        yield_now::<R>();
        core::hint::spin_loop();
    }
}
