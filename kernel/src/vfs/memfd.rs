use super::{VfsNode, VfsStat, SysResult};
use abi::errors::Errno;
use alloc::sync::Arc;
use spin::Mutex;

/// A memory-backed file descriptor (MemFD).
/// Allocates a contiguous physical memory region at creation.
pub struct MemFdNode {
    inner: Arc<Mutex<MemFdInner>>,
}

struct MemFdInner {
    phys_base: u64,
    size: usize,
    name: alloc::string::String,
}

impl MemFdNode {
    pub fn new(name: &str, size: usize) -> SysResult<Self> {
        let page_size = 4096usize;
        let aligned_size = (size + page_size - 1) & !(page_size - 1);
        let page_count = aligned_size / page_size;

        // Allocate contiguous physical memory
        // ACT III: Using the system's contiguous allocator
        let phys_base = crate::memory::alloc_contiguous_frames(page_count)
            .ok_or(Errno::ENOMEM)?;

        // Zero the memory
        let hhdm = crate::boot_info::get().map(|i| i.hhdm_offset).unwrap_or(0);
        unsafe {
            core::ptr::write_bytes((phys_base + hhdm) as *mut u8, 0, aligned_size);
        }

        Ok(Self {
            inner: Arc::new(Mutex::new(MemFdInner {
                phys_base,
                size: aligned_size,
                name: alloc::string::String::from(name),
            })),
        })
    }
}

impl VfsNode for MemFdNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let inner = self.inner.lock();
        let off = offset as usize;
        if off >= inner.size {
            return Ok(0);
        }
        let avail = inner.size - off;
        let n = buf.len().min(avail);
        
        let hhdm = crate::boot_info::get().map(|i| i.hhdm_offset).unwrap_or(0);
        let src = unsafe { core::slice::from_raw_parts((inner.phys_base + hhdm + offset) as *const u8, n) };
        buf[..n].copy_from_slice(src);
        Ok(n)
    }

    fn write(&self, offset: u64, buf: &[u8]) -> SysResult<usize> {
        let inner = self.inner.lock();
        let off = offset as usize;
        if off >= inner.size {
            return Err(Errno::ENOSPC);
        }
        let avail = inner.size - off;
        let n = buf.len().min(avail);

        let hhdm = crate::boot_info::get().map(|i| i.hhdm_offset).unwrap_or(0);
        let dst = unsafe { core::slice::from_raw_parts_mut((inner.phys_base + hhdm + offset) as *mut u8, n) };
        dst[..n].copy_from_slice(&buf[..n]);
        Ok(n)
    }

    fn stat(&self) -> SysResult<VfsStat> {
        let inner = self.inner.lock();
        Ok(VfsStat {
            mode: VfsStat::S_IFREG | 0o666,
            size: inner.size as u64,
            ino: inner.phys_base, // Use phys_base as unique ino for now
        })
    }

    fn phys_region(&self) -> SysResult<(u64, usize)> {
        let inner = self.inner.lock();
        Ok((inner.phys_base, inner.size))
    }
}

impl Drop for MemFdInner {
    fn drop(&mut self) {
        let page_count = self.size / 4096;
        unsafe {
            crate::memory::free_contiguous_frames(self.phys_base, page_count);
        }
    }
}
