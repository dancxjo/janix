use crate::declare::type_tag::fnv1a64;
use abi::SymbolId;

// Deterministic symbol IDs.
// Hash FNV-1a 64-bit of canonical strings.

// Kind names
pub const SYM_KIND: SymbolId = SymbolId(fnv1a64("Kind"));
pub const SYM_SCHEMA: SymbolId = SymbolId(fnv1a64("Schema"));
pub const SYM_LINK: SymbolId = SymbolId(fnv1a64("Link"));
pub const SYM_INTENT: SymbolId = SymbolId(fnv1a64("Intent")); // Assuming "Intent" is the string
pub const SYM_OBSERVATION: SymbolId = SymbolId(fnv1a64("Observation"));
pub const SYM_RESULT: SymbolId = SymbolId(fnv1a64("Result"));

// Properties and field names
pub const SYM_NAME: SymbolId = SymbolId(fnv1a64("name"));
pub const SYM_VERSION: SymbolId = SymbolId(fnv1a64("version"));
pub const SYM_SCHEMA_REF: SymbolId = SymbolId(fnv1a64("schema"));
pub const SYM_BODY_TYPE: SymbolId = SymbolId(fnv1a64("body_type")); // Guessing string
pub const SYM_LINK_RULES: SymbolId = SymbolId(fnv1a64("link_rules")); // Guessing string
pub const SYM_FROM: SymbolId = SymbolId(fnv1a64("from"));
pub const SYM_TO: SymbolId = SymbolId(fnv1a64("to"));
pub const SYM_PREDICATE: SymbolId = SymbolId(fnv1a64("predicate"));
pub const SYM_STATE: SymbolId = SymbolId(fnv1a64("state"));

pub const SYM_CREATED_AT: SymbolId = SymbolId(fnv1a64("created_at_ns")); // Matching B.4 list? B.4 has created_at_ns

// Core kind symbols
pub const SYM_TIME_NOW: SymbolId = SymbolId(fnv1a64("TimeNow"));
pub const SYM_PROCESS: SymbolId = SymbolId(fnv1a64("Process"));
pub const SYM_THREAD: SymbolId = SymbolId(fnv1a64("Thread"));
pub const SYM_CAPABILITY: SymbolId = SymbolId(fnv1a64("Capability"));
pub const SYM_GRAPH: SymbolId = SymbolId(fnv1a64("Graph"));
pub const SYM_MOUNT: SymbolId = SymbolId(fnv1a64("Mount"));
pub const SYM_GRAPH_PROVIDER: SymbolId = SymbolId(fnv1a64("GraphProvider"));
pub const SYM_BUFFER: SymbolId = SymbolId(fnv1a64("Buffer"));
pub const SYM_STREAM: SymbolId = SymbolId(fnv1a64("Stream"));

// Predicate kind symbols
pub const SYM_OWNS: SymbolId = SymbolId(fnv1a64("OWNS"));
pub const SYM_HAS_CAP: SymbolId = SymbolId(fnv1a64("HAS_CAP"));
pub const SYM_HAS_SCHEMA: SymbolId = SymbolId(fnv1a64("HAS_SCHEMA"));
pub const SYM_MOUNTS: SymbolId = SymbolId(fnv1a64("MOUNTS"));
pub const SYM_BACKED_BY: SymbolId = SymbolId(fnv1a64("BACKED_BY"));
pub const SYM_LAUNCHES: SymbolId = SymbolId(fnv1a64("LAUNCHES"));

// Input symbols
pub const SYM_KEYBOARD: SymbolId = SymbolId(fnv1a64("Keyboard"));

pub const SYM_KEY_EVENT: SymbolId = SymbolId(fnv1a64("KeyEvent"));
pub const SYM_KEY_EVENT_STREAM: SymbolId = SymbolId(fnv1a64("KeyEventStream"));

pub const SYM_MOUSE: SymbolId = SymbolId(fnv1a64("Mouse"));
pub const SYM_POINTER_EVENT: SymbolId = SymbolId(fnv1a64("PointerEvent"));
pub const SYM_POINTER_EVENT_STREAM: SymbolId = SymbolId(fnv1a64("PointerEventStream"));
pub const SYM_DX: SymbolId = SymbolId(fnv1a64("dx"));
pub const SYM_DY: SymbolId = SymbolId(fnv1a64("dy"));
pub const SYM_SCROLL: SymbolId = SymbolId(fnv1a64("scroll"));
pub const SYM_BUTTONS: SymbolId = SymbolId(fnv1a64("buttons"));

