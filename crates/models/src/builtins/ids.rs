use abi::ThingId;

// Reserved range for builtins (e.g. 1..=100)

// Meta-kinds
pub const THING_KIND_KIND: ThingId = ThingId(1);
pub const THING_SCHEMA_KIND: ThingId = ThingId(2);

// Core Kinds & Schemas are now defined via macros in kinds.rs

// Schemas for Meta-Kinds
pub const THING_KIND_SCHEMA: ThingId = ThingId(11);
pub const THING_SCHEMA_SCHEMA: ThingId = ThingId(12);
