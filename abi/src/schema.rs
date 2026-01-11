//! Shared schema constants for Unified Device Graph v0.2

pub mod source {
    pub const BOOT: u8 = 0;
    pub const PLATFORM: u8 = 1;
    pub const DTB: u8 = 2;
    pub const ACPI: u8 = 3;
    pub const PCI: u8 = 4;
    pub const PROBE: u8 = 5;
}

pub mod confidence {
    pub const LOW: u8 = 0;
    pub const MEDIUM: u8 = 1;
    pub const HIGH: u8 = 2;
}

pub mod keys {
    pub const SOURCE: &str = "source";
    pub const CONFIDENCE: &str = "confidence";
    pub const NAME: &str = "name";
    pub const PHYS_BASE: &str = "phys_base";
    pub const SIZE_BYTES: &str = "size_bytes"; 
    pub const FORMAT: &str = "format";
    pub const HHDM_OFFSET: &str = "hhdm_offset";
    pub const START: &str = "start";
    pub const END: &str = "end";
    pub const WIDTH: &str = "width";
    pub const HEIGHT: &str = "height";
    pub const STRIDE: &str = "stride";
    pub const BPP: &str = "bpp";
    pub const BYTESPACE: &str = "bytespace"; // Used for Sprout shortcut
    pub const IRQ: &str = "irq";
    
    // Legacy mapping (to be deprecated or mapped)
    pub const KIND: &str = "kind";
}

pub mod kinds {
    pub const DEV_HOST: &str = "dev.host";
    pub const DEV_BUS_PLATFORM: &str = "dev.bus.platform";
    pub const FW_TABLE_ACPI: &str = "fw.table.acpi";
    pub const FW_TABLE_DTB: &str = "fw.table.dtb";
    pub const MEM_RANGE: &str = "mem.range";
    pub const DEV_RTC_CMOS: &str = "dev.rtc.cmos";
    pub const BYTESPACE: &str = "bytespace";
    pub const RES_IO_PORT_RANGE: &str = "res.io.port_range";
    pub const DEV_DISPLAY_FRAMEBUFFER: &str = "dev.display.framebuffer";
    pub const PROC_KERNEL: &str = "proc.kernel";
    pub const SVC_ROOT: &str = "svc.root";
    pub const BOOT_MODULE: &str = "boot.module";
    pub const DEV_CPU: &str = "dev.cpu";
    pub const SVC_SCHEDULER: &str = "svc.scheduler";
    pub const FW_BOOT: &str = "fw.boot";
}

pub mod rels {
    pub const HAS_BUS: &str = "HAS_BUS";
    pub const HAS_DEVICE: &str = "HAS_DEVICE";
    pub const HAS_RESOURCE: &str = "HAS_RESOURCE";
    pub const HAS_FIRMWARE: &str = "HAS_FIRMWARE";
    pub const PROVIDES_TABLE: &str = "PROVIDES_TABLE";
    pub const BACKED_BY: &str = "BACKED_BY";
    pub const DERIVED_FROM: &str = "DERIVED_FROM";
    pub const RUNS_ON: &str = "RUNS_ON";
    pub const PROVIDES: &str = "PROVIDES";
    pub const HAS_CPU: &str = "HAS_CPU";
    pub const HAS_MEMORY_RANGE: &str = "HAS_MEMORY_RANGE";
    pub const HAS_MODULE: &str = "HAS_MODULE";
}
