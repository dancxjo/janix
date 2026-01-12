#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UserLogRequest {
    pub level: u32, 
    pub line: u32,
    pub msg_ptr: u64,
    pub msg_len: u64,
    pub file_ptr: u64,
    pub file_len: u64,
    pub mod_ptr: u64,
    pub mod_len: u64,
}
