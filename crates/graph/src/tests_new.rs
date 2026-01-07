#[cfg(test)]
use crate::store::GraphStore;
#[cfg(test)]
use abi::ids::sym;
#[cfg(test)]
use alloc::vec;
#[cfg(test)]
use alloc::vec::Vec;

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

#[test]
fn test_relationships_from_paged() {
    let mut store = GraphStore::new();
    let from = store.create_thing(sym("from")).unwrap();
    let mut targets = Vec::new();

    // Create 10 relationships
    for i in 0..10 {
        let to = store.create_thing(sym("to")).unwrap();
        let rel_id = store.create_relationship(sym("rel"), from, to).unwrap();
        targets.push(rel_id);
    }

    // 1. Fetch all
    let (rels, total) = store.relationships_from_paged(from, 0, 100);
    assert_eq!(total, 10);
    assert_eq!(rels.len(), 10);
    assert_eq!(rels[0].id, targets[0]);
    assert_eq!(rels[9].id, targets[9]);

    // 2. Skip 5, take 5
    let (rels, total) = store.relationships_from_paged(from, 5, 5);
    assert_eq!(total, 10);
    assert_eq!(rels.len(), 5);
    assert_eq!(rels[0].id, targets[5]);

    // 3. Skip 5, take 2
    let (rels, total) = store.relationships_from_paged(from, 5, 2);
    assert_eq!(total, 10);
    assert_eq!(rels.len(), 2);
    assert_eq!(rels[0].id, targets[5]);
    assert_eq!(rels[1].id, targets[6]);

    // 4. Skip beyond end
    let (rels, total) = store.relationships_from_paged(from, 20, 5);
    assert_eq!(total, 10);
    assert_eq!(rels.len(), 0);

    // 5. Take 0
    let (rels, total) = store.relationships_from_paged(from, 0, 0);
    assert_eq!(total, 10);
    assert_eq!(rels.len(), 0);
}
