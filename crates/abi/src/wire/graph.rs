use crate::wire::typed::TypedBytes;
use crate::SymbolId;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum GraphOp<'a> {
    SymbolIntern {
        text: &'a str,
    },
    SymbolResolve {
        id: SymbolId,
    },
    Log {
        text: &'a str,
    },
    Watch {
        query: &'a str,
    },
    WriteTyped {
        path: &'a str,
        value: TypedBytes,
    },
    // CRUD Ops
    CreateThing {
        kind: crate::SymbolId,
        value: alloc::vec::Vec<u8>,
    },
    GetThing {
        id: crate::ThingId,
    },
    UpdateThing {
        id: crate::ThingId,
        value: alloc::vec::Vec<u8>,
    },
    // Link Ops
    AddLink {
        from: crate::ThingId,
        to: crate::ThingId,
        kind: crate::SymbolId,
    },
    ScanLinks {
        from: Option<crate::ThingId>,
        to: Option<crate::ThingId>,
        kind: Option<crate::SymbolId>,
    },
    Batch(alloc::vec::Vec<GraphOp<'a>>),
    DeleteThing {
        id: crate::ThingId,
    },
    ReadBytes {
        id: crate::ThingId,
        offset: u64,
        len: u32,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GraphReply {
    SymbolInterned { id: SymbolId },
    SymbolResolved { text: String },
    Ack,
    Error,
    Thing { bytes: alloc::vec::Vec<u8> },
    TypedValue(TypedBytes),
    Links(alloc::vec::Vec<(crate::ThingId, crate::ThingId, crate::SymbolId)>),
    BatchReply(alloc::vec::Vec<GraphReply>),
    Created { id: crate::ThingId },
    Bytes { bytes: alloc::vec::Vec<u8> },
}
