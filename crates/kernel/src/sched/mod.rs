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

    fn spawn(&mut self, _name: &'static str, as_opt: Option<Arc<AddressSpace>>) -> TaskId {
        let id = TaskId(self.next_id);
        self.next_id += 1;

        // Allocate stack
        let stack_size = 64 * 1024; // 64KB
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

        let address_space = as_opt.unwrap_or_else(|| Arc::new(AddressSpace::new().expect("failed create AS")));
        let mut task = Task::new(id, task_thing, stack_ptr, address_space);
        
        // Initialize state (New -> Ready)
        store::with_store(|s| task.set_state(s, TaskState::Ready));

        self.tasks.push(task);
        
        // Add to run queue (requires re-borrowing task/thing)
        let thing = self.tasks.last().unwrap().thing;
        self.run_queue.push_back(id, thing);

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

pub fn configure_task_context(id: TaskId, entry: u64, _user_stack: u64) {
    let mut guard = SCHEDULER.lock();
    if let Some(sched) = guard.as_mut() {
        if let Some(task) = sched.tasks.iter_mut().find(|t| t.id == id) {
             // Use Kernel Stack Top implicitly allocated by spawn
             let k_stack_top = (task.stack_ptr & !0xf) as *mut u64;

             unsafe {
                #[cfg(target_arch = "x86_64")]
                {
                     // Alignment: 
                     // Stack Top (16 aligned).
                     // Pushes: SS, RSP, RFLAGS, CS, RIP (5 words, 40 bytes).
                     // Regs (15 words, 120 bytes).
                     // Total 160 bytes. 16-byte aligned.
                     // Matches ABI. NO padding needed.
                     let mut sp = k_stack_top.sub(1);
                     *sp = 0xdeadbeef; // Align

                     // CPU Frame (5 words)
                     sp = sp.sub(1); *sp = 0x1b; // SS (User Data)
                     sp = sp.sub(1); *sp = _user_stack; // RSP (User Stack)
                     sp = sp.sub(1); *sp = 0x202; // RFLAGS
                     sp = sp.sub(1); *sp = 0x23; // CS (User Code)
                     sp = sp.sub(1); *sp = entry; // RIP 

                     // Regs
                     sp = sp.sub(15);
                     core::ptr::write_bytes(sp as *mut u8, 0, 15 * 8);
                     
                     task.stack_ptr = sp as u64;
                }

                #[cfg(target_arch = "aarch64")]
                {
                     use crate::machine::aarch64::TrapFrame;
                     let layout = core::alloc::Layout::new::<TrapFrame>();
                     let mut sp = k_stack_top as *mut u8;
                     sp = sp.sub(layout.size());
                     core::ptr::write_bytes(sp, 0, layout.size());
                     let frame = &mut *(sp as *mut TrapFrame);
                     frame.elr_el1 = entry;
                     frame.spsr_el1 = 0x3c5; 
                     task.stack_ptr = sp as u64;
                }

                #[cfg(target_arch = "riscv64")]
                {
                     use crate::machine::riscv64::TrapFrame;
                     let layout = core::alloc::Layout::new::<TrapFrame>();
                     let mut sp = k_stack_top as *mut u8;
                     sp = sp.sub(layout.size());
                     core::ptr::write_bytes(sp, 0, layout.size());
                     let frame = &mut *(sp as *mut TrapFrame);
                     frame.sepc = entry;
                     frame.sstatus = (1 << 8) | (1 << 5); 
                     task.stack_ptr = sp as u64;
                }

                #[cfg(target_arch = "loongarch64")]
                {
                     use crate::machine::loongarch64::TrapFrame;
                     let layout = core::alloc::Layout::new::<TrapFrame>();
                     let mut sp = k_stack_top as *mut u8;
                     sp = sp.sub(layout.size());
                     core::ptr::write_bytes(sp, 0, layout.size());
                     let frame = &mut *(sp as *mut TrapFrame);
                     frame.era = entry;
                     frame.prmd = 0x4;
                     task.stack_ptr = sp as u64;
                }
             }
        }
    }
}

pub fn exit_current_task(_code: i32) -> ! {
    loop { crate::machine::idle(); }
}

pub fn with_current_task<F, R>(f: F) -> Option<R> where F: FnOnce(&mut Task) -> R {
    let mut guard = SCHEDULER.lock();
    let sched = guard.as_mut()?; // Return None if not init
    let curr = sched.cpu.current_task?; // Return None if no current task
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
            if let Some(tid) = sched.cpu.current_task {
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
    let mut guard = SCHEDULER.lock();
    let sched = guard.as_mut().expect("sched not init");
    
    // Use Shared Kernel Address Space for kernel threads
    let k_as = AddressSpace::new_kernel_share().expect("failed share kernel AS");
    let id = sched.spawn(name, Some(Arc::new(k_as)));
    
    let task = sched.tasks.iter_mut().find(|t| t.id == id).unwrap();
    
    // Setup Context
    use crate::machine::Context;
    let mut _ctx = Context::default();
    
    // stack_ptr in Task currently points to top of stack (u64).
    // We need pointer to mutable memory.
    // Safety: We allocated it and leaked it, so it's valid.
    let stack_top = (task.stack_ptr & !0xf) as *mut u64; // Force align to 16 bytes
    
    unsafe {
        #[cfg(target_arch = "x86_64")]
        {
             // Trap Frame for Ring 0 (Kernel) -> Ring 0
             // IRETQ from Ring 0 to Ring 0 pops: [RIP] [CS] [RFLAGS] (3 words).
             // It does NOT pop RSP or SS.
             
             // Alignment Requirements:
             // SysV ABI requires RSP % 16 == 8 on entry to the function (simulating a 'call').
             // After 'iretq' pops 3 words (24 bytes), we want RSP to end in ...8.
             // implied: RSP_before_iret + 24 = ...8
             //          RSP_before_iret + 8  = ...8 (mod 16)
             //          RSP_before_iret      = ...0 (mod 16)
             
             // We start with stack_top (16-byte aligned).
             // We push 3 words (24 bytes).
             // To have RSP_before_iret aligned to 16, we need 8 bytes of padding.
             // stack_top (0) - 8 (pad) - 24 (frame) = -32 = 0 (mod 16).
             
             let mut sp = stack_top;
             
             // 1. Padding / Fake Return Address (8 bytes)
             sp = sp.sub(1);
             *sp = 0xdeadbeef;
             
             // 2. CPU Frame (3 words: RFLAGS, CS, RIP)
             // RFLAGS
             sp = sp.sub(1);
             *sp = 0x202; // IF=1, enable interrupts
             
             // CS
             sp = sp.sub(1);
             *sp = 0x08; // Kernel Code (RPL 0)
             
             // RIP
             sp = sp.sub(1);
             *sp = entry as usize as u64;
             
             // Registers?
             // The trampoline pops registers (r15..rax). 
             // We need to push 15 words of zeros for the registers too!
             // Wait, `iretq` is the END of the trampoline.
             // The `task_switch` or however we get here...
             // Ah, `switch_to` switches context.
             // `sched::tick` switches stack.
             // The `timer_interrupt_trampoline` pops `regs`, THEN does `iretq`.
             // So we MUST have the registers on the stack below the frame!
             
             // Full layout:
             // [Padding 8]
             // [RFLAGS] [CS] [RIP]  (24)
             // [Regs 15 * 8]        (120)
             
             // Register push logic:
             sp = sp.sub(15);
             core::ptr::write_bytes(sp as *mut u8, 0, 15 * 8);
             
             task.stack_ptr = sp as u64;
             
             // Note: ping/pong tasks take no args.
        }

        #[cfg(target_arch = "aarch64")]
        {
             use crate::machine::aarch64::TrapFrame;
             // Layout: 36 u64s (288 bytes)
             let layout = core::alloc::Layout::new::<TrapFrame>();
             let mut sp = stack_top as *mut u8;
             sp = sp.sub(layout.size());
             
             // Zero the frame
             core::ptr::write_bytes(sp, 0, layout.size());
             
             let frame = &mut *(sp as *mut TrapFrame);
             
             frame.elr_el1 = entry as usize as u64;
             frame.spsr_el1 = 0x3c5; // EL1h, DAIF=1111 (Masked on entry) -> Task enables IRQ if it wants?
                                     // Actually, kernel threads usually start with IRQs enabled?
                                     // If we use 0x05 (DAIF=0000), IRQs are enabled.
                                     // Let's enable them later via irq_enable?
                                     // Safer to start masked.
             
             task.stack_ptr = sp as u64;
        }

        #[cfg(target_arch = "riscv64")]
        {
             use crate::machine::riscv64::TrapFrame;
             let layout = core::alloc::Layout::new::<TrapFrame>();
             let mut sp = stack_top as *mut u8;
             sp = sp.sub(layout.size());
             
             core::ptr::write_bytes(sp, 0, layout.size());
             
             let frame = &mut *(sp as *mut TrapFrame);
             
             frame.sepc = entry as usize as u64;
             // sstatus: SPP=1 (Supervisor), SPIE=1 (Enable IRQ on restore)
             frame.sstatus = (1 << 8) | (1 << 5); 
             
             task.stack_ptr = sp as u64;
        }

        #[cfg(target_arch = "loongarch64")]
        {
             use crate::machine::loongarch64::TrapFrame;
             let layout = core::alloc::Layout::new::<TrapFrame>();
             let mut sp = stack_top as *mut u8;
             sp = sp.sub(layout.size());
             
             core::ptr::write_bytes(sp, 0, layout.size());
             
             let frame = &mut *(sp as *mut TrapFrame);
             
             frame.era = entry as usize as u64;
             // PRMD: PPLV=0 (Kernel), PIE=1 (Enable IRQ on restore)
             frame.prmd = 0x4;
             
             task.stack_ptr = sp as u64;
        }
             // If we needed args, we would set RDI (which is part of regs).
             // RDI is pushed 6th (if push rax first).
             // stack: [rax, rbx, rcx, rdx, rsi, rdi, ...]
             // sp points to rax.
             // sp+5 = rdi.
             // *sp.add(5) = arg;
        }
        

    
    // task.stack_ptr = ctx.sp; // Replaced by manual set

    
    id
}

// Helper for Sprout (empty spawn)
pub fn spawn_empty(name: &'static str) -> TaskId {
    let mut guard = SCHEDULER.lock();
    let sched = guard.as_mut().expect("sched not init");
    sched.spawn(name, None)
}


use core::sync::atomic::{AtomicU64, Ordering};
pub static TIMER_TICKS: AtomicU64 = AtomicU64::new(0);

/// Called by timer interrupt.
/// Returns Some(new_sp) if switch needed, None if stay.
pub fn tick(current_sp: u64) -> Option<u64> {
    // 1. Inc timer ticks
    let ticks = TIMER_TICKS.fetch_add(1, Ordering::Relaxed);
    if ticks == 0 {
        crate::serial::write(b"SCHED: first tick!\n");
    }
    
    let mut guard = SCHEDULER.lock();
    let sched = guard.as_mut()?;
    
    // Default: Round Robin
    // 1. Save current SP to current task (if we have one)
    if let Some(curr) = sched.cpu.current_task {
        if let Some(t) = sched.tasks.iter_mut().find(|t| t.id == curr) {
            t.stack_ptr = current_sp;
            // State: Running -> Ready
             t.state = TaskState::Ready;
             // Enqueue
             let thing = t.thing;
             sched.run_queue.push_back(curr, thing);
        }
    }
    
    // 2. Pick next
    if let Some(next) = sched.run_queue.pop_front() {
        sched.cpu.current_task = Some(next);
        let t = sched.tasks.iter_mut().find(|t| t.id == next).unwrap();
        
        t.state = TaskState::Running;
        // set_on_cpu requires graph lock, skipping for now to avoid deadlock in IRQ
        // store::with_store(|s| t.set_on_cpu(s, sched.cpu.thing));

        // Set Kernel Stack for Syscall/Traps
        crate::machine::machine().set_kernel_stack(t.stack_top);

        // Activate Address Space
        t.address_space.activate();
        
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
