extern crate alloc;
use crate::symbols::hash::fnv1a64;
use abi::SymbolId;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

#[derive(Debug)]
pub enum SymbolError {
    InvalidUtf8,
    Collision { id: SymbolId },
    NotFound,
    Store,
    BuiltinMismatch { expected: SymbolId, got: SymbolId },
}

pub struct SymbolTable {
    map: BTreeMap<SymbolId, Vec<u8>>,
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn seed_builtin(&mut self, text: &'static str) -> Result<SymbolId, SymbolError> {
        let id = SymbolId(fnv1a64(text));
        self.insert_exact(id, text.as_bytes())
    }

    pub fn intern(&mut self, text: &str) -> Result<SymbolId, SymbolError> {
        let id = SymbolId(fnv1a64(text));
        self.insert_exact(id, text.as_bytes())
    }

    fn insert_exact(&mut self, id: SymbolId, bytes: &[u8]) -> Result<SymbolId, SymbolError> {
        // validate utf8 (text must be utf8)
        core::str::from_utf8(bytes).map_err(|_| SymbolError::InvalidUtf8)?;

        if let Some(existing) = self.map.get(&id) {
            if existing.as_slice() == bytes {
                return Ok(id);
            }
            return Err(SymbolError::Collision { id });
        }

        self.map.insert(id, bytes.to_vec());
        Ok(id)
    }

    pub fn resolve(&self, id: SymbolId) -> Option<&[u8]> {
        self.map.get(&id).map(|v| v.as_slice())
    }

    pub fn load_from_store(
        &mut self,
        store: &mut dyn crate::symbols::store::SymbolStore,
    ) -> Result<(), SymbolError> {
        let entries = store.load().map_err(|_| SymbolError::Store)?;
        for (id, bytes) in entries {
            self.insert_exact(id, &bytes)?;
        }
        Ok(())
    }

    pub fn persist_to_store(
        &self,
        store: &mut dyn crate::symbols::store::SymbolStore,
    ) -> Result<(), SymbolError> {
        // produce deterministic snapshot
        let mut snapshot: Vec<(SymbolId, &[u8])> = Vec::new();
        for (id, bytes) in self.map.iter() {
            snapshot.push((*id, bytes.as_slice()));
        }
        store.save(&snapshot).map_err(|_| SymbolError::Store)?;
        Ok(())
    }
}
