extern crate alloc;

use crate::graph_kinds;
use crate::graph_kinds::KIND_LINK;
use abi::{Link, NodeId, Predicate, ProcessId, PropKey, PropType, PropValue, ThingId};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::alloc::Layout;

use crate::journal;
use alloc::string::String;

/// Zero-sized marker handle for the global graph state.
///
/// The backing storage is kept in module-level statics; this handle exists so
/// callers can take `&mut Graph` to make data flow explicit (e.g. the
/// scheduler) without changing the global storage model yet.
#[derive(Clone, Copy, Default, Debug)]
pub struct Graph;

pub trait LinkStore {
    fn create_link(&mut self, src: ThingId, pred: Predicate, dst: ThingId) -> Option<ThingId>;
    fn delete_link(&mut self, id: ThingId) -> bool;

    fn links_from(&self, src: ThingId) -> &[ThingId];
    fn links_to(&self, dst: ThingId) -> &[ThingId];
    fn links_with_pred(&self, pred: Predicate) -> &[ThingId];

    fn links_from_pred(&self, src: ThingId, pred: Predicate) -> &[ThingId];
    fn links_to_pred(&self, dst: ThingId, pred: Predicate) -> &[ThingId];

    fn neighbor_dsts(&self, src: ThingId, pred: Predicate) -> impl Iterator<Item = ThingId> + '_;
}

/// A change emitted by the graph when things, properties, or links mutate.
#[derive(Debug, Clone)]
pub enum GraphEvent {
    ThingCreated {
        id: ThingId,
        kind: &'static str,
        kind_id: ThingId,
    },
    ThingDeleted {
        id: ThingId,
        kind: &'static str,
        kind_id: ThingId,
    },
    PropUpdated {
        id: ThingId,
        kind: &'static str,
        kind_id: ThingId,
        key: &'static str,
        old: Option<PropValue>,
        new: PropValue,
    },
    LinkAdded(Link),
    LinkRemoved(Link),
}

pub type GraphListener = fn(&GraphEvent);

#[derive(Clone, Copy)]
struct NodeListener {
    kind: &'static str,
    listener: GraphListener,
}

#[derive(Clone, Copy)]
struct PropListener {
    kind: &'static str,
    key: &'static str,
    listener: GraphListener,
}

#[derive(Clone, Copy)]
struct LinkListener {
    pred: Predicate,
    listener: GraphListener,
}

// Error message constants - these are stable and used by tests
const ERR_NO_SCHEMA: &str = "No schema registered for this kind";
const ERR_SCHEMA_ALREADY_REGISTERED: &str = "Schema already registered";
const ERR_SCHEMA_STORAGE_FULL: &str = "Schema storage full";
const ERR_TYPE_MISMATCH: &str = "Property type mismatch";
const ERR_PROPERTY_NOT_IN_SCHEMA: &str = "Property not in schema";
const ERR_TOO_MANY_SCHEMA_PROPS: &str = "Too many properties in schema";

#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub id: NodeId,
    pub value: u64,
}

const MAX_PROPS_PER_THING: usize = 8;
const MAX_NODE_LISTENERS: usize = 16;
const MAX_PROP_LISTENERS: usize = 16;
const MAX_LINK_LISTENERS: usize = 16;

#[derive(Debug, Clone)]
pub struct ThingNode {
    pub id: ThingId,
    pub kind: &'static str,
    pub kind_id: ThingId,
    pub props: [Option<(PropKey, PropValue)>; MAX_PROPS_PER_THING],
    pub owner_process: Option<ProcessId>,
}

const MAX_NODES: usize = 256;

// SAFETY: NODES and NEXT_ID are only accessed from single-threaded kernel context.
// In a multi-threaded environment, this would need atomic operations or locks.
static mut NODES: [Option<Node>; MAX_NODES] = [None; MAX_NODES];
static mut NEXT_ID: u64 = 0;

#[derive(Debug, Clone)]
struct ThingSlot {
    generation: u32,
    thing: Option<ThingNode>,
}

struct Slab {
    slots: Vec<ThingSlot>,
    free_indices: Vec<u32>,
}

static mut THINGS_SLAB: Option<Slab> = None;

impl Slab {
    fn alloc(&mut self) -> (u32, u32) {
        if let Some(idx) = self.free_indices.pop() {
            let slot = &mut self.slots[idx as usize];
            slot.generation = slot.generation.wrapping_add(1);
            if slot.thing.is_some() {
                // Should be unreachable if logic is correct
                panic!("Free slot occupied");
            }
            (idx, slot.generation)
        } else {
            let idx = self.slots.len() as u32;
            self.slots.push(ThingSlot {
                generation: 0,
                thing: None,
            });
            (idx, 0)
        }
    }

    fn peek_next_id(&self) -> (u32, u32) {
        if let Some(&idx) = self.free_indices.last() {
            let slot = &self.slots[idx as usize];
            (idx, slot.generation.wrapping_add(1))
        } else {
            (self.slots.len() as u32, 0)
        }
    }
}

fn things_slab() -> &'static mut Slab {
    unsafe {
        let slab_ptr = &raw mut THINGS_SLAB;
        (*slab_ptr).get_or_insert_with(|| Slab {
            slots: Vec::new(),
            free_indices: Vec::new(),
        })
    }
}

static mut LINK_INDEX: Option<LinkIndex> = None;

// Kind Membership Index
// KindId -> List of Member ThingIds
type KindIndex = BTreeMap<ThingId, Vec<ThingId>>;
static mut KIND_INDEX: Option<KindIndex> = None;

// Hot Property Index
// Use `'static str` for property *keys* to avoid heap-allocating keys repeatedly.
struct PropertyIndex {
    by_string: BTreeMap<&'static str, BTreeMap<String, Vec<ThingId>>>, // Key -> Value -> Things
    by_int: BTreeMap<&'static str, BTreeMap<i64, Vec<ThingId>>>,       // Key -> Value -> Things
    by_bool: BTreeMap<&'static str, BTreeMap<bool, Vec<ThingId>>>,     // Key -> Value -> Things
}

impl PropertyIndex {
    fn new() -> Self {
        Self {
            by_string: BTreeMap::new(),
            by_int: BTreeMap::new(),
            by_bool: BTreeMap::new(),
        }
    }
}

static mut PROPERTY_INDEX: Option<PropertyIndex> = None;

