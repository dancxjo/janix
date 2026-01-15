//! Timing and yield functions.

use crate::BootRuntime;
use crate::BootTasking;

use super::types::{ScheduleReason, Scheduler};
use super::SCHEDULER;

#[cfg(any(feature = "sched_debug", debug_assertions))]
use super::{read_cr3, log_context_switch};

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
        let cr3_before = read_cr3();

        rt.tasking().activate_address_space(switch.to_aspace);

        #[cfg(any(feature = "sched_debug", debug_assertions))]
        let cr3_after = read_cr3();
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

        let switch_params = {
            let lock = SCHEDULER.lock();
            let ptr = lock.expect("Scheduler not initialized");
            let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
            sched.schedule_point(ScheduleReason::SleepWait)
        };

        if let Some(switch) = switch_params {
            unsafe {
                let _irq = rt.irq_disable();
                #[cfg(any(feature = "sched_debug", debug_assertions))]
                let cr3_before = read_cr3();
                rt.tasking().activate_address_space(switch.to_aspace);
                #[cfg(any(feature = "sched_debug", debug_assertions))]
                let cr3_after = read_cr3();
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
