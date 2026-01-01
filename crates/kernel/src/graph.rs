//! Place store
//!
//! Minimal in-kernel Thing/Relationship storage for the v0.3 core.
//! This provides the foundational data structure for all system state.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use spin::Mutex;

use crate::symbols;
use abi::ids::{RelationshipId, SymbolId, ThingId};

/// Thing header - metadata for each Thing
///
/// Layout matches v0.3 ABI direction (compact representation)
#[derive(Clone, Debug)]
pub struct ThingHeader {
    /// The kind of this Thing (symbol ID)
    pub kind: SymbolId,
    /// Schema symbol (if applicable)
    pub schema: SymbolId,
    /// Schema version
    pub version: u32,
    /// Creation timestamp (ticks since boot)
    pub created_at: u64,
}

/// A complete Thing with header and payload
#[derive(Clone, Debug)]
struct Thing {
    header: ThingHeader,
    payload: Vec<u8>,
}

/// Global place store
static PLACE_STORE: Mutex<Option<PlaceStore>> = Mutex::new(None);

struct PlaceStore {
    /// Thing table: ThingId -> Thing
    things: BTreeMap<ThingId, Thing>,
    /// Outbound relationship index: from -> [rel_id]
    out_index: BTreeMap<ThingId, Vec<RelationshipId>>,
    /// Inbound relationship index: to -> [rel_id]
    in_index: BTreeMap<ThingId, Vec<RelationshipId>>,
    /// Name index: name (symbol ID) -> ThingId
    name_index: BTreeMap<SymbolId, ThingId>,
    /// Watchers: target -> [watcher_id]
    watchers: BTreeMap<ThingId, Vec<ThingId>>,
    /// Pending events: watcher_id -> queue of event_ids
    pending_events: BTreeMap<ThingId, VecDeque<ThingId>>,
    /// Counter for generating unique IDs
    next_id: u128,
    /// Tick counter for timestamps
    tick: u64,
}

impl PlaceStore {
    fn new() -> Self {
        Self {
            things: BTreeMap::new(),
            out_index: BTreeMap::new(),
            in_index: BTreeMap::new(),
            name_index: BTreeMap::new(),
            watchers: BTreeMap::new(),
            pending_events: BTreeMap::new(),
            next_id: 1,
            tick: 0,
        }
    }

    fn generate_id(&mut self) -> ThingId {
        let id = ThingId(self.next_id);
        self.next_id += 1;
        id
    }

    fn create_thing_internal(&mut self, kind: SymbolId, schema: SymbolId, version: u32) -> ThingId {
        let id = self.generate_id();
        let header = ThingHeader {
            kind,
            schema,
            version,
            created_at: self.tick,
        };
        self.things.insert(
            id,
            Thing {
                header,
                payload: Vec::new(),
            },
        );
        id
    }

    // Public wrapper removed from impl, handles in module `thing_create`
    
    fn set_payload(&mut self, id: ThingId, payload: &[u8]) -> bool {
        if let Some(thing) = self.things.get_mut(&id) {
            thing.payload = payload.to_vec();
            true
        } else {
            false
        }
    }

    fn get_header(&self, id: ThingId) -> Option<&ThingHeader> {
        self.things.get(&id).map(|t| &t.header)
    }

    fn get_payload(&self, id: ThingId) -> Option<&[u8]> {
        self.things.get(&id).map(|t| t.payload.as_slice())
    }

    fn create_relationship_internal(&mut self, from: ThingId, to: ThingId, predicate: SymbolId) -> RelationshipId {
        let kind_rel = symbols::well_known(b"kind.Relationship");
        let rel_id = self.create_thing_internal(kind_rel, SymbolId::INVALID, 1);
        
        let mut payload = Vec::with_capacity(40);
        payload.extend_from_slice(&from.0.to_le_bytes());
        payload.extend_from_slice(&to.0.to_le_bytes());
        payload.extend_from_slice(&predicate.0.to_le_bytes());
        self.set_payload(rel_id, &payload);

        self.out_index.entry(from).or_insert_with(Vec::new).push(rel_id);
        self.in_index.entry(to).or_insert_with(Vec::new).push(rel_id);

        rel_id
    }

    fn thing_count(&self) -> usize {
        self.things.len()
    }

    fn rel_count(&self) -> usize {
        self.out_index.values().map(|v| v.len()).sum()
    }

    fn rebuild_indexes(&mut self) {
        self.out_index.clear();
        self.in_index.clear();

        let kind_rel = symbols::well_known(b"kind.Relationship");
        
        for (&id, thing) in &self.things {
            if thing.header.kind == kind_rel {
                if thing.payload.len() >= 40 {
                    let from_bytes = &thing.payload[0..16];
                    let to_bytes = &thing.payload[16..32];
                    
                    let from = ThingId(u128::from_le_bytes(from_bytes.try_into().unwrap()));
                    let to = ThingId(u128::from_le_bytes(to_bytes.try_into().unwrap()));
                    
                    self.out_index.entry(from).or_insert_with(Vec::new).push(id);
                    self.in_index.entry(to).or_insert_with(Vec::new).push(id);
                }
            }
        }
    }

    fn subscribe(&mut self, watcher: ThingId, target: ThingId) {
        self.watchers.entry(target).or_insert_with(Vec::new).push(watcher);
    }

    fn enqueue(&mut self, watcher: ThingId, event: ThingId) {
        self.pending_events.entry(watcher).or_insert_with(VecDeque::new).push_back(event);
    }
}

/// Initialize the place store
pub fn init() {
    *PLACE_STORE.lock() = Some(PlaceStore::new());
}

