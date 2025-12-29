#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UserBootBlob {
    pub start: u64,
    pub size: u64,
    pub path_ptr: u64,
    pub path_len: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LoadedBootArgs {
    pub hhdm: u64,
    pub framebuffer: Option<(u64, usize)>,
    pub blobs_ptr: u64,
    pub blobs_len: u64,
    pub heap_start: u64,
    pub heap_size: u64,
}
