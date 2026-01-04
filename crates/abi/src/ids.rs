//! Standard identifiers for ThingOS

/// Thing identifier - a 128-bit UUID
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ThingId(pub u128);

impl ThingId {
    /// Create a new ThingId from high and low parts
    pub const fn from_parts(high: u64, low: u64) -> Self {
        ThingId(((high as u128) << 64) | (low as u128))
    }

    /// Get the high 64 bits
    pub const fn high(&self) -> u64 {
        (self.0 >> 64) as u64
    }

    /// Get the low 64 bits
    pub const fn low(&self) -> u64 {
        self.0 as u128 as u64
    }
}

pub type PlaceId = ThingId;
pub type RelationshipId = ThingId;

/// Symbol identifier - a stable u64 mapping
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(transparent)]
pub struct SymbolId(pub u64);

impl SymbolId {
    pub const INVALID: SymbolId = SymbolId(0);
}

/// FNV-1a 64-bit hash
pub const fn symbol_hash(s: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(0x100000001b3);
        i += 1;
    }
    hash
}

/// Create a SymbolId from a string literal at compile time
pub const fn sym(s: &str) -> SymbolId {
    SymbolId(symbol_hash(s))
}

/// Watch identifier - opaque handle for graph/watch events
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct WatchId(pub u64);

pub type PredicateId = SymbolId;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_hashes() {
        // "hash treaty"
        // kind.Window -> 0x2AEC7E1B886B5C47
        assert_eq!(sym("kind.Window").0, 0x2AEC7E1B886B5C47);
        
        // schema.Window@1 -> 0x939AA0A705967517
        assert_eq!(sym("schema.Window@1").0, 0x939AA0A705967517);
        
        // Ensure INVALID is 0 (though FNV doesn't guarantee 0 is impossible, it's unlikely)
        // Ideally we reserve 0.
        assert!(sym("something").0 != 0);
    }
}
