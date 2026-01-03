use anyhow::{Result};
use graph::symbols::SymbolId;
use graph::store::PlaceStore;
use abi::ids::ThingId;
use std::path::Path;

pub struct ToolingStore {
    pub inner: PlaceStore,
}

impl ToolingStore {
    pub fn load_from_json(_path: &Path) -> Result<Self> {
        // Mock for now
        Ok(Self {
            inner: PlaceStore::new(),
        })
    }

    pub fn find_thing_by_name(&self, _name: &str) -> Option<ThingId> {
        None
    }

    pub fn get_relationship_count(&self, _thing: ThingId, _pred: SymbolId) -> usize {
        0
    }
}
