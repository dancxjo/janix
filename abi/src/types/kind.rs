use crate::{Graphable, SymbolId, KindId};

#[derive(Graphable, Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C, packed)]
pub struct Kind {
    pub id: KindId,
    pub name: SymbolId,
}
