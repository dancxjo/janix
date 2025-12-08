#![no_std]
#![no_main]

mod console;
mod dashboard;
mod sys_kernel;
mod boot_model;

use sys_kernel::KernelSys;
use user_app_hello::run as user_app_hello_run;

use core::arch::asm;

use limine::BaseRevision;
use limine::request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker};

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
            let mut console = unsafe { console::Console::from_framebuffer(&framebuffer) };

            // Render dashboard instead of just raw log dump
            dashboard::render_dashboard(&mut console);

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

    // --- NEW: single-task launch of user_app_hello ---
    let sys = KernelSys;
    user_app_hello_run(&sys);
    kernel_core::log("user_app_hello finished");

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
