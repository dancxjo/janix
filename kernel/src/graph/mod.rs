pub use self::ops::GraphEvent;
use crate::symbols;
use abi::{Link, Predicate, ThingId, syscall_defs::SymbolId};
use alloc::format;
use alloc::vec::Vec;
use thing_models::{PropKey, PropValue};

// ...

pub fn get_thing_kind(id: ThingId) -> Option<SymbolId> {
    with_store(|store| store.get_thing_kind(id))
}

pub fn get_revision() -> u64 {
    with_store(|store| store.get_revision())
}

#[cfg(target_arch = "x86_64")]
use x86_64::instructions::interrupts;

#[cfg(not(target_arch = "x86_64"))]
mod interrupts {
    #[inline]
    pub fn without_interrupts<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        f()
    }
}

fn with_store_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut store::GraphStore) -> R,
{
    interrupts::without_interrupts(|| {
        let mut guard = store::things_slab().lock();
        let store = guard.as_mut().expect("GraphStore not initialized");
        f(store)
    })
}

fn with_store<F, R>(f: F) -> R
where
    F: FnOnce(&store::GraphStore) -> R,
{
    interrupts::without_interrupts(|| {
        let guard = store::things_slab().lock();
        let store = guard.as_ref().expect("GraphStore not initialized");
        f(store)
    })
}

pub mod debug;
pub mod events;
pub mod index_links;
pub mod index_props;
pub mod ops;
pub mod schema;
pub mod security;
pub mod actualizer;
pub mod sink;
pub mod store;

// Keep module-level constructor convenience
#[derive(Clone, Copy, Default, Debug)]
pub struct Graph;

impl Graph {
    #[inline]
    pub fn new() -> Self {
        Graph
    }

    #[inline]
    pub fn create_thing(&mut self, kind: SymbolId, props: &[(SymbolId, PropValue)]) -> ThingId {
        create_thing(kind, Vec::from(props))
    }

