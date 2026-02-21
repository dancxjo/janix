//! Preemptive priority-based scheduler
//!
//! This module is split into focused submodules:
//! - `types`: Core data structures and enums
//! - `blocking`: Task blocking and wake primitives  
//! - `hooks`: Type-erased hook system for callers without generic params
//! - `spawn`: Task and thread spawning
//! - `stack`: User stack allocation and fault handling
//! - `sleep`: Timing and yield functions

pub(crate) mod blocking;
pub(crate) mod graph_queue;
pub(crate) mod graphify;
mod hooks;
mod sleep;
mod spawn;
mod stack;
mod types;
mod vm;
pub(crate) mod wait_queue;

// Re-export all public items
pub use blocking::{
    block_current, block_current_erased, init_blocking_hooks, wake_task, wake_task_erased,
};
pub use hooks::{
    add_user_mapping_current, alloc_user_stack_current, check_user_mapping_current,
    current_priority_current, current_tid_current, dump_stats_current, exit_current,
    get_user_mapping_at_current, graph_thing_for_current, handle_user_stack_fault_current, 
    kill_by_tid_current, process_info_current, remove_user_mappings_current, set_priority_current,
    sleep_ticks_current, spawn_process_current, spawn_process_ex_current,
    spawn_user_thread_current, task_status_current, yield_now_current,
};
pub use sleep::{sleep_ms, sleep_ticks, sleep_until, yield_now};
pub use spawn::{
    spawn, spawn_process, spawn_user_task_full, spawn_user_thread, spawn_with_priority,
    user_thread_trampoline, SpawnExResult, StdioSpec,
};
pub use stack::{alloc_user_stack, handle_stack_fault, map_user_page, map_user_page_perms};
pub use types::{
    DEFAULT_TIMESLICE, ScheduleReason, Scheduler, SleepEntry, StackFaultResult, SwitchParams,
};
pub use wait_queue::WaitQueue;

use crate::task::{StartupArg, Task, TaskId, TaskPriority, TaskState};
use crate::{BootRuntime, BootTasking};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use spin::Mutex;

#[cfg(any(feature = "sched_debug", debug_assertions))]
static SWITCH_LOG_COUNT: AtomicUsize = AtomicUsize::new(0);

pub static SCHEDULER: Mutex<Option<usize>> = Mutex::new(None);

/// Global tick counter for debugging scheduler health
pub static TICK_COUNT: AtomicU64 = AtomicU64::new(0);

static PROF_GRAPH_FLUSH_CALLS: AtomicU64 = AtomicU64::new(0);
static PROF_GRAPH_FLUSH_ITEMS: AtomicU64 = AtomicU64::new(0);
static PROF_GRAPH_FLUSH_US_TOTAL: AtomicU64 = AtomicU64::new(0);
static PROF_GRAPH_FLUSH_SLOW: AtomicU64 = AtomicU64::new(0);
static PROF_GRAPH_FLUSH_MAX_US: AtomicU64 = AtomicU64::new(0);
static PROF_RESCHED_TRYLOCK_MISS: AtomicU64 = AtomicU64::new(0);
static PROF_LAST_LOG_TICKS: AtomicU64 = AtomicU64::new(0);

// Diagnostic counters for IPI delivery chain
pub static DIAG_IPI_SENT: AtomicU64 = AtomicU64::new(0);
pub static DIAG_IPI_HANDLER: AtomicU64 = AtomicU64::new(0);
pub static DIAG_HLT_WAKE: AtomicU64 = AtomicU64::new(0);

/// Lock-skip self-healing: when try_resched_if_needed() fails to acquire
/// the scheduler lock, set this flag so the next safe-point yields.
static GLOBAL_NEED_RESCHED: AtomicBool = AtomicBool::new(false);

/// If trylock misses exceed this count in a 2-second window, emit a warning.
const TRYLOCK_MISS_WARN_THRESHOLD: u64 = 50;

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

