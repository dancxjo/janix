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
    }, // Blocking next item
    WriteTyped {
        path: &'a str,
        value: TypedBytes,
    },
    // CRUD Ops
    CreateThing {
        kind: crate::ThingId,
        value: crate::wire::typed::TypedBytes,
    },
    GetThing {
        id: crate::ThingId,
    },
    UpdateThing {
        id: crate::ThingId,
        value: crate::wire::typed::TypedBytes,
    },
    // SetProp { id: crate::ThingId, prop: u32, value: crate::wire::typed::TypedBytes }, // Deprecated for v0.2 smoke

    // Link Ops
    AddLink {
        from: crate::ThingId,
        to: crate::ThingId,
        kind: crate::ThingId,
    },
    ScanLinks {
        from: Option<crate::ThingId>,
        to: Option<crate::ThingId>,
        kind: Option<crate::ThingId>,
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
    MachineCall {
        iface: SymbolId,
        ver: u16,
        instance: SymbolId,
        op: u32,
        payload: alloc::vec::Vec<u8>,
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
    Links(alloc::vec::Vec<(crate::ThingId, crate::ThingId, crate::ThingId)>),
    BatchReply(alloc::vec::Vec<GraphReply>),
    Created { id: crate::ThingId },
    Bytes { bytes: alloc::vec::Vec<u8> },
}
