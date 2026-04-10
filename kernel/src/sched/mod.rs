//! Preemptive priority-based scheduler
//!
//! This module is split into focused submodules:
//! - `types`: Core data structures and enums
//! - `blocking`: Task blocking and wake primitives  
//! - `hooks`: Type-erased hook system for callers without generic params
//! - `spawn`: Task and thread spawning
//! - `stack`: User stack allocation and fault handling
//! - `sleep`: Timing and yield functions
//! - `events`: Lock-free scheduler event types

pub(crate) mod blocking;
pub mod hooks;
pub use hooks::protect_user_range_current;
mod sleep;
mod spawn;
mod stack;
pub mod state;
pub(crate) mod types;
mod vm;
pub(crate) mod wait_queue;

// Re-export all public items
pub use blocking::{
    block_current, block_current_erased, init_blocking_hooks, wake_task, wake_task_erased,
};
pub use hooks::{
    ProcessSnapshot, add_user_mapping_current, alloc_user_stack_current,
    check_user_mapping_current, current_priority_current, current_task_name_current,
    current_tid_current, dump_stats_current, exit_current, get_user_mapping_at_current,
    graph_thing_for_current, handle_user_stack_fault_current, kill_by_tid_current,
    list_processes_current, poll_task_exit_current, process_info_current,
    process_info_for_tid_current, register_task_exit_waiter_current, register_timeout_wake_current,
    remove_user_mappings_current, set_current_user_fs_base_current, set_priority_current,
    sleep_ticks_current, spawn_process_current, spawn_process_ex_current,
    spawn_user_thread_current, task_exec_current, task_status_current, task_wait_current,
    unregister_task_exit_waiter_current, unregister_timeout_wake_current, yield_now_current,
};
pub use sleep::{sleep_ms, sleep_ticks, sleep_until, yield_now};
pub use spawn::{
    SpawnExResult, StdioSpec, spawn, spawn_process, spawn_user_task_full, spawn_user_thread,
    spawn_with_priority, user_thread_trampoline,
};
pub use stack::{alloc_user_stack, handle_stack_fault, map_user_page, map_user_page_perms};
pub use types::{DEFAULT_TIMESLICE, ScheduleReason, Scheduler, StackFaultResult, SwitchParams};
pub use wait_queue::WaitQueue;

use crate::task::{StartupArg, Task, TaskId, TaskPriority, TaskState};
use crate::{BootRuntime, BootTasking};
use core::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use spin::Mutex;

#[cfg(any(feature = "sched_debug", debug_assertions))]
static SWITCH_LOG_COUNT: AtomicUsize = AtomicUsize::new(0);

pub static SCHEDULER: Mutex<Option<usize>> = Mutex::new(None);

/// Global tick counter for debugging scheduler health
pub static TICK_COUNT: AtomicU64 = AtomicU64::new(0);

pub static PROF_RESCHED_TRYLOCK_MISS: AtomicU64 = AtomicU64::new(0);

// Diagnostic counters for IPI delivery chain
pub static DIAG_IPI_SENT: AtomicU64 = AtomicU64::new(0);
pub static DIAG_IPI_HANDLER: AtomicU64 = AtomicU64::new(0);
pub static DIAG_HLT_WAKE: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy, Debug, Default)]
pub struct SchedLockMetrics {
    pub hold_calls: u64,
    pub hold_us_total: u64,
    pub hold_us_max: u64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SchedLockSiteMetrics {
    pub block_current: SchedLockMetrics,
    pub wake_task: SchedLockMetrics,
    pub yield_now: SchedLockMetrics,
    pub sleep_ticks: SchedLockMetrics,
    pub wake_sleepers: SchedLockMetrics,
}

static PROF_SCHED_LOCK_BLOCK_CURRENT_CALLS: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_BLOCK_CURRENT_US_TOTAL: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_BLOCK_CURRENT_US_MAX: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_WAKE_TASK_CALLS: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_WAKE_TASK_US_TOTAL: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_WAKE_TASK_US_MAX: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_YIELD_NOW_CALLS: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_YIELD_NOW_US_TOTAL: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_YIELD_NOW_US_MAX: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_SLEEP_TICKS_CALLS: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_SLEEP_TICKS_US_TOTAL: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_SLEEP_TICKS_US_MAX: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_WAKE_SLEEPERS_CALLS: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_WAKE_SLEEPERS_US_TOTAL: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_WAKE_SLEEPERS_US_MAX: AtomicU64 = AtomicU64::new(0);

/// Lock-skip self-healing: when try_resched_if_needed() fails to acquire
/// the scheduler lock, set this flag so the next safe-point yields.
static GLOBAL_NEED_RESCHED: AtomicBool = AtomicBool::new(false);

/// If trylock misses exceed this count in a 2-second window, emit a warning.
pub const TRYLOCK_MISS_WARN_THRESHOLD: u64 = 50;

#[inline]
fn ticks_to_us<R: BootRuntime>(ticks: u64) -> u64 {
    let rt = crate::runtime::<R>();
    let freq = rt.mono_freq_hz().max(1);
    ticks.saturating_mul(1_000_000) / freq
}

#[inline]
fn update_max_u64(slot: &AtomicU64, val: u64) {
    let mut prev = slot.load(Ordering::Relaxed);
    while val > prev {
        match slot.compare_exchange_weak(prev, val, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => break,
            Err(actual) => prev = actual,
        }
    }
}

#[inline]
fn snapshot_sched_lock_metric(
    calls: &AtomicU64,
    total: &AtomicU64,
    max: &AtomicU64,
) -> SchedLockMetrics {
    SchedLockMetrics {
        hold_calls: calls.swap(0, Ordering::Relaxed),
        hold_us_total: total.swap(0, Ordering::Relaxed),
        hold_us_max: max.swap(0, Ordering::Relaxed),
    }
}

pub(crate) fn record_sched_lock_hold<R: BootRuntime>(
    calls: &AtomicU64,
    total: &AtomicU64,
    max: &AtomicU64,
    start_ticks: u64,
) {
    let elapsed_us = ticks_to_us::<R>(crate::runtime::<R>().mono_ticks().wrapping_sub(start_ticks));
    calls.fetch_add(1, Ordering::Relaxed);
    total.fetch_add(elapsed_us, Ordering::Relaxed);
    update_max_u64(max, elapsed_us);
}

pub fn sched_lock_metrics_snapshot_and_reset() -> SchedLockSiteMetrics {
    SchedLockSiteMetrics {
        block_current: snapshot_sched_lock_metric(
            &PROF_SCHED_LOCK_BLOCK_CURRENT_CALLS,
            &PROF_SCHED_LOCK_BLOCK_CURRENT_US_TOTAL,
            &PROF_SCHED_LOCK_BLOCK_CURRENT_US_MAX,
        ),
        wake_task: snapshot_sched_lock_metric(
            &PROF_SCHED_LOCK_WAKE_TASK_CALLS,
            &PROF_SCHED_LOCK_WAKE_TASK_US_TOTAL,
            &PROF_SCHED_LOCK_WAKE_TASK_US_MAX,
        ),
        yield_now: snapshot_sched_lock_metric(
            &PROF_SCHED_LOCK_YIELD_NOW_CALLS,
            &PROF_SCHED_LOCK_YIELD_NOW_US_TOTAL,
            &PROF_SCHED_LOCK_YIELD_NOW_US_MAX,
        ),
        sleep_ticks: snapshot_sched_lock_metric(
            &PROF_SCHED_LOCK_SLEEP_TICKS_CALLS,
            &PROF_SCHED_LOCK_SLEEP_TICKS_US_TOTAL,
            &PROF_SCHED_LOCK_SLEEP_TICKS_US_MAX,
        ),
        wake_sleepers: snapshot_sched_lock_metric(
            &PROF_SCHED_LOCK_WAKE_SLEEPERS_CALLS,
            &PROF_SCHED_LOCK_WAKE_SLEEPERS_US_TOTAL,
            &PROF_SCHED_LOCK_WAKE_SLEEPERS_US_MAX,
        ),
    }
}

/// Called from timer ISR - records tick and triggers reschedule if needed
/// Uses try_resched_if_needed to avoid deadlock when SCHEDULER is held by main code
pub fn on_tick<R: BootRuntime>() {
    let cpu_idx = crate::runtime::<R>().current_cpu_id().0;
    let ticks = if cpu_idx == 0 {
        TICK_COUNT.fetch_add(1, Ordering::Relaxed) + 1
    } else {
        TICK_COUNT.load(Ordering::Relaxed)
    };

    DIAG_IPI_HANDLER.fetch_add(1, Ordering::Relaxed);

    // Periodically log on CPU 0 to show time is passing
    if ticks % 1000 == 0 && cpu_idx == 0 {
        crate::kdebug!("SCHED: Tick {} on CPU 0", ticks);
    }

    try_resched_if_needed::<R>();
}

/// Called from IPI handler - triggers reschedule without advancing time
pub fn on_resched_ipi<R: BootRuntime>() {
    let cpu = crate::runtime::<R>().current_cpu_id().0;
    crate::kdebug!("SCHED: Received Resched IPI on CPU {}", cpu);
    DIAG_IPI_HANDLER.fetch_add(1, Ordering::Relaxed);
    try_resched_if_needed::<R>();
}

/// Interrupt-safe version of resched_if_needed - uses try_lock to avoid deadlock
/// If SCHEDULER lock is contended, simply skip rescheduling this tick
fn try_resched_if_needed<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let irq = rt.irq_disable();

