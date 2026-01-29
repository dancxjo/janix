#![no_std]
#![no_main]

mod arch;
pub mod console;
mod framebuffer;
mod mem;
mod requests;
pub mod runtime;

use arch::hcf;
use core::assert;
use framebuffer::Framebuffer;
use kernel::BootRuntime;

use requests::{BASE_REVISION, FRAMEBUFFER_REQUEST};

static RUNTIME: arch::CurrentRuntime = arch::create_runtime();

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    // Architecture-specific early initialization (e.g., stack mode switching on AArch64)
    unsafe {
        RUNTIME.early_init();
    }

    assert!(BASE_REVISION.is_supported());

    // Initialize architecture-specific paging (HHDM offset, etc.)
    arch::init_paging();

    // Initialize architecture-specific interrupts (VBAR, etc.)
    unsafe {
        arch::init_interrupts();
    }

    indicate_progress();
    kernel::start(&RUNTIME);
}

fn indicate_progress() {
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            let display = Framebuffer::new(&framebuffer);
            // Initialize framebuffer console for boot logging
            console::init(&display);
            // Register console disable callback for when compositor takes over
            kernel::syscall::handlers::register_console_disable(console::disable);
        }
    }
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    // unsafe { kernel::logging::init(&RUNTIME) };
    unsafe { kernel::logging::force_unlock() };
    if let Some(location) = info.location() {
        kernel::kerror!(
            "KERNEL PANIC Location: {}:{}:{} Message: {}",
            location.file(),
            location.line(),
            location.column(),
            info.message()
        );
    } else {
        kernel::kerror!("KERNEL PANIC Location: unknown Message: {}", info.message());
    }
    hcf()
}
