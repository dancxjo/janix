use abi::{SymbolId, ThingId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KindBody {
    pub name: SymbolId,
    pub version: u32,
    pub schema: ThingId,
}
