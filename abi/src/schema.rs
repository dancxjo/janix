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
    pub const UNIX_SECONDS: &str = "unix_seconds";
    pub const OFFSET_NS: &str = "offset_ns";
    pub const STATE: &str = "state";
    pub const QUALITY: &str = "quality";
    pub const LAST_UPDATED_MONO_NS: &str = "last_updated_mono_ns";

    // Legacy mapping (to be deprecated or mapped)
    pub const KIND: &str = "kind";

    pub const VENDOR_ID: &str = "vendor_id";
    pub const DEVICE_ID: &str = "device_id";
    pub const VENDOR_NAME: &str = "vendor_name";
    pub const DEVICE_NAME: &str = "device_name";
    pub const PCI_NAME_SOURCE: &str = "pci_name_source";
    pub const CLASS_CODE: &str = "class_code";
    pub const SUBCLASS_CODE: &str = "subclass_code";
    pub const PROG_IF: &str = "prog_if";
    pub const REVISION_ID: &str = "revision_id";
    pub const BIND_KIND: &str = "bind_kind";
    pub const BIND_HASH: &str = "bind_hash";
    pub const BUS: &str = "bus";
    pub const DEVICE: &str = "device";
    pub const FUNCTION: &str = "function";
    pub const BAR0: &str = "bar0";
    pub const BAR1: &str = "bar1";
    pub const BAR2: &str = "bar2";
    pub const BAR3: &str = "bar3";
    pub const BAR4: &str = "bar4";
    pub const BAR5: &str = "bar5";
    pub const PORT_START: &str = "port_start";
    pub const PORT_END: &str = "port_end";
    pub const IRQ_MODE: &str = "irq_mode";
    pub const VECTOR: &str = "vector";
    pub const MSI_CAPABLE: &str = "msi_capable";
    pub const MSIX_CAPABLE: &str = "msix_capable";
}

pub mod kinds {
    pub const DEV_HOST: &str = "dev.Host";
    pub const DEV_BUS_PLATFORM: &str = "dev.bus.Platform";
    pub const FW_TABLE_ACPI: &str = "fw.table.Acpi";
    pub const FW_TABLE_DTB: &str = "fw.table.Dtb";
    pub const MEM_RANGE: &str = "mem.Range";
    pub const DEV_RTC_CMOS: &str = "dev.rtc.Cmos";
    pub const BYTESPACE: &str = "Bytespace";
    pub const RES_IO_PORT_RANGE: &str = "res.io.PortRange";
    pub const DEV_DISPLAY_FRAMEBUFFER: &str = "dev.display.Framebuffer";
    pub const PROC_KERNEL: &str = "proc.Kernel";
    pub const SVC_ROOT: &str = "svc.Root";
    pub const BOOT_MODULE: &str = "boot.Module";
    pub const DEV_CPU: &str = "dev.Cpu";
    pub const SVC_SCHEDULER: &str = "svc.Scheduler";
    pub const FW_BOOT: &str = "fw.Boot";
    pub const TIME_WALL_CLOCK_SAMPLE: &str = "time.WallClockSample";
    pub const SVC_TIME_SYSTEM_CLOCK: &str = "svc.time.SystemClock";
    pub const DEV_BUS_PCI: &str = "dev.bus.Pci";
    pub const DEV_PCI_FUNCTION: &str = "dev.pci.Function";
    // LPC / Legacy IO
    pub const DEV_BRIDGE_LPC: &str = "dev.bridge.Lpc";
    pub const DEV_BUS_LEGACY_IO: &str = "dev.bus.LegacyIo";
    pub const DEV_INPUT_PS2_CONTROLLER: &str = "dev.input.Ps2Controller";
    pub const CAP_IOPORT_RANGE: &str = "cap.ioport.Range";
    // Virtio GPU
    pub const DEV_DISPLAY_GPU: &str = "dev.display.Gpu";
    pub const DEV_DISPLAY_SCANOUT: &str = "dev.display.Scanout";
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
    pub const SEEDED_BY: &str = "SEEDED_BY";
    pub const USES: &str = "USES";
    // LPC / Legacy IO
    pub const IMPLEMENTS: &str = "IMPLEMENTS";
    pub const USES_IOPORTS: &str = "USES_IOPORTS";
    pub const HAS_SCANOUT: &str = "HAS_SCANOUT";
}

// Virtio GPU additions
pub mod virtio {
    pub const VENDOR_ID: u16 = 0x1af4;
    pub const GPU_DEVICE_ID: u16 = 0x1050;
}

// HID / Input additions (Bristle v0)
pub mod hid {
    // Service kinds
    pub const SVC_INPUT: &str = "svc.Input";           // Bristle broker
    
    // Device kinds
    pub const DEV_HID_KEYBOARD: &str = "dev.hid.Keyboard";
    pub const DEV_HID_MOUSE: &str = "dev.hid.Mouse";
    pub const DEV_HID_TOUCHPAD: &str = "dev.hid.Touchpad";
    pub const DEV_HID_GAMEPAD: &str = "dev.hid.Gamepad";
    
    // Driver kinds
    pub const DRV_PS2_KEYBOARD: &str = "drv.Ps2Keyboard";
    pub const DRV_PS2_MOUSE: &str = "drv.Ps2Mouse";
    
    // Relations
    pub const REL_CONSUMES: &str = "CONSUMES";         // (svc.Input)-[:CONSUMES]->(dev.hid.Keyboard)
    pub const REL_ROUTES_TO: &str = "ROUTES_TO";       // (svc.Input)-[:ROUTES_TO]->(app.Echo)
    pub const REL_PRODUCES: &str = "PRODUCES";         // (drv.Ps2Keyboard)-[:PRODUCES]->(dev.hid.Keyboard)
}
