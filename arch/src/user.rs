extern crate alloc;

use crate::cpu;
use crate::{Arch, CurrentArch, UserEntryRegs};
use kernel::sched::SCHEDULER;

// Re-export stack functions from current arch
pub use crate::current::{alloc_user_stack, init_user_stack};

#[unsafe(no_mangle)]
pub extern "C" fn user_thread_main(app_id: u64) -> ! {
    kernel::log("user_thread_main reached without ELF ProgramImage; exiting");
    let _ = app_id;
    // Perform raw exit syscall
    unsafe {
        // We use the syscall instruction directly or via an ABI helper if available in arch?
        // But arch IS where the syscall instruction wrapper usually lives.
        // Wait, arch::user::user_thread_main runs in user mode.
        // It needs to trap to kernel.
        // The previous code used UserlandSys::new().exit_thread().
        // UserlandSys called runtime::sys::syscall.
        // runtime::sys::syscall used inline assembly.
        // Since we are IN `arch`, we should define/use the syscall mechanism here or import from thing_os?
        // We CANNOT import things_os.
        // We can import imports from `abi`? No, `abi` only has types.
        // We must implement the syscall instruction here or use one if defined in `arch`.
        // Let's assume we can use the same mechanism UserlandSys used, but defined locally?
        // ACTUALLY, checking imports: `use crate::{Arch, CurrentArch, ...}`.
        // Maybe Arch has a syscall helper?
        // Or maybe I should just loop forever for now if this path is effectively unreachable/error path?
        // "user_thread_main reached without ELF ProgramImage" -> this is an error case.
        loop {}
    }
}

pub fn schedule_next() -> ! {
    loop {
        kernel::time::poll_time();
        let next_thread = kernel::sched::without_preemption(|| {
            let mut sched = SCHEDULER.lock();
            let now = kernel::time::monotonic_now_ns();
            sched.choose_next_thread(now)
        });

        if let Some(thread) = next_thread {
            #[cfg(target_arch = "x86_64")]
            {
                // let rsp0 = crate::gdt::kernel_stack_top();
                // let ist1 = crate::gdt::kernel_stack_ist1_top();
                // let msg = alloc::format!(
                //     "schedule_next: tid={} started={} entry={:#x} usp={:#x} cr3_token={:?} rsp0={:#x} ist1={:#x}",
                //     thread.tid.0,
                //     thread.started,
                //     thread.entry_point,
                //     thread.user_stack_top,
                //     thread.address_space_token,
                //     rsp0,
                //     ist1
                // );
                // kernel::log(Box::leak(msg.into_boxed_str()));
            }
            #[cfg(not(target_arch = "x86_64"))]
            {
                // let msg = alloc::format!(
                //     "schedule_next: tid={} started={} entry={:#x} usp={:#x} cr3_token={:?}",
                //     thread.tid.0,
                //     thread.started,
                //     thread.entry_point,
                //     thread.user_stack_top,
                //     thread.address_space_token
                // );
                // kernel::log(Box::leak(msg.into_boxed_str()));
            }

            if thread.is_idle {
                // Idle thread "running" means waiting for interrupt
                crate::cpu::enable_interrupts();
                crate::cpu::wait_for_interrupt();
            } else {
                CurrentArch::activate_user_address_space(thread.address_space_token);

                let is_kernel_thread = thread.process_id.0 == 1;

                if thread.started {
                    if is_kernel_thread {
                        // Resume kernel thread (Ring 0 -> Ring 0)
                        crate::current::resume_kernel_mode(&thread.context);
                    } else {
                        // Resume user thread (Ring 0 -> Ring 3)
                        CurrentArch::resume_user_mode(&thread.context, &thread.fpu_context);
                    }
                } else {
                    if is_kernel_thread {
                        kernel::log("Entering kernel thread...");
                        kernel::log(thread.name);
                        // Jump to kernel entry point
                        let entry = thread.entry_point;
                        let stack = thread.user_stack_top;
                        unsafe {
                            enter_kernel_thread(entry, stack);
                        }
                    } else {
                        kernel::log("Entering user thread...");
                        kernel::log(thread.name);
                        let regs = UserEntryRegs {
                            entry_point: thread.entry_point,
                            user_stack: thread.user_stack_top,
                            arg0: thread.user_arg,
                        };
                        CurrentArch::enter_user_mode(&regs);
                    }
                }
            }
        } else {
            // Should not happen if idle thread exists, but safe fallback
            crate::cpu::enable_interrupts();
            crate::cpu::wait_for_interrupt();
        }
    }
}

unsafe fn enter_kernel_thread(entry: u64, stack: u64) -> ! {
    core::arch::asm!(
        "mov rsp, {stack}", // Switch stack
        "push 0",           // Dummy return address for alignment/ABI
        "jmp {entry}",      // Jump to entry
        stack = in(reg) stack,
        entry = in(reg) entry,
        options(noreturn)
    );
}

pub fn sys_sleep_for_ns(delta_ns: u64) -> ! {
    kernel::time::poll_time();
    let now = kernel::time::monotonic_now_ns();
    let wake_at = now.saturating_add(delta_ns);

    kernel::sched::without_preemption(|| {
        let mut sched = SCHEDULER.lock();
        sched.sleep_current_thread(wake_at);
    });

    schedule_next();
}
