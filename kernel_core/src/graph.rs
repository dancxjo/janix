use abi::{NodeId, ProcessId, PropKey, PropType, PropValue, ThingId};

/// Zero-sized marker handle for the global graph state.
///
/// The backing storage is kept in module-level statics; this handle exists so
/// callers can take `&mut Graph` to make data flow explicit (e.g. the
/// scheduler) without changing the global storage model yet.
#[derive(Clone, Copy, Default, Debug)]
pub struct Graph;

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
    EdgeAdded {
        from: ThingId,
        edge_kind: &'static str,
        to: ThingId,
    },
    EdgeRemoved {
        from: ThingId,
        edge_kind: &'static str,
        to: ThingId,
    },
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
    edge_kind: &'static str,
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
const MAX_EDGES: usize = 256;
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

const MAX_NODES: usize = 128;
pub const MAX_THINGS: usize = 128;

// SAFETY: NODES and NEXT_ID are only accessed from single-threaded kernel context.
// In a multi-threaded environment, this would need atomic operations or locks.
static mut NODES: [Option<Node>; MAX_NODES] = [None; MAX_NODES];
static mut NEXT_ID: u64 = 0;

static mut THINGS: [Option<ThingNode>; MAX_THINGS] = [const { None }; MAX_THINGS];
static mut NEXT_THING_ID: u64 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    pub from: ThingId,
    pub edge_kind: &'static str,
    pub to: ThingId,
}

static mut EDGES: [Option<Edge>; MAX_EDGES] = [const { None }; MAX_EDGES];

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

/// Initialize the graph subsystem
/// This resets all global state for test isolation and kernel boot
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

        // Clear all edges
        let edges = &raw mut EDGES;
        for slot in (*edges).iter_mut() {
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
    pub fn add_edge(&mut self, from: ThingId, edge_kind: &'static str, to: ThingId) -> bool {
        add_edge(from, edge_kind, to)
    }

    #[inline]
    pub fn remove_edge(&mut self, from: ThingId, edge_kind: &'static str, to: ThingId) -> bool {
        remove_edge(from, edge_kind, to)
    }

    #[inline]
    pub fn neighbors(&self, from: ThingId, edge_kind: &'static str, out: &mut [Option<ThingId>]) {
        neighbors(from, edge_kind, out)
    }
}

