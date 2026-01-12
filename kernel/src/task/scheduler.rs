//! Preemptive priority-based scheduler

use crate::{BootRuntime, BootTasking, MapKind, MapPerms, UserEntry, memory};
use crate::task::{Task, TaskId, TaskState};
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use spin::Mutex;
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

const DEFAULT_USER_STACK_PAGES: usize = 16;
const MAX_USER_STACK_PAGES: usize = 256;

static NEXT_USER_STACK: AtomicU64 = AtomicU64::new(0x7FFF_0000_0000);

#[cfg(any(feature = "sched_debug", debug_assertions))]
static SWITCH_LOG_COUNT: AtomicUsize = AtomicUsize::new(0);
#[cfg(any(feature = "sched_debug", debug_assertions))]
static LAST_SWITCH: AtomicU64 = AtomicU64::new(0);

static mut YIELD_HOOK: Option<fn()> = None;
static mut EXIT_HOOK: Option<fn(i32)> = None;
static mut SPAWN_USER_HOOK: Option<unsafe fn(usize, usize, usize, abi::types::StackInfo) -> TaskId> = None;
static mut SPAWN_PROCESS_HOOK: Option<unsafe fn(&str, usize) -> Option<TaskId>> = None;
static mut CURRENT_TID_HOOK: Option<fn() -> u64> = None;
static mut TASK_STATUS_HOOK: Option<fn(TaskId) -> Option<(TaskState, Option<i32>)>> = None;
static mut ALLOC_USER_STACK_HOOK: Option<fn(usize) -> Option<usize>> = None;
static mut STACK_FAULT_HOOK: Option<unsafe fn(u64) -> StackFaultResult> = None;

