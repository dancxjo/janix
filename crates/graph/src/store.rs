//! Graph store
//!
//! Canonical Thing/Relationship storage.
//! Enforces:
//! - Every Thing has a UUID and Kind.
//! - Relationships are typed Edges.
//! - O(1) traversal via in-memory indexing.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use spin::Mutex;
use alloc::collections::VecDeque;

use abi::ids::{RelationshipId, SymbolId, ThingId};

/// Thing header - metadata for each Thing (Internal)
#[derive(Clone, Debug)]
pub struct ThingHeader {
    pub kind: SymbolId,
    /// Creation timestamp (ticks since boot)
    pub created_at: u64,
}

/// A complete Thing with header and payload
#[derive(Clone, Debug)]
pub struct Thing {
    pub header: ThingHeader,
    payload: Vec<u8>,
}

impl Thing {
    pub fn new(kind: SymbolId) -> Self {
        Self {
            header: ThingHeader {
                kind,
                created_at: 0, // TODO: hook up time
            },
            payload: Vec::new(),
        }
    }
}

/// A relationship edge
#[derive(Clone, Debug)]
pub struct Relationship {
    pub kind: SymbolId,
    pub from: ThingId,
    pub to: ThingId,
}

/// Global place store
static PLACE_STORE: Mutex<Option<PlaceStore>> = Mutex::new(None);

pub struct PlaceStore {
    /// Thing table: ThingId -> Thing
    things: BTreeMap<ThingId, Thing>,
    /// Relationship table: RelationshipId -> Relationship
    relationships: BTreeMap<RelationshipId, Relationship>,
    
    /// Outbound index: from -> [rel_id]
    from_index: BTreeMap<ThingId, Vec<RelationshipId>>,
    /// Inbound index: to -> [rel_id]
    to_index: BTreeMap<ThingId, Vec<RelationshipId>>,
    
    /// Name index: name (symbol ID) -> ThingId
    name_index: BTreeMap<SymbolId, ThingId>,

    /// Watchers using simple subscription (for now)
    watchers: BTreeMap<ThingId, Vec<ThingId>>,
    /// Pending events: watcher_id -> queue of event_ids
    pending_events: BTreeMap<ThingId, VecDeque<ThingId>>,
    
    /// Counter for generating unique IDs (initialized from randomness or monotonic)
    next_id_high: u64,
    next_id_low: u64,
}

impl PlaceStore {
    fn new() -> Self {
        Self {
            things: BTreeMap::new(),
            relationships: BTreeMap::new(),
            from_index: BTreeMap::new(),
            to_index: BTreeMap::new(),
            name_index: BTreeMap::new(),
            watchers: BTreeMap::new(),
            pending_events: BTreeMap::new(),
            next_id_high: 1, // Simple counter for now, should be UUID
            next_id_low: 1,
        }
    }

    fn generate_id(&mut self) -> ThingId {
        // TODO: stronger UUID generation
        let id = ThingId::from_parts(self.next_id_high, self.next_id_low);
        self.next_id_low += 1;
        if self.next_id_low == 0 {
            self.next_id_high += 1;
        }
        id
    }

    pub fn create_thing(&mut self, kind: SymbolId) -> Result<ThingId, &'static str> {
        if kind == SymbolId::INVALID {
            return Err("invalid kind");
        }
        
