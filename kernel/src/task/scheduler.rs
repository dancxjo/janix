use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::alloc::Layout;
use core::sync::atomic::{AtomicBool, Ordering};
use crate::task::{Task, TaskId, TaskState};
// use crate::arch::imp::task::{context_init, context_switch};
use crate::user::elf::UserImage;
use crate::memory::paging::AddressSpace;


// 64KiB stack
const STACK_SIZE: usize = 64 * 1024;
const STACK_ALIGN: usize = 16;

pub struct Scheduler {
    pub current: Option<TaskId>,
    pub runq: VecDeque<TaskId>,
    pub tasks: Vec<Task>,
    pub next_id: TaskId,
    pub need_resched: AtomicBool,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            current: None,
            runq: VecDeque::new(),
            tasks: Vec::new(),
            next_id: 1,
            need_resched: AtomicBool::new(false),
        }
    }

    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn current_id(&self) -> Option<TaskId> {
        self.current
    }

    pub fn init_boot_task(&mut self) {
        // Create a task representing the currently running boot thread.
        // It uses the existing stack, so we don't allocate one.
        // We just need a slot to save its state when we switch away.
        
        let task = Task {
            id: 0,
            state: TaskState::Running,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            simd: crate::simd::SimdState::new(crate::runtime()),
            aspace: None,
            tf: Default::default(),
        };
        
        self.tasks.push(task);
        self.current = Some(0);
        
        // Don't add to runq; it's running.
    }

    pub fn spawn(&mut self, entry: extern "C" fn(usize) -> !, arg: usize) -> TaskId {
        let id = self.next_id;
        self.next_id += 1;

        // Allocate stack
        let layout = Layout::from_size_align(STACK_SIZE, STACK_ALIGN).unwrap();
        let stack_base = unsafe { alloc::alloc::alloc(layout) };
        if stack_base.is_null() {
            panic!("Scheduler::spawn: OOM allocating stack");
        }
        
        // Zero stack (optional, but good for debugging)
        unsafe { core::ptr::write_bytes(stack_base, 0, STACK_SIZE) };

        let stack_top = (stack_base as u64) + (STACK_SIZE as u64);
        
        // Align top to 16 bytes (should be already if size is aligned, but be safe)
        let stack_top = stack_top & !0xF;

        let mut task = Task {
            id,
            state: TaskState::Runnable,
            kstack_base: stack_base,
            kstack_size: STACK_SIZE,
            kstack_top: stack_top,
            ctx: Default::default(),
            simd: crate::simd::SimdState::new(crate::runtime()),
            aspace: None,
            tf: Default::default(),
        };

        // Initialize Arch Context
        crate::runtime().context_init(&mut task.ctx.0, stack_top, entry, arg);

        self.tasks.push(task);
        self.runq.push_back(id);
        
        id
    }

    pub fn spawn_user(&mut self, image: UserImage, aspace: AddressSpace) -> TaskId {
        let id = self.next_id;
        self.next_id += 1;

        // Allocate kernel stack for syscalls/interrupts while in this task
        let layout = Layout::from_size_align(STACK_SIZE, STACK_ALIGN).unwrap();
        let stack_base = unsafe { alloc::alloc::alloc(layout) };
        if stack_base.is_null() {
            panic!("Scheduler::spawn_user: OOM allocating stack");
        }
        unsafe { core::ptr::write_bytes(stack_base, 0, STACK_SIZE) };
        let stack_top = (stack_base as u64) + (STACK_SIZE as u64);
        let stack_top = stack_top & !0xF;

        let mut tf = crate::trap::x86_64::TrapFrame::default();
        tf.user_rip = image.entry;
        tf.user_rsp = image.stack_top;
        tf.user_rflags = 0x202; // IF | Reserved

        let mut task = Task {
            id,
            state: TaskState::Runnable,
            kstack_base: stack_base,
            kstack_size: STACK_SIZE,
            kstack_top: stack_top,
            ctx: Default::default(),
            simd: crate::simd::SimdState::new(crate::runtime()),
            aspace: Some(aspace),
            tf,
        };

        // Initialize Arch Context to jump to user_entry_stub
        crate::runtime().context_init(&mut task.ctx.0, stack_top, user_entry_stub, 0);

        self.tasks.push(task);
        self.runq.push_back(id);
        
        id
    }

    pub fn yield_now(&mut self) {
        let current_id = self.current.expect("yielding without current task");
        
        // Mark current as runnable
        // Find task index
        let idx = self.tasks.iter().position(|t| t.id == current_id).expect("Current task lost");
        self.tasks[idx].state = TaskState::Runnable;
        
        // Requeue
        self.runq.push_back(current_id);
        
        self.schedule();
    }

    fn schedule(&mut self) {
        // Pick next
        let next_id = match self.runq.pop_front() {
            Some(id) => id,
            None => {
                // No other tasks. If current is runnable (yielded), we just continue.
                // If current blocked/died, we would panic or idle.
                // In yield_now(), we just pushed current back. So we should find it.
                // If runq is empty, it means we are the only task.
                return;
            }
        };

        let current_id = self.current.unwrap(); // yield_now ensures this

        if next_id == current_id {
            // Nothing to switch
            // Ensure state is running
             let idx = self.tasks.iter().position(|t| t.id == current_id).unwrap();
             self.tasks[idx].state = TaskState::Running;
             return;
        }

        // Switch needed
        self.current = Some(next_id);
        
        // We need mutable references to old and new contexts.
        // Since they are in the same Vec, we have to be careful.
        
        let old_idx = self.tasks.iter().position(|t| t.id == current_id).unwrap();
        let new_idx = self.tasks.iter().position(|t| t.id == next_id).unwrap();
        
        // Raw pointers to avoid borrow checker hell with split_at_mut
        let tasks_ptr = self.tasks.as_mut_ptr();
        unsafe {
            let old_task = &mut *tasks_ptr.add(old_idx);
            let new_task = &mut *tasks_ptr.add(new_idx);
            
            old_task.state = TaskState::Runnable; // yield logic set it, but verify
            new_task.state = TaskState::Running;
            
            // SIMD Save/Restore (Prompt requirement: "integrates with your SIMD state model")
            old_task.simd.save(crate::runtime());
            new_task.simd.restore(crate::runtime());

            // Update kernel stack for syscalls
            crate::runtime().set_kernel_stack(new_task.kstack_top);

            // LOG
            // kinfo!("Switch {} -> {}", old_task.id, new_task.id);

            // Arch Switch
            // This will return when we are switched back to.
            
            // Switch Address Space if needed (user task)
            if let Some(aspace) = &new_task.aspace {
                aspace.switch();
            }
            
            crate::runtime().context_switch(&mut old_task.ctx.0, new_task.ctx.0);
        }
    }
}

extern "C" fn user_entry_stub(_arg: usize) -> ! {
    let tf = unsafe {
        let ptr = core::ptr::addr_of_mut!(crate::task::SCHEDULER);
        let sched = (*ptr).as_mut().unwrap();
        let curr = sched.current_id().unwrap();
        let idx = sched.tasks.iter().position(|t| t.id == curr).unwrap();
        sched.tasks[idx].tf.clone()
    };
    unsafe {
        crate::user::enter::enter_user_sysret(&tf);
    }
}