static BLOCK_CURRENT_HOOK: core::sync::atomic::AtomicPtr<()> = core::sync::atomic::AtomicPtr::new(core::ptr::null_mut());
static WAKE_TASK_HOOK: core::sync::atomic::AtomicPtr<()> = core::sync::atomic::AtomicPtr::new(core::ptr::null_mut());

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackFaultResult {
    NotStack,
    Grew,
    Overflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleReason {
    PreemptTick,
    CooperativeYield,
    SleepWait,
    BlockedOnIo,
}

pub struct SwitchParams<Ctx, AS> {
    pub from_ctx: *mut Ctx,
    pub to_ctx: *const Ctx,
    pub to_aspace: AS,
    pub from_aspace: AS,
    pub from_tid: TaskId,
    pub to_tid: TaskId,
    pub from_user: bool,
    pub to_user: bool,
}

struct SchedulerMetrics {
    yields: u64,
    pops: u64,
    pushes: u64,
    idle_picks: u64,
    last_flush: u64,
}

pub struct Scheduler<R: BootRuntime> {
    tasks: Vec<Task<R>>,
    runq: VecDeque<TaskId>,
    wait_queue: VecDeque<TaskId>,
    current: Option<TaskId>,
    next_id: TaskId,
    idle_task: Option<TaskId>,
    preempt_disable_depth: usize,
    need_resched: bool,
    metrics: SchedulerMetrics,
}

impl<R: BootRuntime> Scheduler<R> {
    pub fn new() -> Self {
        Scheduler {
            tasks: Vec::new(),
            runq: VecDeque::new(),
            wait_queue: VecDeque::new(),
            current: None,
            next_id: 1,
            idle_task: None,
            preempt_disable_depth: 0,
            need_resched: false,
            metrics: SchedulerMetrics {
                yields: 0,
                pops: 0,
                pushes: 0,
                idle_picks: 0,
                last_flush: 0,
            },
        }
    }

    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn current_id(&self) -> Option<TaskId> {
        self.current
    }

    pub fn init_boot_task(&mut self) {
        let rt = crate::runtime::<R>();

        crate::kinfo!("  Creating boot task...");

        // Boot task needs a valid kernel stack for syscall handling.
        // Even though boot task is a kernel task, if it ever gets scheduled
        // (e.g., via yield), switch() will write its kstack_top to CPU_LOCAL.
        // If kstack_top is 0, subsequent syscalls will crash.
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
        self.tasks.push(task);
        self.current = Some(0);
        crate::kinfo!("  Creating idle task...");

        // Spawn an idle task
        let idle_id = self.spawn(idle_task::<R>, 0);
        self.idle_task = Some(idle_id);

        // Remove idle from runq since we handle it specially
        if let Some(pos) = self.runq.iter().position(|&id| id == idle_id) {
            self.runq.remove(pos);
        }
        crate::kinfo!("  Boot task initialized");
    }

    pub fn spawn(&mut self, entry: extern "C" fn(usize) -> !, arg: usize) -> TaskId {
        let rt = crate::runtime::<R>();
        let id = self.next_id;
        self.next_id += 1;

        let layout = alloc::alloc::Layout::from_size_align(16384, 16).unwrap();
        let stack_base = unsafe { alloc::alloc::alloc(layout) };
        if stack_base.is_null() {
            panic!("Failed to allocate stack for task {}", id);
        }
        let stack_top = (stack_base as u64) + 16384;

        let ctx = rt.tasking().init_kernel_context(entry, stack_top, arg);

        let task: Task<R> = Task {
            id,
            state: TaskState::Runnable,
            kstack_base: stack_base,
            kstack_size: 16384,
            kstack_top: stack_top,
            ctx,
            aspace: rt.tasking().active_address_space(),
            simd: crate::simd::SimdState::new(rt),
            exit_code: None,
            is_user: false,
            stack_info: None,
        };

        self.tasks.push(task);
        self.runq.push_back(id);
        id
    }

    pub fn spawn_user_thread(
        &mut self,
        entry: usize,
        stack: usize,
        arg: usize,
        stack_info: abi::types::StackInfo,
    ) -> TaskId {
        let rt = crate::runtime::<R>();
        let id = self.next_id;
        self.next_id += 1;

        let layout = alloc::alloc::Layout::from_size_align(16384, 16).unwrap();
        let stack_base = unsafe { alloc::alloc::alloc(layout) };
        if stack_base.is_null() {
            panic!("Failed to allocate kernel stack for user thread {}", id);
        }
        let kstack_top = (stack_base as u64) + 16384;

        let aspace = rt.tasking().active_address_space();

        let spec = crate::UserTaskSpec {
            entry: entry as u64,
            stack_top: stack as u64,
            aspace,
            arg,
        };

        let ctx = rt.tasking().init_user_context(spec, kstack_top);

        let task: Task<R> = Task {
            id,
            state: TaskState::Runnable,
            kstack_base: stack_base,
            kstack_size: 16384,
            kstack_top,
            ctx,
            aspace,
            simd: crate::simd::SimdState::new(rt),
            exit_code: None,
            is_user: true,
            stack_info: Some(stack_info),
        };

        self.tasks.push(task);
        self.runq.push_back(id);
        id
    }

    pub fn spawn_user_task(
        &mut self,
        entry: UserEntry,
        aspace: <R::Tasking as BootTasking>::AddressSpace,
        stack_info: abi::types::StackInfo,
    ) -> Option<TaskId> {
        let rt = crate::runtime::<R>();
        let id = self.next_id;

        self.next_id += 1;
        let layout = alloc::alloc::Layout::from_size_align(16384, 16).unwrap();
        let stack_base = unsafe { alloc::alloc::alloc(layout) };
        if stack_base.is_null() {
            // panic!("Failed to allocate stack");
            return None;
        }
        let stack_top = (stack_base as u64) + 16384;

        let user_entry = alloc::boxed::Box::new(entry);
        let entry_ptr = alloc::boxed::Box::into_raw(user_entry) as usize;

        let ctx =
            rt.tasking()
                .init_kernel_context(user_thread_trampoline::<R>, stack_top, entry_ptr);

        let task: Task<R> = Task {
            id,
            state: TaskState::Runnable,
            kstack_base: stack_base,
            kstack_size: 16384,
            kstack_top: stack_top,
            ctx,
            aspace,
            simd: crate::simd::SimdState::new(rt),
            exit_code: None,
            is_user: true,
            stack_info: Some(stack_info),
        };

        self.tasks.push(task);
        self.runq.push_back(id);
        Some(id)
    }

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

        // If we are here, we are scheduling.
        self.prepare_yield()
    }

    pub fn preempt_disable(&mut self) {
        self.preempt_disable_depth += 1;
    }

    fn flush_metrics_if_needed(&mut self) {
        let rt = crate::runtime::<R>();
        let now = rt.mono_ticks();
        let limit = rt.mono_freq_hz() * 2; // 2 seconds log window

        // Initialize if 0
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

            // Reset
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
            // Trigger deferred preemption
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

        // Re-add current task to run queue, unless it is the idle task
        if Some(current_id) != self.idle_task {
            self.runq.push_back(current_id);
            self.metrics.pushes += 1;
        }

        self.prepare_schedule()
    }

    fn prepare_schedule(
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
                // If runq is empty, run idle task if available
                if let Some(idle) = self.idle_task {
                    self.metrics.idle_picks += 1;
                    idle
                } else {
                    return None;
                }
            }
        };

        // Self-link optimization handled below

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

        // Use raw pointers to avoid borrow checker issues with dual mutable access to Vec elements
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

        // Mark as Dead (we use Dead instead of Terminated)
        if let Some(idx) = self.tasks.iter().position(|t| t.id == current_id) {
            self.tasks[idx].state = TaskState::Dead;
            self.tasks[idx].exit_code = Some(code);
        }

        // Do NOT clear current.

        // Schedule next
        unsafe {
            let rt = crate::runtime::<R>();
            // We loop endlessly if schedule returns None
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

pub static SCHEDULER: Mutex<Option<usize>> = Mutex::new(None);

pub fn init<R: BootRuntime>() {
    crate::kinfo!("  Acquiring scheduler lock...");
    let mut lock = SCHEDULER.lock();
    crate::kinfo!("  Lock acquired, checking if initialized...");
    if lock.is_none() {
        crate::kinfo!("  Allocating scheduler...");
        let sched = alloc::boxed::Box::new(Scheduler::<R>::new());
        crate::kinfo!("  Leaking scheduler...");
        let s = alloc::boxed::Box::leak(sched);
        crate::kinfo!("  Initializing boot task...");
        s.init_boot_task();
        crate::kinfo!("  Storing scheduler pointer...");
        *lock = Some(s as *mut Scheduler<R> as usize);
        *lock = Some(s as *mut Scheduler<R> as usize);
        unsafe {
            YIELD_HOOK = Some(yield_now::<R>);
            EXIT_HOOK = Some(exit::<R>);
            SPAWN_USER_HOOK = Some(spawn_user_thread::<R>);
            SPAWN_PROCESS_HOOK = Some(spawn_process::<R>);
            CURRENT_TID_HOOK = Some(current_tid::<R>);
            TASK_STATUS_HOOK = Some(task_status::<R>);
            ALLOC_USER_STACK_HOOK = Some(alloc_user_stack::<R>);
            crate::memory::set_map_user_page_hook(map_user_page::<R>);
            crate::memory::set_map_user_page_perms_hook(map_user_page_perms::<R>);
            STACK_FAULT_HOOK = Some(handle_stack_fault::<R>);
        }
        init_blocking_hooks::<R>();
        crate::kinfo!("  Scheduler initialized");
    }
}

pub fn spawn<R: BootRuntime>(entry: extern "C" fn(usize) -> !, arg: usize) -> TaskId {
    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
    sched.spawn(entry, arg)
}

pub unsafe fn spawn_user_thread<R: BootRuntime>(
    entry: usize,
    stack: usize,
    arg: usize,
    stack_info: abi::types::StackInfo,
) -> TaskId {
    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
    sched.spawn_user_thread(entry, stack, arg, stack_info)
}

pub unsafe fn spawn_user_task_full<R: BootRuntime>(
    entry: UserEntry,
    aspace: <R::Tasking as BootTasking>::AddressSpace,
    stack_info: abi::types::StackInfo,
) -> Option<TaskId> {
    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
    sched.spawn_user_task(entry, aspace, stack_info)
}

pub fn task_status<R: BootRuntime>(id: TaskId) -> Option<(TaskState, Option<i32>)> {
    let lock = SCHEDULER.lock();
    if let Some(ptr) = *lock {
        let sched = unsafe { &*(ptr as *const Scheduler<R>) };
        sched
            .tasks
            .iter()
            .find(|t| t.id == id)
            .map(|t| (t.state, t.exit_code))
    } else {
        None
    }
}

pub unsafe fn task_status_current(id: TaskId) -> Option<(TaskState, Option<i32>)> {
    if let Some(hook) = unsafe { TASK_STATUS_HOOK } {
        hook(id)
    } else {
        None
    }
}

pub unsafe fn spawn_process<R: BootRuntime>(name: &str, arg: usize) -> Option<TaskId> {
    let rt = crate::runtime::<R>();
    let modules = rt.modules();
    // crate::kinfo!("Spawn request: '{}' arg={:x}", name, arg);
    let module = modules.iter().find(|m| {
        if m.name.contains(name) {
            // crate::kinfo!("  Match candidate: '{}' @ {:x}", m.name, m.phys_start);
            true
        } else {
            false
        }
    })?;
    // crate::kinfo!("Loading module: '{}' from {:x}", module.name, module.phys_start);

    let aspace = rt.tasking().make_user_address_space();

    let (mut entry, stack_info) = crate::task::loader::load_module(rt, aspace, module)?;
    entry.arg0 = arg;

    // Create the task
    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };

    sched.spawn_user_task(entry, aspace, stack_info)
}

