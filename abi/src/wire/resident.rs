use crate::ThingId;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ResidentId(pub ThingId);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResidentMapPerms(pub u32);

impl ResidentMapPerms {
    pub const READ: Self = Self(1 << 0);
    pub const WRITE: Self = Self(1 << 1); // implies READ
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResidentAllocArgs {
    pub kind_id: ThingId, // schema kind id
    pub byte_len: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct ResidentAllocResp {
    pub id: ThingId,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResidentMapArgs {
    pub id: ThingId,
    pub perms: ResidentMapPerms,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ResidentMapResp {
    pub user_addr: u64,
    pub byte_len: u32,
    pub _pad: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum RestPolicy {
    SnapshotKeepResident = 0,
    SnapshotEvictResident = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ArchiveRef {
    pub id: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RestResp {
    pub thing_id: crate::ThingId,
    pub archived_ref: ArchiveRef,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(u32)]
pub enum ResidentErrorCode {
    #[default]
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
    BadKind = 10,
    OutOfMemory = 11,
    Internal = 12,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ResidentError {
    pub code: ResidentErrorCode,
    pub aux0: u64,
    pub aux1: u64,
}
