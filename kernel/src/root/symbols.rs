use abi::symbols::{SymbolError, SymbolId};
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Interner {
    // Forward lookup: SymbolId -> String
    // We store the string data here.
    names: Vec<String>,

    // Reverse lookup: String -> SymbolId
    // We duplicate the string key for now to satisfy BTreeMap ownership in no_std easily.
    map: BTreeMap<String, SymbolId>,
}

impl Interner {
    pub fn new() -> Self {
        Self {
            names: Vec::new(),
            map: BTreeMap::new(),
        }
    }

    pub fn intern(&mut self, s: &str) -> SymbolId {
        if let Some(&id) = self.map.get(s) {
            return id;
        }

        let id = self.names.len() as SymbolId;
        let s_owned = String::from(s);

        self.names.push(s_owned.clone());
        self.map.insert(s_owned, id);

        id
    }

    pub fn resolve(&self, id: SymbolId) -> Option<&str> {
        self.names.get(id as usize).map(|s| s.as_str())
    }
}