/// Called from timer ISR - records tick and triggers reschedule if needed
/// Uses try_resched_if_needed to avoid deadlock when SCHEDULER is held by main code
pub fn on_tick<R: BootRuntime>() {
    TICK_COUNT.fetch_add(1, Ordering::Relaxed);
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

                let cr3_before = rt.debug_active_aspace_root();
                rt.tasking().activate_address_space(switch.to_aspace);
                let cr3_after = rt.debug_active_aspace_root();

                // Note: log_context_switch also uses lock internally but that's OK since we dropped ours
                log_context_switch::<R>(&switch, cr3_before, cr3_after);

                unsafe {
                    rt.tasking()
                        .switch(&mut *switch.from_ctx, &*switch.to_ctx, switch.to_tid);
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

/// Process pending graph work items.
/// MUST be called WITHOUT holding the scheduler lock.
fn flush_graph_queue<R: BootRuntime>() {
    use crate::root::graph_anchors;
    use graph_queue::GraphWork;
    let rt = crate::runtime::<R>();
    let t0 = rt.mono_ticks();

    // Time budget: stop after ~2ms to avoid starving other work.
    let budget_ticks = 2 * rt.mono_freq_hz() / 1000;

    // Get scheduler service ThingId for linking. If not ready yet,
    // keep the queued work for a later flush instead of dropping it.
    let sched_thing = match graph_anchors::scheduler_service() {
        Some(id) => id,
        None => return,
    };

    let work_items = graph_queue::drain_n(32);
    if work_items.is_empty() {
        return;
    }
    let item_count = work_items.len() as u64;

    // We'll collect items to be batched
    let mut batch_items = alloc::vec::Vec::with_capacity(work_items.len());

    for item in work_items {
        // Check time budget before each item
        if rt.mono_ticks().wrapping_sub(t0) > budget_ticks {
            break;
        }

        match item {
            GraphWork::CreateThread {
                tid,
                priority,
                is_user,
                name,
                parent_tid,
            } => {
                // Thread creation uses a single batched IPC call (CreateNode + PropSets + Link).
                if let Some(thing_id) = graphify::do_create_thread_node(
                    tid,
                    priority,
                    is_user,
                    name.as_deref(),
                    sched_thing,
                ) {
                    // Use the separate TASK_GRAPH lock — no SCHEDULER.lock() needed.
                    types::set_graph_thing_for_tid(tid, thing_id);
                    let parent_thing = parent_tid.and_then(|ptid| types::graph_thing_for_tid(ptid));

                    if let Some(parent_thing) = parent_thing {
                        graphify::do_link_parent(thing_id, parent_thing, sched_thing);
                    }
                }
            }
            _ => {
                // For other updates, we need the ThingId first
                let tid = match &item {
                    GraphWork::UpdateState { tid, .. } => *tid,
                    GraphWork::SetExitCode { tid, .. } => *tid,
                    GraphWork::SetPriority { tid, .. } => *tid,
                    GraphWork::SetName { tid, .. } => *tid,
                    GraphWork::SetLocation { tid, .. } => *tid,
                    GraphWork::SetAffinity { tid, .. } => *tid,
                    _ => unreachable!(),
                };

                // Use the separate TASK_GRAPH lock — no SCHEDULER.lock() needed.
                if let Some(id) = types::graph_thing_for_tid(tid) {
                    batch_items.push((id, item));
                }
            }
        }
    }

    // Process all collected updates in a single batch
    if !batch_items.is_empty() {
        graphify::do_flush_batch(&batch_items);
    }

    let elapsed_us = ticks_to_us::<R>(rt.mono_ticks().wrapping_sub(t0));
    PROF_GRAPH_FLUSH_CALLS.fetch_add(1, Ordering::Relaxed);
    PROF_GRAPH_FLUSH_ITEMS.fetch_add(item_count, Ordering::Relaxed);
    PROF_GRAPH_FLUSH_US_TOTAL.fetch_add(elapsed_us, Ordering::Relaxed);
    update_max_u64(&PROF_GRAPH_FLUSH_MAX_US, elapsed_us);
    if elapsed_us >= 5_000 {
        PROF_GRAPH_FLUSH_SLOW.fetch_add(1, Ordering::Relaxed);
        // Per-flush log removed — the 2-second summary captures the same stats
        // without hammering the serial port on every call.
    }
}

fn maybe_log_scheduler_profile<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let now = rt.mono_ticks();
    let period = rt.mono_freq_hz().max(1) * 2;
    let last = PROF_LAST_LOG_TICKS.load(Ordering::Relaxed);
    if last != 0 && now.wrapping_sub(last) < period {
        return;
    }
    if PROF_LAST_LOG_TICKS
        .compare_exchange(last, now, Ordering::Relaxed, Ordering::Relaxed)
        .is_err()
    {
        return;
    }

    let calls = PROF_GRAPH_FLUSH_CALLS.swap(0, Ordering::Relaxed);
    let items = PROF_GRAPH_FLUSH_ITEMS.swap(0, Ordering::Relaxed);
    let total_us = PROF_GRAPH_FLUSH_US_TOTAL.swap(0, Ordering::Relaxed);
    let slow = PROF_GRAPH_FLUSH_SLOW.swap(0, Ordering::Relaxed);
    let max_us = PROF_GRAPH_FLUSH_MAX_US.swap(0, Ordering::Relaxed);
    let trylock_miss = PROF_RESCHED_TRYLOCK_MISS.swap(0, Ordering::Relaxed);

    // Threshold warning: if misses are pathological, emit a diagnostic
    if trylock_miss >= TRYLOCK_MISS_WARN_THRESHOLD {
        crate::kwarn!(
            "SCHED: scheduler lock held too long — {} trylock misses in 2s window (threshold {})",
            trylock_miss,
            TRYLOCK_MISS_WARN_THRESHOLD,
        );
    }
    let q = graph_queue::stats_snapshot();
    let avg_us = if calls > 0 { total_us / calls } else { 0 };

    let ipi_tx = DIAG_IPI_SENT.swap(0, Ordering::Relaxed);
    let ipi_rx = DIAG_IPI_HANDLER.swap(0, Ordering::Relaxed);
    let hlt_w = DIAG_HLT_WAKE.swap(0, Ordering::Relaxed);
    crate::kinfo!(
        "PROF: sched 2s: graph_flush calls={} items={} avg_us={} max_us={} slow={} trylock_miss={} qlen={} q_hwm={} q_drop_state={} q_evict={} ipi_tx={} ipi_rx={} hlt_wake={}",
        calls,
        items,
        avg_us,
        max_us,
        slow,
        trylock_miss,
        q.current_len,
        q.high_water_mark,
        q.dropped_update_state,
        q.evicted_critical,
        ipi_tx,
        ipi_rx,
        hlt_w
    );
}

pub fn init<R: BootRuntime>() {
    crate::kinfo!("  Acquiring scheduler lock...");
    let mut lock = SCHEDULER.lock();
    crate::kinfo!("  Lock acquired, checking if initialized...");
    if lock.is_none() {
        crate::kinfo!("  Allocating scheduler...");
        let sched = alloc::boxed::Box::new(types::Scheduler::<R>::new());
        crate::kinfo!("  Leaking scheduler...");
        let s = alloc::boxed::Box::leak(sched);
        crate::kinfo!("  Initializing boot task...");
        init_boot_task::<R>(s);
        crate::kinfo!("  Storing scheduler pointer...");
        *lock = Some(s as *mut types::Scheduler<R> as usize);
        unsafe {
            hooks::YIELD_HOOK = Some(sleep::yield_now::<R>);
            hooks::EXIT_HOOK = Some(exit::<R>);
            hooks::SPAWN_USER_HOOK = Some(spawn::spawn_user_thread::<R>);
            hooks::SPAWN_PROCESS_HOOK = Some(spawn::spawn_process::<R>);
            hooks::CURRENT_TID_HOOK = Some(current_tid::<R>);
            hooks::TASK_STATUS_HOOK = Some(task_status::<R>);
            hooks::SET_PRIORITY_HOOK = Some(set_priority::<R>);
            hooks::CURRENT_PRIORITY_HOOK = Some(current_priority::<R>);
            hooks::ALLOC_USER_STACK_HOOK = Some(stack::alloc_user_stack::<R>);
            hooks::RUN_SCHEDULER_HOOK = Some(crate::task::run_scheduler::<R>);
            hooks::KILL_BY_TID_HOOK = Some(kill_by_tid::<R>);
            hooks::DUMP_STATS_HOOK = Some(crate::task::dump_stats::<R>);
            crate::memory::set_map_user_page_hook(stack::map_user_page::<R>);
            crate::memory::set_map_user_page_perms_hook(stack::map_user_page_perms::<R>);
            crate::memory::set_unmap_user_page_hook(stack::unmap_user_page::<R>);
            hooks::STACK_FAULT_HOOK = Some(stack::handle_stack_fault::<R>);
            hooks::SLEEP_TICKS_HOOK = Some(sleep::sleep_ticks::<R>);
            hooks::ADD_USER_MAPPING_HOOK = Some(vm::add_user_mapping::<R>);
            hooks::REMOVE_USER_MAPPINGS_HOOK = Some(vm::remove_user_mappings::<R>);
            hooks::CHECK_USER_MAPPING_HOOK = Some(vm::check_user_mapping::<R>);
            hooks::GET_USER_MAPPING_AT_HOOK = Some(vm::get_user_mapping_at::<R>);
            hooks::PROCESS_INFO_HOOK = Some(process_info::<R>);
            hooks::SPAWN_PROCESS_EX_HOOK = Some(spawn::spawn_process_ex::<R>);
            hooks::GRAPH_THING_FOR_CURRENT_HOOK = Some(graph_thing_for_current_impl::<R>);
            crate::memory::set_translate_user_page_hook(vm::translate_user_page::<R>);
        }
        blocking::init_blocking_hooks::<R>();
        graph_queue::init();
        crate::contract!("Scheduler initialized");
    }
}

fn init_boot_task<R: BootRuntime>(sched: &mut types::Scheduler<R>) {
    let rt = crate::runtime::<R>();
    let cpu_total = rt.cpu_total_count();

    // Initialize PerCpu state for all CPUs (initially empty/offline)
    for _ in 0..cpu_total {
        sched.per_cpu.push(types::PerCpu::new());
    }

    sched.total_cpu_count = cpu_total;
    sched.online_cpu_count = 1;

    crate::kinfo!("  Creating boot task...");

    let layout = alloc::alloc::Layout::from_size_align(16384, 16).unwrap();
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
        wait_ticks: 0,
        base_priority: TaskPriority::Normal,
    };
    sched.tasks.push(alloc::boxed::Box::new(task));

    // Boot task runs on CPU 0
    sched.per_cpu[0].current = Some(0);

    // Queue graph node creation for the boot task
    graphify::create_thread_node(0, TaskPriority::Normal as u8, false, Some("boot"), None);
    graphify::update_task_state(0, "running");
    // Link boot task to CPU 0
    graphify::update_task_location(0, 0);

    crate::kinfo!("  Creating idle tasks...");

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
        for q in sched.per_cpu.iter_mut().flat_map(|pc| pc.runq.iter_mut()) {
            if let Some(pos) = q.iter().position(|&id| id == idle_id) {
                q.remove(pos);
            }
        }

        // Set as this CPU's idle task
        sched.per_cpu[i].idle_task = Some(idle_id);

        // Pin idle task to its CPU
        if let Some(t) = sched.tasks.iter_mut().find(|t| t.id == idle_id) {
            (**t).affinity = crate::task::Affinity::Pinned(i);
        }
        graphify::set_affinity_node(idle_id, i);
        graphify::set_name(idle_id, &alloc::format!("idle/{}", i));
    }

    // Spawn graph worker task at normal priority (CPU 0 preferred? or Any)
    crate::kinfo!("  Creating graph worker task...");
    let _graph_worker_id = sched.spawn(
        graph_worker_task::<R>,
        StartupArg::None,
        TaskPriority::Normal,
        crate::task::Affinity::Any,
    );

    crate::kinfo!("  Boot task initialized");
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
                // Wake any sleeping tasks whose time has expired
                self.wake_sleepers();

                // Check preemption watchdog
                self.check_preempt_watchdog();

                if self.preempt_disable_depth > 0 {
                    self.need_resched = true;
                    return None;
                }

                // Increment wait times for starving tasks (anti-starvation mechanism)
                let cpu_idx = current_cpu_index::<R>();
                self.increment_wait_times(cpu_idx);
                
                // Apply priority aging to prevent starvation
                self.apply_priority_aging(cpu_idx);

                // If a higher-priority task became runnable (e.g. via wake_task),
                // preempt immediately rather than waiting for timeslice expiry.
                if self.need_resched {
                    self.need_resched = false;
                    return self.prepare_yield();
                }

                // Decrement current task's time slice
                if let Some(current_id) = self.per_cpu.get(cpu_idx).and_then(|pc| pc.current) {
                    if let Some(task) = self.get_task_mut(current_id) {
                        if task.timeslice_remaining > 0 {
                            task.timeslice_remaining -= 1;
                        }
                        if task.timeslice_remaining == 0 {
                            // Reset for next run
                            task.timeslice_remaining = types::DEFAULT_TIMESLICE;
                            // Force reschedule
                            return self.prepare_yield();
                        }
                    }
                }
                return None; // Not expired yet
            }
            ScheduleReason::SafePoint | ScheduleReason::ReschedIfNeeded => {
                // No tick bookkeeping, no timeslice decrement.
                // Simply yield if a reschedule was requested.
                if self.preempt_disable_depth > 0 {
                    self.need_resched = true;
                    return None;
                }
                // Also drain the global atomic flag (set by trylock-miss fallback)
                let global = GLOBAL_NEED_RESCHED.swap(false, Ordering::Acquire);
                if self.need_resched || global {
                    self.need_resched = false;
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

        let mut i = 0;
        while i < self.sleep_queue.len() {
            if self.sleep_queue[i].wake_tick <= now {
                let entry = self.sleep_queue.swap_remove_back(i).unwrap();

                // Task should wake up - add back to run queue
                if let Ok(task_idx) = self.tasks.binary_search_by_key(&entry.task_id, |t| t.id) {
                    self.tasks[task_idx].state = TaskState::Runnable;
                    let task_ref = &self.tasks[task_idx];
                    let priority = task_ref.priority;
                    let target_cpu = if let crate::task::Affinity::Pinned(cpu) = task_ref.affinity {
                        cpu
                    } else if let Some(last) = task_ref.last_cpu {
                        last
                    } else {
                        let idx = spawn::RR_IDX.fetch_add(1, Ordering::Relaxed);
                        idx % self.online_cpu_count
                    };

                    let actual_cpu = if let Some(pc) = self.per_cpu.get_mut(target_cpu) {
                        pc.runq[priority as usize].push_back(entry.task_id);
                        target_cpu
                    } else {
                        if let Some(pc) = self.per_cpu.get_mut(0) {
                            pc.runq[priority as usize].push_back(entry.task_id);
                        }
                        0
                    };

                    let current_prio = self.per_cpu.get(actual_cpu)
                        .and_then(|pc| pc.current)
                        .and_then(|cid| self.tasks.binary_search_by_key(&cid, |t| t.id).ok())
                        .map(|idx| self.tasks[idx].priority as usize)
                        .unwrap_or(0);
                    if (priority as usize) > current_prio {
                        self.need_resched = true;
                    }

                    graphify::update_task_state(entry.task_id, "runnable");
                }
            } else {
                i += 1;
            }
        }
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

        if self.preempt_disable_depth == 0 && self.need_resched {
            self.need_resched = false;
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
        let current_id = self.per_cpu.get(cpu_idx)?.current?;

        self.metrics.yields += 1;

        // Don't push idle task or dead tasks back to runq
        if Some(current_id) != self.per_cpu[cpu_idx].idle_task {
            if let Some(task) = self.get_task(current_id) {
                if task.state != TaskState::Dead {
                    let priority = task.priority;
                    // Push to LOCAL runq (we are yielding on this CPU)
                    self.per_cpu[cpu_idx].runq[priority as usize].push_back(current_id);
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
        if cpu_idx >= self.per_cpu.len() {
            return None;
        }
        let per_cpu_len = self.per_cpu.len();

        // Collect tasks pinned to a different CPU so we can requeue them after scanning.
        let mut misrouted: Vec<(usize, usize, TaskId)> = Vec::new(); // (priority, target_cpu, id)

        let mut next_id = None;
        // Priority scan — skip dead and misrouted tasks
        for p in (1..5).rev() {
            while let Some(id) = self.per_cpu[cpu_idx].runq[p].pop_front() {
                self.metrics.pops += 1;
                // Skip dead tasks that were enqueued before kill took effect
                let task_ref = self.get_task(id);
                if task_ref.map_or(true, |t| t.state == TaskState::Dead) {
                    continue;
                }
                // Skip tasks pinned to a different CPU — collect for requeue
                let task = task_ref.unwrap();
                if let crate::task::Affinity::Pinned(target) = task.affinity {
                    if target != cpu_idx && target < per_cpu_len {
                        misrouted.push((task.priority as usize, target, id));
                        continue;
                    }
                }
                next_id = Some(id);
                break;
            }
            if next_id.is_some() {
                break;
            }
        }

        let next_id = match next_id {
            Some(id) => id,
            None => {
                // Check Idle queue — skip dead and misrouted tasks
                let mut found_idle_q = None;
                while let Some(id) = self.per_cpu[cpu_idx].runq[0].pop_front() {
                    self.metrics.pops += 1;
                    let task_ref = self
                        .tasks
                        .iter()
                        .find(|t| t.id == id);
                    if task_ref.map_or(true, |t| t.state == TaskState::Dead) {
                        continue;
                    }
                    let task = task_ref.unwrap();
                    if let crate::task::Affinity::Pinned(target) = task.affinity {
                        if target != cpu_idx && target < per_cpu_len {
                            misrouted.push((task.priority as usize, target, id));
                            continue;
                        }
                    }
                    found_idle_q = Some(id);
                    break;
                }
                if let Some(id) = found_idle_q {
                    id
                } else if let Some(idle) = self.per_cpu[cpu_idx].idle_task {
                    self.metrics.idle_picks += 1;
                    idle
                } else {
                    // Flush misrouted tasks before returning
                    for (prio, target_cpu, id) in misrouted {
                        self.per_cpu[target_cpu].runq[prio].push_back(id);
                    }
                    return None;
                }
            }
        };

        // Flush misrouted tasks to their correct CPU queues
        for (prio, target_cpu, id) in misrouted {
            self.per_cpu[target_cpu].runq[prio].push_back(id);
        }

        let current_id = self.per_cpu[cpu_idx]
            .current
            .expect("prepare_schedule called without current task");

        if next_id == current_id {
            let idx = self.tasks.iter().position(|t| t.id == current_id).unwrap();
            self.tasks[idx].state = TaskState::Running;
            return None;
        }

        self.per_cpu[cpu_idx].current = Some(next_id);

        // Reset wait time and priority for the newly scheduled task (anti-starvation)
        self.reset_wait_time(next_id);

        let old_idx = self.tasks.iter().position(|t| t.id == current_id).unwrap();
        let new_idx = self.tasks.iter().position(|t| t.id == next_id).unwrap();

        let tasks_ptr = self.tasks.as_mut_ptr();
        unsafe {
            let old_task = &mut **tasks_ptr.add(old_idx);
            let new_task = &mut **tasks_ptr.add(new_idx);

            if old_task.state == TaskState::Running {
                old_task.state = TaskState::Runnable;
                // Hot-path graph emission disabled — fires on every context
                // switch and overwhelms the queue.  Lifecycle events (block,
                // sleep, exit) still update the graph.
                // graphify::update_task_state(old_task.id, "runnable");
            }
            new_task.state = TaskState::Running;
            new_task.last_cpu = Some(cpu_idx);

            // Update the lock-free mapping cache for this CPU so check_user_mapping is fast
            *crate::task::scheduler::vm::CURRENT_MAPPINGS[cpu_idx].lock() = 
                Some(new_task.mappings.clone());

            // Hot-path graph emissions disabled — see comment above.
            // graphify::update_task_state(new_task.id, "running");
            // graphify::update_task_location(new_task.id, cpu_idx);

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
            })
        }
    }

    pub fn terminate_current(&mut self, code: i32) -> SwitchParams<
        <R::Tasking as BootTasking>::Context,
        <R::Tasking as BootTasking>::AddressSpace,
    > {
        let cpu_idx = current_cpu_index::<R>();
        let current_id = self
            .per_cpu
            .get(cpu_idx)
            .and_then(|pc| pc.current)
            .expect("terminate_current called with no current task");

        if let Some(idx) = self.tasks.iter().position(|t| t.id == current_id) {
            self.tasks[idx].state = TaskState::Dead;
            self.tasks[idx].exit_code = Some(code);

            // Queue graph state update and exit code
            graphify::update_task_state(current_id, "dead");
            graphify::set_exit_code(current_id, code);
            
            // Cleanup owned things
            if let Some(owner_thing_id) = types::graph_thing_for_tid(current_id) {
                crate::root::enqueue(crate::root::RootOp::CleanupTaskThings { owner_thing_id });
            }
        }

        // Release any claimed devices
        crate::device_registry::REGISTRY
            .lock()
            .release_all_for_task(current_id);

        loop {
            if let Some(switch) = self.prepare_schedule() {
                return switch;
            }
        }
    }

    pub fn set_priority(&mut self, id: TaskId, priority: TaskPriority) {
        if let Some(idx) = self.tasks.iter().position(|t| t.id == id) {
            let old_priority = self.tasks[idx].priority;
            self.tasks[idx].priority = priority;
            self.tasks[idx].base_priority = priority; // Update base priority for anti-starvation

            // Queue graph priority property update
            graphify::update_task_priority(id, priority as u8);

            // If it's runnable and in a runq, move it to the new runq
            if self.tasks[idx].state == TaskState::Runnable {
                // We need to find WHICH runq it is in if we don't track it.
                // Brute force: check ALL per_cpu runqs? Or check affinity?
                for pc in self.per_cpu.iter_mut() {
                    if let Some(pos) = pc.runq[old_priority as usize]
                        .iter()
                        .position(|&rid| rid == id)
                    {
                        pc.runq[old_priority as usize].remove(pos);
                        pc.runq[priority as usize].push_back(id);
                        break;
                    }
                }
            }
        }
    }

    /// Apply priority aging to prevent starvation.
    ///
    /// This anti-starvation mechanism temporarily boosts the priority of tasks that
    /// have been waiting too long. The algorithm:
    ///
    /// 1. For each task in run queues (except Realtime), calculate boost based on wait time
    /// 2. Boost level = wait_ticks / AGING_THRESHOLD_TICKS (capped at MAX_PRIORITY_BOOST)
    /// 3. effective_priority = base_priority + boost_levels (capped at Realtime)
    /// 4. Update task.priority to reflect the boost
    ///
    /// Example: A Low-priority task waiting for 1000 ticks (2 aging periods at 500 ticks each)
    /// gets boosted by 2 levels: Low -> Normal -> High
    ///
    /// When the task is eventually scheduled, its priority is restored to base_priority
    /// and wait_ticks is reset to 0.
    fn apply_priority_aging(&mut self, cpu_idx: usize) {
        use crate::task::TaskPriority;

        // Collect promoted tasks with a fixed-size buffer to avoid allocations
        // Realistically, very few tasks are promoted at once per CPU run queue.
        // If we exceed this, we just drop the promotion for this tick.
        const MAX_PROMOTIONS: usize = 32;
        let mut promotions: [(usize, TaskId); MAX_PROMOTIONS] = [(0, 0); MAX_PROMOTIONS];
        let mut promo_count = 0;

        // Process each base-priority level (skip Realtime = index 4)
        for base_priority in 0..4 {
            let mut i = 0;
            while i < self.per_cpu[cpu_idx].runq[base_priority].len() {
                let id = self.per_cpu[cpu_idx].runq[base_priority][i];
                let mut promoted = false;

                if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
                    let boost = (task.wait_ticks / types::AGING_THRESHOLD_TICKS) as usize;
                    let boost = boost.min(types::MAX_PRIORITY_BOOST);
                    let eff = (task.base_priority as usize + boost)
                        .min(TaskPriority::Realtime as usize);

                    if eff != base_priority {
                        if promo_count < MAX_PROMOTIONS {
                            task.priority = match eff {
                                0 => TaskPriority::Idle,
                                1 => TaskPriority::Low,
                                2 => TaskPriority::Normal,
                                3 => TaskPriority::High,
                                4 => TaskPriority::Realtime,
                                _ => TaskPriority::Idle,
                            };
                            promotions[promo_count] = (eff, id);
                            promo_count += 1;
                            promoted = true;
                        }
                    }
                }

                if promoted {
                    // Remove from original queue
                    self.per_cpu[cpu_idx].runq[base_priority].remove(i);
                    // Do not advance i, since everything shifted left
                } else {
                    i += 1;
                }
            }
        }

        // Apply promotions
        for idx in 0..promo_count {
            let (target, id) = promotions[idx];
            self.per_cpu[cpu_idx].runq[target].push_back(id);
        }
    }

    fn increment_wait_times(&mut self, cpu_idx: usize) {
        let current_id = self.per_cpu[cpu_idx].current;
        
        // Anti-starvation: increment wait_ticks for tasks waiting in THIS cpu's run queues.
        // We must avoid heap allocations (like Vec) here because this runs in the timer interrupt
        // and could deadlock the global allocator.
        // O(Q * T) complexity where Q is queue length and T is total tasks. Since Q is usually
        // very small and only contains tasks for this specific CPU, this is fast enough.
        for priority in 0..5 {
            let len = self.per_cpu[cpu_idx].runq[priority].len();
            for i in 0..len {
                let task_id = self.per_cpu[cpu_idx].runq[priority][i];
                if Some(task_id) != current_id {
                    if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
                        if task.state == TaskState::Runnable {
                            task.wait_ticks = task.wait_ticks.saturating_add(1);
                        }
                    }
                }
            }
        }
    }

    /// Reset wait time for a task that just got scheduled.
    ///
    /// Part of the anti-starvation mechanism. When a task is scheduled to run, we:
    /// 1. Reset wait_ticks to 0 (it's no longer waiting)
    /// 2. Restore priority to base_priority (remove any aging boost)
    ///
    /// This ensures that aging only provides temporary priority boosts and doesn't
    /// permanently change a task's priority.
    fn reset_wait_time(&mut self, task_id: TaskId) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.wait_ticks = 0;
            // Reset priority to base priority (remove any aging boost)
            task.priority = task.base_priority;
        }
    }

    /// Mark a secondary CPU as online and initialize its idle task.
    pub fn cpu_online(&mut self, cpu_index: usize) {
        crate::kinfo!(
            "SMP: CPU {} online (triggered by scheduler spawn)",
            cpu_index
        );
        self.bringup_in_progress = false;
        self.online_cpu_count += 1;

        // Create idle task for this new CPU
        let i = cpu_index;
        let idle_id = self.spawn(
            idle_task::<R>,
            StartupArg::Raw(i),
            TaskPriority::Idle,
            crate::task::Affinity::Pinned(i),
        );

        // Remove from run queues - idle tasks are special
        for q in self.per_cpu.iter_mut().flat_map(|pc| pc.runq.iter_mut()) {
            if let Some(pos) = q.iter().position(|&id| id == idle_id) {
                q.remove(pos);
            }
        }

        // Set as this CPU's idle task
        self.per_cpu[i].idle_task = Some(idle_id);

        // Pin idle task to its CPU
        if let Some(t) = self.tasks.iter_mut().find(|t| t.id == idle_id) {
            (**t).affinity = crate::task::Affinity::Pinned(i);
        }
        graphify::set_affinity_node(idle_id, i);
        graphify::set_name(idle_id, &alloc::format!("idle/{}", i));
    }

    pub(crate) fn log_context_switch(
        &mut self,
        switch: &SwitchParams<
            <R::Tasking as BootTasking>::Context,
            <R::Tasking as BootTasking>::AddressSpace,
        >,
        cr3_before: u64,
        cr3_after: u64,
    ) {
        let idx = SWITCH_LOG_COUNT.fetch_add(1, Ordering::Relaxed);
        if idx >= 20000 {
            return;
        }

        let cpu_idx = current_cpu_index::<R>();
        let pair = ((switch.from_tid as u64) << 32) | (switch.to_tid as u64);

        if let Some(pc) = self.per_cpu.get_mut(cpu_idx) {
            if pc.last_switch == pair {
                return;
            }
            pc.last_switch = pair;
        }

        // crate::contract!(
        //     "sched.switch: from={} to={} u_from={} u_to={} cr3_b={:#x} cr3_a={:#x}",
        //     switch.from_tid,
        //     switch.to_tid,
        //     switch.from_user,
        //     switch.to_user,
        //     cr3_before,
        //     cr3_after
        // );
    }
}

