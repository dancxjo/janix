use crate::builtins::ids::*;
use crate::builtins::symbols::*;
use crate::kind::KindBody;
use crate::schema::SchemaBody;
use crate::thing::Thing;
use crate::thing_kind;
use crate::value::ThingBody;
use abi::{ThingId, SymbolId};
use alloc::vec::Vec;

use abi::wire::typed::{CodecId, TypeId, TypedBytes};

// Helper to create a Thing
fn make_thing<T: serde::Serialize>(
    id: ThingId,
    kind: SymbolId,
    body_struct: &T,
    body_type_id: ThingId,
) -> Thing {
    let bytes = postcard::to_allocvec(body_struct).expect("builtin encode failed");
    let typed = TypedBytes {
        type_id: TypeId(body_type_id.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes,
    };
    Thing {
        id,
        kind,
        payload: ThingBody::from(&typed).expect("builtin encode failed").bytes,
    }
}

// --- Meta-Kinds (Kind, Schema) ---

pub fn builtin_kind_kind() -> Thing {
    make_thing(
        THING_KIND_KIND,
        SYM_KIND,
        &KindBody {
            name: SYM_KIND,
            version: 1,
            schema: THING_KIND_SCHEMA,
        },
        THING_KIND_KIND,
    )
}

pub fn builtin_schema_kind() -> Thing {
    make_thing(
        THING_SCHEMA_KIND,
        SYM_KIND,
        &KindBody {
            name: SYM_SCHEMA,
            version: 1,
            schema: THING_SCHEMA_SCHEMA,
        },
        THING_KIND_KIND,
    )
}

// --- Schemas for Meta-Kinds ---

fn make_schema_thing(id: ThingId) -> Thing {
    make_thing(
        id,
        SYM_SCHEMA,
        &SchemaBody {
            body_type: 0, // Placeholder
            link_rules: Vec::new(),
        },
        THING_SCHEMA_KIND,
    )
}

use crate::declare::type_tag::fnv1a64;
use crate::schema::LinkRule;
use alloc::vec; // For vec! macro

pub fn builtin_kind_schema() -> Thing {
    make_thing(
        THING_KIND_SCHEMA,
        SYM_SCHEMA,
        &SchemaBody {
            body_type: fnv1a64("thingos.KindBody.v1"), // KindBody type tag
            link_rules: vec![LinkRule {
                predicate_kind: THING_HAS_SCHEMA_KIND,
                min: 1,
                max: Some(1),
            }],
        },
        THING_SCHEMA_KIND,
    )
}

pub fn builtin_schema_schema() -> Thing {
    make_schema_thing(THING_SCHEMA_SCHEMA)
}

// --- Core Kinds (Macro Generated) ---

thing_kind! {
    kind Link {
        id: crate::builtins::ids::THING_LINK_KIND,
        sym: SYM_LINK,
        version: 1,
        body: crate::link::LinkBody,
        type_tag: "thingos.LinkBody.v1",
        schema_id: crate::builtins::ids::THING_LINK_SCHEMA,

        links {
             // Link can link to anything? Or specific rules?
             // For now, minimal rules as placeholders if not specified
        }
    }
}

thing_kind! {
    kind Intent {
        id: crate::builtins::ids::THING_INTENT_KIND,
        sym: SYM_INTENT,
        version: 1,
        body: crate::intent::IntentBody,
        type_tag: "thingos.IntentBody.v1",
        schema_id: crate::builtins::ids::THING_INTENT_SCHEMA,

        links {
            predicate THING_RESULT_KIND min 0 max 1;
            predicate THING_OBSERVATION_KIND min 0 max many;
        }
    }
}

thing_kind! {
    kind Observation {
        id: crate::builtins::ids::THING_OBSERVATION_KIND,
        sym: SYM_OBSERVATION,
        version: 1,
        body: crate::observation::ObservationBody,
        type_tag: "thingos.ObservationBody.v1",
        schema_id: crate::builtins::ids::THING_OBSERVATION_SCHEMA,

        links {}
    }
}

thing_kind! {
    kind Font {
        id: crate::builtins::ids::THING_FONT_KIND,
        sym: SYM_FONT,
        version: 1,
        body: crate::font::FontBody,
        type_tag: "thingos.FontBody.v1",
        schema_id: crate::builtins::ids::THING_FONT_SCHEMA,

        links {}
    }
}

thing_kind! {
    kind Result {
        id: crate::builtins::ids::THING_RESULT_KIND,
        sym: SYM_RESULT,
        version: 1,
        body: crate::result::ResultBody,
        type_tag: "thingos.ResultBody.v1",
        schema_id: crate::builtins::ids::THING_RESULT_SCHEMA,

        links {}
    }
}
