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
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GraphReply {
    SymbolInterned { id: SymbolId },
    SymbolResolved { text: String },
    Ack,
    Error,
    Thing { bytes: alloc::vec::Vec<u8> },
    TypedValue(TypedBytes),
}

