use alloc::string::String;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct VolumeBody {
    pub fs: String,
    pub volume_id: String,
    pub pvd_lba: u32,
    pub root_dir_lba: u32,
    pub root_dir_size: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MountBody {
    pub path: String,
    pub readonly: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DirBody {
    pub name: String,
    pub lba: u32,
    pub size: u32,
    pub expanded: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileBody {
    pub name: String,
    pub size: u64,
    pub lba: u32,
    pub flags: u8,
}
