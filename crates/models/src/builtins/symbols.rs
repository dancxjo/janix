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
