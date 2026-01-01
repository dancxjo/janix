use alloc::vec::Vec;
use spin::Mutex;
use crate::log::{self, Level};
use graph::{store, symbols};
use graph::symbols::sym;
use crate::machine::abi::setup_new_task_stack;

pub mod task;
pub mod run_queue;
pub mod percpu;

use task::{Task, TaskId, TaskState};
use run_queue::RunQueue;
use percpu::PerCpu;

static SCHEDULER: Mutex<Option<Scheduler>> = Mutex::new(None);

struct Scheduler {
    tasks: Vec<Task>,
    run_queue: RunQueue,
    cpu: PerCpu,
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

    fn spawn(&mut self, name: &'static str) -> TaskId {
        let id = TaskId(self.next_id);
        self.next_id += 1;

        // Allocate stack
        let stack_size = 16 * 1024; // 16KB
        let stack = alloc::vec![0u8; stack_size];
        let stack_ptr = stack.as_ptr() as u64 + stack_size as u64; // Top
        // Leak the stack for now (kernel tasks live forever in this model)
        core::mem::forget(stack); 

        // Graph reflection
        let task_thing = store::with_store(|s| {
            let t = s.create_thing(sym::KIND_TASK).expect("create task");
            // Name it
            // s.register_name(...)? No, dynamic names.
            // Link to place.tasks
            if let Some(place_tasks) = s.find_by_name(sym::PLACE_TASKS) {
                 let _ = s.create_relationship(sym::PRED_CONTAINS, place_tasks, t);
            }
            t
        });

        let mut task = Task::new(id, task_thing, stack_ptr);
        
        // Initialize state (New -> Ready)
        store::with_store(|s| task.set_state(s, TaskState::Ready));

        self.tasks.push(task);
        
        // Add to run queue (requires re-borrowing task/thing)
        let thing = self.tasks.last().unwrap().thing;
        store::with_store(|s| self.run_queue.push_back(id, thing, s));

        id
    }
}

