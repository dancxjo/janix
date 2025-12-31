
pub struct UserPageFlags {
    pub writable: bool,
    pub executable: bool,
    pub device: bool,     // framebuffer MMIO / uncached
    pub user: bool,       // always true for user mappings
}

pub trait UserSpace {
    type Root; // arch-specific root page table handle/paddr

    unsafe fn create_root() -> Self::Root;
    unsafe fn activate_root(root: &Self::Root);

    unsafe fn alloc_frame_4k() -> u64; // returns phys addr of a 4KiB frame

    unsafe fn map_4k(root: &mut Self::Root, vaddr: u64, paddr: u64, flags: UserPageFlags);

    /// Write bytes into user VA; implementation may temporarily map frames via HHDM/linear map.
    unsafe fn write_bytes(root: &mut Self::Root, vaddr: u64, bytes: &[u8]);

    /// Required on aarch64; may be no-op on x86_64.
    unsafe fn sync_icache(vaddr: u64, len: usize);
}