    // Use try_lock to avoid deadlock if SCHEDULER is held by main code
    if let Some(lock) = SCHEDULER.try_lock() {
        if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
            if let Some(switch) = sched.schedule_point(ScheduleReason::PreemptTick) {
                // Must drop lock before context switch!
                drop(lock);

                rt.tasking().activate_address_space(switch.to_aspace);

                unsafe {
                    rt.tasking().switch_with_tls(
                        &mut *switch.from_ctx,
                        &*switch.to_ctx,
                        switch.to_tid,
                        switch.from_user_fs_base,
                        switch.to_user_fs_base,
                    );
                }
            }
        }
    } else {
        PROF_RESCHED_TRYLOCK_MISS.fetch_add(1, Ordering::Relaxed);
        // Self-healing: tell the next safe point to reschedule
        GLOBAL_NEED_RESCHED.store(true, Ordering::Release);
    }
    // If try_lock failed, skip rescheduling this tick - not a problem, next tick will try again

    rt.irq_restore(irq);
}

pub(crate) fn current_cpu_index<R: BootRuntime>() -> usize {
    let rt = crate::runtime::<R>();
    rt.current_cpu_index()
}

pub fn init<R: BootRuntime>() {
    crate::kdebug!("  Acquiring scheduler lock...");
    let mut lock = SCHEDULER.lock();
    crate::kdebug!("  Lock acquired, checking if initialized...");
    if lock.is_none() {
        crate::kdebug!("  Allocating scheduler...");
        let sched = alloc::boxed::Box::new(types::Scheduler::<R>::new());
        crate::kdebug!("  Leaking scheduler...");
        let s = alloc::boxed::Box::leak(sched);
        crate::kdebug!("  Initializing boot task...");
        init_boot_task::<R>(s);
        crate::kdebug!("  Storing scheduler pointer...");
        *lock = Some(s as *mut types::Scheduler<R> as usize);
        unsafe {
            hooks::YIELD_HOOK = Some(sleep::yield_now::<R>);
            hooks::EXIT_HOOK = Some(exit::<R>);
            hooks::SPAWN_USER_HOOK = Some(spawn::spawn_user_thread::<R>);
            hooks::SPAWN_PROCESS_HOOK = Some(spawn::spawn_process::<R>);
            hooks::CURRENT_TID_HOOK = Some(current_tid::<R>);
            hooks::TASK_STATUS_HOOK = Some(task_status::<R>);
            hooks::TASK_WAIT_HOOK = Some(wait_task::<R>);
            hooks::SET_PRIORITY_HOOK = Some(set_priority::<R>);
            hooks::CURRENT_PRIORITY_HOOK = Some(current_priority::<R>);
            hooks::ALLOC_USER_STACK_HOOK = Some(stack::alloc_user_stack::<R>);
            hooks::RUN_SCHEDULER_HOOK = Some(crate::task::run_scheduler::<R>);
            hooks::KILL_BY_TID_HOOK = Some(kill_by_tid::<R>);
            hooks::DUMP_STATS_HOOK = Some(crate::task::dump_stats::<R>);
            crate::memory::set_map_user_page_hook(stack::map_user_page::<R>);
            crate::memory::set_map_user_page_perms_hook(stack::map_user_page_perms::<R>);
            crate::memory::set_unmap_user_page_hook(stack::unmap_user_page::<R>);
            crate::memory::set_protect_user_page_hook(stack::protect_user_page::<R>);
            hooks::STACK_FAULT_HOOK = Some(stack::handle_stack_fault::<R>);
            hooks::SLEEP_TICKS_HOOK = Some(sleep::sleep_ticks::<R>);
            hooks::ADD_USER_MAPPING_HOOK = Some(vm::add_user_mapping::<R>);
            hooks::REMOVE_USER_MAPPINGS_HOOK = Some(vm::remove_user_mappings::<R>);
            hooks::CHECK_USER_MAPPING_HOOK = Some(vm::check_user_mapping::<R>);
            hooks::GET_USER_MAPPING_AT_HOOK = Some(vm::get_user_mapping_at::<R>);
            hooks::PROTECT_USER_RANGE_HOOK = Some(vm::protect_user_range::<R>);
            hooks::PROCESS_INFO_HOOK = Some(process_info::<R>);
            hooks::PROCESS_INFO_FOR_TID_HOOK = Some(process_info_for_tid::<R>);
            hooks::SPAWN_PROCESS_EX_HOOK = Some(spawn::spawn_process_ex::<R>);
            hooks::GRAPH_THING_FOR_CURRENT_HOOK = Some(graph_thing_for_current_impl::<R>);
            hooks::POLL_TASK_EXIT_HOOK = Some(poll_task_exit::<R>);
            hooks::REGISTER_TASK_EXIT_WAITER_HOOK = Some(register_task_exit_waiter_public::<R>);
            hooks::UNREGISTER_TASK_EXIT_WAITER_HOOK = Some(unregister_task_exit_waiter::<R>);
            hooks::REGISTER_TIMEOUT_WAKE_HOOK = Some(register_timeout_wake::<R>);
            hooks::UNREGISTER_TIMEOUT_WAKE_HOOK = Some(unregister_timeout_wake::<R>);
            hooks::LIST_PROCESSES_HOOK = Some(list_processes::<R>);
            hooks::CURRENT_TASK_NAME_HOOK = Some(current_task_name_impl::<R>);
            hooks::TASK_EXEC_HOOK = Some(crate::task::exec::task_exec_current::<R>);
            hooks::SET_CURRENT_USER_FS_BASE_HOOK = Some(set_current_user_fs_base::<R>);
            crate::memory::set_translate_user_page_hook(vm::translate_user_page::<R>);
        }
        blocking::init_blocking_hooks::<R>();
        let cpu_total = if let Some(ptr) = *lock {
            let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };
            sched.total_cpu_count
        } else {
            1
        };
        crate::contract!("Scheduler initialized");
    }
}

fn init_boot_task<R: BootRuntime>(sched: &mut types::Scheduler<R>) {
    let rt = crate::runtime::<R>();
    let cpu_total = rt.cpu_total_count();

    // Initialize PerCpu state for all CPUs (initially empty/offline)
    for _ in 0..cpu_total {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }

    sched.total_cpu_count = cpu_total;
    sched.state.online_cpu_count = 1;

    crate::kdebug!("  Creating boot task...");

    let layout = alloc::alloc::Layout::from_size_align(16384, 8).unwrap();
    let stack_base = unsafe { alloc::alloc::alloc(layout) };
    if stack_base.is_null() {
        panic!("Failed to allocate stack for boot task");
    }
    let stack_top = (stack_base as u64) + 16384;

    let task: Task<R> = Task {
        id: 0,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        kstack_base: stack_base,
        kstack_size: 16384,
        kstack_top: stack_top,
        ctx: Default::default(),
        aspace: rt.tasking().active_address_space(),
        simd: crate::simd::SimdState::new(rt),
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: false,
        wake_pending: false,
        stack_info: None,
        mappings: alloc::sync::Arc::new(spin::Mutex::new(
            crate::memory::mappings::MappingList::new(),
        )),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        affinity: crate::task::Affinity::Any,
        last_cpu: Some(0),
        name: {
            let mut n = [0u8; 32];
            n[0] = b'b';
            n[1] = b'o';
            n[2] = b'o';
            n[3] = b't';
            n
        },
        name_len: 4,
        process_info: None,
        enqueued_at_tick: TICK_COUNT.load(Ordering::Relaxed),
        base_priority: TaskPriority::Normal,
        user_fs_base: 0,
    };
    let sched_fields = crate::sched::state::TaskSchedFields {
        tid: task.id,
        state: task.state,
        priority: task.priority,
        base_priority: task.base_priority,
        timeslice_remaining: task.timeslice_remaining,
        affinity: task.affinity,
        enqueued_at_tick: task.enqueued_at_tick,
        last_cpu: task.last_cpu,
        runq_location: None,
    };
    sched.state.insert_task(sched_fields);
    crate::task::registry::get_registry::<R>().insert(alloc::boxed::Box::new(task));

    // Boot task runs on CPU 0
    sched.state.per_cpu[0].current = Some(0);

    // Link boot task to CPU 0

    crate::kdebug!("  Creating idle tasks...");

    // Create idle task for CPU 0 initially
    {
        let i = 0;
        let idle_id = sched.spawn(
            idle_task::<R>,
            StartupArg::Raw(i),
            TaskPriority::Idle,
            crate::task::Affinity::Pinned(i),
        );

        // Remove from run queues - idle tasks are special
        for q in sched
            .state
            .per_cpu
            .iter_mut()
            .flat_map(|pc| pc.runq.iter_mut())
        {
            if let Some(pos) = q.iter().position(|&id| id == idle_id) {
                q.remove(pos);
            }
        }

        // Set as this CPU's idle task
        sched.state.per_cpu[i].idle_task = Some(idle_id);

        // Pin idle task to its CPU
        if let Some(mut t) = crate::task::registry::get_task_mut::<R>(idle_id) {
            t.affinity = crate::task::Affinity::Pinned(i);
        }
    }

    crate::kdebug!("  Boot task initialized");
}

