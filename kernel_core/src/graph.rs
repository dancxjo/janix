extern crate alloc;

use crate::graph_kinds;
use abi::{Edge, EdgePred, NodeId, ProcessId, PropKey, PropType, PropValue, ThingId};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

/// Zero-sized marker handle for the global graph state.
///
/// The backing storage is kept in module-level statics; this handle exists so
/// callers can take `&mut Graph` to make data flow explicit (e.g. the
/// scheduler) without changing the global storage model yet.
#[derive(Clone, Copy, Default, Debug)]
pub struct Graph;

pub trait EdgeStore {
    fn create_edge(&mut self, src: ThingId, pred: EdgePred, dst: ThingId) -> Option<ThingId>;
    fn delete_edge(&mut self, id: ThingId) -> bool;

    fn edges_from(&self, src: ThingId) -> &[ThingId];
    fn edges_to(&self, dst: ThingId) -> &[ThingId];
    fn edges_with_pred(&self, pred: EdgePred) -> &[ThingId];

    fn edges_from_pred(&self, src: ThingId, pred: EdgePred) -> &[ThingId];
    fn edges_to_pred(&self, dst: ThingId, pred: EdgePred) -> &[ThingId];

    fn neighbor_dsts(&self, src: ThingId, pred: EdgePred) -> impl Iterator<Item = ThingId> + '_;
}

/// A change emitted by the graph when nodes, properties, or edges mutate.
#[derive(Debug, Clone)]
pub enum GraphEvent {
    NodeCreated {
        id: ThingId,
        kind: &'static str,
    },
    NodeDeleted {
        id: ThingId,
        kind: &'static str,
    },
    PropChanged {
        id: ThingId,
        kind: &'static str,
        key: &'static str,
        old: Option<PropValue>,
        new: PropValue,
    },
    EdgeAdded(Edge),
    EdgeRemoved(Edge),
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
struct EdgeListener {
    pred: EdgePred,
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
const MAX_EDGE_LISTENERS: usize = 16;

#[derive(Debug, Clone)]
pub struct ThingNode {
    pub id: ThingId,
    pub kind: &'static str,
    pub props: [Option<(PropKey, PropValue)>; MAX_PROPS_PER_THING],
    pub owner_process: Option<ProcessId>,
}

const MAX_NODES: usize = 256;
pub const MAX_THINGS: usize = 512;

// SAFETY: NODES and NEXT_ID are only accessed from single-threaded kernel context.
// In a multi-threaded environment, this would need atomic operations or locks.
static mut NODES: [Option<Node>; MAX_NODES] = [None; MAX_NODES];
static mut NEXT_ID: u64 = 0;

static mut THINGS: [Option<ThingNode>; MAX_THINGS] = [const { None }; MAX_THINGS];
static mut NEXT_THING_ID: u64 = 0;

static mut EDGE_INDEX: Option<EdgeIndex> = None;

static mut NODE_CREATED_LISTENERS: [Option<NodeListener>; MAX_NODE_LISTENERS] =
    [const { None }; MAX_NODE_LISTENERS];
static mut NODE_DELETED_LISTENERS: [Option<NodeListener>; MAX_NODE_LISTENERS] =
    [const { None }; MAX_NODE_LISTENERS];
static mut PROP_CHANGED_LISTENERS: [Option<PropListener>; MAX_PROP_LISTENERS] =
    [const { None }; MAX_PROP_LISTENERS];
static mut EDGE_ADDED_LISTENERS: [Option<EdgeListener>; MAX_EDGE_LISTENERS] =
    [const { None }; MAX_EDGE_LISTENERS];
static mut EDGE_REMOVED_LISTENERS: [Option<EdgeListener>; MAX_EDGE_LISTENERS] =
    [const { None }; MAX_EDGE_LISTENERS];

// Schema storage
const MAX_SCHEMAS: usize = 64;
const MAX_SCHEMA_PROPS: usize = 16;

#[derive(Debug, Clone, Copy)]
pub struct Schema {
    pub kind: &'static str,
    pub description: &'static str,
    pub props: [Option<(&'static str, PropType)>; MAX_SCHEMA_PROPS],
}

// SAFETY: SCHEMAS is only accessed from single-threaded kernel context.
// In a multi-threaded environment, this would need atomic operations or locks.
// We use raw pointers to comply with Rust 2024 edition rules about mutable static references.
static mut SCHEMAS: [Option<Schema>; MAX_SCHEMAS] = [None; MAX_SCHEMAS];

#[derive(Debug, Clone)]
struct EdgeSlot {
    edge: Edge,
    deleted: bool,
}

#[derive(Default, Debug)]
pub struct EdgeIndex {
    edges: BTreeMap<ThingId, EdgeSlot>,
    by_src: BTreeMap<ThingId, Vec<ThingId>>,
    by_dst: BTreeMap<ThingId, Vec<ThingId>>,
    by_pred: BTreeMap<EdgePred, Vec<ThingId>>,
    by_src_pred: BTreeMap<(ThingId, EdgePred), Vec<ThingId>>,
    by_dst_pred: BTreeMap<(ThingId, EdgePred), Vec<ThingId>>,
}

static EMPTY_EDGE_IDS: [ThingId; 0] = [];

impl EdgeIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.edges.clear();
        self.by_src.clear();
        self.by_dst.clear();
        self.by_pred.clear();
        self.by_src_pred.clear();
        self.by_dst_pred.clear();
    }