static mut NODE_CREATED_LISTENERS: [Option<NodeListener>; MAX_NODE_LISTENERS] =
    [const { None }; MAX_NODE_LISTENERS];
static mut NODE_DELETED_LISTENERS: [Option<NodeListener>; MAX_NODE_LISTENERS] =
    [const { None }; MAX_NODE_LISTENERS];
static mut PROP_CHANGED_LISTENERS: [Option<PropListener>; MAX_PROP_LISTENERS] =
    [const { None }; MAX_PROP_LISTENERS];
static mut LINK_ADDED_LISTENERS: [Option<LinkListener>; MAX_LINK_LISTENERS] =
    [const { None }; MAX_LINK_LISTENERS];
static mut LINK_REMOVED_LISTENERS: [Option<LinkListener>; MAX_LINK_LISTENERS] =
    [const { None }; MAX_LINK_LISTENERS];

// Schema storage
const MAX_SCHEMAS: usize = 64;
const MAX_SCHEMA_PROPS: usize = 16;

#[derive(Debug, Clone, Copy)]
pub struct Schema {
    pub kind: &'static str,
    pub description: &'static str,
    pub props: [Option<(&'static str, PropType)>; MAX_SCHEMA_PROPS],
    pub indexed_props: [Option<&'static str>; MAX_SCHEMA_PROPS],
}

// SAFETY: SCHEMAS is only accessed from single-threaded kernel context.
// In a multi-threaded environment, this would need atomic operations or locks.
// We use raw pointers to comply with Rust 2024 edition rules about mutable static references.
// SAFETY: SCHEMAS is only accessed from single-threaded kernel context.
// In a multi-threaded environment, this would need atomic operations or locks.
// We use raw pointers to comply with Rust 2024 edition rules about mutable static references.
static mut SCHEMAS: [Option<Schema>; MAX_SCHEMAS] = [None; MAX_SCHEMAS];

static mut KIND_MAP: Option<BTreeMap<&'static str, ThingId>> = None;

#[derive(Debug, Clone)]
struct LinkSlot {
    link: Link,
    deleted: bool,
}

// Simpler, safer storage for early kernel: map predicate -> (node -> list of link ids).
// This is easier to reason about and avoids brittle CSR offset maintenance.

#[derive(Default, Debug)]
pub struct LinkIndex {
    links: BTreeMap<ThingId, LinkSlot>,
    // Predicate -> (source node -> list of outgoing link ids)
    by_pred_outgoing: BTreeMap<Predicate, BTreeMap<ThingId, Vec<ThingId>>>,
    // Predicate -> (destination node -> list of incoming link ids)
    by_pred_incoming: BTreeMap<Predicate, BTreeMap<ThingId, Vec<ThingId>>>,
}

static EMPTY_LINK_IDS: [ThingId; 0] = [];

impl LinkIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.links.clear();
        self.by_pred_outgoing.clear();
        self.by_pred_incoming.clear();
    }

    pub fn insert(&mut self, link: Link) {
        let previous = self
            .links
            .get(&link.id)
            .filter(|slot| !slot.deleted)
            .map(|slot| slot.link);

        if let Some(existing_link) = previous {
            self.remove_from_indexes(link.id, &existing_link);
        }

        let slot = LinkSlot {
            link,
            deleted: false,
        };
        self.add_to_indexes(slot.link.id, &slot.link);
        self.links.insert(slot.link.id, slot);
    }

    pub fn remove(&mut self, id: ThingId) -> bool {
        if let Some(slot) = self.links.get_mut(&id) {
            if slot.deleted {
                return false;
            }
            let link = slot.link;
            slot.deleted = true;
            self.remove_from_indexes(id, &link);
            return true;
        }
        false
    }

    pub fn link(&self, id: ThingId) -> Option<&Link> {
        self.links
            .get(&id)
            .and_then(|slot| (!slot.deleted).then(|| &slot.link))
    }

    pub fn links_from(&self, _src: ThingId) -> &[ThingId] {
        // CSR optimizes specific predicates, global iteration is slower but possible.
        // For now, we return empty as this pattern should be avoided in favor of specific predicates.
        // Ideally we'd iterate all preds and collect, but that returns Vec not slice.
        &EMPTY_LINK_IDS
    }

    pub fn links_to(&self, _dst: ThingId) -> &[ThingId] {
        &EMPTY_LINK_IDS
    }

    pub fn links_with_pred(&self, _pred: Predicate) -> &[ThingId] {
        // This query is also not well supported by CSR (requires iterating all nodes).
        &EMPTY_LINK_IDS
    }

    pub fn links_from_pred(&self, src: ThingId, pred: Predicate) -> &[ThingId] {
        self.by_pred_outgoing
            .get(&pred)
            .and_then(|map| map.get(&src).map(|v| v.as_slice()))
            .unwrap_or(&EMPTY_LINK_IDS)
    }

    pub fn links_to_pred(&self, dst: ThingId, pred: Predicate) -> &[ThingId] {
        self.by_pred_incoming
            .get(&pred)
            .and_then(|map| map.get(&dst).map(|v| v.as_slice()))
            .unwrap_or(&EMPTY_LINK_IDS)
    }

    pub fn neighbor_dsts(
        &self,
        src: ThingId,
        pred: Predicate,
    ) -> impl Iterator<Item = ThingId> + '_ {
        self.links_from_pred(src, pred)
            .iter()
            .filter_map(|id| self.link(*id))
            .map(|link| link.dst)
    }

    pub fn collect_incident_links(&self, id: ThingId, out: &mut Vec<ThingId>) {
        // Iterate all predicates to find everything connected to `id`.
        for map in self.by_pred_outgoing.values() {
            if let Some(v) = map.get(&id) {
                out.extend_from_slice(v);
            }
        }
        for map in self.by_pred_incoming.values() {
            if let Some(v) = map.get(&id) {
                out.extend_from_slice(v);
            }
        }
    }

    fn add_to_indexes(&mut self, id: ThingId, link: &Link) {
        self.by_pred_outgoing
            .entry(link.pred)
            .or_default()
            .entry(link.src)
            .or_default()
            .push(id);

        self.by_pred_incoming
            .entry(link.pred)
            .or_default()
            .entry(link.dst)
            .or_default()
            .push(id);
    }

