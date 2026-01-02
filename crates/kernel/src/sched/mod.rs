use alloc::vec::Vec;
use spin::Mutex;
use crate::log::{self, Level};
use graph::store;
use graph::symbols::sym;
use crate::memory::space::AddressSpace;
use alloc::sync::Arc;

pub mod task;
pub mod run_queue;
pub mod percpu;

use task::{Task, TaskId, TaskState};
use run_queue::RunQueue;
use percpu::PerCpu;

pub(crate) static SCHEDULER: Mutex<Option<Scheduler>> = Mutex::new(None);

struct Scheduler {
    pub(crate) tasks: Vec<Task>,
    pub(crate) run_queue: RunQueue,
    pub(crate) cpu: PerCpu,
    next_id: u64,
}

impl Scheduler {
    fn new(run_queue_thing: abi::ids::ThingId, cpu_thing: abi::ids::ThingId) -> Self {
        Self {
            tasks: Vec::new(),
            run_queue: RunQueue::new(run_queue_thing),
            cpu: PerCpu::new(0, cpu_thing, run_queue_thing),
            next_id: 1,
        }
    }

    fn spawn(&mut self, name: &'static str, as_opt: Option<Arc<AddressSpace>>) -> TaskId {
        log::klog(Level::Info, "SCHED", &alloc::format!("spawn: name={} start", name));
        let id = TaskId(self.next_id);
        self.next_id += 1;

        // Allocate stack
        log::klog(Level::Info, "SCHED", "spawn: allocating stack...");
        let stack_size = 64 * 1024; // 64KB
        let stack = alloc::vec![0u8; stack_size];
        let stack_ptr = stack.as_ptr() as u64 + stack_size as u64; // Top
        log::klog(Level::Info, "SCHED", "spawn: stack allocated");
        // Leak the stack for now
        core::mem::forget(stack); 

        // Graph reflection
        log::klog(Level::Info, "SCHED", "spawn: graph store start");
        let task_thing = store::with_store(|s| {
            let t = s.create_thing(sym::KIND_TASK).expect("create task");
            if let Some(place_tasks) = s.find_by_name(sym::PLACE_TASKS) {
                 let _ = s.create_relationship(sym::PRED_CONTAINS, place_tasks, t);
            }
            t
        });
        log::klog(Level::Info, "SCHED", "spawn: graph store done");

        let address_space = as_opt.unwrap_or_else(|| Arc::new(AddressSpace::new().expect("failed create AS")));
        log::klog(Level::Info, "SCHED", "spawn: address space handled");
        let mut task = Task::new(id, task_thing, stack_ptr, address_space);
        
        // Initialize state (New -> Ready)
        store::with_store(|s| task.set_state(s, TaskState::Ready));

        self.tasks.push(task);
        
        // Add to run queue
        let thing = self.tasks.last().unwrap().thing;
        self.run_queue.push_back(id, thing);
        
        log::klog(Level::Info, "SCHED", &alloc::format!(
            "spawn: finished id={} name={} stack_top={:#x} stack_size={:#x}", 
            id.0, name, stack_ptr, stack_size
        ));

        id
    }
}

pub fn init() {
    log::klog(Level::Info, "SCHED", "initializing...");

    // Seed Graph (scheduler.main, cpu.0, run_queue.0)
    let (_sched_thing, cpu_thing, rq_thing) = store::with_store(|s| {
        let place_tasks = s.find_by_name(sym::PLACE_TASKS).expect("place.tasks missing");
        
        // scheduler.main
        let sched = s.create_thing(sym::KIND_SCHEDULER).expect("create sched");
        s.register_name(sched, sym::SCHEDULER_MAIN);
        s.create_relationship(sym::PRED_CONTAINS, place_tasks, sched).ok();

        // cpu.0
        let cpu = s.create_thing(sym::KIND_CPU).expect("create cpu");
        // s.register_name(cpu, "cpu.0"); // Need symbol
        s.create_relationship(sym::PRED_CONTAINS, place_tasks, cpu).ok();
        
        // run_queue.0
        let rq = s.create_thing(sym::KIND_RUN_QUEUE).expect("create rq");
        s.create_relationship(sym::PRED_CONTAINS, sched, rq).ok();
        
        (sched, cpu, rq)
    });

    let sched = Scheduler::new(rq_thing, cpu_thing);
    *SCHEDULER.lock() = Some(sched);
}


