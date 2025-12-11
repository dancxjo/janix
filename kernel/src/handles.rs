use abi::ThingId;
use crate::graph;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProcessRef(pub ThingId);

impl ProcessRef {
    pub fn new(id: ThingId) -> Option<Self> {
        let node = graph::get_thing(id)?;
        if node.0 == "Process" {
            Some(Self(id))
        } else {
            None
        }
    }
}