    fn remove_from_indexes(&mut self, id: ThingId, link: &Link) {
        if let Some(map) = self.by_pred_outgoing.get_mut(&link.pred) {
            if let Some(vec) = map.get_mut(&link.src) {
                if let Some(pos) = vec.iter().position(|e| *e == id) {
                    vec.remove(pos);
                }
                if vec.is_empty() {
                    map.remove(&link.src);
                }
            }
            if map.is_empty() {
                self.by_pred_outgoing.remove(&link.pred);
            }
        }

        if let Some(map) = self.by_pred_incoming.get_mut(&link.pred) {
            if let Some(vec) = map.get_mut(&link.dst) {
                if let Some(pos) = vec.iter().position(|e| *e == id) {
                    vec.remove(pos);
                }
                if vec.is_empty() {
                    map.remove(&link.dst);
                }
            }
            if map.is_empty() {
                self.by_pred_incoming.remove(&link.pred);
            }
        }
    }
}

fn link_index_mut() -> &'static mut LinkIndex {
    unsafe {
        let link_index = &raw mut LINK_INDEX;
        if (*link_index).is_none() {
            *link_index = Some(LinkIndex::new());
        }
        (*link_index).as_mut().unwrap()
    }
}

fn link_index_ref() -> &'static LinkIndex {
    unsafe {
        let link_index = &raw mut LINK_INDEX;
        if (*link_index).is_none() {
            *link_index = Some(LinkIndex::new());
        }
        (*link_index).as_ref().unwrap()
    }
}

/// Initialize the graph subsystem.
///
/// This resets all global state for tests and boot. Call this before using the
/// graph APIs to ensure schemas and storage are clean.
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::graph::init();
/// let schema = &[("value", abi::PropType::U64)];
/// k::graph::register_schema("Widget", "A widget", schema).unwrap();
/// let id = k::graph::create_thing("Widget", &[("value", abi::PropValue::U64(1))]).unwrap();
/// let (kind, _) = k::graph::get_thing(id).unwrap();
/// assert_eq!(kind, "Widget");
/// ```
pub fn init() {
    unsafe {
        // Reset counters
        NEXT_ID = 0;

        // Clear all thing storage
        let nodes = &raw mut NODES;
        for slot in (*nodes).iter_mut() {
            *slot = None;
        }

        // Reset slab
        THINGS_SLAB = Some(Slab {
            slots: Vec::new(),
            free_indices: Vec::new(),
        });

        // Clear all schema storage
        let schemas = &raw mut SCHEMAS;
        for slot in (*schemas).iter_mut() {
            *slot = None;
        }

        // Clear listeners
        let node_created = &raw mut NODE_CREATED_LISTENERS;
        for slot in (*node_created).iter_mut() {
            *slot = None;
        }
        let node_deleted = &raw mut NODE_DELETED_LISTENERS;
        for slot in (*node_deleted).iter_mut() {
            *slot = None;
        }
        let prop_changed = &raw mut PROP_CHANGED_LISTENERS;
        for slot in (*prop_changed).iter_mut() {
            *slot = None;
        }
        let link_added = &raw mut LINK_ADDED_LISTENERS;
        for slot in (*link_added).iter_mut() {
            *slot = None;
        }
        let link_removed = &raw mut LINK_REMOVED_LISTENERS;
        for slot in (*link_removed).iter_mut() {
            *slot = None;
        }

        // Clear kind map
        KIND_MAP = None;

        // Clear indexes
        KIND_INDEX = None;
        PROPERTY_INDEX = None;
    }

    link_index_mut().clear();

    static LINK_SCHEMA: &[(&str, PropType)] = &[
        (graph_kinds::PROP_LINK_SRC, PropType::U64),
        (graph_kinds::PROP_LINK_DST, PropType::U64),
        (graph_kinds::PROP_LINK_PRED, PropType::U64),
    ];
    let _ = register_schema(
        graph_kinds::KIND_LINK,
        "A graph link connecting Things by predicate",
        LINK_SCHEMA,
        &[graph_kinds::PROP_LINK_DST, graph_kinds::PROP_LINK_PRED],
    );
}

impl Graph {
    #[inline]
    pub fn new() -> Self {
        Graph
    }

    #[inline]
    pub fn create_thing(
        &mut self,
        kind: &'static str,
        props: &[(PropKey, PropValue)],
    ) -> Option<ThingId> {
        create_thing(kind, props)
    }

    #[inline]
    pub fn update_thing(&mut self, id: ThingId, props: &[(PropKey, PropValue)]) -> bool {
        update_thing(id, props)
    }

    #[inline]
    pub fn get_thing(
        &self,
        id: ThingId,
    ) -> Option<(&'static str, &'static [Option<(PropKey, PropValue)>])> {
        get_thing(id)
    }

    #[inline]
    pub fn add_link(&mut self, src: ThingId, pred: Predicate, dst: ThingId) -> bool {
        create_link(src, pred, dst).is_some()
    }

    #[inline]
    pub fn remove_link(&mut self, src: ThingId, pred: Predicate, dst: ThingId) -> bool {
        remove_link(src, pred, dst)
    }

    #[inline]
    pub fn neighbors(&self, src: ThingId, pred: Predicate, out: &mut [Option<ThingId>]) {
        neighbors(src, pred, out)
    }
}

impl LinkStore for Graph {
    fn create_link(&mut self, src: ThingId, pred: Predicate, dst: ThingId) -> Option<ThingId> {
        create_link(src, pred, dst)
    }

    fn delete_link(&mut self, id: ThingId) -> bool {
        delete_link(id)
    }

    fn links_from(&self, src: ThingId) -> &[ThingId] {
        link_index_ref().links_from(src)
    }

    fn links_to(&self, dst: ThingId) -> &[ThingId] {
        link_index_ref().links_to(dst)
    }

    fn links_with_pred(&self, pred: Predicate) -> &[ThingId] {
        link_index_ref().links_with_pred(pred)
    }

    fn links_from_pred(&self, src: ThingId, pred: Predicate) -> &[ThingId] {
        link_index_ref().links_from_pred(src, pred)
    }

    fn links_to_pred(&self, dst: ThingId, pred: Predicate) -> &[ThingId] {
        link_index_ref().links_to_pred(dst, pred)
    }

    fn neighbor_dsts(&self, src: ThingId, pred: Predicate) -> impl Iterator<Item = ThingId> + '_ {
        link_index_ref().neighbor_dsts(src, pred)
    }
}