pub unsafe fn spawn_process_current(name: &str, arg: usize) -> Option<TaskId> {
    if let Some(hook) = unsafe { SPAWN_PROCESS_HOOK } {
        unsafe { hook(name, arg) }
    } else {
        None
    }
}

pub extern "C" fn user_thread_trampoline<R: BootRuntime>(arg: usize) -> ! {
    crate::kinfo!("Trampoline entered. Arg: 0x{:x}", arg);
    let rt = crate::runtime::<R>();
    let entry_ptr = arg as *mut UserEntry;
    let entry = unsafe { *alloc::boxed::Box::from_raw(entry_ptr) };

    crate::kinfo!(
        "Entering user mode: PC=0x{:x} SP=0x{:x}",
        entry.entry_pc,
        entry.user_sp
    );

    // Safety: we are entering user mode with the provided entry point
    unsafe { rt.tasking().enter_user(entry) }
}

pub fn yield_now<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let switch_params = {
        let lock = SCHEDULER.lock();
        let ptr = lock.expect("Scheduler not initialized");
        let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
        sched.schedule_point(ScheduleReason::CooperativeYield)
    };

    if let Some(switch) = switch_params {
        #[cfg(any(feature = "sched_debug", debug_assertions))]
        let cr3_before = read_cr3();

        rt.tasking().activate_address_space(switch.to_aspace);

        #[cfg(any(feature = "sched_debug", debug_assertions))]
        let cr3_after = read_cr3();
        #[cfg(any(feature = "sched_debug", debug_assertions))]
        log_context_switch::<R>(&switch, cr3_before, cr3_after);

        unsafe {
            rt.tasking().switch(&mut *switch.from_ctx, &*switch.to_ctx);
        }
    }

    rt.irq_restore(_irq);
}

