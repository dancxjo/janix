#![no_std]
#![no_main]

//! The Boot Runtime Abstraction Node (BRAN) is the seed coat around the kernel.
//! It wraps the abstract kernel with the low-level mechanisms to speak to the
//! architecture. Importantly, most hardware belongs in userspace, not here.
//! This is just the layer between the kernel and the boot environment. All
//! speaking with limine should happen here as well. Nothing beyond this layer
//! should know about limine or booting, except through the implementation of
//! the BootRuntime trait.

mod arch;
mod requests;
mod framebuffer;
mod mem;

use arch::{hcf, Runtime};
use framebuffer::Framebuffer;
use core::assert;
// embedded_graphics used internally by framebuffer mod, main only uses high level clear
// which takes u32.
// use embedded_graphics::pixelcolor::Rgb888;
// use embedded_graphics::prelude::*;
// use embedded_graphics::primitives::Rectangle;

// Import requests directly
use requests::{BASE_REVISION, FRAMEBUFFER_REQUEST};

static RUNTIME: Runtime = Runtime::new();

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    assert!(BASE_REVISION.is_supported());
    indicate_progress();
    kernel::start(&RUNTIME);
}

fn indicate_progress() {
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
             let mut display = Framebuffer::new(&framebuffer);
             // Lilac: #C8A2C8 -> 0x00C8A2C8 (assuming strict XRGB)
             display.clear(0x00_C8_A2_C8); 
        }
    }
}


#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    // Re-initialize logging just in case panic happened before kernel::start
    // or if the writer was somehow corrupted (though we can't really fix corruption here).
    // Safety: Single core boot environment.
    unsafe { kernel::logging::init(&RUNTIME) };

    kernel::kerror!("KERNEL PANIC");
    
    if let Some(location) = info.location() {
        kernel::kerror!("Location: {}:{}:{}", location.file(), location.line(), location.column());
    } else {
        kernel::kerror!("Location: unknown");
    }

    kernel::kerror!("Message: {}", info.message());

    hcf()
}
