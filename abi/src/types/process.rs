use crate::{Graphable, SymbolId, ThingId};

#[derive(Graphable, Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C, packed)]
pub struct Process {
    pub id: ThingId,
    pub name: SymbolId,
    pub pid: u64,
}