pub fn sleep_until<R: BootRuntime>(deadline_ticks: u64) {
    let rt = crate::runtime::<R>();
    loop {
        let now = rt.mono_ticks();
        if now >= deadline_ticks {
            break;
        }

        let switch_params = {
            let lock = SCHEDULER.lock();
            let ptr = lock.expect("Scheduler not initialized");
            let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
            sched.schedule_point(ScheduleReason::SleepWait)
        };

        if let Some(switch) = switch_params {
            unsafe {
                let _irq = rt.irq_disable();
                #[cfg(any(feature = "sched_debug", debug_assertions))]
                let cr3_before = read_cr3();
                rt.tasking().activate_address_space(switch.to_aspace);
                #[cfg(any(feature = "sched_debug", debug_assertions))]
                let cr3_after = read_cr3();
                #[cfg(any(feature = "sched_debug", debug_assertions))]
                log_context_switch::<R>(&switch, cr3_before, cr3_after);
                rt.tasking().switch(&mut *switch.from_ctx, &*switch.to_ctx);
                rt.irq_restore(_irq);
            }
        } else {
            // No switch occurred, spin briefly
            core::hint::spin_loop();
        }
    }
}

pub fn sleep_ms<R: BootRuntime>(ms: u64) {
    let rt = crate::runtime::<R>();
    let freq = rt.mono_freq_hz();
    let ticks = (ms * freq) / 1000;
    let deadline = rt.mono_ticks() + ticks;
    sleep_until::<R>(deadline);
}

