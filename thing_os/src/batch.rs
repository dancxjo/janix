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

use abi::{BatchUpdateEntry, KernelRequest, KernelResponse, PropKey, PropValue, ThingId};
use runtime::Sys;

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
    pub fn flush(&mut self, sys: &impl Sys) -> bool {
        if self.updates.is_empty() {
            return true;
        }

        let mut batch_entries = Vec::new();

        for (id, props_map) in &self.updates {
            let mut props_vec: Vec<(PropKey, PropValue)> = Vec::new();
            for (k, v) in props_map {
                props_vec.push((k.clone(), v.clone()));
            }
            let props_slice = Box::leak(props_vec.into_boxed_slice());

            batch_entries.push(BatchUpdateEntry {
                id: *id,
                props: props_slice,
            });
        }

        // Clear updates now that we've captured them
        self.updates.clear();

        let updates_slice = Box::leak(batch_entries.into_boxed_slice());

        match sys.syscall(KernelRequest::ThingBatchUpdate {
            updates: updates_slice,
        }) {
            KernelResponse::Success { .. } => true,
            _ => false,
        }
    }
}
