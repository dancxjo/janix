use crate::{BlobId, Graphable, SymbolId, ThingId};

#[derive(Graphable, Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C, packed)]
pub struct Font {
    pub id: ThingId,
    pub family: SymbolId,
    pub style: SymbolId,
    pub blob: BlobId,
}
