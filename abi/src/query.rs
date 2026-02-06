use crate::symbols::SymbolRefWire;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum QueryOpKind {
    Scan = 1,
    FilterEq = 2,
    Expand = 3,
    Start = 4,
    Anchor = 5,
}

pub const ANCHOR_HOST: u64 = 0;
pub const ANCHOR_ROOT: u64 = 1;
pub const ANCHOR_KERNEL: u64 = 2;
pub const ANCHOR_SCHEDULER: u64 = 3;
pub const ANCHOR_CPU: u64 = 4;

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
    pub id: u64,       // Node ID or Src
    pub kind_rel: u64, // SymbolId of Kind or Rel
    pub val_dst: u64,  // Prop Value or Dst ID
    pub extra: u64,    // Reserved
}
