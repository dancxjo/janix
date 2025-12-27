use serde::{Deserialize, Serialize};
use crate::SymbolId;
use crate::wire::typed::TypedBytes;
use alloc::string::String;

#[derive(Serialize, Deserialize, Debug)]
pub enum GraphOp<'a> {
    SymbolIntern { text: &'a str },
    SymbolResolve { id: SymbolId },
    Log { text: &'a str },
    Watch { query: &'a str }, // Blocking next item
    WriteTyped { path: &'a str, value: TypedBytes },
    // CRUD Ops
    CreateThing { kind: crate::ThingId, value: crate::wire::typed::TypedBytes },
    GetThing { id: crate::ThingId },
    UpdateThing { id: crate::ThingId, value: crate::wire::typed::TypedBytes },
    // SetProp { id: crate::ThingId, prop: u32, value: crate::wire::typed::TypedBytes }, // Deprecated for v0.2 smoke

    
    // Link Ops
    AddLink { from: crate::ThingId, to: crate::ThingId, kind: crate::ThingId },
    ScanLinks { from: Option<crate::ThingId>, to: Option<crate::ThingId>, kind: Option<crate::ThingId> },
    DeleteThing { id: crate::ThingId }, // Putting it here to group structure
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
    Created { id: crate::ThingId },
}

