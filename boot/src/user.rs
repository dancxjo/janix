extern crate alloc;

use crate::arch::{self, Arch, CurrentArch, UserEntryRegs};
use crate::user_app_heartbeat;
use crate::user_app_hello;
use kernel_core::sched::{SCHEDULER, ThreadState};
use userland_rt::{Ring3Sys, Sys};

// Re-export stack functions from current arch
pub use crate::arch::current::{alloc_user_stack, init_user_stack};

#[unsafe(no_mangle)]
pub extern "C" fn user_thread_main(app_id: u64) -> ! {
    let mut sys = Ring3Sys::new();
    match app_id {
        1 => user_app_hello::run(&mut sys),
        2 => user_app_heartbeat::run(&mut sys),
        _ => {}
    }
    sys.exit_thread();
}

pub fn schedule_next() -> ! {
    loop {
        let next_tid = {
            let mut sched = SCHEDULER.lock();
            let now = kernel_core::time::monotonic_now_ns();
            sched.wake_sleepers(now);
            sched.next_runnable()
        };

        if let Some(tid) = next_tid {
            let (regs, context, started, name) = {
                let mut sched = SCHEDULER.lock();
                sched.set_current(tid);
                let thread = sched.thread_mut(tid).unwrap();

                let regs = UserEntryRegs {
                    entry_point: thread.user_entry.unwrap() as u64,
                    user_stack: thread.user_stack_top,
                    arg0: thread.user_arg,
                };
                (regs, thread.context, thread.started, thread.name)
            };

            if started {
                // kernel_core::log("Resuming user thread...");
                CurrentArch::resume_user_mode(&context);
            } else {
                kernel_core::log("Entering user thread...");
                kernel_core::log(name);
                CurrentArch::enter_user_mode(&regs);
            }
        } else {
            kernel_core::log("No runnable threads");
        }
        unsafe {
            #[cfg(target_arch = "x86_64")]
            core::arch::asm!("hlt");
            #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
            core::arch::asm!("wfi");
            #[cfg(target_arch = "loongarch64")]
            core::arch::asm!("idle 0");
        };
    }
}

pub fn sys_sleep_for_ns(delta_ns: u64) -> ! {
    let now = kernel_core::time::monotonic_now_ns();
    let wake_at = now.saturating_add(delta_ns);

    {
        let mut sched = SCHEDULER.lock();
        sched.sleep_current_thread(wake_at);
    }

    schedule_next();
}
