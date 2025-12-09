extern crate alloc;

use crate::arch::{self, Arch, CurrentArch, UserEntryRegs};
use crate::user_app_heartbeat;
use crate::user_app_hello;
use kernel_core::model::{ThreadState, pick_next_thread};
use userland_rt::Ring3Sys;

// Re-export stack functions from current arch
pub use crate::arch::current::{alloc_user_stack, init_user_stack};

#[unsafe(no_mangle)]
pub extern "C" fn user_thread_main(app_id: u64) -> ! {
    let mut sys = Ring3Sys::new();
    match app_id {
        1 => user_app_hello::run(&sys),
        2 => user_app_heartbeat::run(&sys),
        _ => {}
    }
    sys.exit_thread();
}

pub fn schedule_next() -> ! {
    loop {
        if let Some(thread) = pick_next_thread() {
            if thread.state == ThreadState::New {
                thread.state = ThreadState::Running;

                let regs = UserEntryRegs {
                    entry_point: thread.user_entry.unwrap() as u64,
                    user_stack: thread.user_stack_top,
                    arg0: thread.user_arg,
                };

                kernel_core::log("Entering user thread...");
                CurrentArch::enter_user_mode(&regs);
            }
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
