use serde::{Deserialize, Serialize};
use crate::SymbolId;
use alloc::string::String;

#[derive(Serialize, Deserialize, Debug)]
pub enum GraphOp<'a> {
    SymbolIntern { text: &'a str },
    SymbolResolve { id: SymbolId },
    Log { text: &'a str },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GraphReply {
    SymbolInterned { id: SymbolId },
    SymbolResolved { text: String },
    Ack,
    Error,
}
