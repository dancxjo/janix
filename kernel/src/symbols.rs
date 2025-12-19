use alloc::string::String;
use alloc::vec::Vec;
use hashbrown::HashMap;
use spin::Mutex;
pub use abi::syscall_defs::SymbolId;


pub struct SymbolTable {
    unintern: Vec<String>,
    intern: HashMap<String, SymbolId>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            unintern: Vec::new(),
            intern: HashMap::new(),
        }
    }

    pub fn intern(&mut self, s: &str) -> SymbolId {
        if let Some(&id) = self.intern.get(s) {
            return id;
        }

        let id = SymbolId(self.unintern.len() as u32);
        let s_owned = String::from(s);
        self.unintern.push(s_owned.clone());
        self.intern.insert(s_owned, id);
        id
    }

    pub fn resolve(&self, id: SymbolId) -> Option<&str> {
        self.unintern.get(id.0 as usize).map(|s| s.as_str())
    }
}

static SYMBOLS: Mutex<Option<SymbolTable>> = Mutex::new(None);

pub fn init() {
    *SYMBOLS.lock() = Some(SymbolTable::new());
}

pub fn intern(s: &str) -> SymbolId {
    SYMBOLS.lock().as_mut().expect("Symbols not initialized").intern(s)
}

pub fn resolve(id: SymbolId) -> Option<String> {
    SYMBOLS.lock().as_ref().expect("Symbols not initialized").resolve(id).map(String::from)
}

/// Direct access to map, carefully.
pub fn with_symbol_table<F, R>(f: F) -> R
where
    F: FnOnce(&mut SymbolTable) -> R,
{
    let mut guard = SYMBOLS.lock();
    let table = guard.as_mut().expect("Symbols not initialized");
    f(table)
}
