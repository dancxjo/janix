//! Preemptive priority-based scheduler
//!
//! This module is split into focused submodules:
//! - `types`: Core data structures and enums
//! - `blocking`: Task blocking and wake primitives  
//! - `hooks`: Type-erased hook system for callers without generic params
//! - `spawn`: Task and thread spawning
//! - `stack`: User stack allocation and fault handling
//! - `sleep`: Timing and yield functions

mod types;
mod blocking;
mod hooks;
mod spawn;
mod stack;
mod sleep;

// Re-export all public items
pub use types::{ScheduleReason, Scheduler, StackFaultResult, SwitchParams};
pub use blocking::{block_current, block_current_erased, init_blocking_hooks, wake_task, wake_task_erased};
pub use hooks::{
    alloc_user_stack_current, current_tid_current, exit_current,
    handle_user_stack_fault_current, spawn_process_current, spawn_user_thread_current,
    task_status_current, yield_now_current,
};
pub use spawn::{spawn, spawn_process, spawn_user_task_full, spawn_user_thread, user_thread_trampoline};
pub use stack::{alloc_user_stack, handle_stack_fault, map_user_page, map_user_page_perms};
pub use sleep::{sleep_ms, sleep_until, yield_now};

use crate::{BootRuntime, BootTasking};
use crate::task::{Task, TaskId, TaskState};
use spin::Mutex;
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

#[cfg(any(feature = "sched_debug", debug_assertions))]
static SWITCH_LOG_COUNT: AtomicUsize = AtomicUsize::new(0);
#[cfg(any(feature = "sched_debug", debug_assertions))]
static LAST_SWITCH: AtomicU64 = AtomicU64::new(0);

pub static SCHEDULER: Mutex<Option<usize>> = Mutex::new(None);

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
            hooks::ALLOC_USER_STACK_HOOK = Some(stack::alloc_user_stack::<R>);
            crate::memory::set_map_user_page_hook(stack::map_user_page::<R>);
            crate::memory::set_map_user_page_perms_hook(stack::map_user_page_perms::<R>);
            hooks::STACK_FAULT_HOOK = Some(stack::handle_stack_fault::<R>);
        }
        blocking::init_blocking_hooks::<R>();
        crate::kinfo!("  Scheduler initialized");
    }
}

fn init_boot_task<R: BootRuntime>(sched: &mut types::Scheduler<R>) {
    let rt = crate::runtime::<R>();

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
        kstack_base: stack_base,
        kstack_size: 16384,
        kstack_top: stack_top,
        ctx: Default::default(),
        aspace: rt.tasking().active_address_space(),
        simd: crate::simd::SimdState::new(rt),
        exit_code: None,
        is_user: false,
        stack_info: None,
    };
    sched.tasks.push(task);
    sched.current = Some(0);
    crate::kinfo!("  Creating idle task...");

    let idle_id = sched.spawn(idle_task::<R>, 0);
    sched.idle_task = Some(idle_id);

    if let Some(pos) = sched.runq.iter().position(|&id| id == idle_id) {
        sched.runq.remove(pos);
    }
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
                if self.preempt_disable_depth > 0 {
                    self.need_resched = true;
                    return None;
                }
            }
            _ => {}
        }

        self.prepare_yield()
    }

    pub fn preempt_disable(&mut self) {
        self.preempt_disable_depth += 1;
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
            crate::log_event!(
               crate::logging::LogLevel::Info,
               "sched.activity",
               "Scheduler Activity Rollup",
               {
                   yields: self.metrics.yields,
                   pops: self.metrics.pops,
                   pushes: self.metrics.pushes,
                   idle_picks: self.metrics.idle_picks,
                   runq_len: self.runq.len() as u64
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
        let current_id = self.current?;

        self.metrics.yields += 1;

        if Some(current_id) != self.idle_task {
            self.runq.push_back(current_id);
            self.metrics.pushes += 1;
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

        let next_id = match self.runq.pop_front() {
            Some(id) => {
                self.metrics.pops += 1;
                id
            }
            None => {
                if let Some(idle) = self.idle_task {
                    self.metrics.idle_picks += 1;
                    idle
                } else {
                    return None;
                }
            }
        };

        let current_id = self
            .current
            .expect("prepare_schedule called without current task");

        if next_id == current_id {
            let idx = self.tasks.iter().position(|t| t.id == current_id).unwrap();
            self.tasks[idx].state = TaskState::Running;
            return None;
        }

        self.current = Some(next_id);

        let old_idx = self.tasks.iter().position(|t| t.id == current_id).unwrap();
        let new_idx = self.tasks.iter().position(|t| t.id == next_id).unwrap();

        let tasks_ptr = self.tasks.as_mut_ptr();
        unsafe {
            let old_task = &mut *tasks_ptr.add(old_idx);
            let new_task = &mut *tasks_ptr.add(new_idx);

            if old_task.state == TaskState::Running {
                old_task.state = TaskState::Runnable;
            }
            new_task.state = TaskState::Running;
            old_task.simd.save(crate::runtime::<R>());
            new_task.simd.restore(crate::runtime::<R>());

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
        let current_id = self
            .current
            .expect("terminate_current called with no current task");

        if let Some(idx) = self.tasks.iter().position(|t| t.id == current_id) {
            self.tasks[idx].state = TaskState::Dead;
            self.tasks[idx].exit_code = Some(code);
        }

        unsafe {
            let rt = crate::runtime::<R>();
            loop {
                if let Some(switch) = self.prepare_schedule() {
                    #[cfg(any(feature = "sched_debug", debug_assertions))]
                    let cr3_before = read_cr3();
                    rt.tasking().activate_address_space(switch.to_aspace);
                    #[cfg(any(feature = "sched_debug", debug_assertions))]
                    let cr3_after = read_cr3();
                    #[cfg(any(feature = "sched_debug", debug_assertions))]
                    log_context_switch::<R>(&switch, cr3_before, cr3_after);
                    rt.tasking().switch(&mut *switch.from_ctx, &*switch.to_ctx);
                }
            }
        }
    }
}

pub fn task_status<R: BootRuntime>(id: TaskId) -> Option<(TaskState, Option<i32>)> {
    let lock = SCHEDULER.lock();
    if let Some(ptr) = *lock {
        let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };
        sched
            .tasks
            .iter()
            .find(|t| t.id == id)
            .map(|t| (t.state, t.exit_code))
    } else {
        None
    }
}

pub fn current_tid<R: BootRuntime>() -> u64 {
    if let Some(lock) = SCHEDULER.try_lock() {
        if let Some(ptr) = *lock {
            let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };
            sched.current_id().unwrap_or(0)
        } else {
            0
        }
    } else {
        0
    }
}

pub fn exit<R: BootRuntime>(code: i32) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
    sched.terminate_current(code)
}

