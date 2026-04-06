//! Blocking primitives for task synchronization.

use crate::task::TaskState;
use crate::BootRuntime;
use crate::BootTasking;

use super::SCHEDULER;

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
        let current_id = match sched.state.per_cpu.get(cpu).and_then(|pc| pc.current) {
            Some(id) => id,
            None => {
                rt.irq_restore(_irq);
                return;
            }
        };

        // Move current from Running to Blocked
        if let Some(task) = crate::task::registry::get_task_mut::<R>(current_id) {
            if task.wake_pending {
                task.wake_pending = false;
                rt.irq_restore(_irq);
                return;
            }
            task.state = TaskState::Blocked;

            // Queue graph state update
            crate::sched::ring::push_task_state::<R>(current_id, "blocked");
        }

        // Add to wait queue
        sched.state.wait_queue.push_back(current_id);

        // Schedule next
        sched.prepare_schedule()
    };

    if let Some(switch) = switch_params {
        rt.tasking().activate_address_space(switch.to_aspace);

        unsafe {
            rt.tasking()
                .switch(&mut *switch.from_ctx, &*switch.to_ctx, switch.to_tid);
        }
    }

    rt.irq_restore(_irq);
}

pub fn wake_task<R: BootRuntime>(id: u64) {
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

    let tid = id;

    // Remove from wait queue if present
    if let Some(pos) = sched.state.wait_queue.iter().position(|&wid| wid == tid) {
        sched.state.wait_queue.remove(pos);
    }
    // Timed futex waits and sleeps park blocked tasks in the sleep queue.
    // Scrub those entries too so an early wake cannot enqueue the task twice.
    sched.state.sleep_queue.retain(|_, tids| {
        tids.retain(|&sleep_tid| sleep_tid != tid);
        !tids.is_empty()
    });

    // Update state to Runnable and add to runq
    let mut was_blocked = false;
    if let Some(task) = crate::task::registry::get_task_mut::<R>(tid) {
        if task.state == TaskState::Blocked {
            task.state = TaskState::Runnable;
            task.enqueued_at_tick = super::TICK_COUNT.load(core::sync::atomic::Ordering::Relaxed);
            let priority = task.priority;
            let affinity = task.affinity;

            let target_cpu = match affinity {
                crate::task::Affinity::Pinned(cpu) => cpu,
                crate::task::Affinity::Any => {
                    // Try to wake to the last CPU it ran on to avoid immediate migration
                    task.last_cpu
                        .unwrap_or_else(|| super::current_cpu_index::<R>())
                }
            };

            let safe_cpu = if target_cpu < sched.state.per_cpu.len() {
                target_cpu
            } else {
                0
            };
            sched.state.enqueue_task(safe_cpu, priority as usize, tid);

            // If the woken task has higher priority than the currently running
            // task on the target CPU, request a reschedule so we preempt
            // mid-slice rather than waiting for timeslice expiry.
            let current_prio = sched.state.per_cpu[safe_cpu]
                .current
                .and_then(|cid| crate::task::registry::get_task::<R>(cid))
                .map(|t| t.priority as usize)
                .unwrap_or(0);

            if (priority as usize) > current_prio {
                if safe_cpu == super::current_cpu_index::<R>() {
                    sched.state.need_resched = true;
                }
            }

            // If the target CPU is not the current one, send an IPI to wake it up
            if safe_cpu != super::current_cpu_index::<R>() {
                rt.send_ipi(safe_cpu, 0x30); // Use IRQ_RESCHED_VECTOR
            }

            was_blocked = true;
        }
    }

    if was_blocked {
        // Queue graph state update
        crate::sched::ring::push_task_state::<R>(tid, "runnable");
    } else {
        // If the task wasn't blocked, it means it was already runnable or running.
        // In this case, we just set wake_pending to true so it doesn't block
        // if it tries to block immediately after this wake.
        // This handles cases where a task is woken multiple times or woken while running.
        if let Some(task) = crate::task::registry::get_task_mut::<R>(tid) {
            task.wake_pending = true;
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
pub unsafe fn wake_task_erased(id: u64) {
    let ptr = WAKE_TASK_HOOK.load(core::sync::atomic::Ordering::SeqCst);
    if !ptr.is_null() {
        let hook: fn(u64) = unsafe { core::mem::transmute(ptr) };
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
