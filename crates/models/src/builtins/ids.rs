use abi::ThingId;

// Reserved range for builtins (e.g. 1..=100)

// Meta-kinds
pub const THING_KIND_KIND: ThingId = ThingId(1);
pub const THING_SCHEMA_KIND: ThingId = ThingId(2);

// Core Kinds & Schemas are now defined via macros in kinds.rs

// Schemas for Meta-Kinds
pub const THING_KIND_SCHEMA: ThingId = ThingId(11);
pub const THING_SCHEMA_SCHEMA: ThingId = ThingId(12);

// Core meta-circular kinds (manual or macro)
pub const THING_LINK_KIND: ThingId = ThingId(1003);
pub const THING_LINK_SCHEMA: ThingId = ThingId(2003);

pub const THING_INTENT_KIND: ThingId = ThingId(1004);
pub const THING_INTENT_SCHEMA: ThingId = ThingId(2004);

pub const THING_OBSERVATION_KIND: ThingId = ThingId(1005);
pub const THING_OBSERVATION_SCHEMA: ThingId = ThingId(2005);

pub const THING_RESULT_KIND: ThingId = ThingId(1006);
pub const THING_RESULT_SCHEMA: ThingId = ThingId(2006);

// Predicates (100–199)
pub const THING_OWNS_KIND: ThingId = ThingId(100);
pub const THING_OWNS_SCHEMA: ThingId = ThingId(1100);

pub const THING_HAS_CAP_KIND: ThingId = ThingId(101);
pub const THING_HAS_CAP_SCHEMA: ThingId = ThingId(1101);

pub const THING_HAS_SCHEMA_KIND: ThingId = ThingId(102);
pub const THING_HAS_SCHEMA_SCHEMA: ThingId = ThingId(1102);

pub const THING_MOUNTS_KIND: ThingId = ThingId(103);
pub const THING_MOUNTS_SCHEMA: ThingId = ThingId(1103);

pub const THING_BACKED_BY_KIND: ThingId = ThingId(104);
pub const THING_BACKED_BY_SCHEMA: ThingId = ThingId(1104);

pub const THING_LAUNCHES_KIND: ThingId = ThingId(105);
pub const THING_LAUNCHES_SCHEMA: ThingId = ThingId(1105);

// Core kinds (200–399)
pub const THING_TIME_NOW_KIND: ThingId = ThingId(200);
pub const THING_TIME_NOW_SCHEMA: ThingId = ThingId(1200);

pub const THING_PROCESS_KIND: ThingId = ThingId(201);
pub const THING_PROCESS_SCHEMA: ThingId = ThingId(1201);

pub const THING_THREAD_KIND: ThingId = ThingId(202);
pub const THING_THREAD_SCHEMA: ThingId = ThingId(1202);

pub const THING_CAPABILITY_KIND: ThingId = ThingId(203);
pub const THING_CAPABILITY_SCHEMA: ThingId = ThingId(1203);

pub const THING_GRAPH_KIND: ThingId = ThingId(204);
pub const THING_GRAPH_SCHEMA: ThingId = ThingId(1204);

pub const THING_MOUNT_KIND: ThingId = ThingId(205);
pub const THING_MOUNT_SCHEMA: ThingId = ThingId(1205);

pub const THING_GRAPH_PROVIDER_KIND: ThingId = ThingId(206);
pub const THING_GRAPH_PROVIDER_SCHEMA: ThingId = ThingId(1206);

pub const THING_BUFFER_KIND: ThingId = ThingId(207);
pub const THING_BUFFER_SCHEMA: ThingId = ThingId(1207);

pub const THING_STREAM_KIND: ThingId = ThingId(208);
pub const THING_STREAM_SCHEMA: ThingId = ThingId(1208);


pub const THING_KEYBOARD_KIND: ThingId = ThingId(210);
pub const THING_KEYBOARD_SCHEMA: ThingId = ThingId(1210);

pub const THING_KEY_EVENT_KIND: ThingId = ThingId(211);
pub const THING_KEY_EVENT_SCHEMA: ThingId = ThingId(1211);

pub const THING_FONT_KIND: ThingId = ThingId(212);

pub const THING_FONT_SCHEMA: ThingId = ThingId(1212);

pub const THING_BOOT_PROGRAM_KIND: ThingId = ThingId(213);
pub const THING_BOOT_PROGRAM_SCHEMA: ThingId = ThingId(1213);

pub const THING_KEY_EVENT_STREAM_KIND: ThingId = ThingId(214);
pub const THING_KEY_EVENT_STREAM_SCHEMA: ThingId = ThingId(1214);

pub const THING_EMITS_KIND: ThingId = ThingId(106);
pub const THING_EMITS_SCHEMA: ThingId = ThingId(1106);

pub const THING_SUBSCRIBES_TO_KIND: ThingId = ThingId(107);
pub const THING_SUBSCRIBES_TO_SCHEMA: ThingId = ThingId(1107);

pub const THING_BOOT_ROOT: ThingId = ThingId(1000); // The Root Node
pub const THING_LOG_ENTRY_KIND: ThingId = ThingId(220);
pub const THING_LOG_ENTRY_SCHEMA: ThingId = ThingId(1220);

pub const THING_ERROR_KIND: ThingId = ThingId(221);
pub const THING_ERROR_SCHEMA: ThingId = ThingId(1221);

pub const THING_FAULT_KIND: ThingId = ThingId(222);
pub const THING_FAULT_SCHEMA: ThingId = ThingId(1222);

// Singletons (2000+)
pub const THING_TIME_INSTANCE: ThingId = ThingId(2000);
