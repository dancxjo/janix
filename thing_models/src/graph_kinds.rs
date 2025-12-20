//! Canonical graph kind and link identifiers.
//!
//! Moved from `abi` to `thing_models` to reduce ABI surface area.

use abi::Predicate;

pub const KIND_KIND: &str = "Kind";
pub const KIND_THREAD: &str = "Thread";
pub const KIND_PROCESS: &str = "Process";
pub const KIND_CPU_CORE: &str = "CpuCore";
pub const KIND_SLEEP_EVENT: &str = "SleepEvent";
pub const KIND_BOOT_PROFILE: &str = "BootProfile";
pub const KIND_BOOT_PROGRAM: &str = "BootProgram";
pub const KIND_PROGRAM_IMAGE: &str = "ProgramImage";
pub const KIND_FONT_MODULE: &str = "FontModule";
pub const KIND_RAW_MODULE: &str = "RawModule";
pub const KIND_TIME_SOURCE: &str = "TimeSource";
pub const KIND_ALARM_REQUEST: &str = "AlarmRequest";
pub const KIND_ALARM_EVENT: &str = "AlarmEvent";
pub const KIND_IO_PORT_REGION: &str = "IoPortRegion";
pub const KIND_IO_PORT_OP: &str = "IoPortOp";
pub const KIND_INTERRUPT_EVENT: &str = "InterruptEvent";
pub const KIND_KEYSCAN_EVENT: &str = "KeyScanEvent";
pub const KIND_INPUT_CHAR_EVENT: &str = "InputCharEvent";
pub const KIND_MOUSE_PACKET_EVENT: &str = "MousePacketEvent";
pub const KIND_DISPLAY: &str = "Display";
pub const KIND_SHARED_BUFFER: &str = "SharedBuffer";
pub const KIND_DISPLAY_FRAMEBUFFER: &str = "DisplayFramebuffer";
pub const KIND_DISPLAY_FRAME: &str = "DisplayFrame";
pub const KIND_DISPLAY_PRESENT_REQUEST: &str = "DisplayPresentRequest";
pub const KIND_MODE: &str = "Mode";
pub const KIND_INTERRUPT_REQUEST: &str = "InterruptRequest";
pub const PROP_IRQ_LINE: &str = "irq_line";
pub const PROP_ENABLED: &str = "enabled";
pub const KIND_MODE_SWITCH_EVENT: &str = "ModeSwitchEvent";
pub const KIND_PLACE: &str = "Place";
pub const KIND_WINDOW: &str = "Window";
pub const KIND_SURFACE: &str = "Surface";
pub const KIND_VIEW: &str = "View";
pub const KIND_CURSOR: &str = "Cursor";
pub const KIND_LINK: &str = "LNK";

pub const KIND_USB_CONTROLLER: &str = "UsbController";
pub const KIND_USB_DEVICE: &str = "UsbDevice";
pub const KIND_USB_ENDPOINT: &str = "UsbEndpoint";
pub const KIND_USB_TRANSFER_REQUEST: &str = "UsbTransferRequest";
pub const KIND_USB_TRANSFER_RESULT: &str = "UsbTransferResult";

pub const KIND_PCI_DEVICE: &str = "PciDevice";
pub const PROP_BUS: &str = "bus";
pub const PROP_SLOT: &str = "slot";
pub const PROP_FUNC: &str = "func";
pub const PROP_VENDOR_ID: &str = "vendor_id";
pub const PROP_DEVICE_ID: &str = "device_id";
pub const PROP_CLASS_ID: &str = "class_id";
pub const PROP_SUBCLASS_ID: &str = "subclass_id";
pub const PROP_PROG_IF: &str = "prog_if";
pub const PROP_BAR0: &str = "bar0";
pub const PROP_BAR1: &str = "bar1";
pub const PROP_BAR2: &str = "bar2";
pub const PROP_BAR3: &str = "bar3";
pub const PROP_BAR4: &str = "bar4";
pub const PROP_BAR5: &str = "bar5";

pub const KIND_SYSTEM: &str = "System";

pub mod canon {
    use abi::Predicate;

