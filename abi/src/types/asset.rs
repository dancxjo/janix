use crate::{BlobId, Graphable, SymbolId, ThingId};

#[derive(Graphable, Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C, packed)]
pub struct Asset {
    pub id: ThingId,
    pub name: SymbolId,
    pub mime_type: SymbolId,
    pub content: BlobId,
}
