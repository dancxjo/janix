//! Symbol table
//!
//! Provides stable symbol interning for the kernel. Symbols are used
//! for Thing kinds, predicates, and other semantic identifiers.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

use abi::ids::SymbolId;

/// Global symbol table
static SYMBOLS: Mutex<Option<SymbolTable>> = Mutex::new(None);

struct SymbolTable {
    /// Forward mapping: string -> id
    string_to_id: BTreeMap<Vec<u8>, SymbolId>,
    /// Reverse mapping: id -> string
    id_to_string: BTreeMap<SymbolId, Vec<u8>>,
    /// Next available ID
    next_id: u64,
}

impl SymbolTable {
    fn new() -> Self {
        Self {
            string_to_id: BTreeMap::new(),
            id_to_string: BTreeMap::new(),
            next_id: 1, // 0 is reserved for INVALID
        }
    }

    fn intern(&mut self, s: &[u8]) -> SymbolId {
        if let Some(&id) = self.string_to_id.get(s) {
            return id;
        }

        let id = SymbolId(self.next_id);
        self.next_id += 1;

        let bytes = s.to_vec();
        self.string_to_id.insert(bytes.clone(), id);
        self.id_to_string.insert(id, bytes);

        id
    }

    fn resolve(&self, id: SymbolId) -> Option<&[u8]> {
        self.id_to_string.get(&id).map(|v| v.as_slice())
    }
}

/// Initialize the symbol table and seed core symbols
pub fn init() {
    let mut table = SymbolTable::new();

    // Seed core symbols
    seed_symbol(&mut table, b"models.core.log.LogEntry");
    seed_symbol(&mut table, b"models.core.Schema");
    seed_symbol(&mut table, b"models.core.ByteObject");
    seed_symbol(&mut table, b"models.core.Thing");
    seed_symbol(&mut table, b"models.core.Link");
    seed_symbol(&mut table, b"sprout");
    seed_symbol(&mut table, b"bloom");
    seed_symbol(&mut table, b"predicate.version");
    seed_symbol(&mut table, b"kind.Place");
    seed_symbol(&mut table, b"kind.Relationship");
    seed_symbol(&mut table, b"kind.Thing");
    seed_symbol(&mut table, b"predicate.contains");
    seed_symbol(&mut table, b"thing.bloom");
    seed_symbol(&mut table, b"place.desktop");
    seed_symbol(&mut table, b"thing.display.primary");
    seed_symbol(&mut table, b"thing.pointer");
    seed_symbol(&mut table, b"thing.wallpaper.sky");
    seed_symbol(&mut table, b"predicate.spawns");
    seed_symbol(&mut table, b"predicate.provides");
    // Scheduler ontology
    seed_symbol(&mut table, b"place.scheduler");
    seed_symbol(&mut table, b"thing.runqueue.default");
    seed_symbol(&mut table, b"thing.task.sprout");
    seed_symbol(&mut table, b"predicate.state");
    seed_symbol(&mut table, b"state.runnable");
    seed_symbol(&mut table, b"state.running");
    seed_symbol(&mut table, b"state.blocked");
    seed_symbol(&mut table, b"priority.normal");
    seed_symbol(&mut table, b"predicate.priority");

    *SYMBOLS.lock() = Some(table);
}

fn seed_symbol(table: &mut SymbolTable, s: &[u8]) -> SymbolId {
    table.intern(s)
}

/// Intern a symbol, returning its stable ID
///
/// If the symbol already exists, returns the existing ID.
/// Otherwise, assigns a new ID and stores the mapping.
pub fn intern(s: &[u8]) -> SymbolId {
    let mut guard = SYMBOLS.lock();
    match guard.as_mut() {
        Some(table) => table.intern(s),
        None => SymbolId::INVALID,
    }
}

/// Resolve a symbol ID to its string representation
pub fn resolve(id: SymbolId) -> Option<String> {
    let guard = SYMBOLS.lock();
    guard.as_ref().and_then(|table| {
        table
            .resolve(id)
            .and_then(|bytes| core::str::from_utf8(bytes).ok().map(String::from))
    })
}

/// Get a well-known symbol ID by name (for kernel use)
pub fn well_known(name: &[u8]) -> SymbolId {
    intern(name)
}

// Well-known symbol accessors
pub fn sym_log_entry() -> SymbolId {
    well_known(b"models.core.log.LogEntry")
}

pub fn sym_sprout() -> SymbolId {
    well_known(b"sprout")
}

pub fn sym_bloom() -> SymbolId {
    well_known(b"thing.bloom")
}

pub fn sym_desktop() -> SymbolId {
    well_known(b"place.desktop")
}

pub fn sym_display_primary() -> SymbolId {
    well_known(b"thing.display.primary")
}

pub fn sym_pointer() -> SymbolId {
    well_known(b"thing.pointer")
}

pub fn sym_wallpaper_sky() -> SymbolId {
    well_known(b"thing.wallpaper.sky")
}

// Scheduler symbols
pub fn sym_scheduler() -> SymbolId {
    well_known(b"place.scheduler")
}

pub fn sym_runqueue_default() -> SymbolId {
    well_known(b"thing.runqueue.default")
}

pub fn sym_task_sprout() -> SymbolId {
    well_known(b"thing.task.sprout")
}

pub fn sym_pred_state() -> SymbolId {
    well_known(b"predicate.state")
}

pub fn sym_state_running() -> SymbolId {
    well_known(b"state.running")
}
