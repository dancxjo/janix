use crate::{Predicate, ThingId, syscall_defs::SymbolId};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WireWatchSpec {
    pub src_kind: SymbolId,
    pub pred: Predicate,
    pub on: SymbolId,
    pub src_thing: ThingId,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WireEventHeader {
    pub src: ThingId,
    pub pred: Predicate,
    pub on: SymbolId,
    pub payload_len: u32,
}
