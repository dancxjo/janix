use abi::{ThingId, SymbolId};
use serde::{Deserialize, Serialize};
use crate::payload::ThingPayload;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkEndpoints {
    pub from: ThingId,
    pub to: ThingId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkBody {
    pub from: ThingId,
    pub to: ThingId,
    pub predicate: SymbolId,
}

impl ThingPayload for LinkBody {
    const KIND: SymbolId = abi::sym("core.link");
}
