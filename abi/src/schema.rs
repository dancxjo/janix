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

    // XML Properties
    pub const TAG: &str = "tag";
    pub const TEXT: &str = "text";
    pub const ATTR_NAME: &str = "attr_name";
    pub const ATTR_VALUE: &str = "attr_value";

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
    // Storage
    pub const SECTOR_SIZE: &str = "sector_size";
    pub const SECTOR_COUNT: &str = "sector_count";
    pub const LBA48: &str = "lba48";
    pub const ATA_CHANNEL: &str = "ata_channel";
    pub const ATA_DRIVE: &str = "ata_drive";

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
    pub const UI_WINDOW_ICON: &str = "ui.window.icon";
    pub const UI_WINDOW_SHADED: &str = "ui.window.shaded";
    pub const UI_HIDDEN: &str = "ui.hidden";
    pub const UI_Z_INDEX: &str = "ui.z_index";

    // Clock & Binding Properties
    pub const CLOCK_NOW_TEXT: &str = "clock.now_text";
    pub const CLOCK_TICK: &str = "clock.tick";
    pub const BINDING_SOURCE: &str = "binding.source";
    pub const BINDING_TARGET: &str = "binding.target";
    pub const BINDING_MAP: &str = "binding.map";
    pub const BINDING_TO: &str = "binding.to";

    // UI Inline
    pub const UI_INLINE_MODE: &str = "ui.inline.mode";
    pub const UI_SVG_BYTES: &str = "ui.svg_bytes";

    // UI Layout & Style
    pub const UI_LAYOUT_MODE: &str = "ui.layout.mode";
    pub const UI_CENTER_X: &str = "ui.layout.center_x";
    pub const UI_CENTER_Y: &str = "ui.layout.center_y";
    pub const UI_FILL_PARENT: &str = "ui.layout.fill_parent";
    pub const UI_INSET_RIGHT: &str = "ui.layout.inset_right";
    pub const UI_INSET_BOTTOM: &str = "ui.layout.inset_bottom";
    pub const UI_SCROLL_X: &str = "ui.scroll.x";
    pub const UI_SCROLL_Y: &str = "ui.scroll.y";
    pub const UI_CLIP: &str = "ui.clip";
    pub const UI_BG_COLOR: &str = "ui.style.bg_color";
    pub const UI_FG_COLOR: &str = "ui.style.fg_color";
    pub const UI_FONT_SIZE_PX: &str = "ui.style.font_size_px";
    /// Snapshot bytespace id containing a view's latest presented pixels.
    ///
    /// Reserved for presenter-owned updates (Blossom).
    pub const UI_SNAPSHOT_BYTESPACE: &str = "ui.snapshot.bytespace";
    /// Snapshot width in pixels for the presented surface.
    pub const UI_SNAPSHOT_WIDTH: &str = "ui.snapshot.width";
    /// Snapshot height in pixels for the presented surface.
    pub const UI_SNAPSHOT_HEIGHT: &str = "ui.snapshot.height";
    /// Snapshot stride in bytes per row (pixel surfaces).
    pub const UI_SNAPSHOT_STRIDE: &str = "ui.snapshot.stride";
    /// Snapshot format (e.g. RGBA8888) for pixel surfaces.
    pub const UI_SNAPSHOT_FORMAT: &str = "ui.snapshot.format";
    /// Monotonic present epoch for atomic snapshot presentation.
    ///
    /// Reserved for presenter-owned updates (Blossom).
    pub const UI_PRESENT_EPOCH: &str = "ui.present.epoch";
    /// Optional bytespace id for packed damage rects.
    pub const UI_DAMAGE_RECTS_BYTESPACE: &str = "ui.damage.rects.bytespace";
    /// Tile asset bytespace id for UI_TILE nodes (e.g. SVG source).
    pub const UI_TILE_ASSET: &str = "ui.tile.asset";
    /// Optional tile state for UI_TILE nodes (0 = placeholder, 1 = ready).
    pub const UI_TILE_STATE: &str = "ui.tile.state";

    // Font Graph Properties
    pub const FONT_NAME: &str = "font.name";
    pub const FONT_STYLE: &str = "font.style";
    pub const FONT_FAMILY_KEY: &str = "font.family_key";
    pub const FONT_FACE_KEY: &str = "font.face_key";
    pub const FONT_WEIGHT: &str = "font.weight";
    pub const FONT_WIDTH: &str = "font.width";
    pub const FONT_SLOPE: &str = "font.slope";
    pub const FONT_BYTESPACE: &str = "font.bytespace";
    pub const FONT_SIZE_BYTES: &str = "font.size_bytes";
    pub const FONT_COVERAGE_RANGES: &str = "font.coverage_ranges";
    pub const FONT_COVERAGE_COUNT: &str = "font.coverage_count";
    pub const UI_FONT_STACK: &str = "ui.font_stack";
    pub const UI_FONT_DEBUG: &str = "ui.font_debug";
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
    // Storage
    pub const DEV_STORAGE_DISK: &str = "dev.storage.Disk";
    pub const DEV_STORAGE_PARTITION: &str = "dev.storage.Partition";
    pub const SVC_STORAGE: &str = "svc.Storage";

    // UI Kinds
    pub const UI_ROOT: &str = "ui.Root";
    pub const UI_WINDOW: &str = "ui.Window";
    pub const UI_PANEL: &str = "ui.Panel";
    pub const UI_TEXT: &str = "ui.Text";
    pub const UI_IMAGE: &str = "ui.Image";
    pub const UI_OVERLAY: &str = "ui.Overlay";
    pub const UI_INLINE: &str = "ui.Inline";
    pub const UI_VIEWPORT: &str = "ui.Viewport";
    pub const UI_TILE: &str = "ui.Tile";
    pub const UI_TEXT_RUN: &str = "ui.TextRun";
    pub const UI_CHROME: &str = "ui.Chrome";

    // Font Graph Kinds
    pub const FONT_SUPERFAMILY: &str = "font.Superfamily";
    pub const FONT_FAMILY: &str = "font.Family";
    pub const FONT_FACE: &str = "font.Face";
    pub const FONT_FILE: &str = "font.File";
    pub const FONT_COVERAGE: &str = "font.Coverage";

    pub const CLOCK: &str = "Clock";

    pub const BINDING: &str = "Binding";

    // XML Graph Kinds
    pub const XML_DOCUMENT: &str = "xml.Document";
    pub const XML_ELEMENT: &str = "xml.Element";
    pub const XML_ATTRIBUTE: &str = "xml.Attribute";
    pub const XML_TEXT: &str = "xml.Text";
}

