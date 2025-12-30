use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct BootFacts {
    pub hhdm_offset: u64,
    pub rsdp_addr: Option<u64>,
    pub cmdline: Option<String>,
    pub framebuffer: Option<FramebufferInfo>,
    pub modules: Vec<ModuleInfo>,
    pub memory_map: Vec<MemoryRegion>,
}

#[derive(Debug, Clone, Copy)]
pub struct FramebufferInfo {
    pub address: u64,
    pub size: u64, // pitch * height generally, or total mapping size
    pub width: u64,
    pub height: u64,
    pub pitch: u64,
    pub bpp: u16,
    pub red_mask_size: u8,
    pub red_mask_shift: u8,
    pub green_mask_size: u8,
    pub green_mask_shift: u8,
    pub blue_mask_size: u8,
    pub blue_mask_shift: u8,
}

#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub path: String,
    pub start: u64,
    pub size: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryRegionKind {
    Usable,
    Reserved,
    AcpiReclaimable,
    AcpiNvs,
    BadMemory,
    BootloaderReclaimable, // Kernel/Modules/Bootloader data
    Kernel,                // Kernel code/data
    Framebuffer,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
    pub start: u64,
    pub end: u64,
    pub kind: MemoryRegionKind,
}
