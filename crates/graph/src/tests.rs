#[cfg(test)]
use crate::store::GraphStore;
#[cfg(test)]
use abi::ids::sym;
#[cfg(test)]
use alloc::vec;

#[test]
fn test_relationships_from_order() {
    let mut store = GraphStore::new();
    let from = store.create_thing(sym("from")).unwrap();
    let to1 = store.create_thing(sym("to1")).unwrap();
    let to2 = store.create_thing(sym("to2")).unwrap();
    let to3 = store.create_thing(sym("to3")).unwrap();

    let r1 = store.create_relationship(sym("rel"), from, to1).unwrap();
    let r2 = store.create_relationship(sym("rel"), from, to2).unwrap();
    let r3 = store.create_relationship(sym("rel"), from, to3).unwrap();

    // Verify initial order
    let rels = store.relationships_from(from);
    assert_eq!(rels, vec![r1, r2, r3]);

    // Delete middle one
    store.delete_relationship(r2).unwrap();

    // Verify order is preserved
    let rels = store.relationships_from(from);
    assert_eq!(rels, vec![r1, r3]);

    // Add new one
    let r4 = store.create_relationship(sym("rel"), from, to2).unwrap();
    let rels = store.relationships_from(from);
    assert_eq!(rels, vec![r1, r3, r4]);
}

#[test]
fn test_relationship_cleanup() {
    let mut store = GraphStore::new();
    let from = store.create_thing(sym("from")).unwrap();
    let to = store.create_thing(sym("to")).unwrap();
    let r1 = store.create_relationship(sym("rel"), from, to).unwrap();

    store.delete_relationship(r1).unwrap();

    assert!(store.from_index.get(&from).is_none());
}
