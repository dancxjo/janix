//! Thread syscall implementations

use crate::sched::{self, task::TaskId, BlockReason};
use crate::watch;
use abi::ids::WatchId;
use abi::syscall::err;
use abi::types::{WaitFlags, WakeReason};
use abi::wire::SyscallResult;
use crate::memory::map::MapPerms;

/// Spawn a new thread in the same address space as the calling thread.
/// 
/// Arguments:
/// - entry: Entry point function address
/// - arg0: Argument passed to thread via register (RDI on x86_64)
/// - stack_ptr_opt: If 0, kernel allocates user stack; otherwise use provided stack
/// 
/// Returns: New thread's TaskId on success, error code on failure
pub fn sys_thread_spawn(entry: u64, arg0: u64, stack_ptr_opt: u64) -> SyscallResult {
    if entry == 0 {
        return SyscallResult::new(err::EINVAL, 0, 0);
    }

    let result = sched::with_sched(|sched| {
        // Get current task's address space and group
        let curr_id = sched.cpu.current_task;
        let task = match sched.tasks.iter().find(|t| t.id == curr_id) {
            Some(t) => t,
            None => return Err(err::EINVAL),
        };

        let address_space = task.address_space.clone();
        let group_id = if task.group_id != 0 {
            task.group_id
        } else {
            // Create new group for this spawning task if it doesn't have one
            let new_group_id = sched.next_group_id;
            sched.next_group_id += 1;
            
            // Update the current task's group_id
            if let Some(t) = sched.tasks.iter_mut().find(|t| t.id == curr_id) {
                t.group_id = new_group_id;
            }
            new_group_id
        };

        // Allocate KERNEL stack for new thread (for TrapFrame/syscall handling)
        // This lives in kernel heap and is used for trap frames during syscalls/interrupts
        let kernel_stack_size = 64 * 1024; // 64KB kernel stack
        let kernel_stack = alloc::vec![0u8; kernel_stack_size];
        let kernel_stack_top = kernel_stack.as_ptr() as u64 + kernel_stack_size as u64;
        core::mem::forget(kernel_stack);

        // Allocate USER stack in the shared address space
        // Thread N gets stack at: 0x8F00_0000 - N * 0x10_0000 (1MB stride per thread)
        let user_stack_size: u64 = 64 * 1024; // 64KB user stack
        let user_stack_top: u64 = if stack_ptr_opt != 0 {
            // User provided their own stack
            stack_ptr_opt
        } else {
            // Calculate a unique stack region for this thread
            let thread_index = sched.next_id as u64;
            let stack_region_top = 0x8F00_0000u64 - (thread_index * 0x10_0000); // 1MB stride
            let stack_bottom = stack_region_top - user_stack_size;
            
            // Allocate physical memory for the user stack
            let phys_stack = alloc::vec![0u8; user_stack_size as usize];
            let phys_addr = crate::machine::machine().virt_to_phys(phys_stack.as_ptr() as u64);
            core::mem::forget(phys_stack);
            
            // Map it into the shared user address space
            if let Err(e) = address_space.map(
                stack_bottom,
                phys_addr,
                user_stack_size as usize,
                MapPerms::READ | MapPerms::WRITE | MapPerms::USER,
            ) {
                crate::log::klog(
                    crate::log::Level::Info,
                    "THREAD",
                    &alloc::format!("Failed to map user stack at {:#x}: {:?}", stack_bottom, e),
                );
                return Err(err::ENOMEM);
            }
            
            crate::log::klog(
                crate::log::Level::Info,
                "THREAD",
                &alloc::format!(
                    "Mapped user stack: bottom={:#x} top={:#x} phys={:#x}",
                    stack_bottom, stack_region_top, phys_addr
                ),
            );
            
            stack_region_top
        };

        // Create thread Thing in graph
        let task_thing = graph::store::with_store(|s| {
            let t = s.create_thing(graph::symbols::sym::KIND_THREAD).expect("create thread");
            if let Some(graph_tasks) = s.find_by_name(graph::symbols::sym::GRAPH_TASKS) {
                let _ = s.create_relationship(graph::symbols::sym::PRED_CONTAINS, graph_tasks, t);
            }
            t
        });

        // Create new task in the same group
        let new_id = TaskId(sched.next_id);
        sched.next_id += 1;

        let mut new_task = sched::task::Task::new_in_group(
            new_id,
            task_thing,
            kernel_stack_top,  // This is the KERNEL stack top
            address_space,
            group_id,
        );

        // Configure thread context
        // The init_task_context function uses arg0 for BOTH user RSP and RDI.
        // We need to fix this by setting up the TrapFrame correctly:
        // 1. RSP should be user_stack_top (16-byte aligned)
        // 2. RDI should be the actual arg0 passed to this syscall
        use crate::machine::{ArchTask, CpuMode, CurrentArch, TaskContext, TrapFrame};
        
        // x86-64 ABI: RSP must be 16-byte aligned BEFORE the call instruction pushes return addr
        // Since we're simulating entry (not a call), align to 16 bytes
        let aligned_user_stack = user_stack_top & !0xf;
        
        let mut ctx = TaskContext::default();
        CurrentArch::init_task_context(
            &mut ctx,
            entry,
            kernel_stack_top & !0xf,  // Kernel stack for TrapFrame
            CpuMode::User,
            aligned_user_stack,  // This sets both RSP and RDI initially
        );
        
        // Now fix up RDI to be the actual thread argument
        unsafe {
            let frame_ptr = ctx.sp as *mut TrapFrame;
            (*frame_ptr).rdi = arg0;
        }
        
        crate::log::klog(
            crate::log::Level::Info,
            "THREAD",
            &alloc::format!(
                "Thread {} context: entry={:#x} user_sp={:#x} arg0={:#x} kernel_sp={:#x}",
                new_id.0, entry, aligned_user_stack, arg0, kernel_stack_top
            ),
        );
        
        new_task.stack_ptr = ctx.sp;
        new_task.stack_top = kernel_stack_top;

        // Initialize state
        graph::store::with_store(|s| new_task.set_state(s, sched::task::TaskState::Ready));

        let thing = new_task.thing;
        sched.tasks.push(new_task);
        sched.run_queue.push_back(new_id, thing);

        Ok(new_id.0)
    });

    match result {
        Ok(tid) => SyscallResult::new(0, tid, 0),
        Err(e) => SyscallResult::new(e, 0, 0),
    }
}