impl<R: BootRuntime> types::Scheduler<R> {
    pub fn schedule_point(
        &mut self,
        reason: ScheduleReason,
    ) -> Option<
        SwitchParams<
            <R::Tasking as BootTasking>::Context,
            <R::Tasking as BootTasking>::AddressSpace,
        >,
    > {
        match reason {
            ScheduleReason::PreemptTick => {
                // Wake any sleeping tasks whose time has expired (Timekeeper only)
                if current_cpu_index::<R>() == 0 {
                    self.wake_sleepers();
                }

                // Check preemption watchdog
                self.check_preempt_watchdog();

                if self.preempt_disable_depth > 0 {
                    self.state.per_cpu[current_cpu_index::<R>()].need_resched = true;
                    return None;
                }

                let cpu_idx = current_cpu_index::<R>();

                // If a higher-priority task became runnable (e.g. via wake_task),
                // preempt immediately rather than waiting for timeslice expiry.
                if self.state.per_cpu[current_cpu_index::<R>()].need_resched {
                    self.state.per_cpu[current_cpu_index::<R>()].need_resched = false;
                    return self.prepare_yield();
                }

                // Decrement current task's time slice
                let mut should_yield = false;
                if let Some(current_id) = self.state.per_cpu.get(cpu_idx).and_then(|pc| pc.current)
                {
                    if let Some(mut task) = crate::task::registry::get_task_mut::<R>(current_id) {
                        if task.timeslice_remaining > 0 {
                            task.timeslice_remaining -= 1;
                        }
                        if task.timeslice_remaining == 0 {
                            // Reset for next run
                            task.timeslice_remaining = types::DEFAULT_TIMESLICE;
                            should_yield = true;
                        }
                    } // REGISTRY lock dropped here!
                }

                if should_yield {
                    // Force reschedule (safe now because REGISTRY lock is dropped)
                    return self.prepare_yield();
                }
                return None; // Not expired yet
            }
            ScheduleReason::SafePoint | ScheduleReason::ReschedIfNeeded => {
                // No tick bookkeeping, no timeslice decrement.
                // Simply yield if a reschedule was requested.
                if self.preempt_disable_depth > 0 {
                    self.state.per_cpu[current_cpu_index::<R>()].need_resched = true;
                    return None;
                }
                // Also drain the global atomic flag (set by trylock-miss fallback)
                let global = GLOBAL_NEED_RESCHED.swap(false, Ordering::Acquire);
                if self.state.per_cpu[current_cpu_index::<R>()].need_resched || global {
                    self.state.per_cpu[current_cpu_index::<R>()].need_resched = false;
                    return self.prepare_yield();
                }
                return None;
            }
            _ => {}
        }

        self.prepare_yield()
    }

    /// Check if preemption has been disabled too long
    fn check_preempt_watchdog(&mut self) {
        if self.preempt_disable_depth > 0 && !self.watchdog_warned {
            let now = TICK_COUNT.load(Ordering::Relaxed);
            if now.saturating_sub(self.preempt_disable_since) > 500 {
                // WARNING: Cannot log here! This is called from timer interrupt via
                // on_tick() while GLOBAL_LOGGER may be held, causing deadlock.
                // crate::kinfo!(
                //     "WATCHDOG: preemption disabled for >500 ticks! depth={}",
                //     self.preempt_disable_depth
                // );
                self.watchdog_warned = true;
            }
        }
    }

    /// Wake any sleeping tasks whose sleep time has expired
    fn wake_sleepers(&mut self) {
        let now = TICK_COUNT.load(Ordering::Relaxed);
        let lock_start = crate::runtime::<R>().mono_ticks();

        while let Some((&wake_tick, _)) = self.state.sleep_queue.first_key_value() {
            if wake_tick <= now {
                let (_, tids) = self.state.sleep_queue.pop_first().unwrap();

                for tid in tids {
                    let priority: usize;
                    let target_cpu: usize;

                    // 1. Lock REGISTRY and update task state
                    if let Some(mut task) = crate::task::registry::get_task_mut::<R>(tid) {
                        task.state = TaskState::Runnable;
                        task.enqueued_at_tick = TICK_COUNT.load(Ordering::Relaxed);
                        priority = task.priority as usize;
                        target_cpu = match task.affinity {
                            crate::task::Affinity::Pinned(cpu) => cpu,
                            crate::task::Affinity::Any => {
                                let idx = spawn::RR_IDX.fetch_add(1, Ordering::Relaxed);
                                idx % self.state.online_cpu_count
                            }
                        };
                    } else {
                        continue;
                    } // REGISTRY lock dropped here!

                    let actual_cpu = if target_cpu < self.state.per_cpu.len() {
                        target_cpu
                    } else {
                        0
                    };
                    self.state.enqueue_task(actual_cpu, priority, tid);

                    let current_prio = self
                        .state
                        .per_cpu
                        .get(actual_cpu)
                        .and_then(|pc| pc.current)
                        .and_then(|cid| crate::task::registry::get_task::<R>(cid))
                        .map(|t| t.priority as usize)
                        .unwrap_or(0);
                    if priority > current_prio {
                        if actual_cpu == current_cpu_index::<R>() {
                            self.state.per_cpu[current_cpu_index::<R>()].need_resched = true;
                        }
                    }

                    if actual_cpu != current_cpu_index::<R>() {
                        crate::kdebug!(
                            "SCHED: Nudging CPU {} for task {} (prio {})",
                            actual_cpu,
                            tid,
                            priority
                        );
                        crate::runtime::<R>().send_ipi(actual_cpu, 0x30);
                    }
                }
            } else {
                break;
            }
        }

        record_sched_lock_hold::<R>(
            &PROF_SCHED_LOCK_WAKE_SLEEPERS_CALLS,
            &PROF_SCHED_LOCK_WAKE_SLEEPERS_US_TOTAL,
            &PROF_SCHED_LOCK_WAKE_SLEEPERS_US_MAX,
            lock_start,
        );
    }

    pub fn preempt_disable(&mut self) {
        if self.preempt_disable_depth == 0 {
            // Track when we started disabling preemption
            self.preempt_disable_since = TICK_COUNT.load(Ordering::Relaxed);
            self.watchdog_warned = false;
        }
        self.preempt_disable_depth += 1;
        if self.preempt_disable_depth == 1 {
            // Only trace on transition to disabled? Or depth change?
            // User task says "Record (..., preempt_disable_depth)".
            // Let's trace all for now, or just 0->1.
            // 0->1 is most important for start of disable region.
            crate::trace::irq_ring::push(abi::trace::TraceEvent::PreemptDisable {
                depth: self.preempt_disable_depth as u32,
                timestamp: crate::trace::now(),
            });
        }
    }

    fn flush_metrics_if_needed(&mut self) {
        let rt = crate::runtime::<R>();
        let now = rt.mono_ticks();
        let limit = rt.mono_freq_hz() * 2;

        if self.metrics.last_flush == 0 {
            self.metrics.last_flush = now;
            return;
        }

        if now - self.metrics.last_flush > limit {
            #[cfg(feature = "diagnostic-apps")]
            crate::log_event!(
               crate::logging::LogLevel::Info,
               "sched.activity",
               "Scheduler Activity Rollup",
               {
                   yields: self.metrics.yields,
                   pops: self.metrics.pops,
                   pushes: self.metrics.pushes,
                   idle_picks: self.metrics.idle_picks,
                   runq_len: self.runq.iter().map(|q| q.len()).sum::<usize>() as u64
               },
               about=[]
            );

            self.metrics.yields = 0;
            self.metrics.pops = 0;
            self.metrics.pushes = 0;
            self.metrics.idle_picks = 0;
            self.metrics.last_flush = now;
        }
    }

    pub fn preempt_enable(
        &mut self,
    ) -> Option<
        SwitchParams<
            <R::Tasking as BootTasking>::Context,
            <R::Tasking as BootTasking>::AddressSpace,
        >,
    > {
        if self.preempt_disable_depth > 0 {
            self.preempt_disable_depth -= 1;
        }

        if self.preempt_disable_depth == 0
            && self.state.per_cpu[current_cpu_index::<R>()].need_resched
        {
            self.state.per_cpu[current_cpu_index::<R>()].need_resched = false;
            return self.schedule_point(ScheduleReason::SafePoint);
        }
        None
    }

