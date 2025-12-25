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
            pub const [<THING_ $KindName:upper _KIND>]: $crate::abi::ThingId = $crate::abi::ThingId($kind_id);
            pub const [<THING_ $KindName:upper _SCHEMA>]: $crate::abi::ThingId = $crate::abi::ThingId($schema_id);
            
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
                let schema_thing = $crate::Thing {
                    id: [<THING_ $KindName:upper _SCHEMA>],
                    kind: THING_SCHEMA_KIND,
                    body: ThingBody::from(&schema_body).expect("schema encode failed"),
                };

                let kind_body = KindBody {
                    name: $kind_sym,
                    version: $version,
                    schema: [<THING_ $KindName:upper _SCHEMA>],
                };

                // The Kind Thing
                let kind_thing = $crate::Thing {
                    id: [<THING_ $KindName:upper _KIND>],
                    kind: THING_KIND_KIND, 
                    body: ThingBody::from(&kind_body).expect("kind encode failed"),
                };

                [kind_thing, schema_thing]
            }
        }
    };
}

/// Helper to handle `max many` vs `max N` syntax
#[macro_export]
macro_rules! thing_kind_max {
    (many) => { None };
    ($n:literal) => { Some($n) };
}
