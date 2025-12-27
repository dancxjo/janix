use abi::ThingId;
use crate::value::ThingBody;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Thing {
    pub id: ThingId,
    pub kind: ThingId,
    pub body: ThingBody,
}

impl Thing {
    pub const fn new(id: ThingId, kind: ThingId, body: ThingBody) -> Self {
        Self { id, kind, body }
    }

    pub fn try_with<T: Serialize>(
        id: ThingId,
        kind: ThingId,
        body: &T,
    ) -> Result<Self, postcard::Error> {
        let body = ThingBody::from(body)?;
        Ok(Self { id, kind, body })
    }

    pub fn with<T: Serialize>(id: ThingId, kind: ThingId, body: &T) -> Self {
        Self::try_with(id, kind, body).expect("ThingBody encode failed")
    }

    pub fn decode_as<T: serde::de::DeserializeOwned>(&self) -> Result<T, postcard::Error> {
        self.body.decode()
    }
}
