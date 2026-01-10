use alloc::collections::VecDeque;
use alloc::vec::Vec;
use spin::Mutex;

use crate::BootRuntime;
use crate::BootTasking;
use crate::task::{Task, TaskId, TaskState};

pub struct Scheduler<R: BootRuntime> {
    tasks: Vec<Task<R>>,
    runq: VecDeque<TaskId>,
    current: Option<TaskId>,
    next_id: TaskId,
}

impl<R: BootRuntime> Scheduler<R> {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            runq: VecDeque::new(),
            current: None,
            next_id: 0,
        }
    }

    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn current_id(&self) -> Option<TaskId> {
        self.current
    }

    pub fn init_boot_task(&mut self) {
        crate::kinfo!("  Creating boot task...");
        let task = Task {
            id: 0,
            state: TaskState::Running,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: Default::default(),
            simd: crate::simd::SimdState::new(crate::runtime::<R>()),
        };
        crate::kinfo!("  Pushing boot task to list...");
        self.tasks.push(task);
        self.current = Some(0);
        crate::kinfo!("  Boot task created successfully");
    }

    pub fn spawn(&mut self, entry: extern "C" fn(usize) -> !, arg: usize) -> TaskId {
        let rt = crate::runtime::<R>();
        
        self.next_id += 1;
        let id = self.next_id;

        let layout = alloc::alloc::Layout::from_size_align(16384, 16).unwrap();
        let stack_base = unsafe { alloc::alloc::alloc(layout) };
        if stack_base.is_null() {
            panic!("Failed to allocate stack");
        }

        let stack_top = (stack_base as u64) + 16384;
        let ctx = rt.tasking().init_kernel_context(entry, stack_top, arg);
        let aspace = rt.tasking().active_address_space();

        let task = Task {
            id,
            state: TaskState::Runnable,
            kstack_base: stack_base,
            kstack_size: 16384,
            kstack_top: stack_top,
            ctx,
            aspace,
            simd: crate::simd::SimdState::new(rt),
        };

        self.tasks.push(task);
        self.runq.push_back(id);
        id
    }

    pub fn prepare_yield(&mut self) -> Option<(*mut <R::Tasking as BootTasking>::Context, *const <R::Tasking as BootTasking>::Context)> {
        let current_id = self.current?;
        
        // Re-add current task to run queue
        self.runq.push_back(current_id);
        
        self.prepare_schedule()
    }

    fn prepare_schedule(&mut self) -> Option<(*mut <R::Tasking as BootTasking>::Context, *const <R::Tasking as BootTasking>::Context)> {
        let next_id = match self.runq.pop_front() {
            Some(id) => id,
            None => {
                return None;
            }
        };

        let current_id = self.current.unwrap();

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
            
            old_task.state = TaskState::Runnable;
            new_task.state = TaskState::Running;
            
            old_task.simd.save(crate::runtime::<R>());
            new_task.simd.restore(crate::runtime::<R>());
            
            Some((&mut old_task.ctx as *mut _, &new_task.ctx as *const _))
        }
    }
}

pub static SCHEDULER: Mutex<Option<usize>> = Mutex::new(None);
static mut YIELD_HOOK: Option<unsafe fn()> = None;

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
        unsafe { YIELD_HOOK = Some(yield_now::<R>); }
        crate::kinfo!("  Scheduler initialized");
    }
}

pub fn spawn<R: BootRuntime>(entry: extern "C" fn(usize) -> !, arg: usize) -> TaskId {
    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
    sched.spawn(entry, arg)
}

pub fn yield_now<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let irq = rt.irq_disable();
    
    let switch_params = {
        let lock = SCHEDULER.lock();
        let ptr = lock.expect("Scheduler not initialized");
        let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
        sched.prepare_yield()
    };
    
    if let Some((old_ctx, new_ctx)) = switch_params {
        unsafe {
            rt.tasking().switch(&mut *old_ctx, &*new_ctx);
        }
    }
    
    rt.irq_restore(irq);
}

pub unsafe fn yield_now_current() {
    unsafe {
        if let Some(hook) = YIELD_HOOK {
            hook();
        }
    }
}

pub fn dump_stats<R: BootRuntime>() {
    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &*(ptr as *const Scheduler<R>) };
    crate::kinfo!("Sched: tasks={} current={:?}", sched.task_count(), sched.current_id());
}
