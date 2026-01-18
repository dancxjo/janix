use crate::{Graphable, ThingId};

#[derive(Graphable, Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C, packed)]
pub struct Thread {
    pub id: ThingId,
    pub process: ThingId,
    pub tid: u64,
}