/// Return the kind string for a Thing.
pub fn thing_kind(id: ThingId) -> Option<&'static str> {
    get_thing(id).map(|(kind, _)| kind)
}

fn link_from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Option<Link> {
    let mut src: Option<ThingId> = None;
    let mut dst: Option<ThingId> = None;
    let mut pred: Option<Predicate> = None;

    for (key, value) in props.iter().flatten() {
        match *key {
            graph_kinds::PROP_LINK_SRC => {
                if let PropValue::U64(v) = value {
                    src = Some(ThingId(*v));
                }
            }
            graph_kinds::PROP_LINK_DST => {
                if let PropValue::U64(v) = value {
                    dst = Some(ThingId(*v));
                }
            }
            graph_kinds::PROP_LINK_PRED => {
                if let PropValue::U64(v) = value {
                    pred = Some(Predicate(*v));
                }
            }
            _ => {}
        }
    }

    match (src, dst, pred) {
        (Some(src), Some(dst), Some(pred)) => Some(Link { id, src, dst, pred }),
        _ => None,
    }
}

/// Fetch a property from a Thing, cloning the stored value.
pub fn get_prop(id: ThingId, key: PropKey) -> Option<PropValue> {
    get_thing(id).and_then(|(_, props)| {
        props
            .iter()
            .flatten()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v.clone())
    })
}

/// Add a thing to the graph
pub fn add_node(value: u64) -> Option<NodeId> {
    unsafe {
        if NEXT_ID >= MAX_NODES as u64 {
            return None;
        }
        let id = NodeId(NEXT_ID);
        NODES[NEXT_ID as usize] = Some(Node { id, value });
        NEXT_ID += 1;
        Some(id)
    }
}

/// Query a thing in the graph
pub fn query_node(node_id: NodeId) -> Option<u64> {
    unsafe {
        if node_id.0 >= MAX_NODES as u64 {
            return None;
        }
        let idx = node_id.0 as usize;
        NODES[idx].as_ref().map(|n| n.value)
    }
}

/// Iterate over all things in the graph
pub fn iter_things<F>(mut f: F)
where
    F: FnMut(&ThingNode),
{
    let slab = things_slab();
    for slot in slab.slots.iter() {
        if let Some(thing) = &slot.thing {
            f(thing);
        }
    }
}

/// Find the next Thing of the specified kind after a given ThingId.
pub fn next_thing_of_kind(kind: &'static str, start_after: ThingId) -> Option<ThingId> {
    let slab = things_slab();
    let start_idx = if start_after.0 == u64::MAX {
        0
    } else {
        start_after.index() + 1
    };

    for slot in slab.slots.iter().skip(start_idx as usize) {
        if let Some(node) = &slot.thing {
            if node.kind == kind {
                return Some(node.id);
            }
        }
    }
    None
}

fn dispatch_event(event: &GraphEvent) {
    journal::push(event.clone());
    unsafe {
        match event {
            GraphEvent::ThingCreated { kind, .. } => {
                let listeners = &raw const NODE_CREATED_LISTENERS;
                for slot in (*listeners).iter().flatten() {
                    if slot.kind == *kind {
                        (slot.listener)(event);
                    }
                }
            }
            GraphEvent::ThingDeleted { kind, .. } => {
                let listeners = &raw const NODE_DELETED_LISTENERS;
                for slot in (*listeners).iter().flatten() {
                    if slot.kind == *kind {
                        (slot.listener)(event);
                    }
                }
            }
            GraphEvent::PropUpdated { kind, key, .. } => {
                let listeners = &raw const PROP_CHANGED_LISTENERS;
                for slot in (*listeners).iter().flatten() {
                    if slot.kind == *kind && slot.key == *key {
                        (slot.listener)(event);
                    }
                }
            }
            GraphEvent::LinkAdded(link) => {
                let listeners = &raw const LINK_ADDED_LISTENERS;
                for slot in (*listeners).iter().flatten() {
                    if slot.pred == link.pred {
                        (slot.listener)(event);
                    }
                }
            }
            GraphEvent::LinkRemoved(link) => {
                let listeners = &raw const LINK_REMOVED_LISTENERS;
                for slot in (*listeners).iter().flatten() {
                    if slot.pred == link.pred {
                        (slot.listener)(event);
                    }
                }
            }
        }
    }
}

pub fn subscribe_node_created(kind: &'static str, listener: GraphListener) {
    unsafe {
        let listeners = &raw mut NODE_CREATED_LISTENERS;
        for slot in (*listeners).iter_mut() {
            if slot.is_none() {
                *slot = Some(NodeListener { kind, listener });
                return;
            }
        }
    }
}

pub fn subscribe_node_deleted(kind: &'static str, listener: GraphListener) {
    unsafe {
        let listeners = &raw mut NODE_DELETED_LISTENERS;
        for slot in (*listeners).iter_mut() {
            if slot.is_none() {
                *slot = Some(NodeListener { kind, listener });
                return;
            }
        }
    }
}

pub fn subscribe_prop_changed(kind: &'static str, key: &'static str, listener: GraphListener) {
    unsafe {
        let listeners = &raw mut PROP_CHANGED_LISTENERS;
        for slot in (*listeners).iter_mut() {
            if slot.is_none() {
                *slot = Some(PropListener {
                    kind,
                    key,
                    listener,
                });
                return;
            }
        }
    }
}

pub fn subscribe_link_added(pred: Predicate, listener: GraphListener) {
    unsafe {
        let listeners = &raw mut LINK_ADDED_LISTENERS;
        for slot in (*listeners).iter_mut() {
            if slot.is_none() {
                *slot = Some(LinkListener { pred, listener });
                return;
            }
        }
    }
}

pub fn subscribe_link_removed(pred: Predicate, listener: GraphListener) {
    unsafe {
        let listeners = &raw mut LINK_REMOVED_LISTENERS;
        for slot in (*listeners).iter_mut() {
            if slot.is_none() {
                *slot = Some(LinkListener { pred, listener });
                return;
            }
        }
    }
}

