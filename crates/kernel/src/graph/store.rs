use abi::ThingId;
use alloc::collections::{BTreeMap, BTreeSet};
use thing_models::Thing;

#[derive(Debug)]
pub enum GraphError {
    DuplicateId,
}

pub struct GraphStore {
    things: BTreeMap<ThingId, Thing>,
    /// Index of Things by Kind.
    /// Changed from Vec<ThingId> to BTreeSet<ThingId> (Bolt Optimization):
    /// - Improves deletion complexity from O(N) to O(log N).
    /// - Maintains sorted order for efficient pagination via range queries.
    /// - Critical for performance as number of things (e.g., LogEntry) grows.
    kind_index: BTreeMap<ThingId, BTreeSet<ThingId>>,
    /// Index of Links by (from, predicate).
    /// Outer key: from
    /// Inner key: predicate
    /// Value: Set of Link ThingIds
    link_index: BTreeMap<ThingId, BTreeMap<ThingId, BTreeSet<ThingId>>>,
    next_id: u64,
}

impl GraphStore {
    pub fn new() -> Self {
        Self {
            things: BTreeMap::new(),
            kind_index: BTreeMap::new(),
            link_index: BTreeMap::new(),
            next_id: 0x10000000,
        }
    }

    /// Insert a Thing, bypassing schema enforcement.
    /// Only used during bootstrap/seeding.
    pub fn insert_seed(&mut self, thing: Thing) {
        // We panic on seed collision as it implies a bootstrap error
        if let Err(e) = self.insert_thing(thing) {
            panic!("Seed insertion failed: {:?}", e);
        }
    }

    pub fn insert_thing(&mut self, thing: Thing) -> Result<(), GraphError> {
        if self.things.contains_key(&thing.id) {
            return Err(GraphError::DuplicateId);
        }

        let id = thing.id;
        let kind = thing.kind;

        // If it's a link, add to link_index
        if kind == thing_models::builtins::ids::THING_LINK_KIND {
            if let Ok(tb) = thing.body.decode::<abi::wire::typed::TypedBytes>() {
                if let Ok(link) = postcard::from_bytes::<thing_models::link::LinkBody>(&tb.bytes) {
                    self.link_index
                        .entry(link.from)
                        .or_default()
                        .entry(link.predicate)
                        .or_default()
                        .insert(id);
                }
            }
        }

        self.things.insert(id, thing);
        // BTreeSet handles sorting automatically on insertion.
        self.kind_index.entry(kind).or_default().insert(id);

        Ok(())
    }

    pub fn create_thing(&mut self, kind: ThingId, body: thing_models::value::ThingBody) -> ThingId {
        let id = ThingId(self.next_id);
        self.next_id += 1;

        // TODO: Reuse logic but avoid clone?
        let thing = Thing { id, kind, body };

        match self.insert_thing(thing) {
            Ok(_) => id,
            Err(_) => {
                // If collision (very unlikely with counter), retry once?
                // panic for v0.2
                panic!("GraphStore ID collision on create");
            }
        }
    }

