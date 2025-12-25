use serde::{Deserialize, Serialize};
use crate::SymbolId;
use alloc::string::String;

#[derive(Serialize, Deserialize, Debug)]
pub enum GraphOp<'a> {
    SymbolIntern { text: &'a str },
    SymbolResolve { id: SymbolId },
    Log { text: &'a str },
    Watch { query: &'a str }, // Blocking next item
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GraphReply {
    SymbolInterned { id: SymbolId },
    SymbolResolved { text: String },
    Ack,
    Error,
    Thing { bytes: alloc::vec::Vec<u8> },
}