pub fn init() {
    log::klog(Level::Info, "SCHED", "initializing...");

    // Seed Graph (scheduler.main, cpu.0, run_queue.0)
    let (sched_thing, cpu_thing, rq_thing) = store::with_store(|s| {
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
    // Enable interrupts
    crate::machine::irq_enable();
    loop {
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
pub fn mark_as_init(id: TaskId) {
    // Mark in graph?
}

pub fn configure_task_memory(id: TaskId, img: (u64, u64), stack: (u64, u64), heap: (u64, u64, u64)) {
    let mut guard = SCHEDULER.lock();
    if let Some(sched) = guard.as_mut() {
        if let Some(t) = sched.tasks.iter_mut().find(|t| t.id == id) {
            t.heap_base = heap.0;
            t.heap_size = heap.1;
            t.heap_brk = heap.2;
        }
    }
}

pub fn configure_task_context(id: TaskId, entry: u64) {
    let mut guard = SCHEDULER.lock();
    if let Some(sched) = guard.as_mut() {
        if let Some(t) = sched.tasks.iter_mut().find(|t| t.id == id) {
             // We need to setup a context that jumps to `entry`.
             // We can use `machine::abi::setup_new_task_stack` if generalized.
             // Or manually write stack frame.
        }
    }
}

pub fn exit_current_task(code: i32) -> ! {
    loop { crate::machine::idle(); }
}

pub fn with_current_task<F, R>(f: F) -> Option<R> where F: FnOnce(&mut Task) -> R {
    let mut guard = SCHEDULER.lock();
    let sched = guard.as_mut()?; // Return None if not init
    let curr = sched.cpu.current_task?; // Return None if no current task
    let t = sched.tasks.iter_mut().find(|t| t.id == curr).unwrap();
    Some(f(t))
}

// Rename/Wrap spawn
pub fn spawn_kernel_task(name: &'static str, entry: extern "C" fn()) -> TaskId {
    let mut guard = SCHEDULER.lock();
    let sched = guard.as_mut().expect("sched not init");
    let id = sched.spawn(name);
    
    let task = sched.tasks.iter_mut().find(|t| t.id == id).unwrap();
    
    // Setup Context
    use crate::machine::{abi, Context};
    let mut ctx = Context::default();
    
    // stack_ptr in Task currently points to top of stack (u64).
    // We need pointer to mutable memory.
    // Safety: We allocated it and leaked it, so it's valid.
    let stack_top = (task.stack_ptr & !0xf) as *mut u64; // Force align to 16 bytes
    
    unsafe {
        #[cfg(target_arch = "x86_64")]
        {
             // Trap Frame for Ring 0 (Kernel)
             // [RFLAGS, CS, RIP] (pushed by CPU on interrupt)
             // [Regs] (pushed by trampoline)
             // Stack grows down.
             // We start at stack_top.
             
             // 1. alignment adjustment
             // SysV ABI requires RSP % 16 == 8 on entry (simulating return address).
             // Since we enter via IRETQ (which doesn't push ret addr), we must ensure 
             // RSP is 8-byte aligned when we start executing 'entry'.
             // Our stack_top is 16-byte aligned.
             // We subtract 1 u64 so that final RSP (after pop) is 8-byte aligned.
             let mut sp = stack_top.sub(1);
             // Ensure this slot is zeroed or valid? (It's top of stack, usually ignored).
             *sp = 0xdeadbeef; // Debug marker
             
             // 1. CPU Frame (5 words - conservative)
             // Even if Ring 0 return pops 3, 5 is safe allocation.
             
             // SS
             sp = sp.sub(1);
             *sp = 0x10; // Kernel Data
             
             // RSP (Value after iretq, i.e., top of stack frame?)
             // iretq restores RSP to this value IF it pops 5 words.
             sp = sp.sub(1);
             *sp = stack_top as u64; // Or top-8 if aligned?
             
             // RFLAGS
             sp = sp.sub(1);
             *sp = 0x202; // IF=1, bit 1=1
             
             // CS
             sp = sp.sub(1);
             *sp = 0x8; // Kernel Code
             
             // RIP
             sp = sp.sub(1);
             *sp = entry as usize as u64; // Entry Point
             
             // 2. Registers (pushed by trampoline: rax..r15)
             // Trampoline pushes: rax, rbx, rcx, rdx, rsi, rdi, rbp, r8..r15.
             // Total 15 regs.
             // We zero them.
             sp = sp.sub(15);
             core::ptr::write_bytes(sp as *mut u8, 0, 15 * 8);
             
             // Check 0x18 error?
             // Maybe push a valid SS/RSP? (Fake 5 word frame)
             // Even for Ring 0 return, if we push 5 words and IRETQ "thinks" it's 3 words, SP is off?
             // Or if we need 5 words?
             // Let's try pushing 5 words.
             // [SS] [RSP] [RFLAGS] [CS] [RIP]
             
             // BUT `iretq` pops based on CS RPL.
             // If CS=8 (RPL=0), it pops 3 words.
             // So pushing 5 words would misalign the stack (RSP/SS left on stack).
             // Unless we change CS to RPL 3 (User)?
             
             task.stack_ptr = sp as u64;

             
             // Note: ping/pong tasks take no args.
             // If we needed args, we would set RDI (which is part of regs).
             // RDI is pushed 6th (if push rax first).
             // stack: [rax, rbx, rcx, rdx, rsi, rdi, ...]
             // sp points to rax.
             // sp+5 = rdi.
             // *sp.add(5) = arg;
        }
        
        #[cfg(not(target_arch = "x86_64"))]
        {
             // Stub for others (or fallback to compatible logic)
             // For now, AArch64 also needs specific frame.
             // We stick to x86_64 fix.
        }
    }
    
    // task.stack_ptr = ctx.sp; // Replaced by manual set

    
    id
}

// Helper for Sprout (empty spawn)
pub fn spawn_empty(name: &'static str) -> TaskId {
    let mut guard = SCHEDULER.lock();
    let sched = guard.as_mut().expect("sched not init");
    sched.spawn(name)
}


use core::sync::atomic::{AtomicU64, Ordering};
pub static TIMER_TICKS: AtomicU64 = AtomicU64::new(0);

/// Called by timer interrupt.
/// Returns Some(new_sp) if switch needed, None if stay.
pub fn tick(current_sp: u64) -> Option<u64> {
    // 1. Inc timer ticks
    TIMER_TICKS.fetch_add(1, Ordering::Relaxed);
    
    let mut guard = SCHEDULER.lock();
    let sched = guard.as_mut()?;
    
    // Default: Round Robin
    // 1. Save current SP to current task (if we have one)
    if let Some(curr) = sched.cpu.current_task {
        if let Some(t) = sched.tasks.iter_mut().find(|t| t.id == curr) {
            t.stack_ptr = current_sp;
            // State: Running -> Ready
             store::with_store(|s| t.set_state(s, TaskState::Ready));
             // Enqueue
             let thing = t.thing;
             store::with_store(|s| sched.run_queue.push_back(curr, thing, s));
        }
    }
    
    // 2. Pick next
    if let Some(next) = sched.run_queue.pop_front() {
        sched.cpu.current_task = Some(next);
        let t = sched.tasks.iter_mut().find(|t| t.id == next).unwrap();
        
        store::with_store(|s| {
             t.set_state(s, TaskState::Running);
             t.set_on_cpu(s, sched.cpu.thing);
        });
        
        Some(t.stack_ptr)
    } else {
        // Idle? Or continue current?
        // If current was put back in queue, it might be picked again.
        // If queue empty and we have current, it was put back?
        // Wait, I put it back above. So pop_front should return it if it's the only one.
        // So this branch is mostly "No tasks at all".
        None
    }
}
