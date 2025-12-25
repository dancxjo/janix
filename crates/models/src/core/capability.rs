use abi::SymbolId;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CapabilityBody {
    pub name: SymbolId,
}
