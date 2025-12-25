use crate::builtins::ids::*;
use crate::builtins::symbols::*;
use crate::kind::KindBody;
use crate::schema::SchemaBody;
use crate::thing::Thing;
use crate::value::ThingBody;
use abi::ThingId;
use alloc::vec::Vec;

// Helper to create a Thing
fn make_thing<T: serde::Serialize>(id: ThingId, kind: ThingId, body_struct: &T) -> Thing {
    Thing {
        id,
        kind,
        body: ThingBody::from(body_struct).expect("builtin encode failed"),
    }
}

// --- Meta-Kinds (Kind, Schema) ---

pub fn builtin_kind_kind() -> Thing {
    make_thing(
        THING_KIND_KIND,
        THING_KIND_KIND, // Meta-circular
        &KindBody {
            name: SYM_KIND,
            version: 1,
            schema: THING_KIND_SCHEMA,
        },
    )
}

pub fn builtin_schema_kind() -> Thing {
    make_thing(
        THING_SCHEMA_KIND,
        THING_KIND_KIND, // It is a Kind
        &KindBody {
            name: SYM_SCHEMA,
            version: 1,
            schema: THING_SCHEMA_SCHEMA,
        },
    )
}

// --- Core Kinds ---

pub fn builtin_link_kind() -> Thing {
    make_thing(
        THING_LINK_KIND,
        THING_KIND_KIND,
        &KindBody {
            name: SYM_LINK,
            version: 1,
            schema: THING_LINK_SCHEMA,
        },
    )
}

pub fn builtin_intent_kind() -> Thing {
    make_thing(
        THING_INTENT_KIND,
        THING_KIND_KIND,
        &KindBody {
            name: SYM_INTENT,
            version: 1,
            schema: THING_INTENT_SCHEMA,
        },
    )
}

pub fn builtin_observation_kind() -> Thing {
    make_thing(
        THING_OBSERVATION_KIND,
        THING_KIND_KIND,
        &KindBody {
            name: SYM_OBSERVATION,
            version: 1,
            schema: THING_OBSERVATION_SCHEMA,
        },
    )
}

pub fn builtin_result_kind() -> Thing {
    make_thing(
        THING_RESULT_KIND,
        THING_KIND_KIND,
        &KindBody {
            name: SYM_RESULT,
            version: 1,
            schema: THING_RESULT_SCHEMA,
        },
    )
}

// --- Schemas ---

fn make_schema_thing(id: ThingId) -> Thing {
    make_thing(
        id,
        THING_SCHEMA_KIND, // Schema Things have Kind = SYSTEM_SCHEMA
        &SchemaBody {
            body_type: 0, // Placeholder
            link_rules: Vec::new(),
        },
    )
}

pub fn builtin_kind_schema() -> Thing {
    make_schema_thing(THING_KIND_SCHEMA)
}

pub fn builtin_schema_schema() -> Thing {
    make_schema_thing(THING_SCHEMA_SCHEMA)
}

pub fn builtin_link_schema() -> Thing {
    make_schema_thing(THING_LINK_SCHEMA)
}

pub fn builtin_intent_schema() -> Thing {
    make_schema_thing(THING_INTENT_SCHEMA)
}

pub fn builtin_observation_schema() -> Thing {
    make_schema_thing(THING_OBSERVATION_SCHEMA)
}

pub fn builtin_result_schema() -> Thing {
    make_schema_thing(THING_RESULT_SCHEMA)
}