/// Create a new Thing.
///
/// Caller is responsible for ensuring the kind has an appropriate schema if
/// validation is desired.
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::graph::init();
/// k::graph::register_schema("Widget", "A test kind", &[("value", abi::PropType::U64)]).unwrap();
/// let id = k::graph::create_thing("Widget", &[("value", abi::PropValue::U64(5))]).unwrap();
/// let (kind, props) = k::graph::get_thing(id).unwrap();
/// assert_eq!(kind, "Widget");
/// assert!(props.iter().flatten().any(|(k, v)| *k == "value" && matches!(v, abi::PropValue::U64(5))));
/// ```
fn ensure_kind_exists(kind: &'static str) -> ThingId {
    unsafe {
        let map_ptr = &raw mut KIND_MAP;
        let map = (*map_ptr).get_or_insert_with(BTreeMap::new);
        if let Some(id) = map.get(kind) {
            return *id;
        }

        // Bootstrapping logic
        if kind == graph_kinds::KIND_KIND {
            let slab = things_slab();
            let (idx, generation) = slab.peek_next_id();
            let id = ThingId::new(idx, generation);
            map.insert(kind, id);

            // Create the definitive "Kind" thing, which is of kind "Kind" (itself).
            // We pass explicit kind_id to avoid recursion.
            create_thing_internal(
                kind,
                Some(id),
                &[(graph_kinds::PROP_NAME, PropValue::Str(String::from("Kind")))],
                None,
            );
            return id;
        }

        // For any other kind (e.g. "Window"), we need to create a Thing of kind "Kind".
        // This ensures "Kind" exists first.
        let kind_kind_id = ensure_kind_exists(graph_kinds::KIND_KIND);

        let id = create_thing_internal(
            graph_kinds::KIND_KIND,
            Some(kind_kind_id),
            &[(graph_kinds::PROP_NAME, PropValue::Str(String::from(kind)))],
            None,
        )
        .expect("Failed to create kind thing");

        // Update map
        let map_ptr = &raw mut KIND_MAP;
        let map = (*map_ptr).get_or_insert_with(BTreeMap::new);
        map.insert(kind, id);
        id
    }
}

pub fn create_thing(kind: &'static str, props: &[(PropKey, PropValue)]) -> Option<ThingId> {
    create_thing_internal(kind, None, props, None)
}

fn create_thing_internal(
    kind: &'static str,
    explicit_kind_id: Option<ThingId>,
    props: &[(PropKey, PropValue)],
    owner_process: Option<ProcessId>,
) -> Option<ThingId> {
    // Resolve kind_id first to avoid holding slab borrow during recursion
    let kind_id = explicit_kind_id.unwrap_or_else(|| ensure_kind_exists(kind));

    unsafe {
        let slab = things_slab();
        let (idx, generation) = slab.alloc();
        let id = ThingId::new(idx, generation);

        let mut node_props = [const { None }; MAX_PROPS_PER_THING];
        for (i, prop) in props.iter().enumerate() {
            if i >= MAX_PROPS_PER_THING {
                break;
            }
            node_props[i] = Some((prop.0, prop.1.clone()));
        }

        // Mark views dirty if Window created

        let slot = &mut slab.slots[idx as usize];
        slot.thing = Some(ThingNode {
            id,
            kind,
            kind_id,
            props: node_props,
            owner_process,
        });

        // Add to Kind Index
        add_to_kind_index(id, kind_id);

        // Add to Property Index
        // Re-borrow props from the thing we just inserted to be safe?
        // Or just use `props` arg? iterating props arg is safer as we have keys/values
        for (key, value) in props {
            if is_prop_indexed(kind, key) {
                add_to_prop_index(id, key, value);
            }
        }

        dispatch_event(&GraphEvent::ThingCreated { id, kind, kind_id });

        if kind == graph_kinds::KIND_LINK {
            let slab = things_slab();
            let link = link_from_props(
                id,
                &slab.slots[id.index() as usize]
                    .thing
                    .as_ref()
                    .unwrap()
                    .props,
            )
            .expect("Created link but failed to parse props");
            link_index_mut().insert(link);
            dispatch_event(&GraphEvent::LinkAdded(link));
        }

        Some(id)
    }
}

/// Get a Thing by id, returning its kind and properties.
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::graph::init();
/// k::graph::register_schema("Widget", "A test kind", &[("flag", abi::PropType::Bool)]).unwrap();
/// let id = k::graph::create_thing("Widget", &[("flag", abi::PropValue::Bool(true))]).unwrap();
/// let (kind, props) = k::graph::get_thing(id).unwrap();
/// assert_eq!(kind, "Widget");
/// assert!(props.iter().flatten().any(|(k, v)| *k == "flag" && matches!(v, abi::PropValue::Bool(true))));
/// ```
pub fn get_thing(id: ThingId) -> Option<(&'static str, &'static [Option<(PropKey, PropValue)>])> {
    let slab = things_slab();
    let idx = id.index() as usize;
    if idx >= slab.slots.len() {
        return None;
    }
    let slot = &slab.slots[idx];
    if slot.generation != id.generation() {
        return None;
    }
    slot.thing
        .as_ref()
        .map(|n| (n.kind, &n.props as &[Option<(PropKey, PropValue)>]))
}

