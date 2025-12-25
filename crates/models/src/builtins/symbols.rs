use abi::SymbolId;

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