pub(crate) fn log_context_switch<R: BootRuntime>(
    switch: &SwitchParams<
        <R::Tasking as BootTasking>::Context,
        <R::Tasking as BootTasking>::AddressSpace,
    >,
    cr3_before: u64,
    cr3_after: u64,
) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
        sched.log_context_switch(switch, cr3_before, cr3_after);
    }
    rt.irq_restore(_irq);
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
        sched
            .tasks
            .iter()
            .find(|t| t.id == id)
            .map(|t| (t.state, t.exit_code))
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
                .per_cpu
                .get(cpu)
                .and_then(|pc| pc.current)
                .and_then(|tid| sched.get_task(tid))
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

/// Get the current task's ProcessInfo Arc, if any.
pub fn process_info<R: BootRuntime>() -> Option<alloc::sync::Arc<spin::Mutex<crate::task::ProcessInfo>>> {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let result = {
        let lock = SCHEDULER.lock();
        if let Some(ptr) = *lock {
            let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };
            let cpu_idx = current_cpu_index::<R>();
            sched
                .per_cpu
                .get(cpu_idx)
                .and_then(|pc| pc.current)
                .and_then(|tid| sched.get_task(tid))
                .and_then(|t| t.process_info.clone())
        } else {
            None
        }
    };

    rt.irq_restore(_irq);
    result
}

