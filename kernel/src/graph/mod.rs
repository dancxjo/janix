use abi::{ThingId, PropKey, PropValue, Predicate, Link, syscall_defs::SymbolId};
pub use abi::graph_ops::GraphEvent;
use alloc::vec::Vec;
use alloc::format;
use crate::symbols;

// ...

pub fn get_thing_kind(id: ThingId) -> Option<SymbolId> {
    store::things_slab().lock().as_ref().unwrap().get_thing_kind(id)
}

pub mod store;
pub mod schema;
pub mod events;
pub mod debug;
pub mod index_props;
pub mod index_links;
pub mod sink;

// Keep module-level constructor convenience
#[derive(Clone, Copy, Default, Debug)]
pub struct Graph;

impl Graph {
    #[inline]
    pub fn new() -> Self {
        Graph
    }

    #[inline]
    pub fn create_thing(
        &mut self,
        kind: SymbolId,
        props: &[(SymbolId, PropValue)],
    ) -> ThingId {
        create_thing(kind, Vec::from(props))
    }

    #[inline]
    pub fn update_thing(
        &mut self,
        id: ThingId,
        props: &[(SymbolId, PropValue)],
    ) -> bool {
        update_thing(id, Vec::from(props))
    }
    pub fn add_link(&mut self, src: ThingId, pred: Predicate, dst: ThingId) -> bool {
        add_link(src, pred, dst)
    }

    pub fn remove_link(&mut self, src: ThingId, pred: Predicate, dst: ThingId) -> bool {
        remove_link(src, pred, dst)
    }

    pub fn neighbors(&self, src: ThingId, pred: Predicate, out: &mut [Option<ThingId>]) {
        neighbors(src, pred, out)
    }
}

pub fn create_thing(kind: SymbolId, props: Vec<(SymbolId, PropValue)>) -> ThingId {
    let slab_guard = store::things_slab();
    let mut store = slab_guard.lock();
    store.as_mut().unwrap().create_thing(kind, props)
}

pub fn update_thing(id: ThingId, props: Vec<(SymbolId, PropValue)>) -> bool {
    let slab_guard = store::things_slab();
    let mut store = slab_guard.lock();
    store.as_mut().unwrap().update_thing(id, props)
}

pub fn add_link(src: ThingId, pred: Predicate, dst: ThingId) -> bool {
    let slab_guard = store::things_slab();
    let mut store = slab_guard.lock();
    store.as_mut().unwrap().add_link(src, dst, pred)
}

pub fn remove_link(_src: ThingId, _pred: Predicate, _dst: ThingId) -> bool {
    // TODO: Implement remove_link in store
    true
}

pub fn neighbors(src: ThingId, pred: Predicate, out: &mut [Option<ThingId>]) {
    let store_guard = store::things_slab().lock();
    let store_ref = store_guard.as_ref().unwrap();
    // Proxy to store implementation
    // store.get_link(src, pred, idx)
    // Naively fill buffer
    for i in 0..out.len() {
        out[i] = store_ref.get_link(src, pred, i);
    }
}

// Helper for syscall
pub fn next_thing_of_kind_sym(kind: SymbolId, start_after: ThingId) -> Option<ThingId> {
    // iterate store
    let slab_guard = store::things_slab().lock();
    let slab = slab_guard.as_ref().unwrap();
    
    let mut best: Option<ThingId> = None;
    
    for (_id_val, node) in slab.things.iter() {
        if node.id.0 > start_after.0 && node.kind == kind {
            if let Some(current_best) = best {
                if node.id.0 < current_best.0 {
                    best = Some(node.id);
                }
            } else {
                 best = Some(node.id);
            }
        }
    }

    best
}

pub fn next_thing_of_kind(kind: SymbolId, start_after: ThingId) -> Option<ThingId> {
    next_thing_of_kind_sym(kind, start_after)
}

// Legacy alias if needed
pub fn query_node(id: ThingId) -> Option<alloc::vec::Vec<u8>> {
    // Used by Debug console or queries.
    // This used to return byte dump.
    // We can return debug string for now.
    with_thing(id, |t| {
        format!("{:?}", t).into_bytes()
    })
}

// Helper for link target at index
pub fn link_target_at(src: ThingId, pred: Predicate, idx: usize) -> Option<ThingId> {
    let store_guard = store::things_slab().lock();
    let store = store_guard.as_ref().unwrap();
    store.get_link(src, pred, idx)
}

// Validation helper used by lib.rs
pub fn validate_props(_kind: SymbolId, _props: &[(SymbolId, PropValue)]) -> Result<(), &'static str> {
    // TODO: Implement schema validation against prop types
    Ok(())
}

// Helper for iteration
pub fn iter_things<F>(f: F)
where
    F: FnMut(&store::ThingNode),
{
    let guard = store::things_slab().lock();
    let store = guard.as_ref().unwrap();
    // store.things.values().for_each(f); 
    // values() returns iterator.
    // Mutable closure? values() gives &ThingNode. 
    // F is FnMut.
    // for_each consumes iterator.
    store.things.values().for_each(f);
}

pub fn get_prop(id: ThingId, key_str: &str) -> Option<PropValue> {
    // This is slow: interns string to look up.
    // But `sched_graph.rs` uses it with string literal keys.
    // Ideally we intern ONCE.
    // But here we must intern to match.
    // Wait, symbols::intern returns SymbolId (u32, copy).
    // So distinct calls are fine but mutex overhead.
    let key = symbols::intern(key_str);
    let guard = store::things_slab().lock();
    let store = guard.as_ref().unwrap();
    store.get_prop(id, key)
}

pub fn with_thing<F, R>(id: ThingId, f: F) -> Option<R>
where
    F: FnOnce(&store::ThingNode) -> R,
{
    let guard = store::things_slab().lock();
    let store = guard.as_ref().unwrap();
    store.things.get(&id).map(|n| f(n))
}