/// Snapshot-related constants for UI presentation surfaces.
pub mod ui_snapshot {
    /// Pixel format for RGBA8888 surfaces.
    pub const PIXEL_FORMAT_RGBA8888: u64 = 1;
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

    // UI Relations
    pub const CHILD_OF: &str = "CHILD_OF";
    pub const HAS_CHILD: &str = "HAS_CHILD";
    pub const CLIP_TO: &str = "CLIP_TO";
    pub const ROOT_UI: &str = "ROOT_UI";

    // Font Graph Relationships
    pub const FONT_CONTAINS: &str = "font.contains";
    pub const FONT_COVERS: &str = "font.covers";
    pub const FONT_MEMBER_OF: &str = "font.member_of";
    pub const FONT_FALLBACK_AFTER: &str = "font.fallback_after";
    pub const FONT_ALIAS: &str = "font.alias";

    // XML Relationships
    pub const HAS_ROOT: &str = "HAS_ROOT";
    pub const HAS_ATTR: &str = "HAS_ATTR";
}

// Virtio GPU additions
pub mod virtio {
    pub const VENDOR_ID: u16 = 0x1af4;
    pub const GPU_DEVICE_ID: u16 = 0x1050;
}

// HID / Input additions (Bristle v0)
pub mod hid {
    // Service kinds
    pub const SVC_INPUT: &str = "svc.Input"; // Bristle broker

    // Device kinds
    pub const DEV_HID_KEYBOARD: &str = "dev.hid.Keyboard";
    pub const DEV_HID_MOUSE: &str = "dev.hid.Mouse";
    pub const DEV_HID_TOUCHPAD: &str = "dev.hid.Touchpad";
    pub const DEV_HID_GAMEPAD: &str = "dev.hid.Gamepad";

    // Driver kinds
    pub const DRV_PS2_KEYBOARD: &str = "drv.Ps2Keyboard";
    pub const DRV_PS2_MOUSE: &str = "drv.Ps2Mouse";

    // Relations
    pub const REL_CONSUMES: &str = "CONSUMES"; // (svc.Input)-[:CONSUMES]->(dev.hid.Keyboard)
    pub const REL_ROUTES_TO: &str = "ROUTES_TO"; // (svc.Input)-[:ROUTES_TO]->(app.Echo)
    pub const REL_PRODUCES: &str = "PRODUCES"; // (drv.Ps2Keyboard)-[:PRODUCES]->(dev.hid.Keyboard)
}
