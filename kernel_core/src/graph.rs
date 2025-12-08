use abi::{NodeId, PropKey, PropValue, ThingId};

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
}

const MAX_NODES: usize = 128;
const MAX_THINGS: usize = 128;

// SAFETY: NODES and NEXT_ID are only accessed from single-threaded kernel context.
// In a multi-threaded environment, this would need atomic operations or locks.
static mut NODES: [Option<Node>; MAX_NODES] = [None; MAX_NODES];
static mut NEXT_ID: u64 = 0;

static mut THINGS: [Option<ThingNode>; MAX_THINGS] = [None; MAX_THINGS];
static mut NEXT_THING_ID: u64 = 0;

/// Initialize the graph subsystem
pub fn init() {
    unsafe {
        NEXT_ID = 0;
        NEXT_THING_ID = 0;
        // Static init is already None, but being explicit for clarity
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

/// Create a new Thing
pub fn create_thing(kind: &'static str, props: &'static [(PropKey, PropValue)]) -> Option<ThingId> {
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
pub fn update_thing(id: ThingId, props: &'static [(PropKey, PropValue)]) -> bool {
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
