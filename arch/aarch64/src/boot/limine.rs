#![no_std]

use alloc::vec::Vec;
use boot::BootFacts;
pub use limine::memory_map::Entry;
pub use limine::memory_map::EntryType;
use limine::request::*;

// Limine Requests for AArch64
#[used]
#[unsafe(link_section = ".requests")]
pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static RSDP_REQUEST: RsdpRequest = RsdpRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static KERNEL_FILE_REQUEST: ExecutableFileRequest = ExecutableFileRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static MODULE_REQUEST: ModuleRequest = ModuleRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

pub fn collect() -> BootFacts {
    let responses = boot::limine::LimineResponses {
        hhdm_offset: HHDM_REQUEST.get_response().map(|r| r.offset()).unwrap_or(0),
        rsdp: RSDP_REQUEST.get_response().map(|r| r.address() as u64),
        cmdline: KERNEL_FILE_REQUEST
            .get_response()
            .and_then(|r| r.file().string().to_str().ok()),
        framebuffer: FRAMEBUFFER_REQUEST.get_response(),
        modules: MODULE_REQUEST.get_response(),
        memmap: MEMORY_MAP_REQUEST.get_response(),
    };

    boot::limine::to_boot_facts(&responses)
}
