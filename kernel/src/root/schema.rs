//! Canonical Schema for Unified Device Graph v0.1
//!
//! This module defines the interned string constants used for the device graph.
//! Using these constants prevents typos and ensures schema consistency.

pub mod kinds {
    // Core
    pub const HOST: &str = "dev.Host";
    pub const CPU: &str = "dev.Cpu";
    pub const BUS_PLATFORM: &str = "dev.bus.Platform";
    pub const MEM_RANGE: &str = "mem.Range";
    pub const BOOT_MODULE: &str = "boot.Module";

    // Devices
    pub const DISPLAY_FRAMEBUFFER: &str = "dev.display.Framebuffer";
    pub const SERIAL_UART: &str = "dev.serial.Uart";
    pub const RTC_CMOS: &str = "dev.rtc.Cmos";
    pub const TIMER: &str = "dev.Timer";
    pub const INTERRUPT_CONTROLLER: &str = "dev.InterruptController";
    pub const PCI_ROOT: &str = "dev.pci.Root";

    // Firmware
    pub const FW_BOOT: &str = "fw.Boot";
    pub const FW_TABLE_ACPI: &str = "fw.table.Acpi";
    pub const FW_TABLE_DTB: &str = "fw.table.Dtb";

    // Resources
    // Note: 'res' prefixes are for enriched resources
    pub const RES_MMIO_RANGE: &str = "res.mmio.Range";
    pub const RES_IO_PORT_RANGE: &str = "res.io.PortRange";
    pub const RES_IRQ: &str = "res.Irq";
    // pub const RES_DMA: &str = "res.dma"; // Reserved for future

    // UI
    pub const UI_ROOT: &str = "ui.Root";
    pub const UI_WINDOW: &str = "ui.Window";
    pub const UI_PANEL: &str = "ui.Panel";
    pub const UI_TEXT: &str = "ui.Text";
    pub const UI_IMAGE: &str = "ui.Image";
    pub const UI_OVERLAY: &str = "ui.Overlay";
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

    // UI
    pub const CHILD_OF: &str = "CHILD_OF";
    pub const CLIP_TO: &str = "CLIP_TO";
    pub const ROOT_UI: &str = "ROOT_UI";
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

    // UI Properties
    pub const UI_X: &str = "ui.x";
    pub const UI_Y: &str = "ui.y";
    pub const UI_WIDTH: &str = "ui.width";
    pub const UI_HEIGHT: &str = "ui.height";
    pub const UI_COLOR: &str = "ui.color";
    pub const UI_TEXT: &str = "ui.text";
    pub const UI_FONT: &str = "ui.font";
    pub const UI_FONT_SIZE: &str = "ui.font_size";
    pub const UI_RADIUS: &str = "ui.radius";
    pub const UI_TITLE: &str = "ui.title";
    pub const UI_HIDDEN: &str = "ui.hidden";
    pub const UI_Z_INDEX: &str = "ui.z_index";
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