/// Update an existing Thing's properties.
///
/// Keys present in `props` are replaced; absent keys are left unchanged.
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::graph::init();
/// k::graph::register_schema("Widget", "A test kind", &[("value", abi::PropType::U64)]).unwrap();
/// let id = k::graph::create_thing("Widget", &[("value", abi::PropValue::U64(1))]).unwrap();
/// let ok = k::graph::update_thing(id, &[("value", abi::PropValue::U64(2))]);
/// assert!(ok);
/// let (_, props) = k::graph::get_thing(id).unwrap();
/// assert!(props.iter().flatten().any(|(k, v)| *k == "value" && matches!(v, abi::PropValue::U64(2))));
/// ```
pub fn update_thing(id: ThingId, props: &[(PropKey, PropValue)]) -> bool {
    let slab = things_slab();
    let idx = id.index() as usize;
    if idx >= slab.slots.len() {
        return false;
    }
    let slot = &mut slab.slots[idx];
    if slot.generation != id.generation() {
        return false;
    }

    if let Some(node) = slot.thing.as_mut() {
        let is_link = node.kind == graph_kinds::KIND_LINK;
        let previous_link = if is_link {
            link_index_ref().link(id).copied()
        } else {
            None
        };

        for (key, value) in props {
            let mut previous: Option<PropValue> = None;
            // Find existing key to update
            let mut found = false;
            for slot in node.props.iter_mut() {
                if let Some((k, old_val)) = slot {
                    if *k == *key {
                        // Clone old value before mutating slot
                        let old_val_clone = old_val.clone();
                        previous = Some(old_val_clone.clone());

                        *slot = Some((*key, value.clone()));
                        found = true;

                        // Update Index if needed (using cloned old value)
                        if is_prop_indexed(node.kind, key) {
                            remove_from_prop_index(id, key, &old_val_clone);
                            add_to_prop_index(id, key, value);
                        }
                        // Update SoA removed

                        break;
                    }
                }
            }
            // If not found, find empty slot
            if !found {
                for slot in node.props.iter_mut() {
                    if slot.is_none() {
                        *slot = Some((*key, value.clone()));
                        found = true;
                        // Add to index
                        if is_prop_indexed(node.kind, key) {
                            add_to_prop_index(id, key, value);
                        }
                        // Update SoA removed

                        break;
                    }
                }
            }

            dispatch_event(&GraphEvent::PropUpdated {
                id,
                kind: node.kind,
                kind_id: node.kind_id,
                key: *key,
                old: previous,
                new: value.clone(),
            });
        }
        if is_link {
            let new_link = link_from_props(id, &node.props);
            match (previous_link, new_link) {
                (Some(prev), Some(next)) => {
                    if prev != next {
                        if link_index_mut().remove(id) {
                            dispatch_event(&GraphEvent::LinkRemoved(prev));
                        }
                        link_index_mut().insert(next);
                        dispatch_event(&GraphEvent::LinkAdded(next));
                    }
                }
                (Some(prev), None) => {
                    if link_index_mut().remove(id) {
                        dispatch_event(&GraphEvent::LinkRemoved(prev));
                    }
                }
                (None, Some(next)) => {
                    link_index_mut().insert(next);
                    dispatch_event(&GraphEvent::LinkAdded(next));
                }
                (None, None) => {}
            }
        }
        true
    } else {
        false
    }
}

fn remove_incident_links(id: ThingId) {
    let mut links: Vec<ThingId> = Vec::new();
    link_index_ref().collect_incident_links(id, &mut links);
    links.sort_by_key(|t| t.0);
    links.dedup();

    for link_id in links {
        let _ = delete_link(link_id);
    }
}

/// Delete a Thing.
///
/// If deleting an link, associated indexes are cleaned up.
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::graph::init();
/// let id = k::graph::create_thing("Widget", &[]).unwrap();
/// assert!(k::graph::delete_thing(id));
/// assert!(k::graph::get_thing(id).is_none());
/// ```
pub fn delete_thing(id: ThingId) -> bool {
    let slab = things_slab();
    let idx = id.index() as usize;
    if idx >= slab.slots.len() {
        return false;
    }
    let slot = &mut slab.slots[idx];
    if slot.generation != id.generation() {
        return false;
    }

    if let Some(thing) = slot.thing.take() {
        if thing.kind == graph_kinds::KIND_LINK {
            if let Some(link) = link_index_ref().link(id).copied() {
                if link_index_mut().remove(id) {
                    dispatch_event(&GraphEvent::LinkRemoved(link));
                }
            }
        } else {
            remove_incident_links(id);
        }

        // Remove from indexes
        remove_from_kind_index(id, thing.kind_id);
        for prop in thing.props.iter().flatten() {
            if is_prop_indexed(thing.kind, prop.0) {
                remove_from_prop_index(id, prop.0, &prop.1);
            }
        }

        // SoA removal removed

        dispatch_event(&GraphEvent::ThingDeleted {
            id,
            kind: thing.kind,
            kind_id: thing.kind_id,
        });

        // Add to free list
        slab.free_indices.push(idx as u32);

        true
    } else {
        false
    }
}

/// Register a schema for a Thing kind.
///
/// Returns an error if the kind is already registered, too many props are
/// provided, or storage is exhausted.
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::graph::init();
/// k::graph::register_schema("Widget", "A test kind", &[("value", abi::PropType::U64)]).unwrap();
/// // Registering again fails.
/// assert!(k::graph::register_schema("Widget", "dup", &[]).is_err());
/// ```
pub fn register_schema(
    kind: &'static str,
    description: &'static str,
    props: &'static [(&'static str, PropType)],
    indexed_props: &'static [&'static str],
) -> Result<(), &'static str> {
    unsafe {
        let schemas = &raw mut SCHEMAS;
        #[cfg(feature = "trace-schemas")]
        crate::println!("SCHEMAS address: {:p}", schemas);

        // Check if schema already exists
        for schema in (*schemas).iter() {
            if let Some(s) = schema {
                if s.kind == kind {
                    return Err(ERR_SCHEMA_ALREADY_REGISTERED);
                }
            }
        }

        // Find empty slot
        for slot in (*schemas).iter_mut() {
            if slot.is_none() {
                let mut schema_props = [None; MAX_SCHEMA_PROPS];
                let mut schema_indexed_props = [None; MAX_SCHEMA_PROPS];

                for (i, prop) in props.iter().enumerate() {
                    if i >= MAX_SCHEMA_PROPS {
                        return Err(ERR_TOO_MANY_SCHEMA_PROPS);
                    }
                    schema_props[i] = Some(*prop);
                }

                for (i, prop) in indexed_props.iter().enumerate() {
                    if i >= MAX_SCHEMA_PROPS {
                        return Err(ERR_TOO_MANY_SCHEMA_PROPS); // Reusing error
                    }
                    schema_indexed_props[i] = Some(*prop);
                }

                *slot = Some(Schema {
                    kind,
                    description,
                    props: schema_props,
                    indexed_props: schema_indexed_props,
                });
                return Ok(());
            }
        }

        Err(ERR_SCHEMA_STORAGE_FULL)
    }
}

