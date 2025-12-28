extern crate alloc;
use abi::SymbolId;
use alloc::vec::Vec;

pub trait SymbolStore {
    /// Load persisted mappings. Returns a list of (id, utf8 bytes).
    fn load(&mut self) -> Result<Vec<(SymbolId, Vec<u8>)>, SymbolStoreError>;

    /// Persist complete table snapshot (simple v0 approach).
    fn save(&mut self, entries: &[(SymbolId, &[u8])]) -> Result<(), SymbolStoreError>;
}

#[derive(Debug)]
pub enum SymbolStoreError {
    Io,
    Corrupt,
    Unsupported,
}
