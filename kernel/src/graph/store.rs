extern crate alloc;

use crate::graph_kinds;
use abi::{Link, NodeId, Predicate, PropKey, PropValue, ThingId};
use alloc::vec::Vec;

use super::events::{GraphEvent, dispatch_event};
use super::index_links;
use super::index_props::{add_to_prop_index, remove_from_prop_index};
use super::schema::{add_to_kind_index, is_prop_indexed, remove_from_kind_index};

use alloc::string::String;

#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub id: NodeId,
    pub value: u64,
}

const MAX_PROPS_PER_THING: usize = 8;

#[derive(Debug, Clone)]
pub struct ThingNode {
    pub id: ThingId,
    pub kind: &'static str,
    pub kind_id: ThingId,
    pub props: [Option<(PropKey, PropValue)>; MAX_PROPS_PER_THING],
    pub owner_process: Option<abi::ProcessId>,
}

const MAX_NODES: usize = 256;

// SAFETY: NODES and NEXT_ID are only accessed from single-threaded kernel context.
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

    pub(crate) fn peek_next_id(&self) -> (u32, u32) {
        if let Some(&idx) = self.free_indices.last() {
            let slot = &self.slots[idx as usize];
            (idx, slot.generation.wrapping_add(1))
        } else {
            (self.slots.len() as u32, 0)
        }
    }
}
pub(crate) fn things_slab() -> &'static mut Slab {
    unsafe {
        let slab_ptr = &raw mut THINGS_SLAB;
        (*slab_ptr).get_or_insert_with(|| Slab {
            slots: Vec::new(),
            free_indices: Vec::new(),
        })
    }
}

pub(crate) fn peek_next_slab_id() -> (u32, u32) {
    let slab = things_slab();
    slab.peek_next_id()
}
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
    }
}

pub fn query_node(node_id: NodeId) -> Option<u64> {
    unsafe {
        if node_id.0 >= MAX_NODES as u64 {
            return None;
        }
        let idx = node_id.0 as usize;
        NODES[idx].as_ref().map(|n| n.value)
    }
}

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

pub fn get_prop(id: ThingId, key: PropKey) -> Option<PropValue> {
    get_thing(id).and_then(|(_, props)| {
        props
            .iter()
            .flatten()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v.clone())
    })
}

pub fn create_thing(kind: &'static str, props: &[(PropKey, PropValue)]) -> Option<ThingId> {
    create_thing_internal(kind, None, props, None)
}

pub(crate) fn create_thing_internal(
    kind: &'static str,
    explicit_kind_id: Option<ThingId>,
    props: &[(PropKey, PropValue)],
    owner_process: Option<abi::ProcessId>,
) -> Option<ThingId> {
    let kind_id = explicit_kind_id.unwrap_or_else(|| super::schema::ensure_kind_exists(kind));

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

        let slot = &mut slab.slots[idx as usize];
        slot.thing = Some(ThingNode {
            id,
            kind,
            kind_id,
            props: node_props,
            owner_process,
        });

        add_to_kind_index(id, kind_id);

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
            index_links::link_index_mut().insert(link);
            dispatch_event(&GraphEvent::LinkAdded(link));
        }

        Some(id)
    }
}

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
            index_links::link_index_ref().link(id).copied()
        } else {
            None
        };

        for (key, value) in props {
            let mut previous: Option<PropValue> = None;
            let mut found = false;
            for slot in node.props.iter_mut() {
                if let Some((k, old_val)) = slot {
                    if *k == *key {
                        let old_val_clone = old_val.clone();
                        previous = Some(old_val_clone.clone());

                        *slot = Some((*key, value.clone()));
                        found = true;

                        if is_prop_indexed(node.kind, key) {
                            remove_from_prop_index(id, key, &old_val_clone);
                            add_to_prop_index(id, key, value);
                        }
                        break;
                    }
                }
            }
            if !found {
                for slot in node.props.iter_mut() {
                    if slot.is_none() {
                        *slot = Some((*key, value.clone()));
                        found = true;
                        if is_prop_indexed(node.kind, key) {
                            add_to_prop_index(id, key, value);
                        }
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
                        if index_links::link_index_mut().remove(id) {
                            dispatch_event(&GraphEvent::LinkRemoved(prev));
                        }
                        index_links::link_index_mut().insert(next);
                        dispatch_event(&GraphEvent::LinkAdded(next));
                    }
                }
                (Some(prev), None) => {
                    if index_links::link_index_mut().remove(id) {
                        dispatch_event(&GraphEvent::LinkRemoved(prev));
                    }
                }
                (None, Some(next)) => {
                    index_links::link_index_mut().insert(next);
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
    index_links::link_index_ref().collect_incident_links(id, &mut links);
    links.sort_by_key(|t| t.0);
    links.dedup();

    for link_id in links {
        let _ = delete_link(link_id);
    }
}

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
            if let Some(link) = index_links::link_index_ref().link(id).copied() {
                if index_links::link_index_mut().remove(id) {
                    dispatch_event(&GraphEvent::LinkRemoved(link));
                }
            }
        } else {
            remove_incident_links(id);
        }

        remove_from_kind_index(id, thing.kind_id);
        for prop in thing.props.iter().flatten() {
            if is_prop_indexed(thing.kind, prop.0) {
                remove_from_prop_index(id, prop.0, &prop.1);
            }
        }

        dispatch_event(&GraphEvent::ThingDeleted {
            id,
            kind: thing.kind,
            kind_id: thing.kind_id,
        });

        slab.free_indices.push(idx as u32);

        true
    } else {
        false
    }
}

