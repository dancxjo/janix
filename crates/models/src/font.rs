use alloc::vec::Vec;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FontBody {
    pub data: Vec<u8>,
}

impl FontBody {
    pub fn parse(&self) -> Result<fontdue::Font, &'static str> {
         fontdue::Font::from_bytes(&self.data[..], fontdue::FontSettings::default())
    }
}
