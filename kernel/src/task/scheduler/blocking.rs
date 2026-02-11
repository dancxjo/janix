//! Blocking primitives for task synchronization.

use crate::BootRuntime;
use crate::BootTasking;
use crate::task::TaskState;

use super::SCHEDULER;
use super::graphify;
use super::types::Scheduler;

static BLOCK_CURRENT_HOOK: core::sync::atomic::AtomicPtr<()> =
    core::sync::atomic::AtomicPtr::new(core::ptr::null_mut());
pub(crate) static WAKE_TASK_HOOK: core::sync::atomic::AtomicPtr<()> =
    core::sync::atomic::AtomicPtr::new(core::ptr::null_mut());

pub fn block_current<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let switch_params = {
        let lock = SCHEDULER.lock();
        let ptr = lock.expect("Scheduler not initialized");
        let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };

        let cpu = super::current_cpu_index::<R>();
        let current_id = match sched.per_cpu.get(cpu).and_then(|pc| pc.current) {
            Some(id) => id,
            None => {
                rt.irq_restore(_irq);
                return;
            }
        };

        // Move current from Running to Blocked
        if let Some(idx) = sched.tasks.iter().position(|t| t.id == current_id) {
            if sched.tasks[idx].wake_pending {
                sched.tasks[idx].wake_pending = false;
                rt.irq_restore(_irq);
                return;
            }
            sched.tasks[idx].state = TaskState::Blocked;

            // Queue graph state update
            graphify::update_task_state(current_id, "blocked");
        }

        // Add to wait queue
        sched.wait_queue.push_back(current_id);

        // Schedule next
        sched.prepare_schedule()
    };

    if let Some(switch) = switch_params {
        let cr3_before = rt.debug_active_aspace_root();

        rt.tasking().activate_address_space(switch.to_aspace);

        let cr3_after = rt.debug_active_aspace_root();

        {
            let lock = SCHEDULER.lock();
            let ptr = lock.expect("Scheduler not initialized");
            let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
            sched.log_context_switch(&switch, cr3_before, cr3_after);
        }

        unsafe {
            rt.tasking()
                .switch(&mut *switch.from_ctx, &*switch.to_ctx, switch.to_tid);
        }
    }

    rt.irq_restore(_irq);
}

pub fn wake_task<R: BootRuntime>(id: usize) {
    use crate::task::TaskId;

    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let lock = SCHEDULER.lock();
    let ptr = match *lock {
        Some(p) => p,
        None => {
            rt.irq_restore(_irq);
            return;
        }
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
            let priority = sched.tasks[idx].priority;
            let affinity = sched.tasks[idx].affinity;

            let target_cpu = match affinity {
                crate::task::Affinity::Pinned(cpu) => cpu,
                crate::task::Affinity::Any => {
                    // Try to wake to the last CPU it ran on to avoid immediate migration
                    sched.tasks[idx]
                        .last_cpu
                        .unwrap_or_else(|| super::current_cpu_index::<R>())
                }
            };

            let safe_cpu = if target_cpu < sched.per_cpu.len() {
                target_cpu
            } else {
                0
            };
            sched.per_cpu[safe_cpu].runq[priority as usize].push_back(tid);

            // If the target CPU is not the current one, send an IPI to wake it up
            if safe_cpu != super::current_cpu_index::<R>() {
                rt.send_ipi(safe_cpu, 0x30); // Use IRQ_RESCHED_VECTOR
            }

            sched.tasks[idx].wake_pending = false;

            // Queue graph state update
            graphify::update_task_state(tid, "runnable");
        } else {
            sched.tasks[idx].wake_pending = true;
        }
    }

    rt.irq_restore(_irq);
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
