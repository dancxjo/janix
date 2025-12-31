//! Graph store
//!
//! Minimal in-kernel Thing/Link storage for the v0.3 core.
//! This provides the foundational data structure for all system state.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use spin::Mutex;

use crate::symbols::SymbolId;

/// Thing identifier - a 128-bit UUID
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ThingId(pub u128);

impl ThingId {
    /// Create a new ThingId from high and low parts
    pub fn from_parts(high: u64, low: u64) -> Self {
        ThingId(((high as u128) << 64) | (low as u128))
    }

    /// Get the high 64 bits
    pub fn high(&self) -> u64 {
        (self.0 >> 64) as u64
    }

    /// Get the low 64 bits
    pub fn low(&self) -> u64 {
        self.0 as u64
    }
}

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

/// A link between Things
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Link {
    pub src: ThingId,
    pub predicate: SymbolId,
    pub dst: ThingId,
}

/// Global graph store
static GRAPH: Mutex<Option<GraphStore>> = Mutex::new(None);

struct GraphStore {
    /// Thing table: ThingId -> Thing
    things: BTreeMap<ThingId, Thing>,
    /// Link set: (src, pred, dst) for efficient queries
    links: BTreeMap<Link, ()>,
    /// Counter for generating unique IDs
    next_id: u128,
    /// Tick counter for timestamps
    tick: u64,
}

impl GraphStore {
    fn new() -> Self {
        Self {
            things: BTreeMap::new(),
            links: BTreeMap::new(),
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

    fn create_link(&mut self, src: ThingId, predicate: SymbolId, dst: ThingId) {
        let link = Link {
            src,
            predicate,
            dst,
        };
        self.links.insert(link, ());
    }

    fn thing_count(&self) -> usize {
        self.things.len()
    }

    fn link_count(&self) -> usize {
        self.links.len()
    }
}

/// Initialize the graph store
pub fn init() {
    *GRAPH.lock() = Some(GraphStore::new());
}

/// Create a new Thing with the given kind, schema, and version
pub fn thing_create(kind: SymbolId, schema: SymbolId, version: u32) -> ThingId {
    let mut guard = GRAPH.lock();
    match guard.as_mut() {
        Some(store) => store.create_thing(kind, schema, version),
        None => ThingId(0),
    }
}

/// Set the inline payload for a Thing
pub fn thing_set_inline_payload(id: ThingId, payload: &[u8]) -> bool {
    let mut guard = GRAPH.lock();
    match guard.as_mut() {
        Some(store) => store.set_payload(id, payload),
        None => false,
    }
}

/// Get the header of a Thing
pub fn get_header(id: ThingId) -> Option<ThingHeader> {
    let guard = GRAPH.lock();
    guard
        .as_ref()
        .and_then(|store| store.get_header(id).cloned())
}

/// Get the payload of a Thing
pub fn get_payload(id: ThingId) -> Option<Vec<u8>> {
    let guard = GRAPH.lock();
    guard
        .as_ref()
        .and_then(|store| store.get_payload(id).map(Vec::from))
}

/// Create a link between two Things
pub fn link_create(src: ThingId, predicate: SymbolId, dst: ThingId) {
    let mut guard = GRAPH.lock();
    if let Some(store) = guard.as_mut() {
        store.create_link(src, predicate, dst);
    }
}

/// Get statistics about the graph
pub fn stats() -> (usize, usize) {
    let guard = GRAPH.lock();
    match guard.as_ref() {
        Some(store) => (store.thing_count(), store.link_count()),
        None => (0, 0),
    }
}

/// Advance the graph tick (for timestamps)
pub fn tick() {
    let mut guard = GRAPH.lock();
    if let Some(store) = guard.as_mut() {
        store.tick += 1;
    }
}
