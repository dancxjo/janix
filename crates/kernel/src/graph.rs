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
            next_id: 1,
            tick: 0,
        }
    }

    fn generate_id(&mut self) -> ThingId {
        let id = ThingId(self.next_id);
        self.next_id += 1;
        id
    }

    fn create_thing(&mut self, kind: SymbolId, schema: SymbolId, version: u32) -> ThingId {
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

    fn create_relationship(&mut self, from: ThingId, to: ThingId, predicate: SymbolId) -> RelationshipId {
        let kind_rel = symbols::well_known(b"kind.Relationship");
        let rel_id = self.create_thing(kind_rel, SymbolId::INVALID, 1);
        
        // Encode RelationshipBody into payload
        // Minimal encoding: [from_u128, to_u128, pred_u64]
        let mut payload = Vec::with_capacity(40);
        payload.extend_from_slice(&from.0.to_le_bytes());
        payload.extend_from_slice(&to.0.to_le_bytes());
        payload.extend_from_slice(&predicate.0.to_le_bytes());
        self.set_payload(rel_id, &payload);

        // Update indexes
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
                // Decode RelationshipBody from payload
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
}

/// Initialize the place store
pub fn init() {
    *PLACE_STORE.lock() = Some(PlaceStore::new());
}

/// Create a new Thing with the given kind, schema, and version
pub fn thing_create(kind: SymbolId, schema: SymbolId, version: u32) -> ThingId {
    let mut guard = PLACE_STORE.lock();
    match guard.as_mut() {
        Some(store) => store.create_thing(kind, schema, version),
        None => ThingId(0),
    }
}

/// Set the inline payload for a Thing
pub fn thing_set_inline_payload(id: ThingId, payload: &[u8]) -> bool {
    let mut guard = PLACE_STORE.lock();
    match guard.as_mut() {
        Some(store) => store.set_payload(id, payload),
        None => false,
    }
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
    let mut guard = PLACE_STORE.lock();
    match guard.as_mut() {
        Some(store) => store.create_relationship(from, to, predicate),
        None => ThingId(0),
    }
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