pub const SYM_EMITS: SymbolId = SymbolId(fnv1a64("EMITS"));
pub const SYM_SUBSCRIBES_TO: SymbolId = SymbolId(fnv1a64("SUBSCRIBES_TO"));

pub const SYM_BUS: SymbolId = SymbolId(fnv1a64("bus"));
pub const SYM_DEVICE: SymbolId = SymbolId(fnv1a64("device"));
pub const SYM_SCANCODE: SymbolId = SymbolId(fnv1a64("scancode"));
pub const SYM_IS_RELEASE: SymbolId = SymbolId(fnv1a64("is_release"));

pub const SYM_PS2: SymbolId = SymbolId(fnv1a64("ps2"));

pub const SYM_FONT: SymbolId = SymbolId(fnv1a64("Font"));
pub const SYM_BOOT_ROOT: SymbolId = SymbolId(fnv1a64("BootRoot"));
pub const SYM_BOOT_PROGRAM: SymbolId = SymbolId(fnv1a64("BootProgram"));
pub const SYM_BINARY: SymbolId = SymbolId(fnv1a64("binary"));
pub const SYM_PRIORITY: SymbolId = SymbolId(fnv1a64("priority"));

// Module / Asset Symbols
pub const SYM_MODULE: SymbolId = SymbolId(fnv1a64("Module"));
pub const SYM_PROGRAM_IMAGE: SymbolId = SymbolId(fnv1a64("ProgramImage"));
pub const SYM_BITMAP: SymbolId = SymbolId(fnv1a64("Bitmap"));
pub const SYM_HAS_MODULE: SymbolId = SymbolId(fnv1a64("HAS_MODULE"));
pub const SYM_BINARY_IMAGE: SymbolId = SymbolId(fnv1a64("BINARY_IMAGE"));
pub const SYM_ASSET: SymbolId = SymbolId(fnv1a64("ASSET"));
pub const SYM_PROVIDES_FONT: SymbolId = SymbolId(fnv1a64("PROVIDES_FONT"));
pub const SYM_DEFAULT_FONT: SymbolId = SymbolId(fnv1a64("DEFAULT_FONT"));
pub const SYM_USES_MODULE: SymbolId = SymbolId(fnv1a64("USES_MODULE"));

pub const SYM_MIME: SymbolId = SymbolId(fnv1a64("mime"));
pub const SYM_MODULE_TYPE: SymbolId = SymbolId(fnv1a64("module_type"));
pub const SYM_ROLE: SymbolId = SymbolId(fnv1a64("role"));
pub const SYM_SIZE_BYTES: SymbolId = SymbolId(fnv1a64("size_bytes"));
pub const SYM_BASE_PHYS: SymbolId = SymbolId(fnv1a64("base_phys"));
pub const SYM_MODULE_INDEX: SymbolId = SymbolId(fnv1a64("module_index"));
pub const SYM_GLYPH_WIDTH: SymbolId = SymbolId(fnv1a64("glyph_width"));
pub const SYM_GLYPH_HEIGHT: SymbolId = SymbolId(fnv1a64("glyph_height"));
pub const SYM_GLYPH_COUNT: SymbolId = SymbolId(fnv1a64("glyph_count"));
pub const SYM_FORMAT: SymbolId = SymbolId(fnv1a64("format"));
// Diag / Logging
pub const SYM_LOG_ENTRY: SymbolId = SymbolId(fnv1a64("LogEntry"));
pub const SYM_ERROR: SymbolId = SymbolId(fnv1a64("Error"));
pub const SYM_FAULT: SymbolId = SymbolId(fnv1a64("Fault"));

pub const SYM_LEVEL: SymbolId = SymbolId(fnv1a64("level"));
pub const SYM_MESSAGE: SymbolId = SymbolId(fnv1a64("message"));
pub const SYM_SUBSYSTEM: SymbolId = SymbolId(fnv1a64("subsystem"));
pub const SYM_CPU_ID: SymbolId = SymbolId(fnv1a64("cpu_id"));
pub const SYM_THREAD_ID: SymbolId = SymbolId(fnv1a64("thread_id"));
pub const SYM_PROCESS_ID: SymbolId = SymbolId(fnv1a64("process_id"));
pub const SYM_SEQ: SymbolId = SymbolId(fnv1a64("seq"));
pub const SYM_CODE: SymbolId = SymbolId(fnv1a64("code"));
pub const SYM_SEVERITY: SymbolId = SymbolId(fnv1a64("severity"));
pub const SYM_RECOVERABLE: SymbolId = SymbolId(fnv1a64("recoverable"));
pub const SYM_FAULT_KIND: SymbolId = SymbolId(fnv1a64("fault_kind"));
pub const SYM_RIP: SymbolId = SymbolId(fnv1a64("rip"));
pub const SYM_RSP: SymbolId = SymbolId(fnv1a64("rsp"));
pub const SYM_RFLAGS: SymbolId = SymbolId(fnv1a64("rflags"));
pub const SYM_CR2: SymbolId = SymbolId(fnv1a64("cr2"));
pub const SYM_ERROR_CODE: SymbolId = SymbolId(fnv1a64("error_code"));
pub const SYM_ACCESS: SymbolId = SymbolId(fnv1a64("access"));
pub const SYM_ADDRESS_SPACE: SymbolId = SymbolId(fnv1a64("address_space"));
pub const SYM_KILL_ACTION: SymbolId = SymbolId(fnv1a64("kill_action"));

