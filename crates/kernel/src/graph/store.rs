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
    next_id: u64,
}

impl GraphStore {
    pub fn new() -> Self {
        Self {
            things: BTreeMap::new(),
            kind_index: BTreeMap::new(),
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
            thing.body = body;
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
