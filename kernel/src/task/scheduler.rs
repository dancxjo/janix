use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::alloc::Layout;
use crate::task::{Task, TaskId, TaskState};
use crate::{BootRuntime, BootTasking};
use spin::Mutex;

const STACK_SIZE: usize = 64 * 1024;
const STACK_ALIGN: usize = 16;

pub struct Scheduler<R: BootRuntime> {
    pub current: Option<TaskId>,
    pub runq: VecDeque<TaskId>,
    pub tasks: Vec<Task<R>>,
    pub next_id: TaskId,
}

impl<R: BootRuntime> Scheduler<R> {
    pub fn new() -> Self {
        Self {
            current: None,
            runq: VecDeque::new(),
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn current_id(&self) -> Option<TaskId> {
        self.current
    }

    pub fn init_boot_task(&mut self) {
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
        
        self.tasks.push(task);
        self.current = Some(0);
    }

    pub fn spawn(&mut self, entry: extern "C" fn(usize) -> !, arg: usize) -> TaskId {
        let id = self.next_id;
        self.next_id += 1;

        let layout = Layout::from_size_align(STACK_SIZE, STACK_ALIGN).unwrap();
        let stack_base = unsafe { alloc::alloc::alloc(layout) };
        if stack_base.is_null() {
            panic!("Scheduler::spawn: OOM allocating stack");
        }
        
        unsafe { core::ptr::write_bytes(stack_base, 0, STACK_SIZE) };

        let stack_top = (stack_base as u64) + (STACK_SIZE as u64);
        let stack_top = stack_top & !0xF;

        let ctx = crate::runtime::<R>().tasking().init_kernel_context(entry, stack_top, arg);

        let task = Task {
            id,
            state: TaskState::Runnable,
            kstack_base: stack_base,
            kstack_size: STACK_SIZE,
            kstack_top: stack_top,
            ctx,
            aspace: Default::default(),
            simd: crate::simd::SimdState::new(crate::runtime::<R>()),
        };

        self.tasks.push(task);
        self.runq.push_back(id);
        
        id
    }

    pub fn prepare_yield(&mut self) -> Option<(*mut <R::Tasking as BootTasking>::Context, *const <R::Tasking as BootTasking>::Context)> {
        let current_id = self.current.expect("yielding without current task");
        
        let idx = self.tasks.iter().position(|t| t.id == current_id).expect("Current task lost");
        self.tasks[idx].state = TaskState::Runnable;
        
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
    let mut lock = SCHEDULER.lock();
    if lock.is_none() {
        let sched = alloc::boxed::Box::new(Scheduler::<R>::new());
        let s = alloc::boxed::Box::leak(sched);
        s.init_boot_task();
        *lock = Some(s as *mut Scheduler<R> as usize);
        unsafe { YIELD_HOOK = Some(yield_now::<R>); }
    }
}

pub fn spawn<R: BootRuntime>(entry: extern "C" fn(usize) -> !, arg: usize) -> TaskId {
    let mut lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
    sched.spawn(entry, arg)
}

pub fn yield_now<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let irq = rt.irq_disable();
    
    let switch_params = {
        let mut lock = SCHEDULER.lock();
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