pub fn dump_stats<R: BootRuntime>() {
    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };
    crate::kinfo!(
        "Sched: tasks={} current={:?}",
        sched.task_count(),
        sched.current_id()
    );
}

#[cfg(any(feature = "sched_debug", debug_assertions))]
pub(crate) fn read_cr3() -> u64 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let cr3: u64;
        core::arch::asm!("mov {}, cr3", out(reg) cr3);
        cr3
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        0
    }
}

#[cfg(not(any(feature = "sched_debug", debug_assertions)))]
pub(crate) fn read_cr3() -> u64 {
    0
}

#[cfg(any(feature = "sched_debug", debug_assertions))]
pub(crate) fn log_context_switch<R: BootRuntime>(
    switch: &SwitchParams<
        <R::Tasking as BootTasking>::Context,
        <R::Tasking as BootTasking>::AddressSpace,
    >,
    cr3_before: u64,
    cr3_after: u64,
) {
    if switch.from_user == switch.to_user && cr3_before == cr3_after {
        return;
    }

    let idx = SWITCH_LOG_COUNT.fetch_add(1, Ordering::Relaxed);
    if idx >= 64 {
        return;
    }

    let pair = ((switch.from_tid as u64) << 32) | (switch.to_tid as u64);
    let last = LAST_SWITCH.load(Ordering::Relaxed);
    if last == pair {
        return;
    }
    LAST_SWITCH.store(pair, Ordering::Relaxed);

    if idx < 8 || idx % 64 == 0 {
        crate::log_event!(
            crate::logging::LogLevel::Debug,
            "sched.switch",
            "Context switch",
            {
                from_tid: switch.from_tid,
                to_tid: switch.to_tid,
                from_user: switch.from_user as u64,
                to_user: switch.to_user as u64,
                cr3_before: cr3_before,
                cr3_after: cr3_after
            },
            about=[]
        );
    }
}

#[cfg(not(any(feature = "sched_debug", debug_assertions)))]
pub(crate) fn log_context_switch<R: BootRuntime>(
    _switch: &SwitchParams<
        <R::Tasking as BootTasking>::Context,
        <R::Tasking as BootTasking>::AddressSpace,
    >,
    _cr3_before: u64,
    _cr3_after: u64,
) {
}

extern "C" fn idle_task<R: BootRuntime>(_: usize) -> ! {
    let rt = crate::runtime::<R>();
    loop {
        rt.wait_for_interrupt();
    }
}
