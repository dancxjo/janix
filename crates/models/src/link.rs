use abi::ThingId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkEndpoints {
    pub from: ThingId,
    pub to: ThingId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkBody {
    pub from: ThingId,
    pub to: ThingId,
    pub predicate: ThingId,
}