pub unsafe fn yield_now_current() {
    if let Some(hook) = unsafe { YIELD_HOOK } {
        hook();
    }
}

pub unsafe fn current_tid_current() -> u64 {
    if let Some(hook) = unsafe { CURRENT_TID_HOOK } {
        hook()
    } else {
        0
    }
}

pub fn current_tid<R: BootRuntime>() -> u64 {
    // Use try_lock to avoid deadlock when logging during scheduler init
    if let Some(lock) = SCHEDULER.try_lock() {
        if let Some(ptr) = *lock {
            let sched = unsafe { &*(ptr as *const Scheduler<R>) };
            sched.current_id().unwrap_or(0)
        } else {
            0
        }
    } else {
        0 // Lock held (probably by init), return 0
    }
}

pub fn exit<R: BootRuntime>(code: i32) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
    sched.terminate_current(code)
}

pub unsafe fn exit_current(code: i32) {
    if let Some(hook) = unsafe { EXIT_HOOK } {
        hook(code);
    } else {
        // Fallback if no scheduler
        crate::kprintln!("exit_current called without scheduler!");
    }
}

pub unsafe fn spawn_user_thread_current(
    entry: usize,
    stack: usize,
    arg: usize,
    stack_info: abi::types::StackInfo,
) -> Option<TaskId> {
    if let Some(hook) = unsafe { SPAWN_USER_HOOK } {
        Some(unsafe { hook(entry, stack, arg, stack_info) })
    } else {
        None
    }
}

pub unsafe fn handle_user_stack_fault_current(addr: u64) -> StackFaultResult {
    if let Some(hook) = unsafe { STACK_FAULT_HOOK } {
        unsafe { hook(addr) }
    } else {
        StackFaultResult::NotStack
    }
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

pub unsafe fn alloc_user_stack_current(pages: usize) -> Option<usize> {
    unsafe { ALLOC_USER_STACK_HOOK }.and_then(|hook| hook(pages))
}

fn alloc_user_stack<R: BootRuntime>(pages: usize) -> Option<usize> {
    let rt = crate::runtime::<R>();
    let page_size = rt.page_size() as u64;

    let requested_pages = if pages == 0 {
        DEFAULT_USER_STACK_PAGES
    } else {
        pages
    };
    let clamped_pages = core::cmp::min(requested_pages, MAX_USER_STACK_PAGES);
    let total_size = (clamped_pages as u64).saturating_mul(page_size);

    let base = NEXT_USER_STACK.fetch_add(total_size, Ordering::SeqCst);
    let top = base + total_size;

    let aspace = rt.tasking().active_address_space();
    let perms = MapPerms {
        user: true,
        read: true,
        write: true,
        exec: false,
    };
    let hook = crate::GlobalAllocHook;

    let mut virt = base;
    for _ in 0..clamped_pages {
        let phys = memory::alloc_frame()?;
        let hhdm_virt = phys + rt.phys_to_virt_offset();
        unsafe {
            core::ptr::write_bytes(hhdm_virt as *mut u8, 0, page_size as usize);
        }
        rt.tasking()
            .map_page(aspace, virt, phys, perms, MapKind::Normal, &hook)
            .ok()?;
        virt += page_size;
    }

    Some(top as usize)
}

pub fn dump_stats<R: BootRuntime>() {
    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &*(ptr as *const Scheduler<R>) };
    crate::kinfo!(
        "Sched: tasks={} current={:?}",
        sched.task_count(),
        sched.current_id()
    );
}

