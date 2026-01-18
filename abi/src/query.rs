use crate::symbols::SymbolRefWire;
use crate::wire::{ThingId, SymbolId};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum QueryOpKind {
    Scan = 1,
    FilterEq = 2,
    Expand = 3,
    Start = 4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum QueryDir {
    Out = 0,
    In = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct QueryStep {
    pub op: u64,               // QueryOpKind
    pub arg1: u64,             // Limit (Scan), Value (Filter), Dir (Expand)
    pub arg2: u64,             // Reserved / PropValue?
    pub symbol: SymbolRefWire, // Kind (Scan), Key (Filter), Rel (Expand)
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct QueryRow {
    pub id: ThingId,       // Node ID or Src
    pub kind_rel: SymbolId, // SymbolId of Kind or Rel
    pub val_dst: ThingId,  // Prop Value or Dst ID
    pub extra: u64,    // Reserved
}
