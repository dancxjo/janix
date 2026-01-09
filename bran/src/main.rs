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

use arch::{hcf, SerialPort};
use core::assert;
use limine::BaseRevision;
use limine::request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker};

#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    assert!(BASE_REVISION.is_supported());
    indicate_progress();
    let runtime = SerialPort;
    kernel::start(runtime);
}

fn indicate_progress() {
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            for i in 0..100_u64 {
                let pixel_offset = i * framebuffer.pitch() + i * 4;

                unsafe {
                    framebuffer
                        .addr()
                        .add(pixel_offset as usize)
                        .cast::<u32>()
                        .write(0xFFFFFFFF)
                };
            }
        }
    }
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    hcf()
}