    pub fn insert(&mut self, edge: Edge) {
        let previous = self
            .edges
            .get(&edge.id)
            .filter(|slot| !slot.deleted)
            .map(|slot| slot.edge);

        if let Some(existing_edge) = previous {
            self.remove_from_indexes(edge.id, &existing_edge);
        }

        let slot = EdgeSlot {
            edge,
            deleted: false,
        };
        self.add_to_indexes(slot.edge.id, &slot.edge);
        self.edges.insert(slot.edge.id, slot);
    }

    pub fn remove(&mut self, id: ThingId) -> bool {
        if let Some(slot) = self.edges.get_mut(&id) {
            if slot.deleted {
                return false;
            }
            let edge = slot.edge;
            slot.deleted = true;
            self.remove_from_indexes(id, &edge);
            return true;
        }
        false
    }

    pub fn edge(&self, id: ThingId) -> Option<&Edge> {
        self.edges
            .get(&id)
            .and_then(|slot| (!slot.deleted).then(|| &slot.edge))
    }

    pub fn edges_from(&self, src: ThingId) -> &[ThingId] {
        self.slice_for(&self.by_src, &src)
    }

    pub fn edges_to(&self, dst: ThingId) -> &[ThingId] {
        self.slice_for(&self.by_dst, &dst)
    }

    pub fn edges_with_pred(&self, pred: EdgePred) -> &[ThingId] {
        self.slice_for(&self.by_pred, &pred)
    }

    pub fn edges_from_pred(&self, src: ThingId, pred: EdgePred) -> &[ThingId] {
        self.slice_for(&self.by_src_pred, &(src, pred))
    }

    pub fn edges_to_pred(&self, dst: ThingId, pred: EdgePred) -> &[ThingId] {
        self.slice_for(&self.by_dst_pred, &(dst, pred))
    }

    pub fn neighbor_dsts(
        &self,
        src: ThingId,
        pred: EdgePred,
    ) -> impl Iterator<Item = ThingId> + '_ {
        self.edges_from_pred(src, pred)
            .iter()
            .filter_map(|id| self.edge(*id))
            .map(|edge| edge.dst)
    }

