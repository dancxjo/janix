//! Canonical Schema for Unified Device Graph v0.1
//!
//! This module defines the interned string constants used for the device graph.
//! Using these constants prevents typos and ensures schema consistency.

pub mod kinds {
    // Core
    pub const HOST: &str = "dev.host";
    pub const CPU: &str = "dev.cpu";
    pub const BUS_PLATFORM: &str = "dev.bus.platform";
    pub const MEM_RANGE: &str = "mem.range";
    pub const BOOT_MODULE: &str = "boot.module";

    // Devices
    pub const DISPLAY_FRAMEBUFFER: &str = "dev.display.framebuffer";
    pub const SERIAL_UART: &str = "dev.serial.uart";
    pub const RTC_CMOS: &str = "dev.rtc.cmos";
    pub const TIMER: &str = "dev.timer";
    pub const INTERRUPT_CONTROLLER: &str = "dev.interrupt_controller";
    pub const PCI_ROOT: &str = "dev.pci.root";

    // Firmware
    pub const FW_BOOT: &str = "fw.boot";
    pub const FW_TABLE_ACPI: &str = "fw.table.acpi";
    pub const FW_TABLE_DTB: &str = "fw.table.dtb";
    
    // Resources
    // Note: 'res' prefixes are for enriched resources
    pub const RES_MMIO_RANGE: &str = "res.mmio.range";
    pub const RES_IO_PORT_RANGE: &str = "res.io.port_range";
    pub const RES_IRQ: &str = "res.irq";
    // pub const RES_DMA: &str = "res.dma"; // Reserved for future
}

pub mod rels {
    pub const HAS_BUS: &str = "HAS_BUS";
    pub const HAS_DEVICE: &str = "HAS_DEVICE";
    pub const HAS_CPU: &str = "HAS_CPU";
    pub const HAS_MEMORY_RANGE: &str = "HAS_MEMORY_RANGE";
    pub const HAS_MODULE: &str = "HAS_MODULE";
    pub const HAS_FIRMWARE: &str = "HAS_FIRMWARE";
    pub const HAS_RESOURCE: &str = "HAS_RESOURCE";
    pub const PROVIDES_TABLE: &str = "PROVIDES_TABLE";
    pub const DERIVED_FROM: &str = "DERIVED_FROM";
    pub const ALIAS_OF: &str = "ALIAS_OF";
    pub const RUNS_ON: &str = "RUNS_ON";
    pub const PROVIDES: &str = "PROVIDES";
}

pub mod props {
    // Identity
    pub const ARCH: &str = "arch";
    pub const HHDM_OFFSET: &str = "hhdm_offset";
    pub const PLATFORM_PROFILE: &str = "platform_profile";
    pub const NAME: &str = "name";
    pub const ID: &str = "id";
    pub const INDEX: &str = "index";
    pub const VERSION: &str = "version";

    // Memory / Geometry
    pub const START: &str = "start";
    pub const END: &str = "end";
    pub const KIND: &str = "kind";
    pub const PHYS_BASE: &str = "phys_base";
    pub const PHYS_LEN: &str = "phys_len"; // Sometimes used instead of size_bytes for ranges
    pub const SIZE_BYTES: &str = "size_bytes";
    
    // Display
    pub const WIDTH: &str = "width";
    pub const HEIGHT: &str = "height";
    pub const STRIDE: &str = "stride";
    pub const BPP: &str = "bpp";
    pub const FORMAT: &str = "format";

    // Firmware / Resources
    pub const BYTESPACE: &str = "bytespace";
    pub const IRQ: &str = "irq";
    pub const COMPATIBLE: &str = "compatible";
    pub const PATH: &str = "path";

    // Provenance (v0.1)
    pub const SOURCE: &str = "source";
    pub const CONFIDENCE: &str = "confidence";
}

// Common categorical values derived from provenance
pub mod provenance {
    pub const SRC_BOOT: &str = "boot";
    pub const SRC_ACPI: &str = "acpi";
    pub const SRC_DTB: &str = "dtb";
    pub const SRC_PLATFORM: &str = "platform";
    // pub const SRC_PCI: &str = "pci";
    // pub const SRC_PROBE: &str = "probe";

    pub const CONF_HIGH: &str = "high";
    pub const CONF_MEDIUM: &str = "medium";
    pub const CONF_LOW: &str = "low";
}
