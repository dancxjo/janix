//! Blocking primitives for task synchronization.

use crate::BootRuntime;
use crate::BootTasking;
use crate::task::TaskState;

use super::SCHEDULER;

use super::types::Scheduler;

static BLOCK_CURRENT_HOOK: core::sync::atomic::AtomicPtr<()> =
    core::sync::atomic::AtomicPtr::new(core::ptr::null_mut());
pub(crate) static WAKE_TASK_HOOK: core::sync::atomic::AtomicPtr<()> =
    core::sync::atomic::AtomicPtr::new(core::ptr::null_mut());

pub fn block_current<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let mut blocked_id = None;
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
        if let Some(mut task) = crate::task::registry::get_task_mut::<R>(current_id) {
            if task.wake_pending {
                task.wake_pending = false;
                rt.irq_restore(_irq);
                return;
            }
            task.state = TaskState::Blocked;
            blocked_id = Some(current_id);
        }

        // Add to wait queue
        sched.state.wait_queue.push_back(current_id);

        // Schedule next
        sched.prepare_schedule()
    };

    if let Some(id) = blocked_id {
        // Queue graph state update outside the lock to prevent deadlock
        // when push_task_state wakes the drain task.
        crate::sched::ring::push_task_state::<R>(id, "blocked");
    }

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

    let mut safe_cpu = 0;
    let mut is_remote = false;
    let mut was_blocked = false;
    let mut task_priority = 0;

    // 1. Lock REGISTRY to update task state and extract scheduling requirements
    if let Some(mut task) = crate::task::registry::get_task_mut::<R>(id) {
        if task.state == TaskState::Blocked {
            task.state = TaskState::Runnable;
            task.enqueued_at_tick = super::TICK_COUNT.load(core::sync::atomic::Ordering::Relaxed);
            task_priority = task.priority as usize;

            let target_cpu = match task.affinity {
                crate::task::Affinity::Pinned(cpu) => cpu,
                crate::task::Affinity::Any => task
                    .last_cpu
                    .unwrap_or_else(|| super::current_cpu_index::<R>()),
            };
            safe_cpu = target_cpu;
            was_blocked = true;
        } else {
            task.wake_pending = true;
        }
    }

    // 2. Lock SCHEDULER to update queues if the task was blocked
    if was_blocked {
        let lock = SCHEDULER.lock();
        if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };

            if let Some(pos) = sched.state.wait_queue.iter().position(|&wid| wid == id) {
                sched.state.wait_queue.remove(pos);
            }
            sched.state.sleep_queue.retain(|_, tids| {
                tids.retain(|&sleep_tid| sleep_tid != id);
                !tids.is_empty()
            });

            if safe_cpu >= sched.state.per_cpu.len() {
                safe_cpu = 0;
            }

            sched.state.enqueue_task(safe_cpu, task_priority, id);

            let current_prio = sched.state.per_cpu[safe_cpu]
                .current
                .and_then(|cid| crate::task::registry::get_task::<R>(cid))
                .map(|t| t.priority as usize)
                .unwrap_or(0);

            if task_priority > current_prio {
                if safe_cpu == super::current_cpu_index::<R>() {
                    sched.state.per_cpu[safe_cpu].need_resched = true;
                } else {
                    super::GLOBAL_NEED_RESCHED.store(true, core::sync::atomic::Ordering::Release);
                }
            }

            is_remote = safe_cpu != super::current_cpu_index::<R>();
        }
    }

    // 3. Queue graph state update OUTSIDE of all locks
    if was_blocked {
        crate::sched::ring::push_task_state::<R>(id, "runnable");
    }

    // 4. Send IPI OUTSIDE of all locks
    if is_remote {
        rt.send_ipi(safe_cpu, 0x30); // Use IRQ_RESCHED_VECTOR
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
