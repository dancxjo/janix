use alloc::vec::Vec;
use hashbrown::HashMap;
use spin::Mutex;

use abi::{ThingId, PropType, syscall_defs::SymbolId};

pub struct Schema {
    pub kind: SymbolId,
    pub description: SymbolId,
    pub props: HashMap<SymbolId, PropType>,
    pub indexed_props: Vec<SymbolId>,
}

static SCHEMAS: Mutex<Option<HashMap<SymbolId, Schema>>> = Mutex::new(None);

pub fn init() {
    *SCHEMAS.lock() = Some(HashMap::new());
}

pub fn register_schema(
    kind: SymbolId, 
    description: SymbolId, 
    props: Vec<(SymbolId, PropType)>,
    indexed_props: Vec<SymbolId>
) -> Result<(), &'static str> {
    let mut guard = SCHEMAS.lock();
    let schemas = guard.as_mut().expect("Schemas not initialized");
    if schemas.contains_key(&kind) {
        return Err("Schema already registered");
    }

    let mut prop_map = HashMap::new();
    for (name, pt) in props {
        prop_map.insert(name, pt);
    }

    schemas.insert(kind, Schema {
        kind,
        description,
        props: prop_map,
        indexed_props,
    });
    Ok(())
}

pub fn get_schema_props(kind: SymbolId) -> Option<Vec<(SymbolId, PropType)>> {
    SCHEMAS.lock().as_ref().expect("Schemas not initialized").get(&kind).map(|s| {
        s.props.iter().map(|(k, v)| (*k, *v)).collect()
    })
}

pub fn ensure_kind_exists(kind: SymbolId) -> ThingId {
    // Legacy support, perhaps remove? 
    // ThingId derived from hash of SymbolId? Or just use SymbolId as "kind_id" effectively?
    // In new design, kind is just a SymbolId.
    // Return dummy ThingId if needed by old APIs, but ideally we switch to SymbolId.
    ThingId(kind.0 as u64)
}

pub fn is_prop_indexed(kind: SymbolId, key: SymbolId) -> bool {
    SCHEMAS.lock().as_ref().expect("Schemas not initialized").get(&kind)
        .map(|s| s.indexed_props.contains(&key))
        .unwrap_or(false)
}

// Helper to validate props against schema
pub fn validate_props(_kind: SymbolId, _props: &[(SymbolId, abi::PropValue)]) -> Result<(), &'static str> {
    // TODO: Implement validation
    Ok(())
}

pub fn add_to_kind_index(_id: ThingId, _kind: SymbolId) {
    // Stub
}

pub fn remove_from_kind_index(_id: ThingId, _kind: SymbolId) {
    // Stub
}

pub fn get_key_from_id(_kind: SymbolId, key_id: u32) -> Option<alloc::string::String> {
    // Ignoring kind for now assuming global symbols.
    // Return String as symbols::resolve might restrict lifetime or return String.
    // symbols::resolve returns Option<&str> (Step 700). 
    // But serialize.rs expects something compatible with &str in encode.
    // Let's check symbols::resolve again.
    // Step 700: `pub fn resolve(id: SymbolId) -> Option<String>`. 
    // It returns String!
    crate::symbols::resolve(SymbolId(key_id))
}