pub fn run() -> ! {
    log::klog(Level::Info, "SCHED", "entering loop");
    
    // Debug: Check interrupt state BEFORE enabling
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let sp: u64;
        core::arch::asm!("mov {}, sp", out(reg) sp);
        crate::serial::write(b"SCHED: SP=");
        crate::serial::write_hex(sp);
        crate::serial::write(b"\n");

        
        let cntv_ctl: u64;
        core::arch::asm!("mrs {}, cntv_ctl_el0", out(reg) cntv_ctl);
        crate::serial::write(b" CTL=");
        crate::serial::write_hex(cntv_ctl);
        
        let cntv_tval: u64;
        core::arch::asm!("mrs {}, cntv_tval_el0", out(reg) cntv_tval);
        crate::serial::write(b" TVAL=");
        crate::serial::write_hex(cntv_tval);
        crate::serial::write(b"\n");
        
    }


    // Enable interrupts
    crate::machine::machine().irq_enable();

    let mut last_irq_check = 0;
    loop {
        // Probe A2: Check IRQ progress
        #[cfg(target_arch = "aarch64")]
        {
            let irq_hits = crate::machine::aarch64::exception::IRQ_COUNT.load(Ordering::Relaxed);
            if irq_hits != last_irq_check {
                 last_irq_check = irq_hits;
            }
        }

        crate::machine::idle();
    }
}

pub fn yield_current() {
    // For V0.3 Task 03, we rely on preemption (Timer).
    // Manual yield via interrupt?
    // Or call `tick` manually?
    // tick expects SP. We can't easily call it from Rust without saving state.
    // We need an arch-specific `yield` trampoline (int 0xXX or similar).
    // For now, spin (busy wait) or just do nothing if we trust timer.
    // "yield" usually means give up slice.
    // Let's loop hint.
    core::hint::spin_loop(); 
}

// Sprout helpers
pub fn mark_as_init(_id: TaskId) {
    // Mark in graph?
}

pub fn configure_task_memory(id: TaskId, _img: (u64, u64), _stack: (u64, u64), heap: (u64, u64, u64)) {
    let mut guard = SCHEDULER.lock();
    if let Some(sched) = guard.as_mut() {
        if let Some(t) = sched.tasks.iter_mut().find(|t| t.id == id) {
            t.heap_base = heap.0;
            t.heap_size = heap.1;
            t.heap_brk = heap.2;
        }
    }
}

pub fn configure_task_context(id: TaskId, entry: u64, user_stack: u64) {
    use crate::machine::{CurrentArch, ArchTask, CpuMode, TaskContext};
    
    let mut guard = SCHEDULER.lock();
    if let Some(sched) = guard.as_mut() {
        if let Some(task) = sched.tasks.iter_mut().find(|t| t.id == id) {
            // Use Kernel Stack Top implicitly allocated by spawn
            // TrapFrame is built on kernel stack, user_stack is stored in RSP field for user mode
            let kernel_stack_top = task.stack_ptr & !0xf;
            
            crate::log::klog(crate::log::Level::Info, "SCHED", 
                &alloc::format!("configure_ctx: k_stack={:#x} u_stack={:#x} entry={:#x}", 
                    kernel_stack_top, user_stack, entry));

            // Initialize task context using arch-generic trait
            // stack_top = kernel stack (where TrapFrame is built)
            // arg0 = user stack (stored in RSP field for user mode return)
            let mut ctx = TaskContext::default();
            CurrentArch::init_task_context(
                &mut ctx,
                entry,
                kernel_stack_top, // TrapFrame built on kernel stack
                CpuMode::User,
                user_stack, // User stack passed via arg0 for RSP field
            );
            
            task.stack_ptr = ctx.sp;
            
            crate::log::klog(crate::log::Level::Info, "SCHED", 
                &alloc::format!("configure_ctx: finalized sp={:#x}", task.stack_ptr));
        }
    }
}

pub fn exit_current_task(_code: i32) -> ! {
    loop { crate::machine::idle(); }
}

pub fn with_current_task<F, R>(f: F) -> Option<R> where F: FnOnce(&mut Task) -> R {
    let mut guard = SCHEDULER.lock();
    let sched = guard.as_mut()?; // Return None if not init
    let curr = sched.cpu.current_task; 
    if curr.0 == 0 { return None; }
    let t = sched.tasks.iter_mut().find(|t| t.id == curr).unwrap();
    Some(f(t))
}

pub fn with_task<F, R>(id: TaskId, f: F) -> Option<R> where F: FnOnce(&mut Task) -> R {
    let mut guard = SCHEDULER.lock();
    let sched = guard.as_mut()?;
    let t = sched.tasks.iter_mut().find(|t| t.id == id)?;
    Some(f(t))
}

pub fn current_task_id() -> Option<abi::ids::ThingId> {
    // Disable interrupts to prevent deadlock with Timer ISR which also locks SCHEDULER
    let irq_token = crate::machine::irq_disable();
    let res = {
        let guard = SCHEDULER.lock();
        if let Some(sched) = guard.as_ref() {
            let tid = sched.cpu.current_task;
            if tid.0 != 0 {
                 sched.tasks.iter().find(|t| t.id == tid).map(|t| t.thing)
            } else {
                None
            }
        } else {
            None
        }
    };
    crate::machine::irq_restore(irq_token);
    res
}

