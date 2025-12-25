use abi::SymbolId;
use crate::declare::type_tag::fnv1a64;


// Deterministic symbol IDs.
// In a real implementation, these would be hash("string").
// For this task, we pick arbitrary stable values.

// Kind names
pub const SYM_KIND: SymbolId = SymbolId(100);
pub const SYM_SCHEMA: SymbolId = SymbolId(101);
pub const SYM_LINK: SymbolId = SymbolId(102);
pub const SYM_INTENT: SymbolId = SymbolId(103);
pub const SYM_OBSERVATION: SymbolId = SymbolId(104);
pub const SYM_RESULT: SymbolId = SymbolId(105);

// Properties and field names
pub const SYM_NAME: SymbolId = SymbolId(200);
pub const SYM_VERSION: SymbolId = SymbolId(201);
pub const SYM_SCHEMA_REF: SymbolId = SymbolId(202); // "schema"
pub const SYM_BODY_TYPE: SymbolId = SymbolId(203);
pub const SYM_LINK_RULES: SymbolId = SymbolId(204);
pub const SYM_FROM: SymbolId = SymbolId(205);
pub const SYM_TO: SymbolId = SymbolId(206);
pub const SYM_PREDICATE: SymbolId = SymbolId(207);
pub const SYM_STATE: SymbolId = SymbolId(208);

pub const SYM_CREATED_AT: SymbolId = SymbolId(209);

// Core kind symbols
pub const SYM_TIME_NOW: SymbolId = SymbolId(fnv1a64("TimeNow") as u32);
pub const SYM_PROCESS: SymbolId = SymbolId(fnv1a64("Process") as u32);
pub const SYM_THREAD: SymbolId = SymbolId(fnv1a64("Thread") as u32);
pub const SYM_CAPABILITY: SymbolId = SymbolId(fnv1a64("Capability") as u32);
pub const SYM_GRAPH: SymbolId = SymbolId(fnv1a64("Graph") as u32);
pub const SYM_MOUNT: SymbolId = SymbolId(fnv1a64("Mount") as u32);
pub const SYM_GRAPH_PROVIDER: SymbolId = SymbolId(fnv1a64("GraphProvider") as u32);
pub const SYM_BUFFER: SymbolId = SymbolId(fnv1a64("Buffer") as u32);
pub const SYM_STREAM: SymbolId = SymbolId(fnv1a64("Stream") as u32);

// Predicate kind symbols
pub const SYM_OWNS: SymbolId = SymbolId(fnv1a64("OWNS") as u32);
pub const SYM_HAS_CAP: SymbolId = SymbolId(fnv1a64("HAS_CAP") as u32);
pub const SYM_HAS_SCHEMA: SymbolId = SymbolId(fnv1a64("HAS_SCHEMA") as u32);
pub const SYM_MOUNTS: SymbolId = SymbolId(fnv1a64("MOUNTS") as u32);
pub const SYM_BACKED_BY: SymbolId = SymbolId(fnv1a64("BACKED_BY") as u32);
