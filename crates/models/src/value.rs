use alloc::vec::Vec;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThingBody {
    pub bytes: Vec<u8>,
}

impl ThingBody {
    pub fn from<T: Serialize>(t: &T) -> Result<Self, postcard::Error> {
        let bytes = postcard::to_allocvec(t)?;
        Ok(Self { bytes })
    }

    pub fn decode<T: DeserializeOwned>(&self) -> Result<T, postcard::Error> {
        postcard::from_bytes(&self.bytes)
    }
}