pub fn create_link(src: ThingId, pred: Predicate, dst: ThingId) -> Option<ThingId> {
    if let Some(existing) = index_links::link_index_ref()
        .links_from_pred(src, pred)
        .iter()
        .copied()
        .find(|id| match index_links::link_index_ref().link(*id) {
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

pub fn add_link(src: ThingId, pred: Predicate, dst: ThingId) -> bool {
    create_link(src, pred, dst).is_some()
}

pub fn delete_link(link_id: ThingId) -> bool {
    delete_thing(link_id)
}

pub fn remove_link(src: ThingId, pred: Predicate, dst: ThingId) -> bool {
    let candidate = index_links::link_index_ref()
        .links_from_pred(src, pred)
        .iter()
        .copied()
        .find(|id| match index_links::link_index_ref().link(*id) {
            Some(link) => link.dst == dst,
            None => false,
        });

    if let Some(id) = candidate {
        delete_thing(id)
    } else {
        false
    }
}

pub fn neighbors(from: ThingId, pred: Predicate, out: &mut [Option<ThingId>]) {
    for slot in out.iter_mut() {
        *slot = None;
    }

    for (slot, dst) in out
        .iter_mut()
        .zip(index_links::link_index_ref().neighbor_dsts(from, pred))
    {
        *slot = Some(dst);
    }
}

pub fn link_target_at(from: ThingId, pred: Predicate, index: usize) -> Option<ThingId> {
    index_links::link_index_ref()
        .neighbor_dsts(from, pred)
        .nth(index)
}

// Kernel-facing helpers
pub fn kernel_create_user_thing_for_process(
    proc: abi::ProcessId,
    kind: &'static str,
    props: &[(PropKey, PropValue)],
) -> Option<ThingId> {
    create_thing_internal(kind, None, props, Some(proc))
}

pub fn kernel_user_update_thing(
    proc: abi::ProcessId,
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
                return false;
            }

            let is_link = thing.kind == graph_kinds::KIND_LINK;
            let previous_link = if is_link {
                index_links::link_index_ref().link(id).copied()
            } else {
                None
            };

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
                            if index_links::link_index_mut().remove(id) {
                                dispatch_event(&GraphEvent::LinkRemoved(prev));
                            }
                            index_links::link_index_mut().insert(next);
                            dispatch_event(&GraphEvent::LinkAdded(next));
                        }
                    }
                    (Some(prev), None) => {
                        if index_links::link_index_mut().remove(id) {
                            dispatch_event(&GraphEvent::LinkRemoved(prev));
                        }
                    }
                    (None, Some(next)) => {
                        index_links::link_index_mut().insert(next);
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

pub fn cleanup_process_graph(_proc: abi::ProcessId) {
    // TODO
}