pub const SYM_SNIFF: SymbolId = SymbolId(fnv1a64("sniff"));
pub const SYM_VALID: SymbolId = SymbolId(fnv1a64("valid"));
pub const SYM_KIND_STR: SymbolId = SymbolId(fnv1a64("kind"));

pub const SYM_HAS_DEVICE: SymbolId = SymbolId(fnv1a64("HAS_DEVICE"));
pub const SYM_SPAWNED: SymbolId = SymbolId(fnv1a64("SPAWNED"));
pub const SYM_RUNS: SymbolId = SymbolId(fnv1a64("RUNS"));

pub const SYM_DROPPED: SymbolId = SymbolId(fnv1a64("dropped"));
pub const SYM_HEAD_SEQ: SymbolId = SymbolId(fnv1a64("head_seq"));
pub const SYM_CAPACITY: SymbolId = SymbolId(fnv1a64("capacity"));
pub const SYM_LAYOUT: SymbolId = SymbolId(fnv1a64("layout"));
pub const SYM_EVENTS: SymbolId = SymbolId(fnv1a64("events"));

// Devices
pub const SYM_PCI_DEVICE: SymbolId = SymbolId(fnv1a64("PciDevice"));
pub const SYM_SERIAL_PORT: SymbolId = SymbolId(fnv1a64("SerialPort"));
pub const SYM_LOG_STREAM: SymbolId = SymbolId(fnv1a64("LogStream"));
pub const SYM_BLOCK_DEVICE: SymbolId = SymbolId(fnv1a64("BlockDevice"));

// GraphFS
pub const SYM_FILESYSTEM: SymbolId = SymbolId(fnv1a64("FileSystem"));
pub const SYM_FILE: SymbolId = SymbolId(fnv1a64("File"));
pub const SYM_DIR: SymbolId = SymbolId(fnv1a64("Dir"));
pub const SYM_VOLUME: SymbolId = SymbolId(fnv1a64("Volume"));

// Links
pub const SYM_HAS_BLOCK_DEVICE: SymbolId = SymbolId(fnv1a64("HAS_BLOCK_DEVICE"));
pub const SYM_HAS_VOLUME: SymbolId = SymbolId(fnv1a64("HAS_VOLUME"));
pub const SYM_HAS_MOUNT: SymbolId = SymbolId(fnv1a64("HAS_MOUNT"));
pub const SYM_HAS_ENTRY: SymbolId = SymbolId(fnv1a64("HAS_ENTRY"));
pub const SYM_BACKED_BY_DEVICE: SymbolId = SymbolId(fnv1a64("BACKED_BY_DEVICE"));
pub const SYM_ON_VOLUME: SymbolId = SymbolId(fnv1a64("ON_VOLUME"));
pub const SYM_IS_MOUNTED_ON: SymbolId = SymbolId(fnv1a64("IS_MOUNTED_ON"));
pub const SYM_CONTAINS_FILE: SymbolId = SymbolId(fnv1a64("CONTAINS_FILE"));
pub const SYM_HAS_KEYBOARD: SymbolId = SymbolId(fnv1a64("HAS_KEYBOARD"));

// Other IDs seen in dump or ids.rs
pub const SYM_HAS_CONSOLE: SymbolId = SymbolId(fnv1a64("HAS_CONSOLE"));
pub const SYM_HAS_TIME_NOW: SymbolId = SymbolId(fnv1a64("HAS_TIME_NOW"));

// Framebuffer
pub const SYM_DISPLAY_FRAMEBUFFER: SymbolId = SymbolId(fnv1a64("DisplayFramebuffer"));
pub const SYM_SCANOUT_BUFFER: SymbolId = SymbolId(fnv1a64("ScanoutBuffer"));

