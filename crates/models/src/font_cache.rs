use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FontCache {
    pub entries: Vec<FontCacheEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FontCacheEntry {
    pub path: String,
    pub family: String,
    pub weight: u16,
    pub italic: bool,
}
