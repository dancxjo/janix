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
    get_user_mapping_at_current, handle_user_stack_fault_current, kill_by_tid_current,
    remove_user_mappings_current, set_priority_current, sleep_ticks_current, spawn_process_current,
    spawn_user_thread_current, task_status_current, yield_now_current,
};
pub use sleep::{sleep_ms, sleep_ticks, sleep_until, yield_now};
pub use spawn::{
    spawn, spawn_process, spawn_user_task_full, spawn_user_thread, spawn_with_priority,
    user_thread_trampoline,
};
pub use stack::{alloc_user_stack, handle_stack_fault, map_user_page, map_user_page_perms};
pub use types::{
    DEFAULT_TIMESLICE, ScheduleReason, Scheduler, SleepEntry, StackFaultResult, SwitchParams,
};
pub use wait_queue::WaitQueue;

use crate::task::{StartupArg, Task, TaskId, TaskPriority, TaskState};
use crate::{BootRuntime, BootTasking};
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
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
                    let parent_thing = {
                        let lock = SCHEDULER.lock();
                        if let Some(ptr) = *lock {
                            let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
                            sched.task_graph.insert(tid, thing_id);
                            parent_tid.and_then(|ptid| sched.task_graph.get(&ptid).copied())
                        } else {
                            None
                        }
                    };

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

                let thing_id = {
                    let lock = SCHEDULER.lock();
                    if let Some(ptr) = *lock {
                        let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };
                        sched.task_graph.get(&tid).copied()
                    } else {
                        None
                    }
                };

                if let Some(id) = thing_id {
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
    let q = graph_queue::stats_snapshot();
    let avg_us = if calls > 0 { total_us / calls } else { 0 };

    crate::kinfo!(
        "PROF: sched 2s: graph_flush calls={} items={} avg_us={} max_us={} slow={} trylock_miss={} qlen={} q_hwm={} q_drop_state={} q_evict={}",
        calls,
        items,
        avg_us,
        max_us,
        slow,
        trylock_miss,
        q.current_len,
        q.high_water_mark,
        q.dropped_update_state,
        q.evicted_critical
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
            crate::memory::set_translate_user_page_hook(vm::translate_user_page::<R>);
        }
        blocking::init_blocking_hooks::<R>();
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

                // Decrement current task's time slice
                let cpu_idx = current_cpu_index::<R>();
                if let Some(current_id) = self.per_cpu.get(cpu_idx).and_then(|pc| pc.current) {
                    if let Some(task) = self.tasks.iter_mut().find(|t| t.id == current_id) {
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

        // Process sleep queue - we need to drain and rebuild since entries may not be sorted
        let mut remaining = alloc::collections::VecDeque::new();

        while let Some(entry) = self.sleep_queue.pop_front() {
            if entry.wake_tick <= now {
                // Task should wake up - add back to run queue
                if let Some(task) = self.tasks.iter().find(|t| t.id == entry.task_id) {
                    let priority = task.priority;
                    // Wake to *a* CPU.
                    // If pinned, wake to that CPU's runq.
                    // If Any, wake to current CPU or round robin?
                    // For now, wake to current CPU's runq for simplicity, or CPU 0 logic?
                    // Let's rely on basic 'current CPU' push for wake.
                    // Ideally we'd remember where it slept, but we don't store that.
                    // Let's use affinity.

                    let target_cpu = if let crate::task::Affinity::Pinned(cpu) = task.affinity {
                        cpu
                    } else if let Some(last) = task.last_cpu {
                        // Use last CPU to prevent drifting
                        last
                    } else {
                        // Fallback to round-robin if no history
                        let idx = spawn::RR_IDX.fetch_add(1, Ordering::Relaxed);
                        idx % self.online_cpu_count
                    };

                    if let Some(pc) = self.per_cpu.get_mut(target_cpu) {
                        pc.runq[priority as usize].push_back(entry.task_id);
                    } else {
                        // Fallback to CPU 0 if invalid target
                        if let Some(pc) = self.per_cpu.get_mut(0) {
                            pc.runq[priority as usize].push_back(entry.task_id);
                        }
                    }

                    // Queue graph state update from sleeping to runnable
                    graphify::update_task_state(entry.task_id, "runnable");
                }
            } else {
                // Still sleeping
                remaining.push_back(entry);
            }
        }

        self.sleep_queue = remaining;
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
            return self.schedule_point(ScheduleReason::PreemptTick);
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
            if let Some(task) = self.tasks.iter().find(|t| t.id == current_id) {
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
        let pc = self.per_cpu.get_mut(cpu_idx)?;

        let mut next_id = None;
        // Priority scan — skip dead tasks
        for p in (1..5).rev() {
            while let Some(id) = pc.runq[p].pop_front() {
                self.metrics.pops += 1;
                // Skip dead tasks that were enqueued before kill took effect
                if self
                    .tasks
                    .iter()
                    .find(|t| t.id == id)
                    .map_or(true, |t| t.state == TaskState::Dead)
                {
                    continue;
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
                // Check Idle queue — skip dead tasks
                let mut found_idle_q = None;
                while let Some(id) = pc.runq[0].pop_front() {
                    self.metrics.pops += 1;
                    if self
                        .tasks
                        .iter()
                        .find(|t| t.id == id)
                        .map_or(true, |t| t.state == TaskState::Dead)
                    {
                        continue;
                    }
                    found_idle_q = Some(id);
                    break;
                }
                if let Some(id) = found_idle_q {
                    id
                } else if let Some(idle) = pc.idle_task {
                    self.metrics.idle_picks += 1;
                    idle
                } else {
                    return None;
                }
            }
        };

        let current_id = pc
            .current
            .expect("prepare_schedule called without current task");

        if next_id == current_id {
            let idx = self.tasks.iter().position(|t| t.id == current_id).unwrap();
            self.tasks[idx].state = TaskState::Running;
            return None;
        }

        pc.current = Some(next_id);

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

            // STRICT AFFINITY CHECK
            if let crate::task::Affinity::Pinned(pinned_cpu) = new_task.affinity {
                if pinned_cpu != cpu_idx {
                    crate::kprintln!(
                        "FATAL SCHED BUG: CPU {} picked Task {} which is pinned to CPU {}!",
                        cpu_idx,
                        new_task.id,
                        pinned_cpu
                    );
                    // For now, just log it, but we could panic here if we are sure.
                }
            }

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

    pub fn terminate_current(&mut self, code: i32) -> ! {
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
        }

        // Release any claimed devices
        crate::device_registry::REGISTRY
            .lock()
            .release_all_for_task(current_id);

        unsafe {
            let rt = crate::runtime::<R>();
            loop {
                if let Some(switch) = self.prepare_schedule() {
                    #[cfg(any(feature = "sched_debug", debug_assertions))]
                    let cr3_before = rt.debug_active_aspace_root();
                    rt.tasking().activate_address_space(switch.to_aspace);
                    #[cfg(any(feature = "sched_debug", debug_assertions))]
                    let cr3_after = rt.debug_active_aspace_root();
                    #[cfg(any(feature = "sched_debug", debug_assertions))]
                    self.log_context_switch(&switch, cr3_before, cr3_after);
                    rt.tasking()
                        .switch(&mut *switch.from_ctx, &*switch.to_ctx, switch.to_tid);
                }
            }
        }
    }

    pub fn set_priority(&mut self, id: TaskId, priority: TaskPriority) {
        if let Some(idx) = self.tasks.iter().position(|t| t.id == id) {
            let old_priority = self.tasks[idx].priority;
            self.tasks[idx].priority = priority;

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
                .and_then(|tid| sched.tasks.iter().find(|t| t.id == tid))
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

pub fn exit<R: BootRuntime>(code: i32) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
    sched.terminate_current(code)
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
        // Sleep between flushes — graph bookkeeping is non-critical and the
        // tight yield loop was starving interactive tasks (bloom cursor).
        sleep::sleep_ms::<R>(100);
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