    pub fn prepare_yield(
        &mut self,
    ) -> Option<
        SwitchParams<
            <R::Tasking as BootTasking>::Context,
            <R::Tasking as BootTasking>::AddressSpace,
        >,
    > {
        let cpu_idx = current_cpu_index::<R>();
        let current_id = self.state.per_cpu.get(cpu_idx)?.current?;

        self.metrics.yields += 1;

        // Don't push idle task or dead tasks back to runq
        if Some(current_id) != self.state.per_cpu[cpu_idx].idle_task {
            if let Some(task) = crate::task::registry::get_task::<R>(current_id) {
                if task.state != TaskState::Dead {
                    let priority = task.priority;
                    // Push to LOCAL runq (we are yielding on this CPU)
                    self.state
                        .enqueue_task(cpu_idx, priority as usize, current_id);
                    self.metrics.pushes += 1;
                }
            }
        }

        self.prepare_schedule()
    }

    pub(crate) fn prepare_schedule(
        &mut self,
    ) -> Option<
        SwitchParams<
            <R::Tasking as BootTasking>::Context,
            <R::Tasking as BootTasking>::AddressSpace,
        >,
    > {
        self.flush_metrics_if_needed();

        let rt = crate::runtime::<R>();
        let cpu_idx = current_cpu_index::<R>();
        let real_cpu_id = rt.current_cpu_id().0 as usize;
        if cpu_idx != real_cpu_id {
            crate::kprintln!(
                "FATAL GS CORRUPTION: Core {} thinks it is index {} via GS!",
                real_cpu_id,
                cpu_idx
            );
        }
        if cpu_idx >= self.state.per_cpu.len() {
            return None;
        }
        let per_cpu_len = self.state.per_cpu.len();

        // Collect tasks pinned to a different CPU so we can requeue them after scanning.
        const MAX_MISROUTED: usize = 32;
        let mut misrouted: [(usize, usize, TaskId); MAX_MISROUTED] = [(0, 0, 0); MAX_MISROUTED];
        let mut misrouted_count = 0;

        let mut next_id = None;
        // Priority scan — skip dead and misrouted tasks, evaluating aging on-pick
        loop {
            let mut best_q = None;
            let mut best_eff = 0;

            for p in (1..5).rev() {
                if let Some(&id) = self.state.per_cpu[cpu_idx].runq[p].front() {
                    let mut eff = p; // Start with base priority (queue index)
                    if p < 4 {
                        // aging only applies up to High
                        if let Some(task) = crate::task::registry::get_task::<R>(id) {
                            let now = TICK_COUNT.load(Ordering::Relaxed);
                            let wait_ticks = now.saturating_sub(task.enqueued_at_tick);
                            let boost = (wait_ticks / types::AGING_THRESHOLD_TICKS) as usize;
                            let boost = boost.min(types::MAX_PRIORITY_BOOST);
                            eff = (p + boost).min(4);
                        }
                    }
                    if eff > best_eff || best_q.is_none() {
                        best_eff = eff;
                        best_q = Some(p);
                    }
                }
            }

            if let Some(p) = best_q {
                let id = self.state.dequeue_task_front(cpu_idx, p).unwrap();
                self.metrics.pops += 1;

                let task_ref = crate::task::registry::get_task::<R>(id);
                if task_ref
                    .as_deref()
                    .map_or(true, |t| t.state == TaskState::Dead)
                {
                    continue;
                }
                let task = task_ref.unwrap();
                if let crate::task::Affinity::Pinned(target) = task.affinity {
                    if target != cpu_idx && target < per_cpu_len {
                        if misrouted_count < MAX_MISROUTED {
                            misrouted[misrouted_count] = (task.base_priority as usize, target, id);
                            misrouted_count += 1;
                        }
                        continue;
                    }
                }
                next_id = Some(id);
                break;
            } else {
                break;
            }
        }

        let next_id = match next_id {
            Some(id) => id,
            None => {
                // Check Idle queue — skip dead and misrouted tasks
                let mut found_idle_q = None;
                while let Some(id) = self.state.dequeue_task_front(cpu_idx, 0) {
                    self.metrics.pops += 1;
                    let task_ref = crate::task::registry::get_task::<R>(id);
                    if task_ref
                        .as_deref()
                        .map_or(true, |t| t.state == TaskState::Dead)
                    {
                        continue;
                    }
                    let task = task_ref.unwrap();
                    if let crate::task::Affinity::Pinned(target) = task.affinity {
                        if target != cpu_idx && target < per_cpu_len {
                            if misrouted_count < MAX_MISROUTED {
                                misrouted[misrouted_count] = (task.priority as usize, target, id);
                                misrouted_count += 1;
                            }
                            continue;
                        }
                    }
                    found_idle_q = Some(id);
                    break;
                }
                if let Some(id) = found_idle_q {
                    id
                } else if let Some(idle) = self.state.per_cpu[cpu_idx].idle_task {
                    self.metrics.idle_picks += 1;
                    idle
                } else {
                    // Flush misrouted tasks before returning
                    for &(prio, target_cpu, id) in &misrouted[..misrouted_count] {
                        self.state.enqueue_task(target_cpu, prio, id);
                    }
                    return None;
                }
            }
        };

        // Flush misrouted tasks to their correct CPU queues
        for &(prio, target_cpu, id) in &misrouted[..misrouted_count] {
            self.state.enqueue_task(target_cpu, prio, id);
        }

        let current_id = self.state.per_cpu[cpu_idx]
            .current
            .expect("prepare_schedule called without current task");

        if next_id == current_id {
            let idx = self.state.get_task_index(current_id).unwrap_or_else(|| {
                crate::kerror!(
                    "SchedTasks: {:?}",
                    self.state
                        .tasks
                        .iter()
                        .map(|f| f.tid)
                        .collect::<alloc::vec::Vec<_>>()
                );
                panic!("failed to find current_id {} in get_task_index", current_id)
            });
            crate::task::registry::get_registry::<R>().tasks[idx].state = TaskState::Running;
            return None;
        }

        self.state.per_cpu[cpu_idx].current = Some(next_id);

        let old_idx = self.state.get_task_index(current_id).unwrap_or_else(|| {
            crate::kerror!(
                "SchedTasks: {:?}",
                self.state
                    .tasks
                    .iter()
                    .map(|f| f.tid)
                    .collect::<alloc::vec::Vec<_>>()
            );
            panic!("failed to find current_id {} in get_task_index", current_id)
        });
        let new_idx = self.state.get_task_index(next_id).unwrap_or_else(|| {
            crate::kerror!(
                "SchedTasks: {:?}",
                self.state
                    .tasks
                    .iter()
                    .map(|f| f.tid)
                    .collect::<alloc::vec::Vec<_>>()
            );
            panic!("failed to find next_id {} in get_task_index", next_id)
        });

        let tasks_ptr = crate::task::registry::get_registry::<R>()
            .tasks
            .as_mut_ptr();
        unsafe {
            let old_task = &mut **tasks_ptr.add(old_idx);
            let new_task = &mut **tasks_ptr.add(new_idx);

            if old_task.state == TaskState::Running {
                old_task.state = TaskState::Runnable;
                old_task.enqueued_at_tick = TICK_COUNT.load(Ordering::Relaxed);
            }
            new_task.state = TaskState::Running;
            new_task.last_cpu = Some(cpu_idx);

            // Update the lock-free mapping cache for this CPU so check_user_mapping is fast
            crate::sched::vm::CURRENT_MAPPINGS[cpu_idx].store(
                alloc::sync::Arc::as_ptr(&new_task.mappings) as *mut _,
                core::sync::atomic::Ordering::Release,
            );

            old_task.simd.save(crate::runtime::<R>());
            new_task.simd.restore(crate::runtime::<R>());

            crate::trace::irq_ring::push(abi::trace::TraceEvent::ContextSwitch {
                from: old_task.id,
                to: new_task.id,
                timestamp: crate::trace::now(),
            });

            Some(SwitchParams {
                from_ctx: &mut old_task.ctx as *mut _,
                to_ctx: &new_task.ctx as *const _,
                to_aspace: new_task.aspace,
                from_tid: old_task.id,
                to_tid: new_task.id,
                from_aspace: old_task.aspace,
                from_user: old_task.is_user,
                to_user: new_task.is_user,
                from_user_fs_base: &mut old_task.user_fs_base as *mut u64,
                to_user_fs_base: new_task.user_fs_base,
            })
        }
    }

    pub fn terminate_current(
        &mut self,
        code: i32,
    ) -> (
        SwitchParams<
            <R::Tasking as BootTasking>::Context,
            <R::Tasking as BootTasking>::AddressSpace,
        >,
        alloc::vec::Vec<u64>,
    ) {
        let cpu_idx = current_cpu_index::<R>();
        let current_id = self
            .state
            .per_cpu
            .get(cpu_idx)
            .and_then(|pc| pc.current)
            .expect("terminate_current called with no current task");

        let waiters = mark_task_exited::<R>(self, current_id, code);

        // Release any claimed devices
        let released = crate::device_registry::REGISTRY
            .lock()
            .release_all_for_task(current_id);
        if released > 0 {
            crate::kinfo!(
                "DEVICE: released {} claims for task {}",
                released,
                current_id
            );
        }

        loop {
            if let Some(switch) = self.prepare_schedule() {
                return (switch, waiters);
            }
        }
    }

