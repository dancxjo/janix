use crate::{Graphable, ThingId, SymbolId};

#[derive(Graphable, Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C, packed)]
pub struct Task {
    pub id: ThingId,
    pub name: SymbolId,
    pub parent: ThingId,
    pub state: u32,
}
