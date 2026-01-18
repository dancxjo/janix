use abi::symbols::SymbolId;
use alloc::collections::BTreeMap;
use alloc::string::String;
use blake3::Hasher;

pub struct Interner {
    // Forward lookup: SymbolId -> String
    names: BTreeMap<SymbolId, String>,

    // Reverse lookup: String -> SymbolId
    map: BTreeMap<String, SymbolId>,
}

impl Interner {
    pub fn new() -> Self {
        Self {
            names: BTreeMap::new(),
            map: BTreeMap::new(),
        }
    }

    pub fn intern(&mut self, s: &str) -> SymbolId {
        if let Some(&id) = self.map.get(s) {
            return id;
        }

        // Generate hash-based ID
        let mut hasher = Hasher::new();
        hasher.update(s.as_bytes());
        let hash = hasher.finalize();
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(&hash.as_bytes()[0..16]);
        let id = SymbolId(bytes);

        let s_owned = String::from(s);

        self.names.insert(id, s_owned.clone());
        self.map.insert(s_owned, id);

        id
    }

    pub fn resolve(&self, id: SymbolId) -> Option<&str> {
        self.names.get(&id).map(|s| s.as_str())
    }
}
