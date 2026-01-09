
pub type JournalIndex = u64;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpKind {
    PutNode = 1,
    PutEdge = 2,
    DelNode = 3,
    DelEdge = 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct JournalOp {
    pub kind: OpKind,
    pub a: u64,     // node id or src
    pub b: u64,     // edge id or dst
    pub key: u64,   // reserved (v1)
    pub value: u64, // reserved (v1)
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WatchEvent {
    pub index: JournalIndex,
    pub op: JournalOp,
}