/// Get the graph ThingId for the current task, if any.
fn graph_thing_for_current_impl<R: BootRuntime>() -> Option<u64> {
    let rt = crate::runtime::<R>();
    let tid = rt.current_tid();
    // Use the separate TASK_GRAPH lock — no SCHEDULER.lock() needed.
    types::graph_thing_for_tid(tid)
}

pub fn exit<R: BootRuntime>(code: i32) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let switch = {
        let mut lock = SCHEDULER.lock();
        let ptr = lock.expect("Scheduler not initialized");
        let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
        sched.terminate_current(code)
    };

    #[cfg(any(feature = "sched_debug", debug_assertions))]
    let cr3_before = rt.debug_active_aspace_root();
    unsafe {
        rt.tasking().activate_address_space(switch.to_aspace);
    }
    #[cfg(any(feature = "sched_debug", debug_assertions))]
    let cr3_after = rt.debug_active_aspace_root();
    
    #[cfg(any(feature = "sched_debug", debug_assertions))]
    log_context_switch::<R>(&switch, cr3_before, cr3_after);

    unsafe {
        rt.tasking()
            .switch(&mut *(switch.from_ctx as *mut _), &*switch.to_ctx, switch.to_tid);
    }
    
    unreachable!("Thread continued after terminating!");
}