    pub const P_PROC_OWNS_THREAD: Predicate = Predicate(0x0001);
    pub const P_SCHED_RUNS_ON: Predicate = Predicate(0x0002);
    pub const P_SCHED_SLEEPS_UNTIL: Predicate = Predicate(0x0003);
    pub const P_BOOT_LAUNCHES: Predicate = Predicate(0x0004);
    pub const P_INIT_SPAWNED: Predicate = Predicate(0x0005);
    pub const P_DISPLAY_SCANOUT: Predicate = Predicate(0x0006);
    pub const P_MODE_PLACE: Predicate = Predicate(0x0007);
    pub const P_WINDOW_SURFACE: Predicate = Predicate(0x0008);
    pub const P_PLACE_WINDOW: Predicate = Predicate(0x0009);

    pub const P_MODE_HAS_WINDOW: Predicate = Predicate(0x0010);
    pub const P_DISPLAY_FRONT_BUFFER: Predicate = Predicate(0x0011);
    pub const P_DISPLAY_BACK_BUFFER: Predicate = Predicate(0x0012);
    pub const P_WINDOW_HAS_SURFACE: Predicate = Predicate(0x0013);
    pub const P_ACTIVE_MODE: Predicate = Predicate(0x0015);
    pub const P_APP_OWNS_WINDOW: Predicate = Predicate(0x0016);
    pub const P_DISPLAY_HAS_FRONT_BUFFER: Predicate = Predicate(0x0020);
    pub const P_DISPLAY_HAS_BACK_BUFFER: Predicate = Predicate(0x0021);
    pub const P_HAS_ACTIVE_MODE: Predicate = Predicate(0x0022);
    pub const P_ABOUT: Predicate = Predicate(0x0023);
    pub const P_RUNNING: Predicate = Predicate(0x0024);
    pub const P_RESPAWNED_FROM: Predicate = Predicate(0x0025);
}

pub const LINK_OWNS_THREAD: Predicate = canon::P_PROC_OWNS_THREAD;
pub const LINK_RUNS_ON: Predicate = canon::P_SCHED_RUNS_ON;
pub const LINK_SLEEPS_UNTIL: Predicate = canon::P_SCHED_SLEEPS_UNTIL;
pub const LINK_LAUNCHES: Predicate = canon::P_BOOT_LAUNCHES;
pub const LINK_SPAWNED: Predicate = canon::P_INIT_SPAWNED;
pub const LINK_DISPLAY_SCANOUT: Predicate = canon::P_DISPLAY_SCANOUT;
pub const LINK_DISPLAY_FRONT_BUFFER: Predicate = canon::P_DISPLAY_FRONT_BUFFER;
pub const LINK_DISPLAY_BACK_BUFFER: Predicate = canon::P_DISPLAY_BACK_BUFFER;
pub const LINK_MODE_PLACE: Predicate = canon::P_MODE_PLACE;
pub const LINK_MODE_HAS_WINDOW: Predicate = canon::P_MODE_HAS_WINDOW;
pub const LINK_WINDOW_SURFACE: Predicate = canon::P_WINDOW_SURFACE;
pub const LINK_WINDOW_HAS_SURFACE: Predicate = canon::P_WINDOW_HAS_SURFACE;
pub const LINK_PLACE_WINDOW: Predicate = canon::P_PLACE_WINDOW;
pub const LINK_ACTIVE_MODE: Predicate = canon::P_ACTIVE_MODE;
pub const LINK_APP_OWNS_WINDOW: Predicate = canon::P_APP_OWNS_WINDOW;
pub const LINK_DISPLAY_HAS_FRONT_BUFFER: Predicate = canon::P_DISPLAY_HAS_FRONT_BUFFER;
pub const LINK_DISPLAY_HAS_BACK_BUFFER: Predicate = canon::P_DISPLAY_HAS_BACK_BUFFER;
pub const LINK_HAS_ACTIVE_MODE: Predicate = canon::P_HAS_ACTIVE_MODE;
pub const LINK_ABOUT: Predicate = canon::P_ABOUT;
pub const LINK_RUNNING: Predicate = canon::P_RUNNING;
pub const LINK_RESPAWNED_FROM: Predicate = canon::P_RESPAWNED_FROM;

