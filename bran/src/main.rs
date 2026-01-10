#![no_std]
#![no_main]

mod arch;
mod requests;
mod framebuffer;
mod mem;
pub mod runtime;

use arch::hcf;
use framebuffer::Framebuffer;
use core::assert;

use requests::{BASE_REVISION, FRAMEBUFFER_REQUEST};

static RUNTIME: arch::CurrentRuntime = arch::create_runtime();

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    assert!(BASE_REVISION.is_supported());
    
    // Initialize architecture-specific paging (HHDM offset, etc.)
    arch::init_paging();
    
    indicate_progress();
    kernel::start(&RUNTIME);
}

fn indicate_progress() {
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
             let mut display = Framebuffer::new(&framebuffer);
             display.clear(0x00_C8_A2_C8); 
        }
    }
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    unsafe { kernel::logging::init(&RUNTIME) };
    kernel::kerror!("KERNEL PANIC");
    if let Some(location) = info.location() {
        kernel::kerror!("Location: {}:{}:{}", location.file(), location.line(), location.column());
    }
    kernel::kerror!("Message: {}", info.message());
    hcf()
}