    pub fn update_thing(
        &mut self,
        id: ThingId,
        body: thing_models::value::ThingBody,
    ) -> Result<(), ()> {
        if let Some(thing) = self.things.get_mut(&id) {
            // If it's a link, we need to update the index.
            // First remove the old entry from index.
            if thing.kind == thing_models::builtins::ids::THING_LINK_KIND {
                if let Ok(tb) = thing.body.decode::<abi::wire::typed::TypedBytes>() {
                    if let Ok(link) = postcard::from_bytes::<thing_models::link::LinkBody>(&tb.bytes) {
                        if let Some(preds) = self.link_index.get_mut(&link.from) {
                            if let Some(set) = preds.get_mut(&link.predicate) {
                                set.remove(&id);
                                if set.is_empty() {
                                    preds.remove(&link.predicate);
                                }
                            }
                            if preds.is_empty() {
                                self.link_index.remove(&link.from);
                            }
                        }
                    }
                }
            }

            thing.body = body;

            // Now re-insert into index with new body
            if thing.kind == thing_models::builtins::ids::THING_LINK_KIND {
                 if let Ok(tb) = thing.body.decode::<abi::wire::typed::TypedBytes>() {
                    if let Ok(link) = postcard::from_bytes::<thing_models::link::LinkBody>(&tb.bytes) {
                        self.link_index
                            .entry(link.from)
                            .or_default()
                            .entry(link.predicate)
                            .or_default()
                            .insert(id);
                    }
                }
            }
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn delete_thing(&mut self, id: ThingId) -> Result<(), ()> {
        if let Some(thing) = self.things.remove(&id) {
            // Remove from kind_index
            if let Some(set) = self.kind_index.get_mut(&thing.kind) {
                set.remove(&id);
            }

            // Remove from link_index if applicable
            if thing.kind == thing_models::builtins::ids::THING_LINK_KIND {
                if let Ok(tb) = thing.body.decode::<abi::wire::typed::TypedBytes>() {
                    if let Ok(link) = postcard::from_bytes::<thing_models::link::LinkBody>(&tb.bytes) {
                        if let Some(preds) = self.link_index.get_mut(&link.from) {
                            if let Some(set) = preds.get_mut(&link.predicate) {
                                set.remove(&id);
                                if set.is_empty() {
                                    preds.remove(&link.predicate);
                                }
                            }
                            if preds.is_empty() {
                                self.link_index.remove(&link.from);
                            }
                        }
                    }
                }
            }

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get(&self, id: ThingId) -> Option<&Thing> {
        self.things.get(&id)
    }

    pub fn list(&self) -> impl Iterator<Item = &Thing> {
        self.things.values()
    }

    pub fn iter_kind(&self, kind: ThingId) -> impl Iterator<Item = &Thing> {
        self.kind_index
            .get(&kind)
            .into_iter()
            .flat_map(|ids| ids.iter())
            .filter_map(|id| self.things.get(id))
    }

    pub fn iter_links_from(&self, from: ThingId) -> impl Iterator<Item = &Thing> {
        self.link_index
            .get(&from)
            .into_iter()
            .flat_map(|preds| preds.values())
            .flat_map(|ids| ids.iter())
            .filter_map(|id| self.things.get(id))
    }

    pub fn iter_links_from_kind(&self, from: ThingId, predicate: ThingId) -> impl Iterator<Item = &Thing> {
         self.link_index
            .get(&from)
            .and_then(|preds| preds.get(&predicate))
            .into_iter()
            .flat_map(|ids| ids.iter())
            .filter_map(|id| self.things.get(id))
    }

    pub fn next_thing_of_kind(&self, kind: ThingId, start_after: ThingId) -> Option<ThingId> {
        if let Some(set) = self.kind_index.get(&kind) {
            use core::ops::Bound::{Excluded, Unbounded};
            // Efficiently find the first item strictly greater than start_after.
            set.range((Excluded(start_after), Unbounded))
                .next()
                .copied()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use abi::ThingId;
    use thing_models::builtins::ids::{THING_BOOT_ROOT, THING_LINK_KIND};
    use thing_models::link::LinkBody;
    use abi::wire::typed::TypedBytes;
    use alloc::vec;
    use alloc::vec::Vec;

    #[test]
    fn test_link_index() {
        let mut store = GraphStore::new();

        // Create nodes
        let node1 = ThingId(100);
        let node2 = ThingId(101);
        let node3 = ThingId(102);

        // Helper to create link body
        let create_link = |from, to, predicate| {
            let body = LinkBody { from, to, predicate };
            let bytes = postcard::to_allocvec(&body).unwrap();
            let typed = TypedBytes {
                type_id: abi::wire::typed::TypeId(THING_LINK_KIND.0 as u128),
                codec_id: abi::wire::typed::CodecId::POSTCARD,
                bytes,
            };
            thing_models::value::ThingBody::from(&typed).unwrap()
        };

        // Create links
        let pred1 = ThingId(999);
        let pred2 = ThingId(888);

        // Link 1: node1 -> node2 (pred1)
        let l1 = store.create_thing(THING_LINK_KIND, create_link(node1, node2, pred1));

        // Link 2: node1 -> node3 (pred1)
        let l2 = store.create_thing(THING_LINK_KIND, create_link(node1, node3, pred1));

        // Link 3: node2 -> node3 (pred1)
        let l3 = store.create_thing(THING_LINK_KIND, create_link(node2, node3, pred1));

        // Link 4: node1 -> node2 (pred2)
        let l4 = store.create_thing(THING_LINK_KIND, create_link(node1, node2, pred2));

        // Verify iter_links_from
        let links_from_node1: Vec<ThingId> = store.iter_links_from(node1).map(|t| t.id).collect();
        assert_eq!(links_from_node1.len(), 3); // l1, l2, l4
        assert!(links_from_node1.contains(&l1));
        assert!(links_from_node1.contains(&l2));
        assert!(links_from_node1.contains(&l4));

        let links_from_node2: Vec<ThingId> = store.iter_links_from(node2).map(|t| t.id).collect();
        assert_eq!(links_from_node2.len(), 1);
        assert!(links_from_node2.contains(&l3));

        // Verify iter_links_from_kind
        let links_from_node1_pred1: Vec<ThingId> = store.iter_links_from_kind(node1, pred1).map(|t| t.id).collect();
        assert_eq!(links_from_node1_pred1.len(), 2); // l1, l2
        assert!(links_from_node1_pred1.contains(&l1));
        assert!(links_from_node1_pred1.contains(&l2));

        let links_from_node1_pred2: Vec<ThingId> = store.iter_links_from_kind(node1, pred2).map(|t| t.id).collect();
        assert_eq!(links_from_node1_pred2.len(), 1); // l4
        assert!(links_from_node1_pred2.contains(&l4));

        // Verify deletion
        store.delete_thing(l1).unwrap();
        let links_from_node1_after: Vec<ThingId> = store.iter_links_from(node1).map(|t| t.id).collect();
        assert_eq!(links_from_node1_after.len(), 2); // l2, l4
        assert!(!links_from_node1_after.contains(&l1));

        // Verify update (change 'to' target)
        // Note: GraphStore update logic currently doesn't re-index on update (wait, I implemented it!)
        let new_body = create_link(node1, node3, pred2); // Change l4 to point to node3
        store.update_thing(l4, new_body).unwrap();

        // Verify index is updated?
        // My implementation checks 'from' and 'predicate' for indexing. Changing 'to' doesn't change the index key.
        // But if I changed 'from' or 'predicate', it should move.

        let new_body_moved = create_link(node2, node1, pred2); // Move l4 from node1 to node2
        store.update_thing(l4, new_body_moved).unwrap();

        let links_from_node1_moved: Vec<ThingId> = store.iter_links_from(node1).map(|t| t.id).collect();
        assert_eq!(links_from_node1_moved.len(), 1); // Only l2 remaining

        let links_from_node2_moved: Vec<ThingId> = store.iter_links_from(node2).map(|t| t.id).collect();
        assert_eq!(links_from_node2_moved.len(), 2); // l3 and l4
        assert!(links_from_node2_moved.contains(&l4));
    }
}
