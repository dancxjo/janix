use alloc::string::String;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FilesystemBody {
    pub name: String,
    pub kind: FilesystemKind,
    pub read_only: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FilesystemKind {
    Iso9660,
    Fat32,
    Ext2,
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileBody {
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
    // For ISO9660, we might need the sector location
    pub start_sector: u32,
}
