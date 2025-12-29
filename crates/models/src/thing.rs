use crate::value::ThingBody;
use abi::{ThingId, SymbolId};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Thing {
    pub id: ThingId,
    pub kind: SymbolId,
    pub payload: Vec<u8>,
}

impl Thing {
    pub fn new(id: ThingId, kind: SymbolId, payload: Vec<u8>) -> Self {
        Self { id, kind, payload }
    }

    pub fn try_with<T: Serialize>(
        id: ThingId,
        kind: SymbolId,
        body: &T,
    ) -> Result<Self, postcard::Error> {
        let bytes = postcard::to_allocvec(body)?;
        Ok(Self { id, kind, payload: bytes })
    }

    pub fn with<T: Serialize>(id: ThingId, kind: SymbolId, body: &T) -> Self {
        Self::try_with(id, kind, body).expect("ThingBody encode failed")
    }

    pub fn decode_as<T: serde::de::DeserializeOwned>(&self) -> Result<T, postcard::Error> {
        postcard::from_bytes(&self.payload)
    }

    // Compatibility helper for code expecting a ThingBody
    pub fn body(&self) -> ThingBody {
        ThingBody { bytes: self.payload.clone() }
    }
}
