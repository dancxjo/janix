use abi::SymbolId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CapabilityBody {
    pub name: SymbolId,
}
