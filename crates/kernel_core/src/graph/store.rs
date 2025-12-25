use abi::ThingId;
use alloc::collections::BTreeMap;
use thing_models::Thing;

pub struct GraphStore {
    things: BTreeMap<ThingId, Thing>,
}

impl GraphStore {
    pub fn new() -> Self {
        Self {
            things: BTreeMap::new(),
        }
    }

    /// Insert a Thing, bypassing schema enforcement.
    /// Only used during bootstrap/seeding.
    pub fn insert_seed(&mut self, thing: Thing) {
        self.things.insert(thing.id, thing);
    }

    pub fn get(&self, id: ThingId) -> Option<&Thing> {
        self.things.get(&id)
    }

    pub fn list(&self) -> impl Iterator<Item = &Thing> {
        self.things.values()
    }
}
