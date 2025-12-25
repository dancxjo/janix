use abi::ThingId;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use thing_models::Thing;

#[derive(Debug)]
pub enum GraphError {
    DuplicateId,
}

pub struct GraphStore {
    things: BTreeMap<ThingId, Thing>,
    kind_index: BTreeMap<ThingId, Vec<ThingId>>,
}

impl GraphStore {
    pub fn new() -> Self {
        Self {
            things: BTreeMap::new(),
            kind_index: BTreeMap::new(),
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
        self.kind_index.entry(kind).or_default().push(id);
        
        // Ensure index order (monotonic IDs)
        // If IDs are not inserted in order, we might need to sort.
        // Assuming insert_thing is called with largely monotonic IDs or we mitigate.
        // trunk assumed monotonic creation. Here we might have random inserts.
        // We should sort or rely on usage.
        // Let's sort to be safe for next_thing_of_kind binary_search.
        // But sorting on every insert is O(N log N) or O(N).
        // `trunk` pushed and assumed order because `create_thing` generated sequential IDs.
        // Here, we accept arbitrary IDs. So we must maintain order.
        let list = self.kind_index.get_mut(&kind).unwrap(); // we just pushed, so it exists
        // If the new ID is larger than the last (common case), we are good.
        if list.len() > 1 && id < list[list.len() - 2] {
             list.sort();
        }

        Ok(())
    }

    pub fn get(&self, id: ThingId) -> Option<&Thing> {
        self.things.get(&id)
    }

    pub fn list(&self) -> impl Iterator<Item = &Thing> {
        self.things.values()
    }

    pub fn next_thing_of_kind(&self, kind: ThingId, start_after: ThingId) -> Option<ThingId> {
        if let Some(list) = self.kind_index.get(&kind) {
            // list should be sorted by ThingId
            // find first element > start_after
            let idx = match list.binary_search(&start_after) {
                Ok(i) => i + 1,
                Err(i) => i,
            };
            if idx < list.len() {
                Some(list[idx])
            } else {
                None
            }
        } else {
            None
        }
    }
}
