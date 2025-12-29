use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ByteSpaceCreateReq {
    pub len: u64,
    pub flags: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ByteSpaceCreateResp {
    pub id: u64,
    pub len: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ByteSpaceMapReq {
    pub id: u64,
    pub user_va_hint: u64,
    pub offset: u64,
    pub len: u64,
    pub prot: u32,  // Prot flags (READ/WRITE/EXEC)
    pub flags: u32, // Map flags (SHARED/PRIVATE/FIXED)
}

#[derive(Serialize, Deserialize)]
pub struct ByteSpaceMapResp {
    pub user_addr: u64,
}
