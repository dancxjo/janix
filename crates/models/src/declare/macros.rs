/// Macro to declare a Kind, its Schema, and generate seed functions.
///
/// Syntax:
///
/// ```rust,ignore
/// thing_kind! {
///     kind Intent {
///         id: 1001,
///         sym: SYM_INTENT,
///         version: 1,
///         body: IntentBody,
///         type_tag: "thingos.IntentBody.v1",
///         schema_id: 2001,
///         
///         links {
///             predicate THING_RESULT_KIND min 0 max 1;
///             predicate THING_OBSERVATION_KIND min 0 max many;
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! thing_kind {
    (
        kind $KindName:ident {
            id: $kind_id:expr,
            sym: $kind_sym:expr,
            version: $version:expr,
            body: $BodyType:ty,
            type_tag: $type_tag_str:expr,
            schema_id: $schema_id:expr,

            links {
                $(
                    predicate $pred_kind:ident
                    min $min:literal
                    max $max:tt
                );* $(;)?
            }
        }
    ) => {
        paste::paste! {
            pub const [<THING_ $KindName:upper _KIND>]: $crate::abi::ThingId = $kind_id;
            pub const [<THING_ $KindName:upper _SCHEMA>]: $crate::abi::ThingId = $schema_id;

            pub const [<$KindName:upper _TYPE_TAG>]: $crate::declare::TypeTag =
                $crate::declare::type_tag($type_tag_str);

            #[allow(unused_imports)]
            pub fn [<seed_ $KindName:snake _kind>]() -> [$crate::Thing; 2] {
                use alloc::vec;
                use $crate::schema::{SchemaBody, LinkRule};
                use $crate::kind::KindBody;
                use $crate::value::ThingBody;
                use $crate::builtins::ids::*;

                let schema_body = SchemaBody {
                    body_type: [<$KindName:upper _TYPE_TAG>].0,
                    link_rules: vec![
                        $(
                            LinkRule {
                                predicate_kind: $pred_kind,
                                min: $min,
                                max: $crate::thing_kind_max!($max),
                            },
                        )*
                    ]
                };

                // The Schema Thing
                let schema_bytes = postcard::to_allocvec(&schema_body).expect("schema encode failed");
                let schema_typed = $crate::abi::wire::typed::TypedBytes {
                    type_id: $crate::abi::wire::typed::TypeId(THING_SCHEMA_KIND.0 as u128),
                    codec_id: $crate::abi::wire::typed::CodecId::POSTCARD,
                    bytes: schema_bytes,
                };

                let schema_thing = $crate::Thing {
                    id: [<THING_ $KindName:upper _SCHEMA>],
                    kind: $crate::builtins::symbols::SYM_SCHEMA,
                    payload: ThingBody::from(&schema_typed).expect("schema encode failed").bytes,
                };

                // The Kind Thing
                let kind_body = KindBody {
                    name: $kind_sym,
                    version: $version,
                    schema: [<THING_ $KindName:upper _SCHEMA>],
                };

                let kind_bytes = postcard::to_allocvec(&kind_body).expect("kind encode failed");
                let kind_typed = $crate::abi::wire::typed::TypedBytes {
                    type_id: $crate::abi::wire::typed::TypeId(THING_KIND_KIND.0 as u128),
                    codec_id: $crate::abi::wire::typed::CodecId::POSTCARD,
                    bytes: kind_bytes,
                };

                let kind_thing = $crate::Thing {
                    id: [<THING_ $KindName:upper _KIND>],
                    kind: $crate::builtins::symbols::SYM_KIND,
                    payload: ThingBody::from(&kind_typed).expect("kind encode failed").bytes,
                };

                [kind_thing, schema_thing]
            }
        }
    };
}

// Helper macro for predicate kinds
#[macro_export]
macro_rules! predicate_kind {
    (
      kind $Name:ident {
        id: $id:expr,
        sym: $sym:expr,
        version: $version:expr,
        schema_id: $schema_id:expr
      }
    ) => {
        $crate::thing_kind! {
            kind $Name {
                id: $id,
                sym: $sym,
                version: $version,
                body: $crate::builtins::predicates::PredicateBody,
                type_tag: "thingos.PredicateBody.v1",
                schema_id: $schema_id,
                links {}
            }
        }
    };
}

/// Helper to handle `max many` vs `max N` syntax
#[macro_export]
macro_rules! thing_kind_max {
    (many) => {
        None
    };
    ($n:literal) => {
        Some($n)
    };
}
