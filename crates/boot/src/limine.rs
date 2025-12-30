use crate::{BootFacts, FramebufferInfo, MemoryRegion, MemoryRegionKind, ModuleInfo};
use alloc::string::ToString;
use alloc::vec::Vec;
use limine::memory_map::EntryType;

/// Holds references to the raw Limine responses collected by the arch wrapper.
pub struct LimineResponses<'a> {
    pub hhdm_offset: u64,
    pub rsdp: Option<u64>,
    pub cmdline: Option<&'a str>,
    pub framebuffer: Option<&'a limine::response::FramebufferResponse>,
    pub modules: Option<&'a limine::response::ModuleResponse>,
    pub memmap: Option<&'a limine::response::MemoryMapResponse>,
}

pub fn to_boot_facts(responses: &LimineResponses) -> BootFacts {
    let framebuffer = responses
        .framebuffer
        .and_then(|r| r.framebuffers().next())
        .map(|fb| FramebufferInfo {
            address: fb.addr() as u64,
            size: (fb.pitch() as u64) * (fb.height() as u64),
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
        });

    let modules = responses
        .modules
        .map(|r| {
            r.modules()
                .iter()
                .map(|m| ModuleInfo {
                    path: m.path().to_str().unwrap_or("?").to_string(),
                    start: m.addr() as u64,
                    size: m.size(),
                })
                .collect()
        })
        .unwrap_or_default();

    let memory_map = responses
        .memmap
        .map(|r| {
            r.entries()
                .iter()
                .map(|e| MemoryRegion {
                    start: e.base,
                    end: e.base + e.length,
                    kind: match e.entry_type {
                        EntryType::USABLE => MemoryRegionKind::Usable,
                        EntryType::RESERVED => MemoryRegionKind::Reserved,
                        EntryType::ACPI_RECLAIMABLE => MemoryRegionKind::AcpiReclaimable,
                        EntryType::ACPI_NVS => MemoryRegionKind::AcpiNvs,
                        EntryType::BAD_MEMORY => MemoryRegionKind::BadMemory,
                        EntryType::BOOTLOADER_RECLAIMABLE => {
                            MemoryRegionKind::BootloaderReclaimable
                        }
                        EntryType::EXECUTABLE_AND_MODULES => MemoryRegionKind::Kernel,
                        EntryType::FRAMEBUFFER => MemoryRegionKind::Framebuffer,
                        _ => MemoryRegionKind::Unknown,
                    },
                })
                .collect()
        })
        .unwrap_or_default();

    BootFacts {
        hhdm_offset: responses.hhdm_offset,
        rsdp_addr: responses.rsdp,
        cmdline: responses.cmdline.map(|s| s.to_string()),
        framebuffer,
        modules,
        memory_map,
    }
}
