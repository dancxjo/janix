//! Bytespace: First-class memory objects backed by physical pages.
//!
//! Bytespaces are mappable, page-granular memory regions that can be:
//! - Used as framebuffer backing stores
//! - Shared between processes
//! - Mapped into userspace address spaces
//! - Read/written via syscalls (for snapshots and tooling)

use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::Mutex;

/// A bytespace backed by contiguous physical memory.
#[derive(Clone)]
pub struct Bytespace {
    /// Kernel virtual address (HHDM-mapped)
    pub kernel_va: usize,
    /// Physical address base
    pub phys_base: u64,
    /// Size in bytes (page-aligned)
    pub len: usize,
    /// Number of 4K pages
    pub page_count: usize,
    /// Access flags (reserved for future use)
    pub flags: u64,
    /// Whether this bytespace owns its memory (vs borrowing from boot/ACPI)
    pub owned: bool,
}

pub type BytespaceHandle = Arc<Mutex<Bytespace>>;

/// Mapping record for tracking userspace mappings
pub struct BytespaceMapping {
    pub bytespace_id: u64,
    pub tid: u64,
    pub user_va: u64,
    pub len: usize,
}

/// Global mapping registry for v0. Later can be per-process.
static MAPPINGS: Mutex<Vec<BytespaceMapping>> = Mutex::new(Vec::new());

pub fn create(len: usize, hhdm_offset: u64) -> Option<BytespaceHandle> {
    // Round up to page boundary
    let page_count = (len + 4095) / 4096;
    let aligned_len = page_count * 4096;

    // Allocate contiguous physical frames
    let phys_base = crate::memory::alloc_contiguous_frames(page_count)?;
    let kernel_va = (phys_base + hhdm_offset) as usize;

    // Zero the memory
    unsafe {
        core::ptr::write_bytes(kernel_va as *mut u8, 0, aligned_len);
    }

    Some(Arc::new(Mutex::new(Bytespace {
        kernel_va,
        phys_base,
        len: aligned_len,
        page_count,
        flags: 0,
        owned: true,
    })))
}

/// Create a bytespace from an existing physical pointer (for boot modules, ACPI, etc).
/// This unifies the old `create_from_ptr` - the pointer is treated as a kernel VA.
pub fn create_from_ptr(kernel_va: usize, len: usize, hhdm_offset: u64) -> BytespaceHandle {
    let page_count = (len + 4095) / 4096;
    let phys_base = if kernel_va as u64 >= hhdm_offset {
        kernel_va as u64 - hhdm_offset
    } else {
        kernel_va as u64 // Already physical or identity-mapped
    };

    Arc::new(Mutex::new(Bytespace {
        kernel_va,
        phys_base,
        len,
        page_count,
        flags: 0,
        owned: false, // Don't free these pages
    }))
}

/// Record a mapping
pub fn record_mapping(bytespace_id: u64, tid: u64, user_va: u64, len: usize) {
    MAPPINGS.lock().push(BytespaceMapping {
        bytespace_id,
        tid,
        user_va,
        len,
    });
}

/// Find and remove a mapping, returning it if found
pub fn remove_mapping(bytespace_id: u64, tid: u64, user_va: u64) -> Option<BytespaceMapping> {
    let mut mappings = MAPPINGS.lock();
    if let Some(idx) = mappings
        .iter()
        .position(|m| m.bytespace_id == bytespace_id && m.tid == tid && m.user_va == user_va)
    {
        Some(mappings.remove(idx))
    } else {
        None
    }
}

/// Find mapping by bytespace and tid
pub fn find_mapping(bytespace_id: u64, tid: u64) -> Option<(u64, usize)> {
    let mappings = MAPPINGS.lock();
    mappings
        .iter()
        .find(|m| m.bytespace_id == bytespace_id && m.tid == tid)
        .map(|m| (m.user_va, m.len))
}