    #[inline]
    pub fn update_thing(&mut self, id: ThingId, props: &[(SymbolId, PropValue)]) -> bool {
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


pub fn apply_mutation(actor: security::Actor, mutation: security::Mutation) -> Result<security::MutationResult, &'static str> {
    security::check_policy(&actor, &mutation)?;
    match mutation {
        security::Mutation::CreateThing { kind, props } => {
             // Validate props against schema before creating
            if let Err(e) = validate_props(kind, props) {
                return Err(e);
            }
            let props_vec = Vec::from(props);
            let id = with_store_mut(|store| store.create_thing(kind, props_vec));
            debug::print_thing_created(id, kind, props);
            events::dispatch_event(&GraphEvent::ThingCreated(id));
            Ok(security::MutationResult::Created(id))
        }
        security::Mutation::UpdateThing { id, props } => {
            let props_vec = Vec::from(props);
            let updated = with_store_mut(|store| store.update_thing(id, props_vec));
            if updated {
                events::dispatch_event(&GraphEvent::ThingUpdated(id));
            }
            Ok(security::MutationResult::Updated(updated))
        }
        security::Mutation::AddLink { src, pred, dst } => {
            let success = with_store_mut(|store| store.add_link(src, dst, pred));
            if success {
                debug::print_link_created(src, pred, dst);
            }
            Ok(security::MutationResult::Linked(success))
        }
        security::Mutation::RemoveLink { src, pred, dst } => {
             let success = with_store_mut(|store| store.remove_link(src, dst, pred));
            if success {
                debug::print_link_removed(src, pred, dst);
            }
            Ok(security::MutationResult::Unlinked(success))
        }
        security::Mutation::DeclareSchema { kind, fingerprint: _, definition } => {
             let (desc, props, indexed) = schema::decode_schema(definition)?;
             let result = schema::register_schema(kind, desc, props, indexed);
             match result {
                 Ok(abi::SchemaRegistryOutcome::Created) | Ok(abi::SchemaRegistryOutcome::AlreadyRegisteredSame) => {
                     Ok(security::MutationResult::Declared)
                 }
                 Ok(abi::SchemaRegistryOutcome::Conflict) => {
                     Err("Schema declaration conflict")
                 }
                 Err(e) => Err(e),
             }
        }
    }
}

pub fn create_thing(kind: SymbolId, props: Vec<(SymbolId, PropValue)>) -> ThingId {
    let mutation = security::Mutation::CreateThing {
        kind,
        props: &props,
    };
    match apply_mutation(security::Actor::Kernel, mutation) {
        Ok(security::MutationResult::Created(id)) => id,
        Ok(_) => panic!("create_thing: wrong result type"),
        Err(e) => panic!("create_thing failed: {}", e),
    }
}

pub fn update_thing(id: ThingId, props: Vec<(SymbolId, PropValue)>) -> bool {
    let mutation = security::Mutation::UpdateThing {
        id,
        props: &props,
    };
    match apply_mutation(security::Actor::Kernel, mutation) {
        Ok(security::MutationResult::Updated(b)) => b,
        Ok(_) => panic!("update_thing: wrong result type"),
        Err(e) => {
            // update_thing wrapper used to return bool, so if it fails policy/validation we return false?
            // But Kernel shouldn't fail policy.
            crate::log(&format!("update_thing failed: {}", e));
            false
        }
    }
}

pub fn add_link(src: ThingId, pred: Predicate, dst: ThingId) -> bool {
    let mutation = security::Mutation::AddLink { src, pred, dst };
    match apply_mutation(security::Actor::Kernel, mutation) {
        Ok(security::MutationResult::Linked(b)) => b,
        Ok(_) => panic!("add_link: wrong result type"),
        Err(e) => { 
             crate::log(&format!("add_link failed: {}", e));
             false
        }
    }
}

pub fn remove_link(src: ThingId, pred: Predicate, dst: ThingId) -> bool {
    let mutation = security::Mutation::RemoveLink { src, pred, dst };
    match apply_mutation(security::Actor::Kernel, mutation) {
        Ok(security::MutationResult::Unlinked(b)) => b,
        Ok(_) => panic!("remove_link: wrong result type"),
        Err(e) => {
              crate::log(&format!("remove_link failed: {}", e));
              false
        }
    }
}

pub fn neighbors(src: ThingId, pred: Predicate, out: &mut [Option<ThingId>]) {
    with_store(|store| {
        for i in 0..out.len() {
            out[i] = store.get_link(src, pred, i);
        }
    })
}

// Helper for syscall
pub fn next_thing_of_kind_sym(kind: SymbolId, start_after: ThingId) -> Option<ThingId> {
    with_store(|store| store.next_thing_of_kind(kind, start_after))
}

pub fn next_thing_of_kind(kind: SymbolId, start_after: ThingId) -> Option<ThingId> {
    next_thing_of_kind_sym(kind, start_after)
}

// Legacy alias if needed
pub fn query_node(id: ThingId) -> Option<alloc::vec::Vec<u8>> {
    // Used by Debug console or queries.
    // This used to return byte dump.
    // We can return debug string for now.
    with_thing(id, |t| format!("{:?}", t).into_bytes())
}

// Helper for link target at index
pub fn link_target_at(src: ThingId, pred: Predicate, idx: usize) -> Option<ThingId> {
    with_store(|store| store.get_link(src, pred, idx))
}

// Validation helper used by lib.rs
pub fn validate_props(kind: SymbolId, props: &[(SymbolId, PropValue)]) -> Result<(), &'static str> {
    schema::validate_props(kind, props)
}

// Helper for iteration
pub fn iter_things<F>(mut f: F)
where
    F: FnMut(&store::ThingNode),
{
    // FnMut requires exclusive access to F if we call it multiple times.
    // without_interrupts handles closure.
    with_store(|store| {
        store.things.values().for_each(|n| f(n));
    });
}

pub fn get_prop(id: ThingId, key_str: &str) -> Option<PropValue> {
    // This is slow: interns string to look up.
    let key = symbols::intern(key_str);
    with_store(|store| store.get_prop(id, key))
}

pub fn with_thing<F, R>(id: ThingId, f: F) -> Option<R>
where
    F: FnOnce(&store::ThingNode) -> R,
{
    with_store(|store| store.things.get(&id).map(|n| f(n)))
}
