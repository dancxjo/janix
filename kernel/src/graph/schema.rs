
use alloc::vec::Vec;
use hashbrown::HashMap;
use spin::Mutex;
use alloc::format;

use abi::{ThingId, PropType, PropValue, syscall_defs::SymbolId};

pub struct Schema {
    pub kind: SymbolId,
    pub description: SymbolId,
    pub props: HashMap<SymbolId, PropType>,
    pub indexed_props: Vec<SymbolId>,
    pub fingerprint: u64,
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
) -> Result<abi::SchemaRegistryOutcome, &'static str> {
    let mut guard = SCHEMAS.lock();
    let schemas = guard.as_mut().expect("Schemas not initialized");
    
    // Calculate fingerprint
    // For stability, we must sort the props by key before hashing, or use an order-independent hash.
    // Since we receive a Vec, let's sort a view of it.
    let mut sorted_props = props.clone();
    sorted_props.sort_by(|a, b| a.0.0.cmp(&b.0.0));
    
    let mut hasher = hashbrown::hash_map::DefaultHashBuilder::default().build_hasher();
    use core::hash::{Hash, Hasher, BuildHasher};
    
    // Hash relevant parts
    kind.hash(&mut hasher);
    // Description is technically part of the schema definition, but if we want to be permissive about desc changes
    // we might exclude it from the "strict" fingerprint, or track it separately?
    // User request says: "AlreadyRegisteredSame: success (no-op) if the existing schema is byte-for-byte identical."
    // This implies description too.
    description.hash(&mut hasher); 
    
    for (k, t) in &sorted_props {
        k.hash(&mut hasher);
        t.hash(&mut hasher);
    }
    for k in &indexed_props {
        k.hash(&mut hasher);
    }
    
    let fingerprint = hasher.finish();

    if let Some(existing) = schemas.get(&kind) {
        if existing.fingerprint == fingerprint {
            return Ok(abi::SchemaRegistryOutcome::AlreadyRegisteredSame);
        }
        
        // Fingerprint mismatch. Could be desc change or props change.
        // If it's just description, maybe we can be permissive? 
        // "ConflictDifferent: a deterministic error if the kind exists but the schema differs."
        // Strict interpretation: ANY difference is a conflict.
        
        // Let's log details
        let d1 = crate::symbols::resolve(existing.description).unwrap_or(alloc::string::String::from("?"));
        let d2 = crate::symbols::resolve(description).unwrap_or(alloc::string::String::from("?"));
        crate::log::log_message(&format!(
            "Schema Conflict for kind {:?}. Existing fp={:x}, New fp={:x}. Desc '{:?}' vs '{:?}'", 
            kind, existing.fingerprint, fingerprint, d1, d2
        ));
        
        return Ok(abi::SchemaRegistryOutcome::Conflict);
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
        fingerprint,
    });
    Ok(abi::SchemaRegistryOutcome::Created)
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
pub fn validate_props(kind: SymbolId, props: &[(SymbolId, abi::PropValue)]) -> Result<(), &'static str> {
    let guard = SCHEMAS.lock();
    let schemas = guard.as_ref().expect("Schemas not initialized");

    // If no schema exists for the kind, we default to allowing (schemaless).
    let schema = match schemas.get(&kind) {
        Some(s) => s,
        None => return Ok(()),
    };

    for (key, value) in props {
        if let Some(expected_type) = schema.props.get(key) {
            let valid = match (expected_type, value) {
                (PropType::U64, PropValue::U64(_)) => true,
                (PropType::I64, PropValue::I64(_)) => true,
                (PropType::Bool, PropValue::Bool(_)) => true,
                (PropType::Str, PropValue::Str(_)) => true,
                (PropType::Blob, PropValue::Blob(_)) => true,
                (PropType::Symbol, PropValue::Symbol(_)) => true,
                _ => false,
            };

            if !valid {
                 return Err("Schema validation failed: property type mismatch");
            }
        } else {
             // Strict schema: Unknown property is an error.
             return Err("Schema validation failed: unknown property");
        }
    }

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
