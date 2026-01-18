use crate::{Graphable, ThingId, SymbolId};

#[derive(Graphable, Copy, Clone, Debug, Eq, PartialEq, Default)]
#[repr(C, packed)]
pub struct Edge {
    pub from: ThingId,
    pub predicate: SymbolId,
    pub to: ThingId,
    pub flags: u32,
}
