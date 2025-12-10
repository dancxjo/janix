extern crate alloc;

use crate::arch::{Arch, CurrentArch, UserEntryRegs};
use kernel_core::sched::SCHEDULER;
use userland_rt::UserlandSys;

// Re-export stack functions from current arch
pub use crate::arch::current::{alloc_user_stack, init_user_stack};

#[unsafe(no_mangle)]
pub extern "C" fn user_thread_main(app_id: u64) -> ! {
    let mut sys = UserlandSys::new();
    kernel_core::log("user_thread_main reached without ELF ProgramImage; exiting");
    let _ = app_id;
    sys.exit_thread();
}

pub fn schedule_next() -> ! {
    loop {
        kernel_core::time::poll_time();
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
            kernel_core::time::poll_time();
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
    kernel_core::time::poll_time();
    let now = kernel_core::time::monotonic_now_ns();
    let wake_at = now.saturating_add(delta_ns);

    {
        let mut sched = SCHEDULER.lock();
        sched.sleep_current_thread(wake_at);
    }

    schedule_next();
}
