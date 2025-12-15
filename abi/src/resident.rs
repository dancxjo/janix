#![no_std]

use crate::ThingId;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResidentHandle(pub u64);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ResidentAllocResp {
    pub thing_id: ThingId,
    pub handle: ResidentHandle,
    pub user_addr: u64,
    pub byte_len: u32,
    pub _pad: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum ResidentMapPerms {
    ReadOnly = 0,
    ReadWrite = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ResidentMapResp {
    pub user_addr: u64,
    pub byte_len: u32,
    pub _pad: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum RestPolicy {
    SnapshotKeepResident = 0,
    SnapshotEvictResident = 1, // free resident after archive commit
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RestResp {
    pub thing_id: crate::ThingId,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum ResidentErrorCode {
    Success = 0,
    Unknown = 1,
    PermissionDenied = 2,
    NotFound = 3,
    BadThing = 4,
    NotResident = 5,
    AlreadyMappedRw = 6,
    SerializeFailed = 7,
    Bounds = 8,
    BadHeader = 9,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ResidentError {
    pub code: ResidentErrorCode,
    pub aux0: u64,
    pub aux1: u64,
}
