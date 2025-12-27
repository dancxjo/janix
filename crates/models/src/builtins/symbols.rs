use abi::SymbolId;
use crate::declare::type_tag::fnv1a64;


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
