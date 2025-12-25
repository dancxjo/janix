use models::builtins;
use models::builtins::ids::*;
use models::*;
use models::core::process::*;
use models::core::vgs::*;
use models::schema::{SchemaBody, LinkRule};
use models::declare::type_tag::TypeTag;

#[test]
fn test_core_ontology_seed() {
    let things = builtins::builtin_seed_things();
    
    // Helper to find a thing by ID
    let find = |id: models::abi::ThingId| things.iter().find(|t| t.id == id).cloned();

    // 1. Verify all new Kind IDs exist
    let core_kinds = vec![
        THING_TIME_NOW_KIND,
        THING_PROCESS_KIND,
        THING_THREAD_KIND,
        THING_CAPABILITY_KIND,
        THING_GRAPH_KIND,
        THING_MOUNT_KIND,
        THING_GRAPH_PROVIDER_KIND,
        THING_BUFFER_KIND,
        THING_STREAM_KIND,
        THING_OWNS_KIND,
        THING_HAS_CAP_KIND,
        THING_BACKED_BY_KIND,
        THING_MOUNTS_KIND,
        THING_HAS_SCHEMA_KIND,
    ];

    for id in core_kinds {
        assert!(find(id).is_some(), "Missing Kind ThingId: {:?}", id);
    }

    // 2. Decode and validate Kind and Schema for Process
    let process_kind_thing = find(THING_PROCESS_KIND).expect("Process Kind");
    let process_kind_body: KindBody = process_kind_thing.body.decode().expect("Decode Process KindBody");
    assert_eq!(process_kind_body.schema, THING_PROCESS_SCHEMA);

    let process_schema_thing = find(THING_PROCESS_SCHEMA).expect("Process Schema");
    let process_schema_body: SchemaBody = process_schema_thing.body.decode().expect("Decode Process SchemaBody");
    
    // Verify type tag
    // ProcessBody -> "thingos.ProcessBody.v1"
    let expected_tag = models::declare::type_tag("thingos.ProcessBody.v1");
    assert_eq!(process_schema_body.body_type, expected_tag.0);

    // Verify link rules: OWNS, HAS_CAP
    let has_owns = process_schema_body.link_rules.iter().any(|r| r.predicate_kind == THING_OWNS_KIND);
    let has_cap = process_schema_body.link_rules.iter().any(|r| r.predicate_kind == THING_HAS_CAP_KIND);
    assert!(has_owns, "Process must allow OWNS");
    assert!(has_cap, "Process must allow HAS_CAP");

    // 3. Verify Mount Schema
    let mount_schema_thing = find(THING_MOUNT_SCHEMA).expect("Mount Schema");
    let mount_schema_body: SchemaBody = mount_schema_thing.body.decode().expect("Decode Mount SchemaBody");
    
    // Verify MOUNTS min=1 max=1
    let mounts_rule = mount_schema_body.link_rules.iter().find(|r| r.predicate_kind == THING_MOUNTS_KIND)
        .expect("Mount must have MOUNTS rule");
    assert_eq!(mounts_rule.min, 1);
    assert_eq!(mounts_rule.max, Some(1));

    // 4. Verify Kind Meta-Schema (manual update test)
    let kind_schema_thing = find(THING_KIND_SCHEMA).expect("Kind Schema");
    let kind_schema_body: SchemaBody = kind_schema_thing.body.decode().expect("Decode Kind SchemaBody");
    let has_schema_rule = kind_schema_body.link_rules.iter().find(|r| r.predicate_kind == THING_HAS_SCHEMA_KIND)
        .expect("Kind must have HAS_SCHEMA rule");
    assert_eq!(has_schema_rule.min, 1);
    assert_eq!(has_schema_rule.max, Some(1));
}