use alloc::collections::VecDeque;

/// Create a new Thing with the given kind, schema, and version
pub fn thing_create(kind: SymbolId, schema: SymbolId, version: u32) -> ThingId {
    let id = {
        let mut guard = PLACE_STORE.lock();
        match guard.as_mut() {
            Some(store) => store.create_thing_internal(kind, schema, version),
            None => return ThingId(0),
        }
    };
    emit_event_mutation(id, None);
    id
}

/// Set the inline payload for a Thing
pub fn thing_set_inline_payload(id: ThingId, payload: &[u8]) -> bool {
    let result = {
        let mut guard = PLACE_STORE.lock();
        match guard.as_mut() {
            Some(store) => store.set_payload(id, payload),
            None => false,
        }
    };
    if result {
        // Payload change is a mutation? Not in simple spec, but let's emit for target
        emit_event_mutation(id, None);
    }
    result
}

/// Get the header of a Thing
pub fn get_header(id: ThingId) -> Option<ThingHeader> {
    let guard = PLACE_STORE.lock();
    guard
        .as_ref()
        .and_then(|store| store.get_header(id).cloned())
}

/// Get the payload of a Thing
pub fn get_payload(id: ThingId) -> Option<Vec<u8>> {
    let guard = PLACE_STORE.lock();
    guard
        .as_ref()
        .and_then(|store| store.get_payload(id).map(Vec::from))
}

/// Create a relationship between two Things
pub fn relationship_create(from: ThingId, to: ThingId, predicate: SymbolId) -> RelationshipId {
    let id = {
        let mut guard = PLACE_STORE.lock();
        match guard.as_mut() {
            Some(store) => store.create_relationship_internal(from, to, predicate),
            None => return ThingId(0),
        }
    };
    emit_event_mutation(from, Some(id));
    // Also emit for 'to'? Spec says "On mutation touching watched target". 
    // Creating a relationship touches both 'from' and 'to'.
    emit_event_mutation(to, Some(id));
    id
}

// Internal helper for event emission
fn emit_event_mutation(target: ThingId, cause: Option<ThingId>) {
    let mut guard = PLACE_STORE.lock();
    let store = match guard.as_mut() {
        Some(s) => s,
        None => return,
    };

    // Check if anyone watching this target
    let watchers = match store.watchers.get(&target) {
        Some(w) => w.clone(),
        None => return,
    };

    if watchers.is_empty() {
        return;
    }

    // Create Event Thing (Quietly!)
    let kind_event = symbols::well_known(b"kind.Event");
    let event_id = store.create_thing_internal(kind_event, SymbolId::INVALID, 1);
    
    // Create Relationship event --targets--> target
    let pred_targets = symbols::well_known(b"predicate.targets");
    store.create_relationship_internal(event_id, target, pred_targets);

    // If cause is provided, link it too
    if let Some(cause_id) = cause {
        store.create_relationship_internal(event_id, cause_id, pred_targets);
    }

    // Enqueue
    for watcher in watchers {
        store.enqueue(watcher, event_id);
    }
}

/// Subscribe a watcher to a target
pub fn watch(watcher: ThingId, target: ThingId) {
    let mut guard = PLACE_STORE.lock();
    if let Some(store) = guard.as_mut() {
        store.subscribe(watcher, target);
        // spec says "thing.bloom --watches--> place.desktop"
        // we should create that relationship too, physically
        // but for now the syscall `sys_watch` calls this internal logic
    }
}

/// Pop next event for a watcher
pub fn dequeue_event(watcher: ThingId) -> Option<ThingId> {
    let mut guard = PLACE_STORE.lock();
    guard.as_mut().and_then(|store| {
        store.pending_events.get_mut(&watcher).and_then(|q| q.pop_front())
    })
}

/// Get relationships from a Thing
pub fn relationships_from(from: ThingId) -> Vec<RelationshipId> {
    let guard = PLACE_STORE.lock();
    guard.as_ref().map(|store| {
        store.out_index.get(&from).cloned().unwrap_or_default()
    }).unwrap_or_default()
}

/// Get relationships to a Thing
pub fn relationships_to(to: ThingId) -> Vec<RelationshipId> {
    let guard = PLACE_STORE.lock();
    guard.as_ref().map(|store| {
        store.in_index.get(&to).cloned().unwrap_or_default()
    }).unwrap_or_default()
}

/// Get statistics about the store
pub fn stats() -> (usize, usize) {
    let guard = PLACE_STORE.lock();
    match guard.as_ref() {
        Some(store) => (store.thing_count(), store.rel_count()),
        None => (0, 0),
    }
}

/// Rebuild all indexes from Relationship Things in the store
pub fn rebuild_indexes() {
    let mut guard = PLACE_STORE.lock();
    if let Some(store) = guard.as_mut() {
        store.rebuild_indexes();
    }
}

/// Register a name for a Thing
pub fn thing_register_name(id: ThingId, name: SymbolId) {
    let mut guard = PLACE_STORE.lock();
    if let Some(store) = guard.as_mut() {
        store.name_index.insert(name, id);
    }
}

/// Find a Thing ID by its registered name
pub fn find_thing_by_name(name: SymbolId) -> Option<ThingId> {
    let guard = PLACE_STORE.lock();
    guard.as_ref().and_then(|store| store.name_index.get(&name).cloned())
}


/// Advance the place store tick (for timestamps)
pub fn tick() {
    let mut guard = PLACE_STORE.lock();
    if let Some(store) = guard.as_mut() {
        store.tick += 1;
    }
}

