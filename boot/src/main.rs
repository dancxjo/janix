#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![cfg_attr(target_arch = "x86_64", feature(abi_x86_interrupt))]

mod arch;
mod boot_model;
mod console;
mod dashboard;
#[cfg(target_arch = "x86_64")]
mod gdt;
mod heap;
mod serial;
mod user;

use crate::arch::{Arch, CurrentArch};
use user_app_heartbeat;
use user_app_hello;
use user_app_thread_dashboard;
use userland_rt::KernelSys;

use core::arch::asm;

use limine::BaseRevision;
use limine::request::{FramebufferRequest, HhdmRequest, RequestsEndMarker, RequestsStartMarker};

/// Sets the base revision to the latest revision supported by the crate.
#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

/// Define the start and end markers for Limine requests.
#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    // Initialize serial console first (best effort)
    // We use 0 offset initially; Semihosting doesn't need offset.
    serial::arch::init_serial(0);

    kernel_core::log("Serial initialized. Preparing to switch stack...");

    #[cfg(target_arch = "aarch64")]
    {
        // Initialize exception vector table early
        arch::aarch64::trap::init();
        
        // Switch to SP_EL1 for kernel stack
        unsafe { arch::aarch64::trap::jump_to_el1_stack(kmain_inner); }
    }

    #[cfg(not(target_arch = "aarch64"))]
    {
        kmain_inner();
    }
}

unsafe extern "C" fn kmain_inner() -> ! {
    kernel_core::log("Inside kmain_inner");
    // All limine requests must also be referenced in a called function
    assert!(BASE_REVISION.is_supported());

    // Initialize kernel core
    kernel_core::init();

    // Initialize GDT
    #[cfg(target_arch = "x86_64")]
    {
        gdt::init();
    }

    // Initialize syscall handler (and IDT/traps)
    CurrentArch::install_syscall_handler();

    // Initialize user stack mapping
    if let Some(hhdm_response) = boot_model::HHDM_REQUEST.get_response() {
        let offset = hhdm_response.offset();
        // serial::arch::init_serial(offset); // Already inited
        unsafe { user::init_user_stack(offset) };
    }

    // Log startup message
    kernel_core::log("ThingOS booting...");

    // Create builtin kernel Things
    kernel_core::create_builtin_things();

    // Initialize boot graph with memory and scheduling Things
    boot_model::seed_memory_graph_from_limine();
    boot_model::seed_cpu_graph_from_limine();

    // Initialize console on the Limine framebuffer
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            unsafe { console::init_global(&framebuffer) };

            // Run the demo app through the same Sys trait as the host:
            // let sys = KernelSys;
            kernel_core::log("Launching user_app_hello from kernel...");
            // user_app_hello::run(&sys);
            let stack1 = user::alloc_user_stack();
            // kernel_core::model::create_user_thread_for_app(100, 1, user::user_thread_main, stack1);
            {
                let mut sched = kernel_core::sched::SCHEDULER.lock();
                let p1 = sched.add_process("user_app_hello");
                sched.add_thread(p1, "hello", user::user_thread_main, 1, stack1);
            }
            kernel_core::log("... created process 100");
            kernel_core::log("... created thread 101 in process 100");

            kernel_core::log("Launching user_app_heartbeat from kernel...");
            // user_app_heartbeat::run(&sys);
            let stack2 = user::alloc_user_stack();
            // kernel_core::model::create_user_thread_for_app(200, 2, user::user_thread_main, stack2);
            {
                let mut sched = kernel_core::sched::SCHEDULER.lock();
                let p2 = sched.add_process("user_app_heartbeat");
                sched.add_thread(p2, "heartbeat", user::user_thread_main, 2, stack2);
            }
            kernel_core::log("... created process 200");
            kernel_core::log("... created thread 201 in process 200");

            kernel_core::log("Launching user_app_thread_dashboard from kernel...");
            let stack3 = user::alloc_user_stack();
            {
                let mut sched = kernel_core::sched::SCHEDULER.lock();
                let p3 = sched.add_process("user_app_thread_dashboard");
                // Launch with app_id = 3
                sched.add_thread(p3, "dashboard", user::user_thread_main, 3, stack3);
            }
            kernel_core::log("... created process 300");
            kernel_core::log("... created thread 301 in process 300");

            // TEMPORARY: Test user mode entry
            kernel_core::log("Entering first user thread...");
            user::schedule_next();

            /*
            kernel_core::log("Starting scheduler loop...");
            for _ in 0..100 {
                if let Some(thread) = kernel_core::model::scheduler_tick() {
                    match thread.tid {
                        101 => user_app_hello::tick(&sys),
                        201 => user_app_heartbeat::tick(&sys),
                        _ => {}
                    }
                }
            }
            kernel_core::log("Scheduler loop finished.");
            */

            // Render dashboard instead of just raw log dump
            console::with_console(|console| {
                dashboard::render_dashboard(console);
            });

            // Optionally: halt, or spin
            loop {
                unsafe {
                    #[cfg(target_arch = "x86_64")]
                    asm!("hlt");
                    #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
                    asm!("wfi");
                    #[cfg(target_arch = "loongarch64")]
                    asm!("idle 0");
                }
            }
        }
    }

    kernel_core::log("ThingOS started successfully");

    hcf();
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    kernel_core::log("PANIC!");
    hcf();
}

fn hcf() -> ! {
    loop {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            asm!("hlt");
            #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
            asm!("wfi");
            #[cfg(target_arch = "loongarch64")]
            asm!("idle 0");
        }
    }
}
