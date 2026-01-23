//! Task and thread spawning functions.

use crate::task::{Task, TaskId, TaskState};
use crate::{BootRuntime, BootTasking, UserEntry};

use super::SCHEDULER;
use super::types::{DEFAULT_TIMESLICE, Scheduler};

impl<R: BootRuntime> Scheduler<R> {
    pub fn spawn(
        &mut self,
        entry: extern "C" fn(usize) -> !,
        arg: usize,
        priority: crate::task::TaskPriority,
    ) -> TaskId {
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
            priority,
            kstack_base: stack_base,
            kstack_size: 16384,
            kstack_top: stack_top,
            ctx,
            aspace: rt.tasking().active_address_space(),
            simd: crate::simd::SimdState::new(rt),
            exit_code: None,
            is_user: false,
            wake_pending: false,
            stack_info: None,
            timeslice_remaining: DEFAULT_TIMESLICE,
        };

        self.tasks.insert(id, task);
        self.runq[priority as usize].push_back(id);
        id
    }

    pub fn spawn_user_thread(
        &mut self,
        entry: usize,
        stack: usize,
        arg: usize,
        stack_info: abi::types::StackInfo,
        priority: crate::task::TaskPriority,
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
            priority,
            kstack_base: stack_base,
            kstack_size: 16384,
            kstack_top,
            ctx,
            aspace,
            simd: crate::simd::SimdState::new(rt),
            exit_code: None,
            is_user: true,
            wake_pending: false,
            stack_info: Some(stack_info),
            timeslice_remaining: DEFAULT_TIMESLICE,
        };

        self.tasks.insert(id, task);
        self.runq[priority as usize].push_back(id);
        id
    }

    pub fn spawn_user_task(
        &mut self,
        entry: UserEntry,
        aspace: <R::Tasking as BootTasking>::AddressSpace,
        stack_info: abi::types::StackInfo,
        priority: crate::task::TaskPriority,
    ) -> Option<TaskId> {
        let rt = crate::runtime::<R>();
        let id = self.next_id;

        self.next_id += 1;
        let layout = alloc::alloc::Layout::from_size_align(16384, 16).unwrap();
        let stack_base = unsafe { alloc::alloc::alloc(layout) };
        if stack_base.is_null() {
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
            priority,
            kstack_base: stack_base,
            kstack_size: 16384,
            kstack_top: stack_top,
            ctx,
            aspace,
            simd: crate::simd::SimdState::new(rt),
            exit_code: None,
            is_user: true,
            wake_pending: false,
            stack_info: Some(stack_info),
            timeslice_remaining: DEFAULT_TIMESLICE,
        };

        self.tasks.insert(id, task);
        self.runq[priority as usize].push_back(id);
        Some(id)
    }
}

pub fn spawn<R: BootRuntime>(entry: extern "C" fn(usize) -> !, arg: usize) -> TaskId {
    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
    sched.spawn(entry, arg, crate::task::TaskPriority::Normal)
}

pub fn spawn_with_priority<R: BootRuntime>(
    entry: extern "C" fn(usize) -> !,
    arg: usize,
    priority: crate::task::TaskPriority,
) -> TaskId {
    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
    sched.spawn(entry, arg, priority)
}

pub unsafe fn spawn_user_thread<R: BootRuntime>(
    entry: usize,
    stack: usize,
    arg: usize,
    stack_info: abi::types::StackInfo,
    priority: crate::task::TaskPriority,
) -> TaskId {
    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
    sched.spawn_user_thread(entry, stack, arg, stack_info, priority)
}

pub unsafe fn spawn_user_task_full<R: BootRuntime>(
    entry: UserEntry,
    aspace: <R::Tasking as BootTasking>::AddressSpace,
    stack_info: abi::types::StackInfo,
    priority: crate::task::TaskPriority,
) -> Option<TaskId> {
    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
    sched.spawn_user_task(entry, aspace, stack_info, priority)
}

pub unsafe fn spawn_process<R: BootRuntime>(name: &str, arg: usize) -> Option<TaskId> {
    unsafe { spawn_process_with_priority::<R>(name, arg, crate::task::TaskPriority::Normal) }
}

pub unsafe fn spawn_process_with_priority<R: BootRuntime>(
    name: &str,
    arg: usize,
    priority: crate::task::TaskPriority,
) -> Option<TaskId> {
    let rt = crate::runtime::<R>();
    let modules = rt.modules();
    let module = modules.iter().find(|m| m.name.contains(name))?;

    let aspace = rt.tasking().make_user_address_space();

    let (mut entry, stack_info) = crate::task::loader::load_module(rt, aspace, module)?;
    entry.arg0 = arg;

    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };

    sched.spawn_user_task(entry, aspace, stack_info, priority)
}

pub extern "C" fn user_thread_trampoline<R: BootRuntime>(arg: usize) -> ! {
    crate::kinfo!("Trampoline entered. Arg: 0x{:x}", arg);
    let rt = crate::runtime::<R>();
    let entry_ptr = arg as *mut UserEntry;
    let entry = unsafe { *alloc::boxed::Box::from_raw(entry_ptr) };

    crate::kprintln!(
        "USER_TRAMPOLINE: PC=0x{:x} SP=0x{:x} ARG0=0x{:x}",
        entry.entry_pc,
        entry.user_sp,
        entry.arg0
    );

    // Safety: we are entering user mode with the provided entry point
    unsafe { rt.tasking().enter_user(entry) }
}
