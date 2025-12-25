use abi::ThingId;

// Reserved range for builtins (e.g. 1..=100)

// Meta-kinds
pub const THING_KIND_KIND: ThingId = ThingId(1);
pub const THING_SCHEMA_KIND: ThingId = ThingId(2);

// Core Kinds
pub const THING_LINK_KIND: ThingId = ThingId(3);
pub const THING_INTENT_KIND: ThingId = ThingId(4);
pub const THING_OBSERVATION_KIND: ThingId = ThingId(5);
pub const THING_RESULT_KIND: ThingId = ThingId(6);

// Schemas for the above kinds
pub const THING_KIND_SCHEMA: ThingId = ThingId(11);
pub const THING_SCHEMA_SCHEMA: ThingId = ThingId(12);
pub const THING_LINK_SCHEMA: ThingId = ThingId(13);
pub const THING_INTENT_SCHEMA: ThingId = ThingId(14);
pub const THING_OBSERVATION_SCHEMA: ThingId = ThingId(15);
pub const THING_RESULT_SCHEMA: ThingId = ThingId(16);
