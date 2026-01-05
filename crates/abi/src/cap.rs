use crate::ids::ThingId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u32)]
pub enum CapOp {
    Log = 1,
    GraphCreate = 2,
    GraphLink = 3,
    GraphUnlink = 4,
    GraphRead = 5,
    GraphWrite = 6,
    GraphWatch = 7,
    MemManage = 8,
    Hardware = 9,
    GrantCaps = 10,
    IoPort = 11,
    PciConfigRead = 12,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapScope {
    Global,
    Thing(ThingId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cap {
    pub op: CapOp,
    pub scope: CapScope,
}