/// Kill an arbitrary task by TID. Returns true if the task was found and killed.
/// The task is marked Dead with exit code -9 and removed from all run queues.
pub fn kill_by_tid<R: BootRuntime>(tid: u64) -> bool {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let lock = SCHEDULER.lock();
    if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };

        // Don't allow killing the current task via this path
        let cpu_idx = current_cpu_index::<R>();
        if let Some(current_id) = sched.per_cpu.get(cpu_idx).and_then(|pc| pc.current) {
            if current_id == tid {
                rt.irq_restore(_irq);
                return false;
            }
        }

        if let Some(idx) = sched.tasks.iter().position(|t| t.id == tid) {
            if sched.tasks[idx].state == TaskState::Dead {
                rt.irq_restore(_irq);
                return false; // Already dead
            }

            sched.tasks[idx].state = TaskState::Dead;
            sched.tasks[idx].exit_code = Some(-9);

            // Remove from all run queues
            for pc in sched.per_cpu.iter_mut() {
                for q in pc.runq.iter_mut() {
                    if let Some(pos) = q.iter().position(|&id| id == tid) {
                        q.remove(pos);
                    }
                }
            }

            // Remove from sleep queue
            sched.sleep_queue.retain(|e| e.task_id != tid);

            // Queue graph state update
            graphify::update_task_state(tid, "dead");
            graphify::set_exit_code(tid, -9);
            
            // Cleanup owned things
            if let Some(owner_thing_id) = types::graph_thing_for_tid(tid) {
                crate::root::enqueue(crate::root::RootOp::CleanupTaskThings { owner_thing_id });
            }

            // Release any claimed devices
            crate::device_registry::REGISTRY
                .lock()
                .release_all_for_task(tid);

            crate::kinfo!("SCHED: Killed task {} (SIGKILL)", tid);
            rt.irq_restore(_irq);
            return true;
        }
    }
    rt.irq_restore(_irq);
    false
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
        sched.online_cpu_count,
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
    for task in sched.tasks.iter() {
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
    for (i, pc) in sched.per_cpu.iter().enumerate() {
        if i >= sched.online_cpu_count {
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

    crate::kprint!("Sleep queue: {} tasks\n", sched.sleep_queue.len());
    crate::kprint!(
        "=== {} tasks, {} runnable ===\n\n",
        sched.tasks.len(),
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

/// Dedicated task for processing deferred graph work.
/// Runs at low priority and yields after each flush.
extern "C" fn graph_worker_task<R: BootRuntime>(_: usize) -> ! {
    loop {
        // Process any pending graph work
        flush_graph_queue::<R>();
        maybe_log_scheduler_profile::<R>();
        // Sleep between flushes — more frequent, smaller batches reduce
        // per-call latency and contention.
        sleep::sleep_ms::<R>(50);
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
    crate::kinfo!("SMP: Secondary CPU {} online!", cpu_index);

    // Enter scheduler loop via the hook which bootstraps this CPU.
    // The run_scheduler hook will call bootstrap_cpu to set up this CPU's
    // current task before entering the yield loop.
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
    use crate::{BootRuntime, BootRuntimeBase, BootTasking, MapKind, MapPerms, UserEntry, UserTaskSpec};

    // Mock types for testing - copy from spawn.rs tests
    #[derive(Default, Copy, Clone)]
    struct MockContext(usize);
    #[derive(Clone, Copy, Default)]
    struct MockAddressSpace(u64);

    struct MockRuntime;
    impl BootRuntimeBase for MockRuntime {
        fn putchar(&self, _c: u8) {}
        fn mono_ticks(&self) -> u64 { 0 }
        fn mono_freq_hz(&self) -> u64 { 1 }
        fn init_secondary_cpu(&self, _cpu_index: usize) {}
    }
    impl BootRuntime for MockRuntime {
        type Tasking = MockRuntime;
        fn tasking(&self) -> &Self { self }
        fn halt(&self) -> ! { loop {} }
        fn irq_disable(&self) -> crate::IrqState { crate::IrqState(0) }
        fn irq_restore(&self, _state: crate::IrqState) {}
        fn phys_memory_map(&self) -> &'static [crate::PhysRange] { &[] }
        fn phys_to_virt_offset(&self) -> u64 { 0 }
        fn modules(&self) -> &'static [crate::BootModuleDesc] { &[] }
        fn framebuffer(&self) -> Option<crate::FramebufferInfo> { None }
        fn simd_state_layout(&self) -> (usize, usize) { (0, 1) }
        unsafe fn simd_save(&self, _ptr: *mut u8) {}
        unsafe fn simd_restore(&self, _ptr: *const u8) {}
    }
    impl BootTasking for MockRuntime {
        type Runtime = MockRuntime;
        type Context = MockContext;
        type AddressSpace = MockAddressSpace;
        fn init(&self, _hhdm: u64) {}
        fn init_kernel_context(&self, _entry: extern "C" fn(usize) -> !, _st: u64, _arg: usize) -> Self::Context {
            MockContext(_arg)
        }
        fn init_user_context(&self, _spec: UserTaskSpec<Self::AddressSpace>, _kst: u64) -> Self::Context {
            MockContext(_spec.arg)
        }
        unsafe fn switch(&self, _f: &mut Self::Context, _t: &Self::Context, _tid: u64) {}
        unsafe fn enter_user(&self, _e: UserEntry) -> ! { loop {} }
        fn make_user_address_space(&self) -> Self::AddressSpace { MockAddressSpace(0) }
        fn active_address_space(&self) -> Self::AddressSpace { MockAddressSpace(0) }
        fn activate_address_space(&self, _as: Self::AddressSpace) {}
        fn map_page(&self, _as: Self::AddressSpace, _v: u64, _p: u64, _pr: MapPerms, _k: MapKind, _a: &dyn crate::FrameAllocatorHook) -> Result<(), ()> { Ok(()) }
        fn unmap_page(&self, _as: Self::AddressSpace, _v: u64) -> Result<Option<u64>, ()> { Ok(None) }
        fn translate(&self, _as: Self::AddressSpace, _v: u64) -> Option<u64> { None }
        fn tlb_flush_page(&self, _v: u64) {}
    }

    #[test]
    fn test_wait_ticks_increment() {
        static RUNTIME: MockRuntime = MockRuntime;
        // Test that wait_ticks increments for waiting tasks
        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.per_cpu.push(types::PerCpu::new());
        
        // Create a mock task with low priority
        let task = crate::task::Task {
            id: 1,
            state: TaskState::Runnable,
            priority: TaskPriority::Low,
            base_priority: TaskPriority::Low,
            wait_ticks: 0,
            exit_code: None,
            is_user: false,
            wake_pending: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
        };
        
        sched.tasks.push(alloc::boxed::Box::new(task));
        sched.per_cpu[0].runq[TaskPriority::Low as usize].push_back(1);
        sched.per_cpu[0].current = Some(0); // Different task is running
        
        // Increment wait times
        sched.increment_wait_times(0);
        
        // Verify wait_ticks incremented
        let task = sched.get_task(1).unwrap();
        assert_eq!(task.wait_ticks, 1);
    }

    #[test]
    fn test_priority_aging_boost() {
        static RUNTIME: MockRuntime = MockRuntime;
        // Test that tasks waiting too long get priority boost
        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.per_cpu.push(types::PerCpu::new());
        
        // Create a low-priority task that has been waiting
        let task = crate::task::Task {
            id: 1,
            state: TaskState::Runnable,
            priority: TaskPriority::Low,
            base_priority: TaskPriority::Low,
            wait_ticks: types::AGING_THRESHOLD_TICKS, // Waited long enough for boost
            exit_code: None,
            is_user: false,
            wake_pending: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
        };
        
        sched.tasks.push(alloc::boxed::Box::new(task));
        sched.per_cpu[0].runq[TaskPriority::Low as usize].push_back(1);
        sched.per_cpu[0].current = Some(0);
        
        // Apply aging
        sched.apply_priority_aging(0);
        
        // Verify priority was boosted
        let task = sched.get_task(1).unwrap();
        assert!(task.priority > TaskPriority::Low, "Priority should be boosted");
        assert_eq!(task.base_priority, TaskPriority::Low, "Base priority should remain unchanged");

        // Verify the task was physically moved between queues
        assert!(
            sched.per_cpu[0].runq[TaskPriority::Low as usize].is_empty(),
            "Task should have been removed from the Low queue"
        );
        assert!(
            sched.per_cpu[0].runq[task.priority as usize]
                .iter()
                .any(|&id| id == 1),
            "Task should be in the boosted priority queue"
        );
    }

    #[test]
    fn test_reset_wait_time_on_schedule() {
        static RUNTIME: MockRuntime = MockRuntime;
        // Test that wait_ticks resets when task is scheduled
        let mut sched = types::Scheduler::<MockRuntime>::new();
        
        // Create a task that has been waiting
        let task = crate::task::Task {
            id: 1,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Low,
            wait_ticks: 100, // Has been waiting
            exit_code: None,
            is_user: false,
            wake_pending: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
        };
        
        sched.tasks.push(alloc::boxed::Box::new(task));
        
        // Reset wait time
        sched.reset_wait_time(1);
        
        // Verify wait_ticks reset and priority restored
        let task = sched.get_task(1).unwrap();
        assert_eq!(task.wait_ticks, 0);
        assert_eq!(task.priority, task.base_priority);
    }

    #[test]
    fn test_wake_preempts_lower_priority() {
        use core::sync::atomic::Ordering;

        static RUNTIME: MockRuntime = MockRuntime;
        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.per_cpu.push(types::PerCpu::new());

        // Task 1: Normal priority, currently running
        let normal_task = crate::task::Task {
            id: 1,
            state: TaskState::Running,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            wait_ticks: 0,
            exit_code: None,
            is_user: false,
            wake_pending: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
        };

        // Task 2: Realtime priority, sleeping (about to wake)
        let rt_task = crate::task::Task {
            id: 2,
            state: TaskState::Runnable,
            priority: TaskPriority::Realtime,
            base_priority: TaskPriority::Realtime,
            wait_ticks: 0,
            exit_code: None,
            is_user: false,
            wake_pending: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
        };

        sched.tasks.push(alloc::boxed::Box::new(normal_task));
        sched.tasks.push(alloc::boxed::Box::new(rt_task));
        sched.per_cpu[0].current = Some(1); // Normal task is running

        // Put RT task in sleep queue with wake_tick in the past
        TICK_COUNT.store(100, Ordering::Relaxed);
        sched.sleep_queue.push_back(types::SleepEntry {
            task_id: 2,
            wake_tick: 50, // already expired
        });

        // Before: need_resched should be false
        assert!(!sched.need_resched, "need_resched should start false");

        // Wake sleepers — should detect RT > Normal and set need_resched
        sched.wake_sleepers();

        // Verify need_resched was set
        assert!(
            sched.need_resched,
            "need_resched should be true after waking a higher-priority task"
        );

        // Verify RT task was enqueued to the Realtime runq
        assert!(
            sched.per_cpu[0].runq[TaskPriority::Realtime as usize]
                .iter()
                .any(|&id| id == 2),
            "RT task should be in the Realtime run queue"
        );

        // Now simulate schedule: prepare_yield should pick the RT task
        sched.need_resched = false; // clear so prepare_yield runs clean
        let switch = sched.prepare_yield();
        assert!(switch.is_some(), "Should produce a context switch");
        let switch = switch.unwrap();
        assert_eq!(switch.to_tid, 2, "Scheduler should switch to the RT task");
        assert_eq!(
            switch.from_tid, 1,
            "Scheduler should switch away from the Normal task"
        );
    }
}
