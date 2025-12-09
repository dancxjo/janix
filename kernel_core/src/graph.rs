use abi::{NodeId, ProcessId, PropKey, PropType, PropValue, ThingId};

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

#[derive(Debug, Clone, Copy)]
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

static mut THINGS: [Option<ThingNode>; MAX_THINGS] = [None; MAX_THINGS];
static mut NEXT_THING_ID: u64 = 0;

// Schema storage
const MAX_SCHEMAS: usize = 64;
const MAX_SCHEMA_PROPS: usize = 16;

#[derive(Debug, Clone, Copy)]
pub struct Schema {
    pub kind: &'static str,
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
    }
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

/// Create a new Thing
pub fn create_thing(kind: &'static str, props: &[(PropKey, PropValue)]) -> Option<ThingId> {
    unsafe {
        if NEXT_THING_ID >= MAX_THINGS as u64 {
            return None;
        }
        let id = ThingId(NEXT_THING_ID);
        let mut node_props = [None; MAX_PROPS_PER_THING];
        for (i, prop) in props.iter().enumerate() {
            if i >= MAX_PROPS_PER_THING {
                break;
            }
            node_props[i] = Some(*prop);
        }

        THINGS[NEXT_THING_ID as usize] = Some(ThingNode {
            id,
            kind,
            props: node_props,
            owner_process: None,
        });
        NEXT_THING_ID += 1;
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
                // Find existing key to update
                let mut found = false;
                for slot in node.props.iter_mut() {
                    if let Some((k, _)) = slot {
                        if *k == *key {
                            *slot = Some((*key, *value));
                            found = true;
                            break;
                        }
                    }
                }
                // If not found, find empty slot
                if !found {
                    for slot in node.props.iter_mut() {
                        if slot.is_none() {
                            *slot = Some((*key, *value));
                            break;
                        }
                    }
                }
            }
            true
        } else {
            false
        }
    }
}

/// Register a schema
pub fn register_schema(
    kind: &'static str,
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
        let mut node_props = [None; MAX_PROPS_PER_THING];
        for (i, prop) in props.iter().enumerate() {
            if i >= MAX_PROPS_PER_THING {
                break;
            }
            node_props[i] = Some(*prop);
        }

        THINGS[NEXT_THING_ID as usize] = Some(ThingNode {
            id,
            kind,
            props: node_props,
            owner_process: Some(proc),
        });
        NEXT_THING_ID += 1;
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
                            thing.props[i] = Some((*key, *value));
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    for i in 0..MAX_PROPS_PER_THING {
                        if thing.props[i].is_none() {
                            thing.props[i] = Some((*key, *value));
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

pub fn cleanup_process_graph(proc: ProcessId) {
    // TODO: delete or mark Things owned by proc
}