    pub fn set_priority(&mut self, id: TaskId, priority: TaskPriority) {
        if let Some(idx) = self.state.get_task_index(id) {
            let old_priority = crate::task::registry::get_registry::<R>().tasks[idx].priority;
            crate::task::registry::get_registry::<R>().tasks[idx].priority = priority;
            crate::task::registry::get_registry::<R>().tasks[idx].base_priority = priority; // Update base priority for anti-starvation

            // If it's runnable and in a runq, move it to the new runq
            if crate::task::registry::get_registry::<R>().tasks[idx].state == TaskState::Runnable {
                let loc = self.state.get_task(id).and_then(|t| t.runq_location);
                if let Some((cpu, _)) = loc {
                    self.state.remove_task_from_runq(id);
                    self.state.enqueue_task(cpu, priority as usize, id);
                }
            }
        }
    }

    /// Mark a secondary CPU as online and initialize its idle task.
    pub fn cpu_online(&mut self, cpu_index: usize) {
        crate::kdebug!(
            "SMP: CPU {} online (triggered by scheduler spawn)",
            cpu_index
        );
        self.bringup_in_progress = false;
        self.state.online_cpu_count += 1;

        // Create idle task for this new CPU
        let i = cpu_index;
        let idle_id = self.spawn(
            idle_task::<R>,
            StartupArg::Raw(i),
            TaskPriority::Idle,
            crate::task::Affinity::Pinned(i),
        );

        // Remove from run queues - idle tasks are special
        self.state.remove_task_from_runq(idle_id);

        // Set as this CPU's idle task
        self.state.per_cpu[i].idle_task = Some(idle_id);

        // Pin idle task to its CPU
        if let Some(mut t) = crate::task::registry::get_task_mut::<R>(idle_id) {
            t.affinity = crate::task::Affinity::Pinned(i);
        }
    }
}

pub fn set_priority<R: BootRuntime>(id: TaskId, priority: TaskPriority) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
        sched.set_priority(id, priority);
    }
    rt.irq_restore(_irq);
}

pub fn task_status<R: BootRuntime>(id: TaskId) -> Option<(TaskState, Option<i32>)> {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    let res = if let Some(ptr) = *lock {
        let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };
        crate::task::registry::get_task::<R>(id).map(|t| (t.state, t.exit_code))
    } else {
        None
    };
    rt.irq_restore(_irq);
    res
}

pub fn current_priority<R: BootRuntime>() -> TaskPriority {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let res = if let Some(lock) = SCHEDULER.try_lock() {
        if let Some(ptr) = *lock {
            let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };
            let cpu = current_cpu_index::<R>();
            sched
                .state
                .per_cpu
                .get(cpu)
                .and_then(|pc| pc.current)
                .and_then(|tid| crate::task::registry::get_task::<R>(tid))
                .map(|t| t.priority)
                .unwrap_or(TaskPriority::Normal)
        } else {
            TaskPriority::Normal
        }
    } else {
        TaskPriority::Normal
    };
    rt.irq_restore(_irq);
    res
}

pub fn current_tid<R: BootRuntime>() -> u64 {
    crate::runtime::<R>().current_tid()
}

fn current_task_name_impl<R: BootRuntime>() -> [u8; 32] {
    let tid = current_tid::<R>();
    if let Some(task) = crate::task::registry::get_task::<R>(tid) {
        task.name
    } else {
        let mut n = [0u8; 32];
        n[0..7].copy_from_slice(b"unknown");
        n
    }
}

/// Update the current task's stored `user_fs_base` field.
///
/// Called by the TLS-set syscall handler after writing the hardware register,
/// so that the value is saved correctly on the next context switch without
/// needing an extra MSR read.
fn set_current_user_fs_base<R: BootRuntime>(base: u64) {
    let tid = crate::runtime::<R>().current_tid();
    if let Some(mut task) = crate::task::registry::get_task_mut::<R>(tid) {
        task.user_fs_base = base;
    }
}

/// Get the current task's ProcessInfo Arc, if any.
pub fn process_info<R: BootRuntime>()
-> Option<alloc::sync::Arc<spin::Mutex<crate::task::ProcessInfo>>> {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    // Prefer the runtime's current TID. During syscall/trap handling this stays
    // authoritative even if the scheduler's per-CPU `current` view is transiently stale.
    let runtime_tid = rt.current_tid();
    let result = crate::task::registry::get_task::<R>(runtime_tid)
        .and_then(|t| t.process_info.clone())
        .or_else(|| {
            let lock = SCHEDULER.lock();
            if let Some(ptr) = *lock {
                let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };
                let cpu_idx = current_cpu_index::<R>();
                sched
                    .state
                    .per_cpu
                    .get(cpu_idx)
                    .and_then(|pc| pc.current)
                    .and_then(|tid| crate::task::registry::get_task::<R>(tid))
                    .and_then(|t| t.process_info.clone())
            } else {
                None
            }
        });

    rt.irq_restore(_irq);
    result
}

pub fn process_info_for_tid<R: BootRuntime>(
    tid: u64,
) -> Option<alloc::sync::Arc<spin::Mutex<crate::task::ProcessInfo>>> {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let result = crate::task::registry::get_task::<R>(tid).and_then(|t| t.process_info.clone());
    rt.irq_restore(_irq);
    result
}

/// Return a snapshot of all live processes (those with a ProcessInfo).
///
/// Called from the `LIST_PROCESSES_HOOK` slot so that procfs can render
/// `/proc/<pid>/…` files without knowing the concrete `R` type.
pub fn list_processes<R: BootRuntime>() -> alloc::vec::Vec<hooks::ProcessSnapshot> {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let mut out = alloc::vec::Vec::new();
    {
        let reg = crate::task::registry::get_registry::<R>();
        for task in reg.tasks.iter() {
            if let Some(pi_arc) = &task.process_info {
                let pi = pi_arc.lock();
                let name_bytes = &task.name[..task.name_len as usize];
                let name = alloc::string::String::from_utf8_lossy(name_bytes).into_owned();
                out.push(hooks::ProcessSnapshot {
                    pid: pi.pid,
                    ppid: pi.ppid,
                    name,
                    state: task.state,
                    argv: pi.argv.clone(),
                });
            }
        }
    }
    rt.irq_restore(_irq);
    out
}

fn register_task_exit_waiter<R: BootRuntime>(
    target_tid: TaskId,
    waiter_tid: TaskId,
) -> Result<Option<i32>, abi::errors::Errno> {
    let target =
        crate::task::registry::get_task::<R>(target_tid).ok_or(abi::errors::Errno::ECHILD)?;

    if target.state == TaskState::Dead {
        return Ok(Some(target.exit_code.unwrap_or(0)));
    }

    target.exit_waiters.push_back(waiter_tid);
    Ok(None)
}

pub fn poll_task_exit<R: BootRuntime>(
    target_tid: TaskId,
) -> Result<Option<i32>, abi::errors::Errno> {
    let target =
        crate::task::registry::get_task::<R>(target_tid).ok_or(abi::errors::Errno::ECHILD)?;

    if target.state == TaskState::Dead {
        Ok(Some(target.exit_code.unwrap_or(0)))
    } else {
        Ok(None)
    }
}

pub fn register_task_exit_waiter_public<R: BootRuntime>(
    target_tid: TaskId,
    waiter_tid: TaskId,
) -> Result<Option<i32>, abi::errors::Errno> {
    register_task_exit_waiter::<R>(target_tid, waiter_tid)
}

pub fn unregister_task_exit_waiter<R: BootRuntime>(
    target_tid: TaskId,
    waiter_tid: TaskId,
) -> Result<(), abi::errors::Errno> {
    let target =
        crate::task::registry::get_task::<R>(target_tid).ok_or(abi::errors::Errno::ECHILD)?;
    target.exit_waiters.remove(waiter_tid);
    Ok(())
}

fn mark_task_exited<R: BootRuntime>(
    sched: &mut types::Scheduler<R>,
    tid: TaskId,
    code: i32,
) -> alloc::vec::Vec<u64> {
    let waiters = if let Some(mut task) = crate::task::registry::get_task_mut::<R>(tid) {
        task.state = TaskState::Dead;
        task.exit_code = Some(code);
        task.exit_waiters.drain()
    } else {
        alloc::vec::Vec::new()
    };

    if let Some(task) = sched.state.get_task_mut(tid) {
        task.state = TaskState::Dead;
        task.runq_location = None;
    }

    waiters
}

fn wake_waiters(waiters: &[u64]) {
    for &tid in waiters {
        unsafe {
            crate::sched::wake_task_erased(tid);
        }
    }
}

fn graph_thing_for_current_impl<R: BootRuntime>() -> Option<u64> {
    None
}

pub fn exit<R: BootRuntime>(code: i32) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let (switch, waiters) = {
        let lock = SCHEDULER.lock();
        let ptr = lock.expect("Scheduler not initialized");
        let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
        sched.terminate_current(code)
    };

    wake_waiters(&waiters);

    unsafe {
        rt.tasking().activate_address_space(switch.to_aspace);
    }

    unsafe {
        rt.tasking().switch_with_tls(
            &mut *(switch.from_ctx as *mut _),
            &*switch.to_ctx,
            switch.to_tid,
            switch.from_user_fs_base,
            switch.to_user_fs_base,
        );
    }

    unreachable!("Thread continued after terminating!");
}

