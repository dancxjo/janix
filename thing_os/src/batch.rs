#[cfg(target_os = "none")]
use alloc::collections::BTreeMap;
#[cfg(not(target_os = "none"))]
use std::collections::BTreeMap;

#[cfg(target_os = "none")]
use alloc::vec::Vec;
#[cfg(not(target_os = "none"))]
use std::vec::Vec;

#[cfg(target_os = "none")]
use alloc::boxed::Box;
#[cfg(not(target_os = "none"))]
use std::boxed::Box;
use alloc::string::ToString;

use abi::{KernelRequest, KernelResponse, PropKey, PropValue, ThingId};
use abi::wire::common::{UserSlice, UserPtr};
use abi::wire::graph::{BatchUpdateEntry, WireProp, WirePropValue};
use crate::syscalls::{syscall, sys_symbol_intern};

pub struct Batcher {
    updates: BTreeMap<ThingId, BTreeMap<PropKey, PropValue>>,
}

impl Batcher {
    pub fn new() -> Self {
        Self {
            updates: BTreeMap::new(),
        }
    }

    /// Queue an update. If the property is already pending, it overwrites the old value (coalescing).
    pub fn update(&mut self, id: ThingId, key: PropKey, val: PropValue) {
        let props = self.updates.entry(id).or_insert_with(BTreeMap::new);
        props.insert(key, val);
    }

    /// Flush all queued updates as a single system call.
    pub fn flush(&mut self) -> bool {
        if self.updates.is_empty() {
            return true;
        }

        let mut batch_entries = Vec::new();

        for (id, props_map) in &self.updates {
            let mut wire_props = Vec::with_capacity(props_map.len());
            
            for (k, v) in props_map {
                let key_sym = sys_symbol_intern(k);
                let val_wire = match v {
                    PropValue::U64(val) => WirePropValue::u64(*val),
                    PropValue::I64(val) => WirePropValue::i64(*val),
                    PropValue::Bool(val) => WirePropValue::bool(*val),
                    PropValue::Str(val) => {
                         let sym = sys_symbol_intern(val);
                         WirePropValue::sym(sym)
                    }
                    PropValue::Symbol(sym) => WirePropValue::sym(*sym),
                    PropValue::Blob(blob) => {
                        let ptr = blob.as_ptr() as u64;
                        let len = blob.len() as u64;
                        WirePropValue::blob(ptr, len)
                    }
                };
                
                wire_props.push(WireProp {
                    key: key_sym,
                    value: val_wire,
                    _pad: 0,
                });
            }
            
            let props_slice = Box::leak(wire_props.into_boxed_slice());

            batch_entries.push(BatchUpdateEntry {
                id: *id,
                props_ptr: UserPtr::new(props_slice.as_ptr() as u64),
                props_len: props_slice.len() as u64,
            });
        }

        // Clear updates now that we've captured them
        self.updates.clear();

        let updates_slice = Box::leak(batch_entries.into_boxed_slice());

        match syscall(KernelRequest::ThingBatchUpdate {
            updates: UserSlice::from_slice(updates_slice),
        }) {
            KernelResponse::Success { .. } => true,
            _ => false,
        }
    }
}
