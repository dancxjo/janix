extern crate alloc;

use crate::arch::{Arch, CurrentArch, UserEntryRegs};
use crate::heartbeat;
use crate::hello;
use crate::init_app;
use crate::thread_dashboard;
use kernel_core::sched::SCHEDULER;
use userland_rt::UserlandSys;

// Re-export stack functions from current arch
pub use crate::arch::current::{alloc_user_stack, init_user_stack};

#[unsafe(no_mangle)]
pub extern "C" fn user_thread_main(app_id: u64) -> ! {
    let mut sys = UserlandSys::new();
    match app_id {
        0 => init_app::run(&mut sys),
        1 => hello::run(&mut sys),
        2 => heartbeat::run(&mut sys),
        3 => thread_dashboard::run(&mut sys),
        _ => {}
    }
    sys.exit_thread();
}

pub fn schedule_next() -> ! {
    loop {
        let next_thread = {
            let mut sched = SCHEDULER.lock();
            let now = kernel_core::time::monotonic_now_ns();
            sched.choose_next_thread(now)
        };

        if let Some(thread) = next_thread {
            CurrentArch::activate_user_address_space(thread.address_space_token);
            if thread.started {
                CurrentArch::resume_user_mode(&thread.context);
            } else {
                kernel_core::log("Entering user thread...");
                kernel_core::log(thread.name);
                let regs = UserEntryRegs {
                    entry_point: thread.entry_point,
                    user_stack: thread.user_stack_top,
                    arg0: thread.user_arg,
                };
                CurrentArch::enter_user_mode(&regs);
            }
        } else {
            kernel_core::log("No runnable threads");
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