/// Exit the current thread with the given exit code.
/// Wakes any threads waiting to join this thread.
pub fn sys_thread_exit(code: i32) -> ! {
    // Mark thread as dead and set exit code
    if let Some(task_id) = sched::current_task_handle() {
        sched::with_sched(|sched| {
            if let Some(task) = sched.tasks.iter_mut().find(|t| t.id == task_id) {
                task.exit_code = Some(code);
                
                // Wake all joiners
                let joiners: alloc::vec::Vec<TaskId> = task.joiners.drain(..).collect();
                for joiner in joiners {
                    if let Some(joining_task) = sched.tasks.iter_mut().find(|t| t.id == joiner) {
                        if matches!(joining_task.state, sched::task::TaskState::Blocked(BlockReason::Join(target)) if target == task_id) {
                            joining_task.state = sched::task::TaskState::Ready;
                            joining_task.wake_reason = Some(WakeReason::join_completed(code as u64));
                            let thing = joining_task.thing;
                            sched.run_queue.push_back(joiner, thing);
                        }
                    }
                }
            }
        });
    }

    // Use existing exit mechanism
    sched::exit_current_task(code);
}

/// Wait for a thread to exit.
/// 
/// Arguments:
/// - tid: Target thread's TaskId
/// - timeout_ticks: 0 = infinite wait, otherwise tick deadline
/// 
/// Returns: (status, exit_code) on success
pub fn sys_thread_join(tid: u64, timeout_ticks: u64) -> SyscallResult {
    let target_id = TaskId(tid);
    
    let result = sched::with_sched(|sched| {
        // Check if target exists
        let target = match sched.tasks.iter().find(|t| t.id == target_id) {
            Some(t) => t,
            None => return Err(err::ENOENT),
        };

        // If target is already dead, return immediately
        if let Some(code) = target.exit_code {
            return Ok(code as u64);
        }

        // Register current task as a joiner
        let curr_id = sched.cpu.current_task;
        if let Some(target) = sched.tasks.iter_mut().find(|t| t.id == target_id) {
            target.joiners.push(curr_id);
        }

        // Block current task
        if let Some(curr) = sched.tasks.iter_mut().find(|t| t.id == curr_id) {
            curr.state = sched::task::TaskState::Blocked(BlockReason::Join(target_id));
            curr.wake_reason = None;
        }

        // Register timeout if specified
        if timeout_ticks > 0 {
            let deadline = crate::sched::TIMER_TICKS.load(core::sync::atomic::Ordering::Relaxed) + timeout_ticks;
            watch::register_wait(curr_id, &[], WaitFlags::empty(), Some(deadline));
        }

        Err(err::EAGAIN) // Signal that we need to yield
    });

    match result {
        Ok(code) => SyscallResult::new(0, code, 0),
        Err(err::EAGAIN) => {
            // Yield to let other threads run; will resume when target exits
            sched::yield_current();
            // Check wake reason when we return
            if let Some(reason) = sched::take_wake_reason() {
                SyscallResult::new(0, reason.arg0(), 0)
            } else {
                SyscallResult::new(err::EAGAIN, 0, 0)
            }
        }
        Err(e) => SyscallResult::new(e, 0, 0),
    }
}

/// Block current thread until a watch event arrives.
pub fn sys_thread_block_on_watch(watch_id: u64) -> SyscallResult {
    if let Some(task_id) = sched::current_task_handle() {
        let wid = WatchId(watch_id);
        
        // Register wait on the watch
        watch::register_wait(task_id, &[wid], WaitFlags::empty(), None);
        
        // Block current task
        sched::block_current(BlockReason::WatchWait);
        
        // Yield to scheduler
        sched::yield_current();
        
        SyscallResult::new(0, 0, 0)
    } else {
        SyscallResult::new(err::EINVAL, 0, 0)
    }
}
