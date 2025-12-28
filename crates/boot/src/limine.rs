use limine::request::*;
use limine::memory_map::EntryType;
use crate::bootinfo::{BootInfo, FramebufferInfo, ModuleInfo, MemoryRegion, MemoryRegionKind};
use alloc::string::ToString;

// Requests
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

pub fn collect() -> BootInfo {
    let hhdm_offset = HHDM_REQUEST.get_response()
        .map(|r| r.offset())
        .unwrap_or(0);

    let rsdp_addr = RSDP_REQUEST.get_response()
        .map(|r| r.address() as u64);

    let cmdline = KERNEL_FILE_REQUEST.get_response()
        .and_then(|r| r.file().string().to_str().ok())
        .map(|s| s.to_string());

    let framebuffer = FRAMEBUFFER_REQUEST.get_response()
        .and_then(|r| r.framebuffers().next())
        .map(|fb| {
             // Calculate effective address relative to HHDM if needed, 
             // but here we just return physical or virtual as Limine gives it.
             // Usually Limine gives HHDM-mapped address for `addr`.
             // We will let the bridge sort out the HHDM math if it needs physical.
             FramebufferInfo {
                 address: fb.addr() as u64,
                 size: (fb.pitch() as u64) * (fb.height() as u64), // Approximate
                 width: fb.width(),
                 height: fb.height(),
                 pitch: fb.pitch(),
                 bpp: fb.bpp(),
                 red_mask_size: fb.red_mask_size(),
                 red_mask_shift: fb.red_mask_shift(),
                 green_mask_size: fb.green_mask_size(),
                 green_mask_shift: fb.green_mask_shift(),
                 blue_mask_size: fb.blue_mask_size(),
                 blue_mask_shift: fb.blue_mask_shift(),
             }
        });

    let modules = MODULE_REQUEST.get_response()
        .map(|r| r.modules().iter().map(|m| ModuleInfo {
            path: m.path().to_str().unwrap_or("?").to_string(),
            start: m.addr() as u64,
            size: m.size(),
        }).collect())
        .unwrap_or_default();

    let memory_map = MEMORY_MAP_REQUEST.get_response()
        .map(|r| r.entries().iter().map(|e| MemoryRegion {
            start: e.base,
            end: e.base + e.length,
            kind: match e.entry_type {
                EntryType::USABLE => MemoryRegionKind::Usable,
                EntryType::RESERVED => MemoryRegionKind::Reserved,
                EntryType::ACPI_RECLAIMABLE => MemoryRegionKind::AcpiReclaimable,
                EntryType::ACPI_NVS => MemoryRegionKind::AcpiNvs,
                EntryType::BAD_MEMORY => MemoryRegionKind::BadMemory,
                EntryType::BOOTLOADER_RECLAIMABLE => MemoryRegionKind::BootloaderReclaimable,
                EntryType::EXECUTABLE_AND_MODULES => MemoryRegionKind::Kernel,
                EntryType::FRAMEBUFFER => MemoryRegionKind::Framebuffer,
                _ => MemoryRegionKind::Unknown,
            },
        }).collect())
        .unwrap_or_default();

    BootInfo {
        hhdm_offset,
        rsdp_addr,
        cmdline,
        framebuffer,
        modules,
        memory_map,
    }
}