        let id = self.generate_id();
        let thing = Thing::new(kind);
        self.things.insert(id, thing);
        Ok(id)
    }

    #[allow(dead_code)]
    fn get_thing(&self, id: ThingId) -> Option<&Thing> {
        self.things.get(&id)
    }

    pub fn create_relationship(&mut self, kind: SymbolId, from: ThingId, to: ThingId) -> Result<RelationshipId, &'static str> {
        if !self.things.contains_key(&from) {
            return Err("source thing not found");
        }
        if !self.things.contains_key(&to) {
            return Err("target thing not found");
        }

        let id = self.generate_id();
        let rel = Relationship { kind, from, to };
        
        self.relationships.insert(id, rel);
        self.from_index.entry(from).or_default().push(id);
        self.to_index.entry(to).or_default().push(id);
        
        Ok(id)
    }

    pub fn relationships_from(&self, from: ThingId) -> &[RelationshipId] {
        self.from_index.get(&from).map(|v| v.as_slice()).unwrap_or(&[])
    }

    pub fn set_payload(&mut self, id: ThingId, payload: &[u8]) -> bool {
        if let Some(thing) = self.things.get_mut(&id) {
            thing.payload = payload.to_vec();
            true
        } else {
            false
        }
    }

    fn get_payload(&self, id: ThingId) -> Option<&[u8]> {
        self.things.get(&id).map(|t| t.payload.as_slice())
    }

    pub fn register_name(&mut self, id: ThingId, name: SymbolId) {
        self.name_index.insert(name, id);
    }

    pub fn find_by_name(&self, name: SymbolId) -> Option<ThingId> {
        self.name_index.get(&name).cloned()
    }

    pub fn watch(&mut self, watcher: ThingId, target: ThingId) {
        self.watchers.entry(target).or_default().push(watcher);
    }

    #[allow(dead_code)]
    fn enqueue(&mut self, watcher: ThingId, event: ThingId) {
        self.pending_events.entry(watcher).or_default().push_back(event);
    }
    
    fn dequeue(&mut self, watcher: ThingId) -> Option<ThingId> {
        self.pending_events.get_mut(&watcher).and_then(|q| q.pop_front())
    }
}

/// Initialize the place store
pub fn init() {
    let store = PlaceStore::new();
    *PLACE_STORE.lock() = Some(store);
}

pub fn is_initialized() -> bool {
    PLACE_STORE.lock().is_some()
}

// --- Public API ---

pub fn thing_create(kind: SymbolId) -> ThingId {
    let mut guard = PLACE_STORE.lock();
    guard.as_mut()
        .expect("PlaceStore not initialized")
        .create_thing(kind)
        .expect("Failed to create thing")
}

pub fn thing_exists(id: ThingId) -> bool {
    let guard = PLACE_STORE.lock();
    guard.as_ref()
        .expect("PlaceStore not initialized")
        .things.contains_key(&id)
}

pub fn relationship_create(kind: SymbolId, from: ThingId, to: ThingId) -> RelationshipId {
    let mut guard = PLACE_STORE.lock();
    guard.as_mut()
        .expect("PlaceStore not initialized")
        .create_relationship(kind, from, to)
        .expect("Failed to create relationship")
}

pub fn relationships_from(from: ThingId) -> Vec<RelationshipId> {
    let guard = PLACE_STORE.lock();
    guard.as_ref()
        .expect("PlaceStore not initialized")
        .relationships_from(from)
        .to_vec()
}

pub fn get_relationship(id: RelationshipId) -> Option<Relationship> {
    let guard = PLACE_STORE.lock();
    guard.as_ref()
        .expect("PlaceStore not initialized")
        .relationships.get(&id).cloned()
}

pub fn thing_set_inline_payload(id: ThingId, payload: &[u8]) -> bool {
    let mut guard = PLACE_STORE.lock();
    guard.as_mut()
        .expect("PlaceStore not initialized")
        .set_payload(id, payload)
}

pub fn get_payload(id: ThingId) -> Option<Vec<u8>> {
    let guard = PLACE_STORE.lock();
    guard.as_ref()
        .expect("PlaceStore not initialized")
        .get_payload(id)
        .map(|s| s.to_vec())
}

pub fn thing_register_name(id: ThingId, name: SymbolId) {
    let mut guard = PLACE_STORE.lock();
    guard.as_mut()
        .expect("PlaceStore not initialized")
        .register_name(id, name)
}

pub fn find_thing_by_name(name: SymbolId) -> Option<ThingId> {
    let guard = PLACE_STORE.lock();
    guard.as_ref()
        .expect("PlaceStore not initialized")
        .find_by_name(name)
}

pub fn watch(watcher: ThingId, target: ThingId) {
    let mut guard = PLACE_STORE.lock();
    guard.as_mut()
        .expect("PlaceStore not initialized")
        .watch(watcher, target)
}

pub fn dequeue_event(watcher: ThingId) -> Option<ThingId> {
    let mut guard = PLACE_STORE.lock();
    guard.as_mut()
        .expect("PlaceStore not initialized")
        .dequeue(watcher)
}

pub fn with_store<F, R>(f: F) -> R
where
    F: FnOnce(&mut PlaceStore) -> R,
{
    let mut guard = PLACE_STORE.lock();
    let store = guard.as_mut().expect("PlaceStore not initialized");
    f(store)
}