/// Map a user page in the current address space
unsafe fn map_user_page<R: BootRuntime>(virt: u64, phys: u64) -> Result<(), ()> {
    use crate::{FrameAllocatorHook, MapKind, MapPerms};
    
    struct MapHook;
    impl FrameAllocatorHook for MapHook {
        fn alloc_frame(&self) -> Option<u64> {
            crate::memory::alloc_frame()
        }
    }
    
    let rt = crate::runtime::<R>();
    let aspace = rt.tasking().active_address_space();
    let perms = MapPerms {
        user: true,
        read: true,
        write: true,
        exec: false,
    };
    let hook = MapHook;
    
    rt.tasking().map_page(aspace, virt, phys, perms, MapKind::Normal, &hook)?;
    rt.tasking().tlb_flush_page(virt);
    
    Ok(())
}

/// Map a user page with explicit permissions in the current address space.
unsafe fn map_user_page_perms<R: BootRuntime>(
    virt: u64,
    phys: u64,
    perms: MapPerms,
) -> Result<(), ()> {
    use crate::{FrameAllocatorHook, MapKind};

    struct MapHook;
    impl FrameAllocatorHook for MapHook {
        fn alloc_frame(&self) -> Option<u64> {
            crate::memory::alloc_frame()
        }
    }

    let rt = crate::runtime::<R>();
    let aspace = rt.tasking().active_address_space();
    let hook = MapHook;

    rt.tasking()
        .map_page(aspace, virt, phys, perms, MapKind::Normal, &hook)?;
    rt.tasking().tlb_flush_page(virt);
    Ok(())
}

unsafe fn handle_stack_fault<R: BootRuntime>(addr: u64) -> StackFaultResult {
    let rt = crate::runtime::<R>();
    let page_size = rt.page_size() as u64;

    let lock = SCHEDULER.lock();
    let ptr = match *lock {
        Some(ptr) => ptr,
        None => return StackFaultResult::NotStack,
    };
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
    let current_id = match sched.current {
        Some(id) => id,
        None => return StackFaultResult::NotStack,
    };
    let idx = match sched.tasks.iter().position(|t| t.id == current_id) {
        Some(i) => i,
        None => return StackFaultResult::NotStack,
    };

    let info = match sched.tasks[idx].stack_info {
        Some(info) => info,
        None => return StackFaultResult::NotStack,
    };

    let guard_start = info.guard_start as u64;
    let guard_end = info.guard_end as u64;
    let reserve_start = info.reserve_start as u64;
    let reserve_end = info.reserve_end as u64;
    let committed_start = info.committed_start as u64;

    if addr >= guard_start && addr < guard_end {
        return StackFaultResult::Overflow;
    }

    if addr < reserve_start || addr >= reserve_end || addr >= committed_start {
        return StackFaultResult::NotStack;
    }

    let fault_page = addr & !(page_size - 1);
    let grow_chunk = core::cmp::max(info.grow_chunk_bytes as u64, page_size);
    let mut new_commit_start = committed_start.saturating_sub(grow_chunk);
    new_commit_start &= !(page_size - 1);
    if new_commit_start > fault_page {
        new_commit_start = fault_page;
    }
    if new_commit_start < reserve_start {
        new_commit_start = reserve_start;
    }

    if new_commit_start == committed_start {
        return StackFaultResult::NotStack;
    }

    let hhdm = crate::boot_info::get().map(|i| i.hhdm_offset).unwrap_or(0);
    let perms = MapPerms {
        user: true,
        read: true,
        write: true,
        exec: false,
    };

    let mut virt = new_commit_start;
    while virt < committed_start {
        let phys = match crate::memory::alloc_frame() {
            Some(p) => p,
            None => return StackFaultResult::NotStack,
        };
        let hhdm_virt = phys + hhdm;
        unsafe {
            core::ptr::write_bytes(hhdm_virt as *mut u8, 0, page_size as usize);
        }
        if unsafe { crate::memory::map_user_page_with_perms(virt, phys, perms) }.is_err() {
            return StackFaultResult::NotStack;
        }
        virt += page_size;
    }

    sched.tasks[idx].stack_info = Some(abi::types::StackInfo {
        committed_start: new_commit_start as usize,
        ..info
    });

    StackFaultResult::Grew
}

