use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootAddrKind {
    Phys,        // needs hhdm translation to read
    Virt,        // already a usable pointer (kernel virtual)
}

#[derive(Debug, Clone)]
pub struct BootBlob {
    pub path: String,
    pub start: u64,
    pub size: u64,
    pub addr_kind: BootAddrKind,
}

pub unsafe fn blob_as_slice(blob: &BootBlob, hhdm: u64) -> &'static [u8] {
    let virt = match blob.addr_kind {
        BootAddrKind::Phys => hhdm + blob.start,
        BootAddrKind::Virt => blob.start,
    };
    core::slice::from_raw_parts(virt as *const u8, blob.size as usize)
}