pub const KIND_PROCESS_EXIT_EVENT: &str = "ProcessExitEvent";
pub const PROP_EXIT_REASON: &str = "exit_reason";
pub const PROP_EXIT_CODE: &str = "exit_code";

pub const PROP_WIDTH: &str = "width";
pub const PROP_HEIGHT: &str = "height";
pub const PROP_STRIDE: &str = "stride";
pub const PROP_PIXEL_FORMAT: &str = "pixel_format";
pub const PROP_NAME: &str = "name";
pub const PROP_LAYOUT_MODE: &str = "layout_mode";
pub const PROP_POWER_STATE: &str = "power_state";
pub const PROP_REFRESH_INTERVAL_NS: &str = "refresh_interval_ns";
pub const PROP_FRAMES_PRESENTED: &str = "frames_presented";
pub const PROP_LAST_PRESENT_NS: &str = "last_present_ns";
pub const PROP_MODE_INDEX: &str = "mode_index";
pub const PROP_MODE_PLACE: &str = "mode_place";
pub const PROP_MODE_ACTIVE: &str = "mode_active";
pub const PROP_MODE_LAYOUT_POLICY: &str = "layout_policy";
pub const PROP_WINDOW_X: &str = "x";
pub const PROP_WINDOW_Y: &str = "y";
pub const PROP_WINDOW_WIDTH: &str = "window_width";
pub const PROP_WINDOW_HEIGHT: &str = "window_height";
pub const PROP_Z_INDEX: &str = "z_index";
pub const PROP_WINDOW_ACTIVE: &str = "window_active";
pub const PROP_TITLE: &str = "title";
pub const PROP_SURFACE_KIND: &str = "surface_kind";
pub const PROP_SURFACE_TEXT: &str = "surface_text";
pub const PROP_WINDOW_ID: &str = "window_id";
pub const PROP_TIMESTAMP: &str = "timestamp";
pub const PROP_PLACE_ID: &str = "place_id";
pub const PROP_FRAMEBUFFER_ID: &str = "framebuffer_id";
pub const PROP_FRAME_INDEX: &str = "frame_index";
pub const PROP_REQUESTED_AT_NS: &str = "requested_at_ns";
pub const PROP_PRESENTED_AT_NS: &str = "presented_at_ns";
pub const PROP_COMPLETED: &str = "completed";
pub const PROP_IDENTIFIER: &str = "identifier";
pub const PROP_MODULE_INDEX: &str = "module_index";
pub const PROP_BASE_PHYS: &str = "base_phys";
pub const PROP_SIZE: &str = "size";
pub const PROP_DISPLAY_ACTIVE_BUFFER_INDEX: &str = "active_buffer_index";
pub const PROP_DRAGGABLE: &str = "draggable";
pub const PROP_RESIZABLE: &str = "resizable";
pub const PROP_CLOSABLE: &str = "closable";
pub const PROP_MINIMIZABLE: &str = "minimizable";
pub const PROP_SHARED_BUFFER_ID: &str = "shared_buffer_id";
pub const PROP_FONT_NAME: &str = "font_name";
pub const PROP_RAW_KIND: &str = "raw_kind";

pub const PROP_LINK_SRC: &str = "link_src";
pub const PROP_LINK_DST: &str = "link_dst";
pub const PROP_LINK_PRED: &str = "link_pred";

pub const PROP_RESPAWN_POLICY: &str = "respawn_policy";
pub const RESPAWN_NEVER: &str = "Never";
pub const RESPAWN_ALWAYS: &str = "Always";
pub const RESPAWN_ON_CRASH: &str = "OnCrash";

pub const PROP_VISIBLE: &str = "visible";
pub const PROP_TEXT: &str = "text";
pub const PROP_FONT_SIZE: &str = "font_size";
pub const PROP_FG_COLOR: &str = "fg_color";
pub const PROP_BG_COLOR: &str = "bg_color";
pub const PROP_CURSOR_SHAPE: &str = "cursor_shape";