/// Return the kind string for a Thing.
pub fn thing_kind(id: ThingId) -> Option<&'static str> {
    get_thing(id).map(|(kind, _)| kind)
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
            GraphEvent::EdgeAdded { edge_kind, .. } => {
                let listeners = &raw const EDGE_ADDED_LISTENERS;
                for slot in (*listeners).iter().flatten() {
                    if slot.edge_kind == *edge_kind {
                        (slot.listener)(event);
                    }
                }
            }
            GraphEvent::EdgeRemoved { edge_kind, .. } => {
                let listeners = &raw const EDGE_REMOVED_LISTENERS;
                for slot in (*listeners).iter().flatten() {
                    if slot.edge_kind == *edge_kind {
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

pub fn subscribe_edge_added(edge_kind: &'static str, listener: GraphListener) {
    unsafe {
        let listeners = &raw mut EDGE_ADDED_LISTENERS;
        for slot in (*listeners).iter_mut() {
            if slot.is_none() {
                *slot = Some(EdgeListener {
                    edge_kind,
                    listener,
                });
                return;
            }
        }
    }
}

pub fn subscribe_edge_removed(edge_kind: &'static str, listener: GraphListener) {
    unsafe {
        let listeners = &raw mut EDGE_REMOVED_LISTENERS;
        for slot in (*listeners).iter_mut() {
            if slot.is_none() {
                *slot = Some(EdgeListener {
                    edge_kind,
                    listener,
                });
                return;
            }
        }
    }
}

/// Create a new Thing
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

        THINGS[NEXT_THING_ID as usize] = Some(ThingNode {
            id,
            kind,
            props: node_props,
            owner_process: None,
        });
        NEXT_THING_ID += 1;
        dispatch_event(&GraphEvent::NodeCreated { id, kind });
        Some(id)
    }
}

/// Get a Thing
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

/// Update a Thing
pub fn update_thing(id: ThingId, props: &[(PropKey, PropValue)]) -> bool {
    unsafe {
        if id.0 >= MAX_THINGS as u64 {
            return false;
        }
        if let Some(node) = THINGS[id.0 as usize].as_mut() {
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
            true
        } else {
            false
        }
    }
}

/// Delete a Thing
pub fn delete_thing(id: ThingId) -> bool {
    unsafe {
        if id.0 >= MAX_THINGS as u64 {
            return false;
        }
        if let Some(thing) = THINGS[id.0 as usize].take() {
            // remove edges attached
            let edges = &raw mut EDGES;
            for slot in (*edges).iter_mut() {
                if let Some(edge) = slot {
                    if edge.from == id || edge.to == id {
                        let removed = *edge;
                        *slot = None;
                        dispatch_event(&GraphEvent::EdgeRemoved {
                            from: removed.from,
                            edge_kind: removed.edge_kind,
                            to: removed.to,
                        });
                    }
                }
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

/// Register a schema
pub fn register_schema(
    kind: &'static str,
    description: &'static str,
    props: &'static [(&'static str, PropType)],
) -> Result<(), &'static str> {
    unsafe {
        let schemas = &raw mut SCHEMAS;

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

/// Get a schema (returns static reference to props array)
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

/// Get a schema description
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

/// Validate properties against schema
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

        THINGS[NEXT_THING_ID as usize] = Some(ThingNode {
            id,
            kind,
            props: node_props,
            owner_process: Some(proc),
        });
        NEXT_THING_ID += 1;
        dispatch_event(&GraphEvent::NodeCreated { id, kind });
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

            // Update props logic
            for (key, value) in props {
                let mut found = false;
                for i in 0..MAX_PROPS_PER_THING {
                    if let Some((k, _)) = thing.props[i] {
                        if k == *key {
                            thing.props[i] = Some((*key, value.clone()));
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    for i in 0..MAX_PROPS_PER_THING {
                        if thing.props[i].is_none() {
                            thing.props[i] = Some((*key, value.clone()));
                            break;
                        }
                    }
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

/// Add an edge between two Things.
pub fn add_edge(from: ThingId, edge_kind: &'static str, to: ThingId) -> bool {
    unsafe {
        let edges = &raw mut EDGES;
        for slot in (*edges).iter_mut() {
            if let Some(edge) = slot {
                if edge.from == from && edge.to == to && edge.edge_kind == edge_kind {
                    return true; // already exists
                }
            }
        }
        for slot in (*edges).iter_mut() {
            if slot.is_none() {
                *slot = Some(Edge {
                    from,
                    edge_kind,
                    to,
                });
                dispatch_event(&GraphEvent::EdgeAdded {
                    from,
                    edge_kind,
                    to,
                });
                return true;
            }
        }
    }
    false
}

/// Remove an edge; returns true if removed.
pub fn remove_edge(from: ThingId, edge_kind: &'static str, to: ThingId) -> bool {
    unsafe {
        let edges = &raw mut EDGES;
        for slot in (*edges).iter_mut() {
            if let Some(edge) = slot {
                if edge.from == from && edge.edge_kind == edge_kind && edge.to == to {
                    *slot = None;
                    dispatch_event(&GraphEvent::EdgeRemoved {
                        from,
                        edge_kind,
                        to,
                    });
                    return true;
                }
            }
        }
    }
    false
}

/// Collect neighbors from outgoing edges of a given kind.
pub fn neighbors(from: ThingId, edge_kind: &'static str, out: &mut [Option<ThingId>]) {
    for slot in out.iter_mut() {
        *slot = None;
    }
    unsafe {
        let edges = &raw const EDGES;
        let mut idx = 0;
        for edge in (*edges).iter().flatten() {
            if edge.from == from && edge.edge_kind == edge_kind && idx < out.len() {
                out[idx] = Some(edge.to);
                idx += 1;
            }
        }
    }
}

/// Return the target ThingId for the edge at `index` with the provided `edge_kind`.
pub fn edge_target_at(from: ThingId, edge_kind: &'static str, index: usize) -> Option<ThingId> {
    unsafe {
        let edges = &raw const EDGES;
        let mut count = 0;
        for edge in (*edges).iter().flatten() {
            if edge.from == from && edge.edge_kind == edge_kind {
                if count == index {
                    return Some(edge.to);
                }
                count += 1;
            }
        }
    }
    None
}