// Rename/Wrap spawn
pub fn spawn_kernel_task(name: &'static str, entry: extern "C" fn()) -> TaskId {
    use crate::machine::{CurrentArch, ArchTask, CpuMode, TaskContext};
    
    let mut guard = SCHEDULER.lock();
    let sched = guard.as_mut().expect("sched not init");
    
    // Use Shared Kernel Address Space for kernel threads
    let k_as = AddressSpace::new_kernel_share().expect("failed share kernel AS");
    let id = sched.spawn(name, Some(Arc::new(k_as)));
    
    let task = sched.tasks.iter_mut().find(|t| t.id == id).unwrap();
    
    // Initialize task context using arch-generic trait
    let stack_top = task.stack_ptr & !0xf; // 16-byte align
    let mut ctx = TaskContext::default();
    CurrentArch::init_task_context(
        &mut ctx,
        entry as usize as u64,
        stack_top,
        CpuMode::Kernel,
        0, // arg0
    );
    
    task.stack_ptr = ctx.sp;
    
    id
}

// Helper for Sprout (empty spawn)
pub fn spawn_empty(name: &'static str) -> TaskId {
    log::klog(Level::Info, "SCHED", "spawn_empty: locking...");
    let mut guard = SCHEDULER.lock();
    log::klog(Level::Info, "SCHED", "spawn_empty: locked");
    let sched = guard.as_mut().expect("sched not init");
    let res = sched.spawn(name, None);
    log::klog(Level::Info, "SCHED", "spawn_empty: done");
    res
}


use core::sync::atomic::{AtomicU64, Ordering};
pub static TIMER_TICKS: AtomicU64 = AtomicU64::new(0);

/// Called by timer interrupt.
/// Returns new_sp or 0 if no switch.
pub fn tick(current_sp: u64) -> u64 {
    // 1. Inc timer ticks
    let ticks = TIMER_TICKS.fetch_add(1, Ordering::Relaxed);
    if ticks == 0 {
        crate::serial::write(b"SCHED: first tick!\n");
    }
    
    let mut guard = SCHEDULER.lock();
    let sched = guard.as_mut();
    if sched.is_none() { return 0; }
    let sched = sched.unwrap();
    
    let prev_task = sched.cpu.current_task;

    // Default: Round Robin
    // 1. Save current SP to current task (if we have one)
    if prev_task.0 != 0 {
        if let Some(t) = sched.tasks.iter_mut().find(|t| t.id == prev_task) {
            // Only save SP if this task has executed at least once.
            // first_run tasks have prepared contexts that must not be overwritten.
            if !t.first_run {
                t.stack_ptr = current_sp;
            }
            // State: Running -> Ready
             t.state = TaskState::Ready;
             // Enqueue
             let thing = t.thing;
             sched.run_queue.push_back(curr, thing);
        }
    }
    
    // 2. Pick next
    if let Some(next) = sched.run_queue.pop_front() {
        sched.cpu.current_task = next;
        let t = sched.tasks.iter_mut().find(|t| t.id == next).unwrap();
        
        crate::serial::write(b"TICK: switch ");
        if prev_task.0 != 0 {
             crate::serial::write_num(prev_task.0);
        } else {
             crate::serial::write(b"IDLE");
        }
        crate::serial::write(b" -> ");
        crate::serial::write_num(next.0);
        crate::serial::write(b" sp=");
        crate::serial::write_hex(t.stack_ptr);

        // Probe P4: Stack Sanity Check
        if t.stack_ptr > t.stack_top {
             crate::serial::write(b" [STACK_OVERFLOW_DETECTED]");
        }
        // Check alignment
        if t.stack_ptr & 0xf != 0 {
             crate::serial::write(b" [SP_MISALIGN]");
        }

        crate::serial::write(b"\n");
        
        t.state = TaskState::Running;
        t.first_run = false; // Mark as having started execution
        // set_on_cpu requires graph lock, skipping for now to avoid deadlock in IRQ
        // store::with_store(|s| t.set_on_cpu(s, sched.cpu.thing));

        // Set Kernel Stack for Syscall/Traps
        crate::machine::machine().set_kernel_stack(t.stack_top);

        // DON'T activate address space yet - we're still on the old stack!
        // The interrupt handler will switch stacks first, then we activate.
        // Actually, we need to return info about whether to switch AS.
        // For now, let's defer activation.
        
        let new_sp = t.stack_ptr;
        
        // Now activate AFTER we have the SP but before we return
        // This way the stack switch in assembly happens with the new AS active
        t.address_space.activate();
        
        new_sp
    } else {
        // Idle? Or continue current?
        // If current was put back in queue, it might be picked again.
        // If queue empty and we have current, it was put back?
        // Wait, I put it back above. So pop_front should return it if it's the only one.
        // So this branch is mostly "No tasks at all".
        0
    }
}