// ============================================================================
// BLOCKING PRIMITIVES
// ============================================================================

pub fn block_current<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let switch_params = {
        let lock = SCHEDULER.lock();
        let ptr = lock.expect("Scheduler not initialized");
        let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };

        let current_id = match sched.current {
            Some(id) => id,
            None => {
                rt.irq_restore(_irq);
                return;
            }
        };

        // Move current from Running to Blocked
        if let Some(idx) = sched.tasks.iter().position(|t| t.id == current_id) {
            sched.tasks[idx].state = TaskState::Blocked;
        }

        // Add to wait queue
        sched.wait_queue.push_back(current_id);

        // Schedule next
        sched.prepare_schedule()
    };

    if let Some(switch) = switch_params {
        #[cfg(any(feature = "sched_debug", debug_assertions))]
        let cr3_before = read_cr3();

        rt.tasking().activate_address_space(switch.to_aspace);

        #[cfg(any(feature = "sched_debug", debug_assertions))]
        let cr3_after = read_cr3();
        #[cfg(any(feature = "sched_debug", debug_assertions))]
        log_context_switch::<R>(&switch, cr3_before, cr3_after);

        unsafe {
            rt.tasking().switch(&mut *switch.from_ctx, &*switch.to_ctx);
        }
    }

    rt.irq_restore(_irq);
}

pub fn wake_task<R: BootRuntime>(id: usize) {
    let lock = SCHEDULER.lock();
    let ptr = match *lock {
        Some(p) => p,
        None => return,
    };
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };

    let tid = id as TaskId;

    // Remove from wait queue if present
    if let Some(pos) = sched.wait_queue.iter().position(|&wid| wid == tid) {
        sched.wait_queue.remove(pos);
    }

    // Update state to Runnable and add to runq
    if let Some(idx) = sched.tasks.iter().position(|t| t.id == tid) {
        if sched.tasks[idx].state == TaskState::Blocked {
            sched.tasks[idx].state = TaskState::Runnable;
            sched.runq.push_back(tid);
        }
    }
}

/// Type-erased block for use from IRQ module
pub unsafe fn block_current_erased() {
    let ptr = BLOCK_CURRENT_HOOK.load(core::sync::atomic::Ordering::SeqCst);
    if !ptr.is_null() {
        let hook: fn() = unsafe { core::mem::transmute(ptr) };
        hook();
    }
}

/// Type-erased wake for use from IRQ module  
pub unsafe fn wake_task_erased(id: usize) {
    let ptr = WAKE_TASK_HOOK.load(core::sync::atomic::Ordering::SeqCst);
    if !ptr.is_null() {
        let hook: fn(usize) = unsafe { core::mem::transmute(ptr) };
        hook(id);
    }
}

/// Initialize blocking hooks during scheduler init
pub fn init_blocking_hooks<R: BootRuntime>() {
    BLOCK_CURRENT_HOOK.store(
        block_current::<R> as *mut (),
        core::sync::atomic::Ordering::SeqCst,
    );
    WAKE_TASK_HOOK.store(
        wake_task::<R> as *mut (),
        core::sync::atomic::Ordering::SeqCst,
    );
}

extern "C" fn idle_task<R: BootRuntime>(_: usize) -> ! {
    let rt = crate::runtime::<R>();
    loop {
        rt.wait_for_interrupt();
    }
}