    fn slice_for<'a, K: Ord>(
        &'a self,
        map: &'a BTreeMap<K, Vec<ThingId>>,
        key: &K,
    ) -> &'a [ThingId] {
        map.get(key)
            .map(|v| v.as_slice())
            .unwrap_or(&EMPTY_EDGE_IDS)
    }

    fn add_to_indexes(&mut self, id: ThingId, edge: &Edge) {
        Self::push_unique(self.by_src.entry(edge.src).or_default(), id);
        Self::push_unique(self.by_dst.entry(edge.dst).or_default(), id);
        Self::push_unique(self.by_pred.entry(edge.pred).or_default(), id);
        Self::push_unique(
            self.by_src_pred.entry((edge.src, edge.pred)).or_default(),
            id,
        );
        Self::push_unique(
            self.by_dst_pred.entry((edge.dst, edge.pred)).or_default(),
            id,
        );
    }

    fn remove_from_indexes(&mut self, id: ThingId, edge: &Edge) {
        if let Some(v) = self.by_src.get_mut(&edge.src) {
            v.retain(|e| *e != id);
        }
        if let Some(v) = self.by_dst.get_mut(&edge.dst) {
            v.retain(|e| *e != id);
        }
        if let Some(v) = self.by_pred.get_mut(&edge.pred) {
            v.retain(|e| *e != id);
        }
        if let Some(v) = self.by_src_pred.get_mut(&(edge.src, edge.pred)) {
            v.retain(|e| *e != id);
        }
        if let Some(v) = self.by_dst_pred.get_mut(&(edge.dst, edge.pred)) {
            v.retain(|e| *e != id);
        }
    }

    fn push_unique(vec: &mut Vec<ThingId>, id: ThingId) {
        if !vec.contains(&id) {
            vec.push(id);
        }
    }
}

fn edge_index_mut() -> &'static mut EdgeIndex {
    unsafe {
        let edge_index = &raw mut EDGE_INDEX;
        if (*edge_index).is_none() {
            *edge_index = Some(EdgeIndex::new());
        }
        (*edge_index).as_mut().unwrap()
    }
}

fn edge_index_ref() -> &'static EdgeIndex {
    unsafe {
        let edge_index = &raw mut EDGE_INDEX;
        if (*edge_index).is_none() {
            *edge_index = Some(EdgeIndex::new());
        }
        (*edge_index).as_ref().unwrap()
    }
}

/// Initialize the graph subsystem.
///
/// This resets all global state for tests and boot. Call this before using the
/// graph APIs to ensure schemas and storage are clean.
///
/// # Examples
/// ```
/// # use kernel_core as k;
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
        NEXT_THING_ID = 0;

        // Clear all node storage
        let nodes = &raw mut NODES;
        for slot in (*nodes).iter_mut() {
            *slot = None;
        }

        // Clear all Thing storage
        let things = &raw mut THINGS;
        for slot in (*things).iter_mut() {
            *slot = None;
        }

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
        let edge_added = &raw mut EDGE_ADDED_LISTENERS;
        for slot in (*edge_added).iter_mut() {
            *slot = None;
        }
        let edge_removed = &raw mut EDGE_REMOVED_LISTENERS;
        for slot in (*edge_removed).iter_mut() {
            *slot = None;
        }
    }

    edge_index_mut().clear();

    static EDGE_SCHEMA: &[(&str, PropType)] = &[
        (graph_kinds::PROP_EDGE_SRC, PropType::U64),
        (graph_kinds::PROP_EDGE_DST, PropType::U64),
        (graph_kinds::PROP_EDGE_PRED, PropType::U64),
    ];
    let _ = register_schema(
        graph_kinds::KIND_EDGE,
        "A graph edge connecting Things by predicate",
        EDGE_SCHEMA,
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
    pub fn add_edge(&mut self, from: ThingId, pred: EdgePred, to: ThingId) -> bool {
        create_edge(from, pred, to).is_some()
    }

    #[inline]
    pub fn remove_edge(&mut self, from: ThingId, pred: EdgePred, to: ThingId) -> bool {
        remove_edge(from, pred, to)
    }

    #[inline]
    pub fn neighbors(&self, from: ThingId, pred: EdgePred, out: &mut [Option<ThingId>]) {
        neighbors(from, pred, out)
    }
}

impl EdgeStore for Graph {
    fn create_edge(&mut self, src: ThingId, pred: EdgePred, dst: ThingId) -> Option<ThingId> {
        create_edge(src, pred, dst)
    }

    fn delete_edge(&mut self, id: ThingId) -> bool {
        delete_edge(id)
    }

    fn edges_from(&self, src: ThingId) -> &[ThingId] {
        edge_index_ref().edges_from(src)
    }

    fn edges_to(&self, dst: ThingId) -> &[ThingId] {
        edge_index_ref().edges_to(dst)
    }