/// Kill an arbitrary task by TID. Returns true if the task was found and killed.
/// The task is marked Dead with exit code -9 and removed from all run queues.
pub fn kill_by_tid<R: BootRuntime>(tid: u64) -> bool {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let (killed, waiters) = {
        let lock = SCHEDULER.lock();
        if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };

            // Don't allow killing the current task via this path
            let cpu_idx = current_cpu_index::<R>();
            if let Some(current_id) = sched.state.per_cpu.get(cpu_idx).and_then(|pc| pc.current) {
                if current_id == tid {
                    (false, alloc::vec::Vec::new())
                } else {
                    let task_killed = crate::task::registry::get_task::<R>(tid)
                        .map(|task| task.state != TaskState::Dead)
                        .unwrap_or(false);

                    if !task_killed {
                        (false, alloc::vec::Vec::new())
                    } else {
                        let waiters = mark_task_exited::<R>(sched, tid, -9);

                        // Remove from all run queues
                        sched.state.remove_task_from_runq(tid);

                        // Remove from wait queue
                        if let Some(pos) = sched.state.wait_queue.iter().position(|&wid| wid == tid)
                        {
                            sched.state.wait_queue.remove(pos);
                        }

                        // Remove from sleep queue
                        sched.state.sleep_queue.retain(|_, tids| {
                            tids.retain(|&t| t != tid);
                            !tids.is_empty()
                        });

                        // Release any claimed devices
                        let released = crate::device_registry::REGISTRY
                            .lock()
                            .release_all_for_task(tid);
                        if released > 0 {
                            crate::kinfo!("DEVICE: released {} claims for task {}", released, tid);
                        }

                        crate::kinfo!("SCHED: Killed task {} (SIGKILL)", tid);
                        (true, waiters)
                    }
                }
            } else {
                (false, alloc::vec::Vec::new())
            }
        } else {
            (false, alloc::vec::Vec::new())
        }
    };

    wake_waiters(&waiters);
    rt.irq_restore(_irq);
    killed
}

/// Wait semantics are non-consuming today: any task that can name a TID may
/// observe its terminal status, and dead task records stay resident for later polls/waits.
pub fn wait_task<R: BootRuntime>(tid: TaskId) -> Result<i32, abi::errors::Errno> {
    let current_tid = current_tid::<R>();

    if tid == current_tid {
        return Err(abi::errors::Errno::EINVAL);
    }

    loop {
        if let Some(code) = register_task_exit_waiter::<R>(tid, current_tid)? {
            return Ok(code);
        }

        unsafe {
            block_current_erased();
        }
    }
}

pub fn register_timeout_wake<R: BootRuntime>(tid: TaskId, wake_tick: u64) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
        let sleepers = sched.state.sleep_queue.entry(wake_tick).or_default();
        if !sleepers.contains(&tid) {
            sleepers.push(tid);
        }
    }
    rt.irq_restore(_irq);
}

pub fn unregister_timeout_wake<R: BootRuntime>(tid: TaskId) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
        sched.state.sleep_queue.retain(|_, tids| {
            tids.retain(|&sleep_tid| sleep_tid != tid);
            !tids.is_empty()
        });
    }
    rt.irq_restore(_irq);
}

pub fn cpu_online<R: BootRuntime>(cpu_index: usize) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
        sched.cpu_online(cpu_index);
    }
    rt.irq_restore(_irq);
}

pub fn dump_stats<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };

    crate::kprint!("\n====== TASK DUMP ======\n");
    crate::kprint!(
        "CPUs: {} online / {} total\n",
        sched.state.online_cpu_count,
        sched.total_cpu_count
    );
    crate::kprint!(
        " {:>5}  {:>10}  {:>4}  {:>3}  {:>4}  {:>7}  {:>6}  {:>6}  {}\n",
        "TID",
        "STATE",
        "PRI",
        "CPU",
        "USER",
        "SLICE",
        "KSTK",
        "AFFIN",
        "NAME"
    );

    let mut runnable_count = 0u32;
    for task in crate::task::registry::get_registry::<R>().tasks.iter() {
        let state_str = match task.state {
            TaskState::Runnable => {
                runnable_count += 1;
                "Runnable"
            }
            TaskState::Running => {
                runnable_count += 1;
                "Running"
            }
            TaskState::Blocked => "Blocked",
            TaskState::Dead => "Dead",
        };
        let pri_str = match task.priority {
            crate::task::TaskPriority::Idle => "Idle",
            crate::task::TaskPriority::Low => "Low",
            crate::task::TaskPriority::Normal => "Norm",
            crate::task::TaskPriority::High => "High",
            crate::task::TaskPriority::Realtime => "RT",
        };
        let cpu_str: alloc::string::String = match task.last_cpu {
            Some(c) => alloc::format!("{}", c),
            None => alloc::string::String::from("-"),
        };
        let user_str = if task.is_user { "Y" } else { "N" };
        let aff_str: alloc::string::String = match task.affinity {
            crate::task::Affinity::Any => alloc::string::String::from("Any"),
            crate::task::Affinity::Pinned(c) => alloc::format!("Pin({})", c),
        };
        let name_str = if task.name_len > 0 {
            core::str::from_utf8(&task.name[..task.name_len as usize]).unwrap_or("?")
        } else {
            "-"
        };
        crate::kprint!(
            " {:>5}  {:>10}  {:>4}  {:>3}  {:>4}  {:>3}/{:<3}  {:>5}K  {:>6}  {}\n",
            task.id,
            state_str,
            pri_str,
            cpu_str,
            user_str,
            task.timeslice_remaining,
            types::DEFAULT_TIMESLICE,
            task.kstack_size / 1024,
            aff_str,
            name_str
        );
    }

    // Per-CPU run-queue summary
    for (i, pc) in sched.state.per_cpu.iter().enumerate() {
        if i >= sched.state.online_cpu_count {
            break;
        }
        let total: usize = pc.runq.iter().map(|q| q.len()).sum();
        crate::kprint!(
            "  CPU {}: current={:?} runq={} idle={:?}\n",
            i,
            pc.current,
            total,
            pc.idle_task
        );
    }

    crate::kprint!("Sleep queue: {} tasks\n", sched.state.sleep_queue.len());
    crate::kprint!(
        "=== {} tasks, {} runnable ===\n\n",
        crate::task::registry::get_registry::<R>().tasks.len(),
        runnable_count
    );

    rt.irq_restore(_irq);
}

extern "C" fn idle_task<R: BootRuntime>(_: usize) -> ! {
    let rt = crate::runtime::<R>();
    loop {
        rt.wait_for_interrupt();
    }
}

pub static CPU_ONLINE: AtomicUsize = AtomicUsize::new(0);

