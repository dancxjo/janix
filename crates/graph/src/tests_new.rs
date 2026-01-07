#[cfg(test)]
use crate::store::GraphStore;
#[cfg(test)]
use abi::ids::sym;
#[cfg(test)]
use alloc::vec;

#[test]
fn test_relationships_by_kind() {
    let mut store = GraphStore::new();
    let from = store.create_thing(sym("from")).unwrap();
    let to1 = store.create_thing(sym("to1")).unwrap();
    let to2 = store.create_thing(sym("to2")).unwrap();
    let to3 = store.create_thing(sym("to3")).unwrap();

    let kind_a = sym("rel.a");
    let kind_b = sym("rel.b");

    let _ = store.create_relationship(kind_a, from, to1).unwrap();
    let _ = store.create_relationship(kind_b, from, to2).unwrap();
    let _ = store.create_relationship(kind_a, from, to3).unwrap();

    // Verify filter
    let targets_a = store.relationships_by_kind(from, kind_a);
    assert_eq!(targets_a, vec![to1, to3]);

    let targets_b = store.relationships_by_kind(from, kind_b);
    assert_eq!(targets_b, vec![to2]);

    let targets_c = store.relationships_by_kind(from, sym("rel.c"));
    assert!(targets_c.is_empty());
}
