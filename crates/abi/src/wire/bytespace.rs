use serde::{Serialize, Deserialize};
use crate::ThingId;

#[derive(Serialize, Deserialize)]
pub struct ByteSpaceCreateReq {
    pub len: u64,
    pub flags: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ByteSpaceCreateResp {
    pub bytespace_thing_id: ThingId,
}

#[derive(Serialize, Deserialize)]
pub struct ByteSpaceRegisterReq {
    pub bytespace_thing_id: ThingId,
    pub len: u64,
    pub flags: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ByteSpaceRegisterResp {
    pub ok: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ByteSpaceMapReq {
    pub bytespace_thing_id: ThingId,
    pub offset: u64,
    pub len: u64,
    pub prot: u32,
    pub user_va_hint: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ByteSpaceMapResp {
    pub user_addr: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ByteSpaceReadReq {
    pub bytespace_thing_id: ThingId,
    pub offset: u64,
    pub user_dst_ptr: u64,
    pub len: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ByteSpaceReadResp {
    pub bytes_read: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ByteSpaceWriteReq {
    pub bytespace_thing_id: ThingId,
    pub offset: u64,
    pub user_src_ptr: u64,
    pub len: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ByteSpaceWriteResp {
    pub bytes_written: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ByteSpaceCreateAndMapReq {
    pub len: u64,
    pub flags: u32,
    pub prot: u32,
    pub user_va_hint: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ByteSpaceCreateAndMapResp {
    pub bytespace_thing_id: ThingId,
    pub user_addr: u64,
}
