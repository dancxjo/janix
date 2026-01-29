use crate::{Graphable, PredicateId, SymbolId};

#[derive(Graphable, Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C, packed)]
pub struct Predicate {
    pub id: PredicateId,
    pub name: SymbolId,
}
