#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![cfg_attr(target_arch = "x86_64", feature(abi_x86_interrupt))]

mod boot_model;
mod console;
mod dashboard;
#[cfg(target_arch = "x86_64")]
mod gdt;
mod heap;
#[cfg(target_arch = "x86_64")]
mod idt;
mod serial;
mod user;

use user_app_heartbeat;
use user_app_hello;
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
    // All limine requests must also be referenced in a called function
    assert!(BASE_REVISION.is_supported());

    // Initialize kernel core
    kernel_core::init();

    // Initialize serial console
    serial::arch::init_serial();

    // Initialize GDT and IDT
    #[cfg(target_arch = "x86_64")]
    {
        gdt::init();
        idt::init();
    }

    // Initialize user stack mapping
    if let Some(hhdm_response) = boot_model::HHDM_REQUEST.get_response() {
        let offset = hhdm_response.offset();
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
            let sys = KernelSys;
            kernel_core::log("Launching user_app_hello from kernel...");
            user_app_hello::run(&sys);

            kernel_core::log("Launching user_app_heartbeat from kernel...");
            user_app_heartbeat::run(&sys);

            // TEMPORARY: Test user mode entry
            kernel_core::log("Entering user mode test...");
            user::enter_user(user::user_test_entry);

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