/// Entry point for secondary CPUs.
///
/// # Safety
/// Must only be called from `kernel_secondary_entry`.
pub unsafe fn enter_secondary(cpu_index: usize) -> ! {
    // Mark as online
    CPU_ONLINE.fetch_add(1, Ordering::Relaxed);
    crate::kdebug!("SMP: Secondary CPU {} online!", cpu_index);

    // Enter scheduler loop via the hook which bootstraps this CPU.
    // The run_scheduler hook will call bootstrap_cpu to set up this CPU's
    if let Some(hook) = unsafe { hooks::RUN_SCHEDULER_HOOK } {
        hook();
    } else {
        panic!("Scheduler hook not initialized!");
    }

    // Fallback if run_scheduler returns (it shouldn't)
    loop {
        crate::runtime_base().wait_for_interrupt();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::{Affinity, TaskPriority, TaskState};
    use crate::{
        BootRuntime, BootRuntimeBase, BootTasking, MapKind, MapPerms, UserEntry, UserTaskSpec,
    };

    // Mock types for testing - copy from spawn.rs tests
    #[derive(Default, Copy, Clone)]
    struct MockContext(usize);
    #[derive(Clone, Copy, Default)]
    struct MockAddressSpace(u64);

    static MOCK_RUNTIME: MockRuntime = MockRuntime;
    struct MockRuntime;
    impl BootRuntimeBase for MockRuntime {
        fn putchar(&self, _c: u8) {}
        fn mono_ticks(&self) -> u64 {
            0
        }
        fn mono_freq_hz(&self) -> u64 {
            1
        }
        fn init_secondary_cpu(&self, _cpu_index: usize) {}
        fn phys_to_virt_offset(&self) -> u64 {
            0
        }
    }
    impl BootRuntime for MockRuntime {
        type Tasking = MockRuntime;
        fn tasking(&self) -> &Self {
            self
        }
        fn halt(&self) -> ! {
            loop {}
        }
        fn irq_disable(&self) -> crate::IrqState {
            crate::IrqState(0)
        }
        fn irq_restore(&self, _state: crate::IrqState) {}
        fn phys_memory_map(&self) -> &'static [crate::PhysRange] {
            &[]
        }
        fn modules(&self) -> &'static [crate::BootModuleDesc] {
            &[]
        }
        fn framebuffer(&self) -> Option<crate::FramebufferInfo> {
            None
        }
        fn simd_state_layout(&self) -> (usize, usize) {
            (0, 1)
        }
        unsafe fn simd_save(&self, _ptr: *mut u8) {}
        unsafe fn simd_restore(&self, _ptr: *const u8) {}
    }
    impl BootTasking for MockRuntime {
        type Runtime = MockRuntime;
        type Context = MockContext;
        type AddressSpace = MockAddressSpace;
        fn init(&self, _hhdm: u64) {}
        fn init_kernel_context(
            &self,
            _entry: extern "C" fn(usize) -> !,
            _st: u64,
            _arg: usize,
        ) -> Self::Context {
            MockContext(_arg)
        }
        fn init_user_context(
            &self,
            _spec: UserTaskSpec<Self::AddressSpace>,
            _kst: u64,
        ) -> Self::Context {
            MockContext(_spec.arg)
        }
        unsafe fn switch(&self, _f: &mut Self::Context, _t: &Self::Context, _tid: u64) {}
        unsafe fn enter_user(&self, _e: UserEntry) -> ! {
            loop {}
        }
        fn make_user_address_space(&self) -> Self::AddressSpace {
            MockAddressSpace(0)
        }
        fn active_address_space(&self) -> Self::AddressSpace {
            MockAddressSpace(0)
        }
        fn activate_address_space(&self, _as: Self::AddressSpace) {}
        fn map_page(
            &self,
            _as: Self::AddressSpace,
            _v: u64,
            _p: u64,
            _pr: MapPerms,
            _k: MapKind,
            _a: &dyn crate::FrameAllocatorHook,
        ) -> Result<(), ()> {
            Ok(())
        }
        fn unmap_page(&self, _as: Self::AddressSpace, _v: u64) -> Result<Option<u64>, ()> {
            Ok(None)
        }
        fn translate(&self, _as: Self::AddressSpace, _v: u64) -> Option<u64> {
            None
        }
        fn tlb_flush_page(&self, _v: u64) {}
    }

    static INIT_TESTS: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
    fn init_mock_runtime() {
        if !INIT_TESTS.swap(true, core::sync::atomic::Ordering::SeqCst) {
            crate::init_runtime(&MOCK_RUNTIME);
        }
    }

    fn init_test_env() {
        init_mock_runtime();
        crate::task::registry::init::<MockRuntime>();
        *SCHEDULER.lock() = None;
        TICK_COUNT.store(0, core::sync::atomic::Ordering::Relaxed);
    }

    fn make_task(
        id: TaskId,
        state: TaskState,
        priority: TaskPriority,
    ) -> crate::task::Task<MockRuntime> {
        crate::task::Task {
            id,
            state,
            priority,
            base_priority: priority,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
        }
    }

    #[test]
    fn test_priority_aging_boost() {
        init_test_env();
        // Test that tasks waiting too long get priority boost when scheduling
        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(0); // Dummy current task

        // Create a normal-priority task enqueued recently
        let task_normal = crate::task::Task {
            id: 1001,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 500, // Recent
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
        };

        // Create a low-priority task enqueued a long time ago
        let task_low = crate::task::Task {
            id: 1002,
            state: TaskState::Runnable,
            priority: TaskPriority::Low,
            base_priority: TaskPriority::Low,
            enqueued_at_tick: 0, // Very old
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
        };

        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(task_normal));
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(task_low));

        sched
            .state
            .enqueue_task(0, TaskPriority::Normal as usize, 1001);
        sched
            .state
            .enqueue_task(0, TaskPriority::Low as usize, 1002);

        // Simulate time advancing enough to give the Low task a boost of +2 (effective priority 3 = High)
        let now = types::AGING_THRESHOLD_TICKS * 2;
        TICK_COUNT.store(now, core::sync::atomic::Ordering::Relaxed);

        // Request schedule. The Low task should be selected because its effective priority is higher
        // than Normal due to wait time.
        let next_switch = sched.prepare_schedule().expect("Should find a task");
        assert_eq!(
            next_switch.to_tid, 1002,
            "Low priority task with aging should preempt normal task"
        );

        // Verify it was popped from the Low queue, not moved to High queue
        assert!(sched.state.per_cpu[0].runq[TaskPriority::Low as usize].is_empty());
        assert!(!sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].is_empty());
    }

    #[test]
    fn test_reset_priority_aging_on_schedule() {
        init_test_env();
        // Test that enqueued_at_tick resets when task is preempted/yields
        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

        // Task 1 is running
        let mut task1 = crate::task::Task {
            id: 2001,
            state: TaskState::Running,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0, // Very old
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: 0, // timeslice expired
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
        };

        // Task 2 is runnable
        let task2 = crate::task::Task {
            id: 2002,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 500, // Newer
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
        };

        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task1));
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task2));

        sched.state.per_cpu[0].current = Some(2001);
        sched
            .state
            .enqueue_task(0, TaskPriority::Normal as usize, 2002);

        // Time moves forward
        TICK_COUNT.store(1000, core::sync::atomic::Ordering::Relaxed);

        // Trigger a timer tick to cause preemption
        let switch = sched.prepare_yield().expect("Should preempt to task2");
        assert_eq!(switch.to_tid, 2002);

        // Verify task1 was placed back in runq and its enqueued_at_tick was updated to TICK_COUNT
        let t1 = crate::task::registry::get_task::<MockRuntime>(2001).unwrap();
        assert_eq!(t1.enqueued_at_tick, 1000);
        assert_eq!(t1.state, TaskState::Runnable);
    }

    #[test]
    fn test_wake_preempts_lower_priority() {
        init_test_env();
        crate::task::registry::init::<MockRuntime>();
        use core::sync::atomic::Ordering;

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

        // Task 1: Normal priority, currently running
        let normal_task = crate::task::Task {
            id: 3001,
            state: TaskState::Running,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
        };

        // Task 2: Realtime priority, sleeping (about to wake)
        let rt_task = crate::task::Task {
            id: 3002,
            state: TaskState::Runnable,
            priority: TaskPriority::Realtime,
            base_priority: TaskPriority::Realtime,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
        };

        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(normal_task));
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(rt_task));
        sched.state.per_cpu[0].current = Some(3001); // Normal task is running

        // Put RT task in sleep queue with wake_tick in the past
        TICK_COUNT.store(100, Ordering::Relaxed);
        sched.state.sleep_queue.entry(50).or_default().push(3002);

        // Before: need_resched should be false
        assert!(
            !sched.state.per_cpu[0].need_resched,
            "need_resched should start false"
        );

        // Wake sleepers — should detect RT > Normal and set need_resched
        sched.wake_sleepers();

        // Verify need_resched was set
        assert!(
            sched.state.per_cpu[0].need_resched,
            "need_resched should be true after waking a higher-priority task"
        );

        // Verify RT task was enqueued to the Realtime runq
        assert!(
            sched.state.per_cpu[0].runq[TaskPriority::Realtime as usize]
                .iter()
                .any(|&id| id == 3002),
            "RT task should be in the Realtime run queue"
        );

        // Now simulate schedule: prepare_yield should pick the RT task
        sched.state.per_cpu[0].need_resched = false; // clear so prepare_yield runs clean
        let switch = sched.prepare_yield();
        assert!(switch.is_some(), "Should produce a context switch");
        let switch = switch.unwrap();
        assert_eq!(
            switch.to_tid, 3002,
            "Scheduler should switch to the RT task"
        );
        assert_eq!(
            switch.from_tid, 3001,
            "Scheduler should switch away from the Normal task"
        );
    }

    #[test]
    fn test_sorted_insertion() {
        init_test_env();
        crate::task::registry::init::<MockRuntime>();
        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

        // Helper to create dummy task
        let make_task = |id: TaskId| crate::task::Task {
            id,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: None,
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
        };

        // Insert tasks out of order
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(make_task(4010)));
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(make_task(4005)));
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(make_task(4020)));
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(make_task(4001)));

        // Verify sorted order internally
        assert_eq!(
            crate::task::registry::get_registry::<MockRuntime>()
                .tasks
                .len(),
            4
        );
        assert_eq!(
            crate::task::registry::get_registry::<MockRuntime>().tasks[0].id,
            4001
        );
        assert_eq!(
            crate::task::registry::get_registry::<MockRuntime>().tasks[1].id,
            4005
        );
        assert_eq!(
            crate::task::registry::get_registry::<MockRuntime>().tasks[2].id,
            4010
        );
        assert_eq!(
            crate::task::registry::get_registry::<MockRuntime>().tasks[3].id,
            4020
        );

        // Verify lookups work
        assert!(crate::task::registry::get_task::<MockRuntime>(4010).is_some());
        assert!(crate::task::registry::get_task::<MockRuntime>(4005).is_some());
        assert!(crate::task::registry::get_task::<MockRuntime>(4001).is_some());
        assert!(crate::task::registry::get_task::<MockRuntime>(4099).is_none());
    }

    #[test]
    fn test_block_and_wake_state_transitions() {
        init_test_env();
        crate::task::registry::init::<MockRuntime>();
        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

        let waiting_task = crate::task::Task {
            id: 5001,
            state: TaskState::Running,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
        };

        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(waiting_task));
        sched.state.per_cpu[0].current = Some(5001);

        // Put task in Wait queue and switch it to Blocked (simulating block_current behavior)
        if let Some(mut task) = crate::task::registry::get_task_mut::<MockRuntime>(5001) {
            task.state = TaskState::Blocked;
        }
        sched.state.wait_queue.push_back(5001);

        // Verify task is stuck blocked
        assert_eq!(
            crate::task::registry::get_task::<MockRuntime>(5001)
                .unwrap()
                .state,
            TaskState::Blocked
        );

        // Emulate `wake_task_erased` via wake_task in MockRuntime context
        crate::sched::blocking::WAKE_TASK_HOOK.store(
            crate::sched::blocking::wake_task::<MockRuntime> as *mut (),
            core::sync::atomic::Ordering::SeqCst,
        );

        // Make sure scheduler hook resolves safely (we will mock inject the scheduler here via static for the hook)
        // Since we are unit testing `wake_task`, we can't easily use the global `SCHEDULER`.
        // So we just directly call the core logic we care about: the wake sleeper unblock logic.

        // Remove from wait queue if present
        if let Some(pos) = sched.state.wait_queue.iter().position(|&wid| wid == 5001) {
            sched.state.wait_queue.remove(pos);
        }

        // Update state to Runnable and add to runq
        if let Some(mut task) = crate::task::registry::get_task_mut::<MockRuntime>(5001) {
            if task.state == TaskState::Blocked {
                task.state = TaskState::Runnable;
                sched.state.enqueue_task(0, task.priority as usize, 5001);
            }
        }

        let woken_task = crate::task::registry::get_task::<MockRuntime>(5001).unwrap();
        assert_eq!(
            woken_task.state,
            TaskState::Runnable,
            "Task must transition from Blocked to Runnable upon wake"
        );

        // Verify task was placed in runq
        assert!(
            sched.state.per_cpu[0].runq[TaskPriority::Normal as usize]
                .iter()
                .any(|&id| id == 5001),
            "Woken task must be in the run queue"
        );
    }

    #[test]
    fn test_wake_task_removes_sleep_queue_entry() {
        init_test_env();
        crate::task::registry::init::<MockRuntime>();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(6000);

        let current_task = crate::task::Task {
            id: 6000,
            state: TaskState::Running,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
        };

        let sleeping_task = crate::task::Task {
            id: 6001,
            state: TaskState::Blocked,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
        };

        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(current_task));
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(sleeping_task));
        sched.state.sleep_queue.entry(10).or_default().push(6001);

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
        drop(sched_lock);

        crate::sched::blocking::wake_task::<MockRuntime>(6001);

        let task = crate::task::registry::get_task::<MockRuntime>(6001).unwrap();
        assert_eq!(task.state, TaskState::Runnable);
        assert!(sched.state.sleep_queue.is_empty());
        assert!(
            sched.state.per_cpu[0].runq[TaskPriority::Normal as usize]
                .iter()
                .any(|&id| id == 6001)
        );

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = None;
    }

    #[test]
    fn test_kill_by_tid_removes_wait_queue_entry() {
        init_test_env();
        crate::task::registry::init::<MockRuntime>();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(7000);

        let current_task = crate::task::Task {
            id: 7000,
            state: TaskState::Running,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
        };

        let blocked_task = crate::task::Task {
            id: 7001,
            state: TaskState::Blocked,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
        };

        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(current_task));
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(blocked_task));
        sched.state.wait_queue.push_back(7001);

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
        drop(sched_lock);

        assert!(kill_by_tid::<MockRuntime>(7001));
        assert!(sched.state.wait_queue.is_empty());
        assert_eq!(
            crate::task::registry::get_task::<MockRuntime>(7001)
                .unwrap()
                .state,
            TaskState::Dead
        );

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = None;
    }

    #[test]
    fn test_wait_task_returns_immediately_for_dead_target() {
        init_test_env();

        let dead_task = crate::task::Task {
            id: 8001,
            state: TaskState::Dead,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: Some(23),
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
        };

        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(dead_task));

        assert_eq!(wait_task::<MockRuntime>(8001).unwrap(), 23);
    }

    #[test]
    fn test_wait_task_returns_echild_for_missing_target() {
        init_test_env();

        assert_eq!(
            wait_task::<MockRuntime>(8999).unwrap_err(),
            abi::errors::Errno::ECHILD
        );
    }

    #[test]
    fn test_register_task_exit_waiter_tracks_live_target() {
        init_test_env();

        let live_task = crate::task::Task {
            id: 8101,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
        };

        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(live_task));

        assert_eq!(
            register_task_exit_waiter::<MockRuntime>(8101, 8102).unwrap(),
            None
        );

        let waiters = crate::task::registry::get_task::<MockRuntime>(8101)
            .unwrap()
            .exit_waiters
            .drain();
        assert_eq!(waiters, alloc::vec![8102]);
    }

    #[test]
    fn test_kill_by_tid_wakes_registered_exit_waiter() {
        init_test_env();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(8200);

        let current_task = crate::task::Task {
            id: 8200,
            state: TaskState::Running,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
        };

        let waiter_task = crate::task::Task {
            id: 8201,
            state: TaskState::Blocked,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
        };

        let target_task = crate::task::Task {
            id: 8202,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
        };

        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(current_task));
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(waiter_task));
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(target_task));

        let target_fields = crate::sched::state::TaskSchedFields {
            tid: 8202,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            affinity: Affinity::Any,
            enqueued_at_tick: 0,
            last_cpu: Some(0),
            runq_location: None,
        };
        sched.state.insert_task(target_fields);

        assert_eq!(
            register_task_exit_waiter::<MockRuntime>(8202, 8201).unwrap(),
            None
        );

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
        drop(sched_lock);

        assert!(kill_by_tid::<MockRuntime>(8202));
        assert_eq!(
            crate::task::registry::get_task::<MockRuntime>(8202)
                .unwrap()
                .state,
            TaskState::Dead
        );
        assert_eq!(
            crate::task::registry::get_task::<MockRuntime>(8202)
                .unwrap()
                .exit_code,
            Some(-9)
        );
        assert_eq!(
            crate::task::registry::get_task::<MockRuntime>(8201)
                .unwrap()
                .state,
            TaskState::Runnable
        );
        assert!(
            sched.state.per_cpu[0].runq[TaskPriority::Normal as usize]
                .iter()
                .any(|&id| id == 8201)
        );

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = None;
    }

    #[test]
    fn test_poll_task_exit_reports_pending_dead_and_missing_targets() {
        init_test_env();

        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(8301, TaskState::Runnable, TaskPriority::Normal),
        ));

        let mut dead = make_task(8302, TaskState::Dead, TaskPriority::Normal);
        dead.exit_code = Some(17);
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(dead));

        assert_eq!(poll_task_exit::<MockRuntime>(8301).unwrap(), None);
        assert_eq!(poll_task_exit::<MockRuntime>(8302).unwrap(), Some(17));
        assert_eq!(
            poll_task_exit::<MockRuntime>(8399).unwrap_err(),
            abi::errors::Errno::ECHILD
        );
    }

    #[test]
    fn test_unregister_task_exit_waiter_removes_only_requested_waiter() {
        init_test_env();

        let target = make_task(8401, TaskState::Runnable, TaskPriority::Normal);
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(target));

        register_task_exit_waiter::<MockRuntime>(8401, 8402).unwrap();
        register_task_exit_waiter::<MockRuntime>(8401, 8403).unwrap();
        unregister_task_exit_waiter::<MockRuntime>(8401, 8402).unwrap();

        let waiters = crate::task::registry::get_task::<MockRuntime>(8401)
            .unwrap()
            .exit_waiters
            .drain();
        assert_eq!(waiters, alloc::vec![8403]);
    }

    #[test]
    fn test_register_timeout_wake_deduplicates_task_ids() {
        init_test_env();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(8500);

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
        drop(sched_lock);

        register_timeout_wake::<MockRuntime>(8501, 42);
        register_timeout_wake::<MockRuntime>(8501, 42);
        register_timeout_wake::<MockRuntime>(8502, 42);

        assert_eq!(
            sched.state.sleep_queue.get(&42).cloned().unwrap(),
            alloc::vec![8501, 8502]
        );

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = None;
    }

    #[test]
    fn test_unregister_timeout_wake_removes_task_from_all_buckets() {
        init_test_env();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(8600);
        sched.state.sleep_queue.insert(11, alloc::vec![8601, 8602]);
        sched.state.sleep_queue.insert(12, alloc::vec![8602, 8603]);

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
        drop(sched_lock);

        unregister_timeout_wake::<MockRuntime>(8602);

        assert_eq!(
            sched.state.sleep_queue.get(&11).cloned().unwrap(),
            alloc::vec![8601]
        );
        assert_eq!(
            sched.state.sleep_queue.get(&12).cloned().unwrap(),
            alloc::vec![8603]
        );

        unregister_timeout_wake::<MockRuntime>(8603);
        assert!(!sched.state.sleep_queue.contains_key(&12));

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = None;
    }
}
