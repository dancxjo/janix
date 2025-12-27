use alloc::string::String;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockDeviceBody {
    pub model: String,
    pub serial: String,
    pub capacity_sectors: u64,
    pub sector_size: u32,
    pub device_type: BlockDeviceType,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BlockDeviceType {
    SATA,
    IDE,
    NVME,
    VIRTIO,
    USB,
    RAM,
}
