//! Graph store
//!
//! Canonical Thing/Relationship storage.
//! Enforces:
//! - Every Thing has a UUID and Kind.
//! - Relationships are typed Edges.
//! - O(1) traversal via in-memory indexing.

use abi::ids::{RelationshipId, SymbolId, ThingId};
use abi::bodies::ThingEnvelopeV1;
use spin::Mutex;
use alloc::collections::BTreeMap;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use abi::ids::crc64;
use thing_codec::GraphClient;

static GRAPH_STORE: Mutex<Option<GraphStore>> = Mutex::new(None);

#[derive(Clone, Debug)]
pub struct Relationship {
    pub id: RelationshipId,
    pub kind: SymbolId,
    pub from: ThingId,
    pub to: ThingId,
}

#[derive(Clone, Debug)]
pub struct ThingHeader {
    pub id: ThingId,
    pub kind: SymbolId,
    pub integrity_digest: u64,
}


#[derive(Clone, Debug)]
pub struct Thing {
    pub header: ThingHeader,
    pub body: Vec<u8>,
}

pub struct GraphStore {
    pub things: BTreeMap<ThingId, Thing>,
    pub relationships: BTreeMap<RelationshipId, Relationship>,
    pub name_index: BTreeMap<SymbolId, ThingId>,
    pub next_id: u64,
    pub watchers: BTreeMap<ThingId, Vec<ThingId>>,
    pub pending_events: BTreeMap<ThingId, VecDeque<ThingId>>,
}

impl GraphStore {
    pub fn new() -> Self {
        Self {
            things: BTreeMap::new(),
            relationships: BTreeMap::new(),
            name_index: BTreeMap::new(),
            next_id: 100,
            watchers: BTreeMap::new(),
            pending_events: BTreeMap::new(),
        }
    }

    pub fn create_thing(&mut self, kind: SymbolId) -> Result<ThingId, i32> {
        let id = ThingId(self.next_id as u128);
        self.next_id += 1;
        let thing = Thing {
            header: ThingHeader {
                id,
                kind,
                integrity_digest: 0,
            },
            body: Vec::new(),
        };
        self.things.insert(id, thing);
        Ok(id)
    }

    pub fn create_relationship(
        &mut self,
        kind: SymbolId,
        from: ThingId,
        to: ThingId,
    ) -> Result<RelationshipId, i32> {
        let id = ThingId(self.next_id as u128);
        self.next_id += 1;
        let rel = Relationship { id, kind, from, to };
        self.relationships.insert(id, rel);

        if let Some(watchers) = self.watchers.get(&from) {
            for &watcher in watchers {
                self.pending_events
                    .entry(watcher)
                    .or_default()
                    .push_back(from);
            }
        }

        Ok(id)
    }

    pub fn relationships_from(&self, from: ThingId) -> Vec<RelationshipId> {
        self.relationships
            .values()
            .filter(|r| r.from == from)
            .map(|r| r.id)
            .collect()
    }

    pub fn set_body(&mut self, id: ThingId, body: &[u8]) -> Result<(), i32> {
        if let Some(thing) = self.things.get_mut(&id) {
            if !body.is_empty() {
                if let Err(_) = validate_envelope(body) {
                    return Err(abi::syscall::err::ERR_INVALID_THING_BODY);
                }
                
                let digest = crc64(body);
                thing.header.integrity_digest = digest;
                thing.body = body.to_vec();
            } else {
                thing.body = Vec::new();
                thing.header.integrity_digest = 0;
            }
            Ok(())
        } else {
            Err(abi::syscall::err::ENOENT)
        }
    }

    pub fn set_payload(&mut self, id: ThingId, payload: &[u8]) -> Result<(), i32> {
        if validate_envelope(payload).is_ok() {
            self.set_body(id, payload)
        } else {
            let header = ThingEnvelopeV1 {
                magic: ThingEnvelopeV1::MAGIC,
                env_version: ThingEnvelopeV1::VERSION,
                flags: 0,
                kind: 0,
                schema_hash: 0,
                schema_version: 0,
                schema_str_len: 0,
                payload_format: 0,
                reserved0: 0,
                payload_len: payload.len() as u32,
                body_len: (core::mem::size_of::<ThingEnvelopeV1>() + payload.len()) as u32,
                integrity: 0,
            };
            let mut bytes = Vec::with_capacity(header.body_len as usize);
            unsafe {
                let ptr = &header as *const _ as *const u8;
                bytes.extend_from_slice(core::slice::from_raw_parts(ptr, core::mem::size_of::<ThingEnvelopeV1>()));
            }
            bytes.extend_from_slice(payload);
            self.set_body(id, &bytes)
        }
    }

    pub fn get_body(&self, id: ThingId) -> Option<&[u8]> {
        self.things.get(&id).map(|t| t.body.as_slice())
    }

    pub fn register_name(&mut self, id: ThingId, name: SymbolId) {
        self.name_index.insert(name, id);
    }

    pub fn find_by_name(&self, name: SymbolId) -> Option<ThingId> {
        self.name_index.get(&name).cloned()
    }

    pub fn watch(&mut self, watcher: ThingId, target: ThingId) {
        self.watchers.entry(target).or_default().push(watcher);
    }

    fn dequeue(&mut self, watcher: ThingId) -> Option<ThingId> {
        self.pending_events
            .get_mut(&watcher)
            .and_then(|q| q.pop_front())
    }
}

