use abi::ThingId;
use crate::graph;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProcessRef(pub ThingId);

impl ProcessRef {
    pub fn new(id: ThingId) -> Option<Self> {
        let is_process = graph::with_thing(id, |thing| thing.kind == "Process").unwrap_or(false);
        if is_process {
            Some(Self(id))
        } else {
            None
        }
    }
}
