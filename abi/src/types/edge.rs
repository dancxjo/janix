use crate::{Graphable, ThingId};

#[derive(Graphable, Copy, Clone, Debug, Eq, PartialEq, Default)]
#[repr(C, packed)]
pub struct Edge {
    pub from: ThingId,
    pub predicate: ThingId, // Using ThingId for now as predicates are Things
    pub to: ThingId,
    pub flags: u32,
}