impl GraphClient for GraphStore {
    fn create_thing(&mut self, kind: SymbolId) -> Result<ThingId, i32> {
        self.create_thing(kind)
    }

    fn set_body(&mut self, id: ThingId, body: &[u8]) -> Result<(), i32> {
        self.set_body(id, body)
    }

    fn get_body(&self, id: ThingId) -> Result<Vec<u8>, i32> {
        self.get_body(id).map(|b| b.to_vec()).ok_or(abi::syscall::err::ENOENT)
    }
}

pub fn validate_envelope(bytes: &[u8]) -> Result<(), i32> {
    if bytes.len() < core::mem::size_of::<ThingEnvelopeV1>() {
        return Err(abi::syscall::err::ERR_INVALID_THING_BODY);
    }

    let env = unsafe { &*(bytes.as_ptr() as *const ThingEnvelopeV1) };

    if env.magic != ThingEnvelopeV1::MAGIC {
        return Err(abi::syscall::err::ERR_INVALID_THING_BODY);
    }

    if env.env_version != ThingEnvelopeV1::VERSION {
        return Err(abi::syscall::err::ERR_INVALID_THING_BODY);
    }

    if env.body_len as usize != bytes.len() {
        return Err(abi::syscall::err::ERR_INVALID_THING_BODY);
    }

    Ok(())
}

pub fn init() {
    let store = GraphStore::new();
    *GRAPH_STORE.lock() = Some(store);
}

static mut IRQ_DISABLE: Option<fn() -> usize> = None;
static mut IRQ_RESTORE: Option<fn(usize)> = None;

pub fn register_irq_callbacks(disable: fn() -> usize, restore: fn(usize)) {
    unsafe {
        IRQ_DISABLE = Some(disable);
        IRQ_RESTORE = Some(restore);
    }
}

fn lock_store_irq() -> (spin::MutexGuard<'static, Option<GraphStore>>, usize) {
    let flags = unsafe {
        if let Some(disable) = IRQ_DISABLE {
            disable()
        } else {
            0
        }
    };
    (GRAPH_STORE.lock(), flags)
}

fn restore_irq(flags: usize) {
    unsafe {
        if let Some(restore) = IRQ_RESTORE {
            restore(flags);
        }
    }
}

fn try_lock_store_irq() -> Option<(spin::MutexGuard<'static, Option<GraphStore>>, usize)> {
    let flags = unsafe {
        if let Some(disable) = IRQ_DISABLE {
            disable()
        } else {
            0
        }
    };
    
    if let Some(guard) = GRAPH_STORE.try_lock() {
        Some((guard, flags))
    } else {
        restore_irq(flags);
        None
    }
}

pub fn is_initialized() -> bool {
    // Blocking check
    let (guard, flags) = lock_store_irq();
    let res = guard.is_some();
    drop(guard);
    restore_irq(flags);
    res
}

/// Non-blocking check for logging. Returns false if locked or not initialized.
pub fn is_ready_for_logging() -> bool {
    if let Some((guard, flags)) = try_lock_store_irq() {
        let res = guard.is_some();
        drop(guard);
        restore_irq(flags);
        res
    } else {
        false
    }
}

pub fn with_store<F, R>(f: F) -> R 
where F: FnOnce(&mut GraphStore) -> R
{
    let (mut guard, flags) = lock_store_irq();
    let res = f(guard.as_mut().expect("GraphStore not initialized"));
    drop(guard);
    restore_irq(flags);
    res
}

pub fn thing_create(kind: SymbolId) -> ThingId {
    with_store(|s| s.create_thing(kind).expect("failed to create thing"))
}

pub fn thing_exists(id: ThingId) -> bool {
    with_store(|s| s.things.contains_key(&id))
}

pub fn relationship_create(kind: SymbolId, from: ThingId, to: ThingId) -> RelationshipId {
    with_store(|s| s.create_relationship(kind, from, to).expect("failed to create rel"))
}

pub fn relationships_from(from: ThingId) -> Vec<RelationshipId> {
    with_store(|s| s.relationships_from(from))
}

pub fn get_relationship(id: RelationshipId) -> Option<Relationship> {
    with_store(|s| s.relationships.get(&id).cloned())
}

pub fn get_thing_header(id: ThingId) -> Option<ThingHeader> {
    with_store(|s| s.things.get(&id).map(|t| t.header.clone()))
}

pub fn thing_set_body(id: ThingId, body: &[u8]) -> Result<(), i32> {
    with_store(|s| s.set_body(id, body))
}

pub fn get_body(id: ThingId) -> Option<Vec<u8>> {
    with_store(|s| s.get_body(id).map(|b| b.to_vec()))
}

pub fn get_payload(id: ThingId) -> Option<Vec<u8>> {
    get_body(id)
}

pub fn thing_set_inline_payload(id: ThingId, payload: &[u8]) {
    let _ = with_store(|s| s.set_payload(id, payload));
}

pub fn thing_register_name(id: ThingId, name: SymbolId) {
    with_store(|s| s.register_name(id, name))
}

pub fn find_thing_by_name(name: SymbolId) -> Option<ThingId> {
    with_store(|s| s.find_by_name(name))
}

pub fn watch(watcher: ThingId, target: ThingId) {
    with_store(|s| s.watch(watcher, target))
}

pub fn dequeue_event(watcher: ThingId) -> Option<ThingId> {
    with_store(|s| s.dequeue(watcher))
}
