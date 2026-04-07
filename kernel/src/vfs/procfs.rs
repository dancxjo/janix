//! procfs — process information filesystem mounted at `/proc`.
//!
//! Provides a read-only view of running processes.  This is a **stub**
//! implementation for ACT III: the directory structure is present and the
//! driver responds to well-known paths, but the data returned is minimal.
//!
//! # Paths exposed
//!
//! | Path            | Contents |
//! |-----------------|----------|
//! | `/proc/version` | Kernel version string |
//! | `/proc/mounts`  | Active mount table (text) |
//!
//! Per-process sub-directories (`/proc/<pid>/`) will be added in a later act
//! once the process registry is fully wired into the VFS.

use alloc::sync::Arc;
use abi::errors::{Errno, SysResult};

use super::{VfsDriver, VfsNode, VfsStat};

// ── ProcFs driver ─────────────────────────────────────────────────────────────

/// The process filesystem driver.  Mounted at `/proc` by `vfs::init`.
pub struct ProcFs;

impl ProcFs {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ProcFs {
    fn default() -> Self {
        Self::new()
    }
}

impl VfsDriver for ProcFs {
    fn lookup(&self, path: &str) -> SysResult<Arc<dyn VfsNode>> {
        match path {
            "" => Ok(Arc::new(ProcDirNode)),
            "version" => Ok(Arc::new(StaticTextNode::new(
                b"Thing-OS v0.1 (janix ACT III)\n",
                2,
            ))),
            "mounts" => Ok(Arc::new(MountsNode)),
            _ => Err(Errno::ENOENT),
        }
    }
}

// ── /proc root directory ──────────────────────────────────────────────────────

struct ProcDirNode;

impl VfsNode for ProcDirNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFDIR | 0o555,
            size: 0,
            ino: 200,
        })
    }
    fn readdir(&self, _offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let entries = b"version\0mounts\0";
        let n = entries.len().min(buf.len());
        buf[..n].copy_from_slice(&entries[..n]);
        Ok(n)
    }
}

// ── Static text node ──────────────────────────────────────────────────────────

/// Returns a fixed byte slice on read.
struct StaticTextNode {
    data: &'static [u8],
    ino: u64,
}

impl StaticTextNode {
    const fn new(data: &'static [u8], ino: u64) -> Self {
        Self { data, ino }
    }
}

impl VfsNode for StaticTextNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let off = offset as usize;
        if off >= self.data.len() {
            return Ok(0);
        }
        let avail = &self.data[off..];
        let n = avail.len().min(buf.len());
        buf[..n].copy_from_slice(&avail[..n]);
        Ok(n)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EROFS)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFREG | 0o444,
            size: self.data.len() as u64,
            ino: self.ino,
        })
    }
}

// ── /proc/mounts ─────────────────────────────────────────────────────────────

/// A dynamic node that renders the current mount table as text.
///
/// The output mirrors a simplified `/proc/mounts` format:
/// ```text
/// <mount_point> ramfs rw 0 0
/// ```
struct MountsNode;

impl VfsNode for MountsNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        // Build the mounts listing dynamically from the global mount table.
        let listing = super::mount::mounts_text();
        let data = listing.as_bytes();
        let off = offset as usize;
        if off >= data.len() {
            return Ok(0);
        }
        let avail = &data[off..];
        let n = avail.len().min(buf.len());
        buf[..n].copy_from_slice(&avail[..n]);
        Ok(n)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EROFS)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFREG | 0o444,
            size: 0, // dynamic — size not known until read
            ino: 201,
        })
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn lookup(path: &str) -> SysResult<Arc<dyn VfsNode>> {
        ProcFs::new().lookup(path)
    }

    #[test]
    fn test_lookup_root_is_dir() {
        let node = lookup("").unwrap();
        let stat = node.stat().unwrap();
        assert!(stat.is_dir());
    }

    #[test]
    fn test_lookup_version() {
        let node = lookup("version").unwrap();
        let mut buf = [0u8; 64];
        let n = node.read(0, &mut buf).unwrap();
        assert!(n > 0);
        assert!(core::str::from_utf8(&buf[..n]).unwrap().contains("Thing-OS"));
    }

    #[test]
    fn test_version_is_readonly() {
        let node = lookup("version").unwrap();
        assert!(matches!(node.write(0, b"hack"), Err(Errno::EROFS)));
    }

    #[test]
    fn test_lookup_unknown_returns_enoent() {
        assert!(matches!(lookup("doesnotexist"), Err(Errno::ENOENT)));
    }

    #[test]
    fn test_readdir_root_lists_entries() {
        let node = lookup("").unwrap();
        let mut buf = [0u8; 64];
        let n = node.readdir(0, &mut buf).unwrap();
        assert!(n > 0);
        let s = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(s.contains("version"));
        assert!(s.contains("mounts"));
    }
}