    fn edges_with_pred(&self, pred: EdgePred) -> &[ThingId] {
        edge_index_ref().edges_with_pred(pred)
    }

    fn edges_from_pred(&self, src: ThingId, pred: EdgePred) -> &[ThingId] {
        edge_index_ref().edges_from_pred(src, pred)
    }

    fn edges_to_pred(&self, dst: ThingId, pred: EdgePred) -> &[ThingId] {
        edge_index_ref().edges_to_pred(dst, pred)
    }

    fn neighbor_dsts(&self, src: ThingId, pred: EdgePred) -> impl Iterator<Item = ThingId> + '_ {
        edge_index_ref().neighbor_dsts(src, pred)
    }
}

/// Return the kind string for a Thing.
pub fn thing_kind(id: ThingId) -> Option<&'static str> {
    get_thing(id).map(|(kind, _)| kind)
}

fn edge_from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Option<Edge> {
    let mut src: Option<ThingId> = None;
    let mut dst: Option<ThingId> = None;
    let mut pred: Option<EdgePred> = None;

    for (key, value) in props.iter().flatten() {
        match *key {
            graph_kinds::PROP_EDGE_SRC => {
                if let PropValue::U64(v) = value {
                    src = Some(ThingId(*v));
                }
            }
            graph_kinds::PROP_EDGE_DST => {
                if let PropValue::U64(v) = value {
                    dst = Some(ThingId(*v));
                }
            }
            graph_kinds::PROP_EDGE_PRED => {
                if let PropValue::U64(v) = value {
                    pred = Some(EdgePred(*v));
                }
            }
            _ => {}
        }
    }

    match (src, dst, pred) {
        (Some(src), Some(dst), Some(pred)) => Some(Edge { id, src, dst, pred }),
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

/// Add a node to the graph
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

/// Query a node in the graph
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
    unsafe {
        let things = &raw const THINGS;
        for slot in (*things).iter().flatten() {
            f(slot);
        }
    }
}

/// Find the next Thing of the specified kind after a given ThingId.
pub fn next_thing_of_kind(kind: &'static str, start_after: ThingId) -> Option<ThingId> {
    unsafe {
        let mut idx = if start_after.0 == u64::MAX {
            0
        } else {
            start_after.0.saturating_add(1)
        };
        while idx < MAX_THINGS as u64 {
            if let Some(node) = THINGS[idx as usize].as_ref() {
                if node.kind == kind {
                    return Some(node.id);
                }
            }
            idx += 1;
        }
    }
    None
}

fn dispatch_event(event: &GraphEvent) {
    unsafe {
        match event {
            GraphEvent::NodeCreated { kind, .. } => {
                let listeners = &raw const NODE_CREATED_LISTENERS;
                for slot in (*listeners).iter().flatten() {
                    if slot.kind == *kind {
                        (slot.listener)(event);
                    }
                }
            }
            GraphEvent::NodeDeleted { kind, .. } => {
                let listeners = &raw const NODE_DELETED_LISTENERS;
                for slot in (*listeners).iter().flatten() {
                    if slot.kind == *kind {
                        (slot.listener)(event);
                    }
                }
            }
            GraphEvent::PropChanged { kind, key, .. } => {
                let listeners = &raw const PROP_CHANGED_LISTENERS;
                for slot in (*listeners).iter().flatten() {
                    if slot.kind == *kind && slot.key == *key {
                        (slot.listener)(event);
                    }
                }
            }
            GraphEvent::EdgeAdded(edge) => {
                let listeners = &raw const EDGE_ADDED_LISTENERS;
                for slot in (*listeners).iter().flatten() {
                    if slot.pred == edge.pred {
                        (slot.listener)(event);
                    }
                }
            }
            GraphEvent::EdgeRemoved(edge) => {
                let listeners = &raw const EDGE_REMOVED_LISTENERS;
                for slot in (*listeners).iter().flatten() {
                    if slot.pred == edge.pred {
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

pub fn subscribe_edge_added(pred: EdgePred, listener: GraphListener) {
    unsafe {
        let listeners = &raw mut EDGE_ADDED_LISTENERS;
        for slot in (*listeners).iter_mut() {
            if slot.is_none() {
                *slot = Some(EdgeListener { pred, listener });
                return;
            }
        }
    }
}

pub fn subscribe_edge_removed(pred: EdgePred, listener: GraphListener) {
    unsafe {
        let listeners = &raw mut EDGE_REMOVED_LISTENERS;
        for slot in (*listeners).iter_mut() {
            if slot.is_none() {
                *slot = Some(EdgeListener { pred, listener });
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
/// # use kernel_core as k;
/// # let _guard = k::test_lock();
/// k::graph::init();
/// k::graph::register_schema("Widget", "A test kind", &[("value", abi::PropType::U64)]).unwrap();
/// let id = k::graph::create_thing("Widget", &[("value", abi::PropValue::U64(5))]).unwrap();
/// let (kind, props) = k::graph::get_thing(id).unwrap();
/// assert_eq!(kind, "Widget");
/// assert!(props.iter().flatten().any(|(k, v)| *k == "value" && matches!(v, abi::PropValue::U64(5))));
/// ```
pub fn create_thing(kind: &'static str, props: &[(PropKey, PropValue)]) -> Option<ThingId> {
    unsafe {
        if NEXT_THING_ID >= MAX_THINGS as u64 {
            return None;
        }
        let id = ThingId(NEXT_THING_ID);
        let mut node_props = [const { None }; MAX_PROPS_PER_THING];
        for (i, prop) in props.iter().enumerate() {
            if i >= MAX_PROPS_PER_THING {
                break;
            }
            node_props[i] = Some((prop.0, prop.1.clone()));
        }

        let edge_for_index = if kind == graph_kinds::KIND_EDGE {
            edge_from_props(id, &node_props)
        } else {
            None
        };

        THINGS[NEXT_THING_ID as usize] = Some(ThingNode {
            id,
            kind,
            props: node_props,
            owner_process: None,
        });
        NEXT_THING_ID += 1;
        dispatch_event(&GraphEvent::NodeCreated { id, kind });
        if let Some(edge) = edge_for_index {
            edge_index_mut().insert(edge);
            dispatch_event(&GraphEvent::EdgeAdded(edge));
        }
        Some(id)
    }
}

/// Get a Thing by id, returning its kind and properties.
///
/// # Examples
/// ```
/// # use kernel_core as k;
/// # let _guard = k::test_lock();
/// k::graph::init();
/// k::graph::register_schema("Widget", "A test kind", &[("flag", abi::PropType::Bool)]).unwrap();
/// let id = k::graph::create_thing("Widget", &[("flag", abi::PropValue::Bool(true))]).unwrap();
/// let (kind, props) = k::graph::get_thing(id).unwrap();
/// assert_eq!(kind, "Widget");
/// assert!(props.iter().flatten().any(|(k, v)| *k == "flag" && matches!(v, abi::PropValue::Bool(true))));
/// ```
pub fn get_thing(id: ThingId) -> Option<(&'static str, &'static [Option<(PropKey, PropValue)>])> {
    unsafe {
        if id.0 >= MAX_THINGS as u64 {
            return None;
        }
        THINGS[id.0 as usize]
            .as_ref()
            .map(|n| (n.kind, &n.props as &[Option<(PropKey, PropValue)>]))
    }
}

/// Update an existing Thing's properties.
///
/// Keys present in `props` are replaced; absent keys are left unchanged.
///
/// # Examples
/// ```
/// # use kernel_core as k;
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
    unsafe {
        if id.0 >= MAX_THINGS as u64 {
            return false;
        }
        if let Some(node) = THINGS[id.0 as usize].as_mut() {
            let is_edge = node.kind == graph_kinds::KIND_EDGE;
            let previous_edge = if is_edge {
                edge_index_ref().edge(id).copied()
            } else {
                None
            };

            for (key, value) in props {
                let mut previous: Option<PropValue> = None;
                // Find existing key to update
                let mut found = false;
                for slot in node.props.iter_mut() {
                    if let Some((k, _)) = slot {
                        if *k == *key {
                            if let Some((_, old_val)) = slot.clone() {
                                previous = Some(old_val);
                            }
                            *slot = Some((*key, value.clone()));
                            found = true;
                            break;
                        }
                    }
                }
                // If not found, find empty slot
                if !found {
                    for slot in node.props.iter_mut() {
                        if slot.is_none() {
                            *slot = Some((*key, value.clone()));
                            break;
                        }
                    }
                }
                dispatch_event(&GraphEvent::PropChanged {
                    id,
                    kind: node.kind,
                    key: *key,
                    old: previous,
                    new: value.clone(),
                });
            }
            if is_edge {
                let new_edge = edge_from_props(id, &node.props);
                match (previous_edge, new_edge) {
                    (Some(prev), Some(next)) => {
                        if prev != next {
                            if edge_index_mut().remove(id) {
                                dispatch_event(&GraphEvent::EdgeRemoved(prev));
                            }
                            edge_index_mut().insert(next);
                            dispatch_event(&GraphEvent::EdgeAdded(next));
                        }
                    }
                    (Some(prev), None) => {
                        if edge_index_mut().remove(id) {
                            dispatch_event(&GraphEvent::EdgeRemoved(prev));
                        }
                    }
                    (None, Some(next)) => {
                        edge_index_mut().insert(next);
                        dispatch_event(&GraphEvent::EdgeAdded(next));
                    }
                    (None, None) => {}
                }
            }
            true
        } else {
            false
        }
    }
}

fn remove_incident_edges(id: ThingId) {
    let mut edges: Vec<ThingId> = Vec::new();
    edges.extend_from_slice(edge_index_ref().edges_from(id));
    edges.extend_from_slice(edge_index_ref().edges_to(id));
    edges.sort_by_key(|t| t.0);
    edges.dedup();

    for edge_id in edges {
        let _ = delete_edge(edge_id);
    }
}

/// Delete a Thing.
///
/// If deleting an edge, associated indexes are cleaned up.
///
/// # Examples
/// ```
/// # use kernel_core as k;
/// # let _guard = k::test_lock();
/// k::graph::init();
/// let id = k::graph::create_thing("Widget", &[]).unwrap();
/// assert!(k::graph::delete_thing(id));
/// assert!(k::graph::get_thing(id).is_none());
/// ```
pub fn delete_thing(id: ThingId) -> bool {
    unsafe {
        if id.0 >= MAX_THINGS as u64 {
            return false;
        }
        if let Some(thing) = THINGS[id.0 as usize].take() {
            if thing.kind == graph_kinds::KIND_EDGE {
                if let Some(edge) = edge_index_ref().edge(id).copied() {
                    if edge_index_mut().remove(id) {
                        dispatch_event(&GraphEvent::EdgeRemoved(edge));
                    }
                }
            } else {
                remove_incident_edges(id);
            }
            dispatch_event(&GraphEvent::NodeDeleted {
                id,
                kind: thing.kind,
            });
            true
        } else {
            false
        }
    }
}

/// Register a schema for a Thing kind.
///
/// Returns an error if the kind is already registered, too many props are
/// provided, or storage is exhausted.
///
/// # Examples
/// ```
/// # use kernel_core as k;
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
) -> Result<(), &'static str> {
    unsafe {
        let schemas = &raw mut SCHEMAS;
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
                for (i, prop) in props.iter().enumerate() {
                    if i >= MAX_SCHEMA_PROPS {
                        return Err(ERR_TOO_MANY_SCHEMA_PROPS);
                    }
                    schema_props[i] = Some(*prop);
                }

                *slot = Some(Schema {
                    kind,
                    description,
                    props: schema_props,
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
/// # use kernel_core as k;
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
        let schemas = &raw const SCHEMAS;
        // Find the schema
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
    unsafe {
        if NEXT_THING_ID >= MAX_THINGS as u64 {
            return None;
        }
        let id = ThingId(NEXT_THING_ID);
        let mut node_props = [const { None }; MAX_PROPS_PER_THING];
        for (i, prop) in props.iter().enumerate() {
            if i >= MAX_PROPS_PER_THING {
                break;
            }
            node_props[i] = Some((prop.0, prop.1.clone()));
        }

        let edge_for_index = if kind == graph_kinds::KIND_EDGE {
            edge_from_props(id, &node_props)
        } else {
            None
        };

        THINGS[NEXT_THING_ID as usize] = Some(ThingNode {
            id,
            kind,
            props: node_props,
            owner_process: Some(proc),
        });
        NEXT_THING_ID += 1;
        dispatch_event(&GraphEvent::NodeCreated { id, kind });
        if let Some(edge) = edge_for_index {
            edge_index_mut().insert(edge);
            dispatch_event(&GraphEvent::EdgeAdded(edge));
        }
        Some(id)
    }
}

/// Update a Thing owned by a process
pub fn kernel_user_update_thing(
    proc: ProcessId,
    id: ThingId,
    props: &[(PropKey, PropValue)],
) -> bool {
    unsafe {
        if id.0 >= MAX_THINGS as u64 {
            return false;
        }
        if let Some(thing) = &mut THINGS[id.0 as usize] {
            if thing.owner_process != Some(proc) {
                return false; // Access denied
            }

            let is_edge = thing.kind == graph_kinds::KIND_EDGE;
            let previous_edge = if is_edge {
                edge_index_ref().edge(id).copied()
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
                            dispatch_event(&GraphEvent::PropChanged {
                                id,
                                kind: thing.kind,
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
                            dispatch_event(&GraphEvent::PropChanged {
                                id,
                                kind: thing.kind,
                                key: *key,
                                old: None,
                                new: value.clone(),
                            });
                            break;
                        }
                    }
                }
            }

            if is_edge {
                let new_edge = edge_from_props(id, &thing.props);
                match (previous_edge, new_edge) {
                    (Some(prev), Some(next)) => {
                        if prev != next {
                            if edge_index_mut().remove(id) {
                                dispatch_event(&GraphEvent::EdgeRemoved(prev));
                            }
                            edge_index_mut().insert(next);
                            dispatch_event(&GraphEvent::EdgeAdded(next));
                        }
                    }
                    (Some(prev), None) => {
                        if edge_index_mut().remove(id) {
                            dispatch_event(&GraphEvent::EdgeRemoved(prev));
                        }
                    }
                    (None, Some(next)) => {
                        edge_index_mut().insert(next);
                        dispatch_event(&GraphEvent::EdgeAdded(next));
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

/// Create an edge Thing and index it.
///
/// If the edge already exists between the endpoints for the predicate, the
/// existing edge id is returned.
///
/// # Examples
/// ```
/// # use kernel_core as k;
/// # let _guard = k::test_lock();
/// k::graph::init();
/// let a = k::graph::create_thing("Thread", &[]).unwrap();
/// let b = k::graph::create_thing("CpuCore", &[]).unwrap();
/// let edge = k::graph::create_edge(a, k::graph_kinds::EDGE_RUNS_ON, b).unwrap();
/// // Calling again returns the same edge id.
/// let edge2 = k::graph::create_edge(a, k::graph_kinds::EDGE_RUNS_ON, b).unwrap();
/// assert_eq!(edge, edge2);
/// ```
pub fn create_edge(src: ThingId, pred: EdgePred, dst: ThingId) -> Option<ThingId> {
    if let Some(existing) = edge_index_ref()
        .edges_from_pred(src, pred)
        .iter()
        .copied()
        .find(|id| match edge_index_ref().edge(*id) {
            Some(edge) => edge.dst == dst,
            None => false,
        })
    {
        return Some(existing);
    }

    let props = &[
        (graph_kinds::PROP_EDGE_SRC, PropValue::U64(src.0)),
        (graph_kinds::PROP_EDGE_DST, PropValue::U64(dst.0)),
        (graph_kinds::PROP_EDGE_PRED, PropValue::U64(pred.0)),
    ];
    create_thing(graph_kinds::KIND_EDGE, props)
}

/// Add an edge between two Things; convenience wrapper returning success.
///
/// # Examples
/// ```
/// # use kernel_core as k;
/// # let _guard = k::test_lock();
/// k::graph::init();
/// let a = k::graph::create_thing("Thread", &[]).unwrap();
/// let b = k::graph::create_thing("CpuCore", &[]).unwrap();
/// assert!(k::graph::add_edge(a, k::graph_kinds::EDGE_RUNS_ON, b));
/// ```
pub fn add_edge(from: ThingId, pred: EdgePred, to: ThingId) -> bool {
    create_edge(from, pred, to).is_some()
}

/// Delete an edge Thing by id.
///
/// Returns `false` if the id does not refer to an edge.
///
/// # Examples
/// ```
/// # use kernel_core as k;
/// # let _guard = k::test_lock();
/// k::graph::init();
/// let a = k::graph::create_thing("Thread", &[]).unwrap();
/// let b = k::graph::create_thing("CpuCore", &[]).unwrap();
/// let edge = k::graph::create_edge(a, k::graph_kinds::EDGE_RUNS_ON, b).unwrap();
/// assert!(k::graph::delete_edge(edge));
/// assert!(k::graph::get_thing(edge).is_none());
/// ```
pub fn delete_edge(id: ThingId) -> bool {
    unsafe {
        if id.0 >= MAX_THINGS as u64 {
            return false;
        }
        match THINGS[id.0 as usize].as_ref() {
            Some(node) if node.kind == graph_kinds::KIND_EDGE => {}
            _ => return false,
        }
    }

    delete_thing(id)
}

/// Remove an edge by endpoints/predicate; returns true if removed.
///
/// # Examples
/// ```
/// # use kernel_core as k;
/// # let _guard = k::test_lock();
/// k::graph::init();
/// let a = k::graph::create_thing("Thread", &[]).unwrap();
/// let b = k::graph::create_thing("CpuCore", &[]).unwrap();
/// k::graph::create_edge(a, k::graph_kinds::EDGE_RUNS_ON, b).unwrap();
/// assert!(k::graph::remove_edge(a, k::graph_kinds::EDGE_RUNS_ON, b));
/// assert!(k::graph::edge_target_at(a, k::graph_kinds::EDGE_RUNS_ON, 0).is_none());
/// ```
pub fn remove_edge(from: ThingId, pred: EdgePred, to: ThingId) -> bool {
    let candidate = edge_index_ref()
        .edges_from_pred(from, pred)
        .iter()
        .copied()
        .find(|id| match edge_index_ref().edge(*id) {
            Some(edge) => edge.dst == to,
            None => false,
        });

    match candidate {
        Some(id) => delete_edge(id),
        None => false,
    }
}

/// Collect neighbors from outgoing edges of a given predicate.
///
/// The provided buffer is cleared then filled in order with matching dst ids.
///
/// # Examples
/// ```
/// # use kernel_core as k;
/// # let _guard = k::test_lock();
/// k::graph::init();
/// let a = k::graph::create_thing("Thread", &[]).unwrap();
/// let b = k::graph::create_thing("CpuCore", &[]).unwrap();
/// k::graph::add_edge(a, k::graph_kinds::EDGE_RUNS_ON, b);
/// let mut out = [None; 2];
/// k::graph::neighbors(a, k::graph_kinds::EDGE_RUNS_ON, &mut out);
/// assert!(out.iter().flatten().any(|id| *id == b));
/// ```
pub fn neighbors(from: ThingId, pred: EdgePred, out: &mut [Option<ThingId>]) {
    for slot in out.iter_mut() {
        *slot = None;
    }

    for (slot, dst) in out
        .iter_mut()
        .zip(edge_index_ref().neighbor_dsts(from, pred))
    {
        *slot = Some(dst);
    }
}

/// Return the target ThingId for the edge at `index` with the provided predicate.
///
/// # Examples
/// ```
/// # use kernel_core as k;
/// # let _guard = k::test_lock();
/// k::graph::init();
/// let a = k::graph::create_thing("Thread", &[]).unwrap();
/// let b = k::graph::create_thing("CpuCore", &[]).unwrap();
/// k::graph::add_edge(a, k::graph_kinds::EDGE_RUNS_ON, b);
/// assert_eq!(k::graph::edge_target_at(a, k::graph_kinds::EDGE_RUNS_ON, 0), Some(b));
/// ```
pub fn edge_target_at(from: ThingId, pred: EdgePred, index: usize) -> Option<ThingId> {
    edge_index_ref().neighbor_dsts(from, pred).nth(index)
}
