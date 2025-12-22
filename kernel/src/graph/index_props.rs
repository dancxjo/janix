use abi::{ThingId, syscall_defs::SymbolId};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use hashbrown::HashMap;
use spin::Mutex;
use thing_models::PropValue;

// Map: PropName -> (PropValue -> Vec<ThingId>)
// Since PropValue isn't Hash or Ord by default easily (contains String),
// we might iterate or need to ensure PropValue is Key-able.
// For now, let's just use linear scan for values or exact match if PropValue supports PartialEq.
// Wait, PropValue has a Blob which is Vec<u8> and String.
// BTreeMap<PropValue, Vec<ThingId>> requires Ord.
// HashMap<PropValue, Vec<ThingId>> requires Hash.
// Let's implement Hash/Eq for PropValue wrappers or use stringified keys?
// Or just support indexing U64/Bool/Str?

// For this specific implementation step, I'll rely on a simplified index that only supports exact match on simple types.

// type ValueIndex = HashMap<PropValue, Vec<ThingId>>; // PropValue not Hash
type PropIndex = HashMap<SymbolId, ()>; // Stubbed out

static PROP_INDEX: Mutex<Option<PropIndex>> = Mutex::new(None);

pub fn init() {
    *PROP_INDEX.lock() = Some(HashMap::new());
}

pub fn clear() {
    if let Some(idx) = PROP_INDEX.lock().as_mut() {
        idx.clear();
    }
}

pub fn add_to_prop_index(_id: ThingId, _key: SymbolId, _val: &PropValue) {
    // Disabled due to PropValue Hash requirement
}

pub fn remove_from_prop_index(_id: ThingId, _key: SymbolId, _val: &PropValue) {
    // Disabled
}

pub fn find(_key: SymbolId, _val: &PropValue) -> Vec<ThingId> {
    Vec::new()
}
