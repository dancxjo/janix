//! Standard identifiers for ThingOS

/// Thing identifier - a 128-bit UUID
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ThingId(pub u128);

impl ThingId {
    /// Create a new ThingId from high and low parts
    pub fn from_parts(high: u64, low: u64) -> Self {
        ThingId(((high as u128) << 64) | (low as u128))
    }

    /// Get the high 64 bits
    pub fn high(&self) -> u64 {
        (self.0 >> 64) as u64
    }

    /// Get the low 64 bits
    pub fn low(&self) -> u64 {
        self.0 as u128 as u64
    }
}

pub type PlaceId = ThingId;
pub type RelationshipId = ThingId;

/// Symbol identifier - a stable u64 mapping
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SymbolId(pub u64);

impl SymbolId {
    pub const INVALID: SymbolId = SymbolId(0);
}

/// Watch identifier - opaque handle for graph/watch events
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct WatchId(pub u64);

pub type PredicateId = SymbolId;