/// Get a schema (returns static reference to props array).
pub fn get_schema_props(kind: &'static str) -> Option<&'static [Option<(&'static str, PropType)>]> {
    unsafe {
        let schemas = &raw const SCHEMAS;
        for schema in (*schemas).iter() {
            if let Some(s) = schema {
                if s.kind == kind {
                    return Some(&s.props[..]);
                }
            }
        }
        None
    }
}

pub fn get_schema_indexed_props(kind: &str) -> Option<&'static [Option<&'static str>]> {
    unsafe {
        let schemas = &raw const SCHEMAS;
        for schema in (*schemas).iter() {
            if let Some(s) = schema {
                if s.kind == kind {
                    return Some(&s.indexed_props[..]);
                }
            }
        }
        None
    }
}

/// Get a schema description.
pub fn get_schema_description(kind: &'static str) -> Option<&'static str> {
    unsafe {
        let schemas = &raw const SCHEMAS;
        for schema in (*schemas).iter() {
            if let Some(s) = schema {
                if s.kind == kind {
                    return Some(s.description);
                }
            }
        }
        None
    }
}

/// Validate properties against a schema.
///
/// Returns an error if the kind is unknown, a property is missing from the
/// schema, or the type mismatches.
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::graph::init();
/// let schema = &[("value", abi::PropType::U64)];
/// k::graph::register_schema("Widget", "A test kind", schema).unwrap();
/// assert!(k::graph::validate_props("Widget", &[("value", abi::PropValue::U64(3))]).is_ok());
/// assert!(k::graph::validate_props("Widget", &[("value", abi::PropValue::Bool(false))]).is_err());
/// ```
pub fn validate_props(
    kind: &'static str,
    props: &[(PropKey, PropValue)],
) -> Result<(), &'static str> {
    unsafe {
        // Find the schema
        let schemas = &raw const SCHEMAS;
        let schema = (*schemas)
            .iter()
            .find_map(|s| s.as_ref().filter(|s| s.kind == kind));

        let schema = match schema {
            Some(s) => s,
            None => return Err(ERR_NO_SCHEMA),
        };

        // Validate each incoming property
        for (key, value) in props {
            // Find the property in the schema
            let mut found = false;
            for prop_def in schema.props.iter() {
                if let Some((schema_key, schema_type)) = prop_def {
                    if *schema_key == *key {
                        found = true;

                        // Check type matches
                        let type_matches = match (schema_type, value) {
                            (PropType::U64, PropValue::U64(_)) => true,
                            (PropType::I64, PropValue::I64(_)) => true,
                            (PropType::Bool, PropValue::Bool(_)) => true,
                            (PropType::Str, PropValue::Str(_)) => true,
                            _ => false,
                        };

                        if !type_matches {
                            return Err(ERR_TYPE_MISMATCH);
                        }
                        break;
                    }
                }
            }

            if !found {
                return Err(ERR_PROPERTY_NOT_IN_SCHEMA);
            }
        }

        Ok(())
    }
}

/// Create a new Thing owned by a process
pub fn kernel_create_user_thing_for_process(
    proc: ProcessId,
    kind: &'static str,
    props: &[(PropKey, PropValue)],
) -> Option<ThingId> {
    create_thing_internal(kind, None, props, Some(proc))
}

/// Update a Thing owned by a process
pub fn kernel_user_update_thing(
    proc: ProcessId,
    id: ThingId,
    props: &[(PropKey, PropValue)],
) -> bool {
    unsafe {
        let slab = things_slab();
        let idx = id.index() as usize;
        if idx >= slab.slots.len() {
            return false;
        }
        let slot = &mut slab.slots[idx];
        if slot.generation != id.generation() {
            return false;
        }

        if let Some(thing) = slot.thing.as_mut() {
            if thing.owner_process != Some(proc) {
                return false; // Access denied
            }

            let is_link = thing.kind == graph_kinds::KIND_LINK;
            let previous_link = if is_link {
                link_index_ref().link(id).copied()
            } else {
                None
            };

            // Update props logic
            for (key, value) in props {
                let mut found = false;
                for i in 0..MAX_PROPS_PER_THING {
                    if let Some((k, _)) = thing.props[i] {
                        if k == *key {
                            let old = thing.props[i].as_ref().map(|(_, v)| v.clone());
                            thing.props[i] = Some((*key, value.clone()));
                            found = true;
                            dispatch_event(&GraphEvent::PropUpdated {
                                id,
                                kind: thing.kind,
                                kind_id: thing.kind_id,
                                key: *key,
                                old,
                                new: value.clone(),
                            });
                            break;
                        }
                    }
                }
                if !found {
                    for i in 0..MAX_PROPS_PER_THING {
                        if thing.props[i].is_none() {
                            thing.props[i] = Some((*key, value.clone()));
                            dispatch_event(&GraphEvent::PropUpdated {
                                id,
                                kind: thing.kind,
                                kind_id: thing.kind_id,
                                key: *key,
                                old: None,
                                new: value.clone(),
                            });
                            break;
                        }
                    }
                }
            }

            if is_link {
                let new_link = link_from_props(id, &thing.props);
                match (previous_link, new_link) {
                    (Some(prev), Some(next)) => {
                        if prev != next {
                            if link_index_mut().remove(id) {
                                dispatch_event(&GraphEvent::LinkRemoved(prev));
                            }
                            link_index_mut().insert(next);
                            dispatch_event(&GraphEvent::LinkAdded(next));
                        }
                    }
                    (Some(prev), None) => {
                        if link_index_mut().remove(id) {
                            dispatch_event(&GraphEvent::LinkRemoved(prev));
                        }
                    }
                    (None, Some(next)) => {
                        link_index_mut().insert(next);
                        dispatch_event(&GraphEvent::LinkAdded(next));
                    }
                    (None, None) => {}
                }
            }
            return true;
        }
        false
    }
}

pub fn cleanup_process_graph(_proc: ProcessId) {
    // TODO: delete or mark Things owned by proc
}

fn is_prop_indexed(kind: &str, key: &str) -> bool {
    get_schema_indexed_props(kind)
        .map(|props| props.iter().any(|p| *p == Some(key)))
        .unwrap_or(false)
}

fn add_to_kind_index(id: ThingId, kind_id: ThingId) {
    unsafe {
        let index = &raw mut KIND_INDEX;
        (*index)
            .get_or_insert_with(BTreeMap::new)
            .entry(kind_id)
            .or_default()
            .push(id);
    }
}

fn remove_from_kind_index(id: ThingId, kind_id: ThingId) {
    unsafe {
        let index = &raw mut KIND_INDEX;
        if let Some(map) = (*index).as_mut() {
            if let Some(vec) = map.get_mut(&kind_id) {
                if let Some(pos) = vec.iter().position(|x| *x == id) {
                    vec.swap_remove(pos);
                }
            }
        }
    }
}

fn add_to_prop_index(id: ThingId, key: PropKey, value: &PropValue) {
    unsafe {
        let index_ptr = &raw mut PROPERTY_INDEX;
        let index = (*index_ptr).get_or_insert_with(PropertyIndex::new);

        match value {
            PropValue::Str(s) => {
                index
                    .by_string
                    .entry(key)
                    .or_default()
                    .entry(s.clone())
                    .or_default()
                    .push(id);
            }
            PropValue::I64(v) => {
                index
                    .by_int
                    .entry(key)
                    .or_default()
                    .entry(*v)
                    .or_default()
                    .push(id);
            }
            PropValue::Bool(v) => {
                index
                    .by_bool
                    .entry(key)
                    .or_default()
                    .entry(*v)
                    .or_default()
                    .push(id);
            }
            _ => {} // U64 and others not indexed for now (as per PropertyIndex struct)
        }
    }
}

fn remove_from_prop_index(id: ThingId, key: PropKey, value: &PropValue) {
    unsafe {
        let index_ptr = &raw mut PROPERTY_INDEX;
        if let Some(index) = (*index_ptr).as_mut() {
            match value {
                PropValue::Str(s) => {
                    if let Some(map) = index.by_string.get_mut(key) {
                        if let Some(vec) = map.get_mut(s) {
                            if let Some(pos) = vec.iter().position(|x| *x == id) {
                                vec.swap_remove(pos);
                            }
                        }
                    }
                }
                PropValue::I64(v) => {
                    if let Some(map) = index.by_int.get_mut(key) {
                        if let Some(vec) = map.get_mut(v) {
                            if let Some(pos) = vec.iter().position(|x| *x == id) {
                                vec.swap_remove(pos);
                            }
                        }
                    }
                }
                PropValue::Bool(v) => {
                    if let Some(map) = index.by_bool.get_mut(key) {
                        if let Some(vec) = map.get_mut(v) {
                            if let Some(pos) = vec.iter().position(|x| *x == id) {
                                vec.swap_remove(pos);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

/// Create a link Thing and index it.
///
/// If the link already exists between the endpoints for the predicate, the
/// existing link id is returned.
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::graph::init();
/// let a = k::graph::create_thing("Thread", &[]).unwrap();
/// let b = k::graph::create_thing("CpuCore", &[]).unwrap();
/// let link = k::graph::create_link(a, k::graph_kinds::LINK_RUNS_ON, b).unwrap();
/// // Calling again returns the same link id.
/// let link2 = k::graph::create_link(a, k::graph_kinds::LINK_RUNS_ON, b).unwrap();
/// assert_eq!(link, link2);
/// ```
pub fn create_link(src: ThingId, pred: Predicate, dst: ThingId) -> Option<ThingId> {
    if let Some(existing) = link_index_ref()
        .links_from_pred(src, pred)
        .iter()
        .copied()
        .find(|id| match link_index_ref().link(*id) {
            Some(link) => link.dst == dst,
            None => false,
        })
    {
        return Some(existing);
    }

    let props = &[
        (graph_kinds::PROP_LINK_SRC, PropValue::U64(src.0)),
        (graph_kinds::PROP_LINK_DST, PropValue::U64(dst.0)),
        (graph_kinds::PROP_LINK_PRED, PropValue::U64(pred.0)),
    ];
    create_thing(graph_kinds::KIND_LINK, props)
}

/// Add a link between two Things; convenience wrapper returning success.
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::graph::init();
/// let a = k::graph::create_thing("Thread", &[]).unwrap();
/// let b = k::graph::create_thing("CpuCore", &[]).unwrap();
/// assert!(k::graph::add_link(a, k::graph_kinds::LINK_RUNS_ON, b));
/// ```
pub fn add_link(src: ThingId, pred: Predicate, dst: ThingId) -> bool {
    create_link(src, pred, dst).is_some()
}

/// Delete a link directly by ID.
pub fn delete_link(link_id: ThingId) -> bool {
    // delete_thing handles link deletion logic via property checks
    delete_thing(link_id)
}

/// Remove a link by endpoints/predicate; returns true if removed.
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::graph::init();
/// let a = k::graph::create_thing("Thread", &[]).unwrap();
/// let b = k::graph::create_thing("CpuCore", &[]).unwrap();
/// k::graph::create_link(a, k::graph_kinds::LINK_RUNS_ON, b).unwrap();
/// assert!(k::graph::remove_link(a, k::graph_kinds::LINK_RUNS_ON, b));
/// assert!(k::graph::link_target_at(a, k::graph_kinds::LINK_RUNS_ON, 0).is_none());
/// ```
pub fn remove_link(src: ThingId, pred: Predicate, dst: ThingId) -> bool {
    let candidate = link_index_ref()
        .links_from_pred(src, pred)
        .iter()
        .copied()
        .find(|id| match link_index_ref().link(*id) {
            Some(link) => link.dst == dst,
            None => false,
        });

    if let Some(id) = candidate {
        delete_thing(id)
    } else {
        false
    }
}

/// Collect neighbors from outgoing links of a given predicate.
///
/// The provided buffer is cleared then filled in order with matching dst ids.
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::graph::init();
/// let a = k::graph::create_thing("Thread", &[]).unwrap();
/// let b = k::graph::create_thing("CpuCore", &[]).unwrap();
/// k::graph::add_link(a, k::graph_kinds::LINK_RUNS_ON, b);
/// let mut out = [None; 2];
/// k::graph::neighbors(a, k::graph_kinds::LINK_RUNS_ON, &mut out);
/// assert!(out.iter().flatten().any(|id| *id == b));
/// ```
pub fn neighbors(from: ThingId, pred: Predicate, out: &mut [Option<ThingId>]) {
    for slot in out.iter_mut() {
        *slot = None;
    }

    for (slot, dst) in out
        .iter_mut()
        .zip(link_index_ref().neighbor_dsts(from, pred))
    {
        *slot = Some(dst);
    }
}

/// Return the target ThingId for the link at `index` with the provided predicate.
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::graph::init();
/// let a = k::graph::create_thing("Thread", &[]).unwrap();
/// let b = k::graph::create_thing("CpuCore", &[]).unwrap();
/// k::graph::add_link(a, k::graph_kinds::LINK_RUNS_ON, b);
/// assert_eq!(k::graph::link_target_at(a, k::graph_kinds::LINK_RUNS_ON, 0), Some(b));
/// ```
pub fn link_target_at(from: ThingId, pred: Predicate, index: usize) -> Option<ThingId> {
    link_index_ref().neighbor_dsts(from, pred).nth(index)
}
