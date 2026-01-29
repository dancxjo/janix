//! Timing and yield functions.

use crate::BootRuntime;
use crate::BootTasking;

use super::SCHEDULER;
use super::types::{ScheduleReason, Scheduler};

#[cfg(any(feature = "sched_debug", debug_assertions))]
use super::log_context_switch;

pub fn yield_now<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let switch_params = {
        let lock = SCHEDULER.lock();
        let ptr = lock.expect("Scheduler not initialized");
        let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
        sched.schedule_point(ScheduleReason::CooperativeYield)
    };

    if let Some(switch) = switch_params {
        #[cfg(any(feature = "sched_debug", debug_assertions))]
        let cr3_before = rt.debug_active_aspace_root();

        rt.tasking().activate_address_space(switch.to_aspace);

        #[cfg(any(feature = "sched_debug", debug_assertions))]
        let cr3_after = rt.debug_active_aspace_root();
        #[cfg(any(feature = "sched_debug", debug_assertions))]
        log_context_switch::<R>(&switch, cr3_before, cr3_after);

        unsafe {
            rt.tasking().switch(&mut *switch.from_ctx, &*switch.to_ctx);
        }
    }

    rt.irq_restore(_irq);
}

/// True blocking sleep - puts task in sleep queue and reschedules
pub fn sleep_ticks<R: BootRuntime>(ticks: u64) {
    if ticks == 0 {
        // Zero sleep = just yield once
        yield_now::<R>();
        return;
    }

    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let switch_params = {
        let lock = SCHEDULER.lock();
        let ptr = lock.expect("Scheduler not initialized");
        let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };

        // Get current task ID
        let current_id = match sched.current {
            Some(id) => id,
            None => {
                // No current task (shouldn't happen)
                return;
            }
        };

        // Calculate wake time and add to sleep queue
        let wake_tick = super::TICK_COUNT.load(core::sync::atomic::Ordering::Relaxed) + ticks;
        sched.sleep_queue.push_back(super::types::SleepEntry {
            task_id: current_id,
            wake_tick,
        });

        // Do NOT push current task to runq - it's now sleeping
        // Just call prepare_schedule to pick next task
        sched.prepare_schedule()
    };

    if let Some(switch) = switch_params {
        #[cfg(any(feature = "sched_debug", debug_assertions))]
        let cr3_before = rt.debug_active_aspace_root();

        rt.tasking().activate_address_space(switch.to_aspace);

        #[cfg(any(feature = "sched_debug", debug_assertions))]
        let cr3_after = rt.debug_active_aspace_root();
        #[cfg(any(feature = "sched_debug", debug_assertions))]
        log_context_switch::<R>(&switch, cr3_before, cr3_after);

        unsafe {
            rt.tasking().switch(&mut *switch.from_ctx, &*switch.to_ctx);
        }
    }

    rt.irq_restore(_irq);
}

pub fn sleep_until<R: BootRuntime>(deadline_ticks: u64) {
    let rt = crate::runtime::<R>();
    loop {
        let now = rt.mono_ticks();
        if now >= deadline_ticks {
            break;
        }

        let _irq = rt.irq_disable();
        let switch_params = {
            let lock = SCHEDULER.lock();
            let ptr = lock.expect("Scheduler not initialized");
            let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
            sched.schedule_point(ScheduleReason::SleepWait)
        };
        rt.irq_restore(_irq);

        if let Some(switch) = switch_params {
            unsafe {
                let _irq = rt.irq_disable();
                #[cfg(any(feature = "sched_debug", debug_assertions))]
                let cr3_before = rt.debug_active_aspace_root();
                rt.tasking().activate_address_space(switch.to_aspace);
                #[cfg(any(feature = "sched_debug", debug_assertions))]
                let cr3_after = rt.debug_active_aspace_root();
                #[cfg(any(feature = "sched_debug", debug_assertions))]
                log_context_switch::<R>(&switch, cr3_before, cr3_after);
                rt.tasking().switch(&mut *switch.from_ctx, &*switch.to_ctx);
                rt.irq_restore(_irq);
            }
        } else {
            // No switch occurred, spin briefly
            core::hint::spin_loop();
        }
    }
}

pub fn sleep_ms<R: BootRuntime>(ms: u64) {
    let rt = crate::runtime::<R>();
    let freq = rt.mono_freq_hz();
    let ticks = (ms * freq) / 1000;
    let deadline = rt.mono_ticks() + ticks;
    sleep_until::<R>(deadline);
}
