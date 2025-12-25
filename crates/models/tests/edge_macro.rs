use models::builtins::ids::*;
use models::builtins::symbols::*;
use models::link_thing;
use models::edge;
use models::ThingBody;
use models::LinkBody;
use models::abi::ThingId;

#[test]
fn test_edge_macro() {
    let from = ThingId(100);
    let to = ThingId(200);
    let pred = THING_OWNS_KIND;
    
    // Test link_thing! macro
    let edge_thing = link_thing!(
        id: ThingId(500),
        from: from,
        to: to,
        pred: pred,
    );

    // Assert minimal properties
    assert_eq!(edge_thing.id, ThingId(500));
    assert_eq!(edge_thing.kind, THING_LINK_KIND);

    // Decode body
    let decoded: LinkBody = edge_thing.body.decode().expect("Decode LinkBody");
    assert_eq!(decoded.from, from);
    assert_eq!(decoded.to, to);
    assert_eq!(decoded.predicate, pred);
}

#[test]
fn test_predicate_kind_macro() {
    // This tests that predicate_kind! expands correctly and compiles
    use models::builtins::predicates::*;
    
    // Check if seed functions exist
    let _ = seed_owns_kind();
    let _ = seed_has_cap_kind();
    
    // Check generated constants
    assert_eq!(THING_OWNS_KIND, models::builtins::ids::THING_OWNS_KIND);
}
