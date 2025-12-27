use abi::SymbolId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GraphBody {
    pub name: SymbolId,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MountBody {
    pub at: SymbolId,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GraphProviderBody {
    pub name: SymbolId,
    pub provider_kind: SymbolId,
}
