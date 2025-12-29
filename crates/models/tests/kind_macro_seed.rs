extern crate alloc;
use models::abi::ThingId;
use models::builtins::symbols::*;
use models::declare::type_tag;
use models::thing_kind;

// Dummy bodies for testing
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TestBody {
    foo: u32,
}

// Define symbol for test if not in builtins, but we can reuse existing logical symbols or mocks
// The macro requires an expr for sym, so we can pass anything that evaluates to SymbolId
const SYM_TEST: models::abi::SymbolId = models::abi::SymbolId(9999);
const THING_TEST_KIND_ID: u64 = 8888;
const THING_TEST_SCHEMA_ID: u64 = 9999;

const PRED_1: models::abi::ThingId = models::abi::ThingId(123);
const PRED_2: models::abi::ThingId = models::abi::ThingId(456);

thing_kind! {
    kind TestKind {
        id: models::abi::ThingId(THING_TEST_KIND_ID),
        sym: SYM_TEST,
        version: 1,
        body: TestBody,
        type_tag: "thingos.TestBody.v1",
        schema_id: models::abi::ThingId(THING_TEST_SCHEMA_ID),

        links {
            predicate PRED_1 min 0 max 1;
            predicate PRED_2 min 1 max many;
        }
    }
}

#[test]
fn test_macro_expansion_and_seeding() {
    let seeds = seed_test_kind_kind();
    assert_eq!(seeds.len(), 2);

    let kind_thing = &seeds[0];
    let schema_thing = &seeds[1];

    // Check Kind Thing
    assert_eq!(kind_thing.id.0, THING_TEST_KIND_ID);
    assert_eq!(kind_thing.kind, models::builtins::ids::THING_KIND_KIND);

    let kind_body: models::KindBody = kind_thing.body.decode().expect("decode kind body");
    assert_eq!(kind_body.name, SYM_TEST);
    assert_eq!(kind_body.version, 1);
    assert_eq!(kind_body.schema.0, THING_TEST_SCHEMA_ID);

    // Check Schema Thing
    assert_eq!(schema_thing.id.0, THING_TEST_SCHEMA_ID);
    assert_eq!(schema_thing.kind, models::builtins::ids::THING_SCHEMA_KIND);

    let schema_body: models::schema::SchemaBody =
        schema_thing.body.decode().expect("decode schema body");

    // Check TypeTag
    let expected_tag = type_tag("thingos.TestBody.v1");
    // models::declare::type_tag uses FNV1a
    assert_eq!(schema_body.body_type, expected_tag.0);

    // Check Link Rules
    assert_eq!(schema_body.link_rules.len(), 2);

    let rule1 = &schema_body.link_rules[0];
    assert_eq!(rule1.predicate_kind.0, 123);
    assert_eq!(rule1.min, 0);
    assert_eq!(rule1.max, Some(1));

    let rule2 = &schema_body.link_rules[1];
    assert_eq!(rule2.predicate_kind.0, 456);
    assert_eq!(rule2.min, 1);
    assert_eq!(rule2.max, None); // many
}
