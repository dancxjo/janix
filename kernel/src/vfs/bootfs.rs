//! bootfs — minimal static boot filesystem.
//!
//! Provides a read-only filesystem whose contents are compiled into the kernel
//! binary at link time.  This serves as the initramfs: essential files are
//! available to early userland before any writable filesystem is populated.
//!
//! The boot filesystem is mounted at `/boot` by [`crate::vfs::init`].
//!
//! # File tree
//!
//! | Path               | Contents                                      |
//! |--------------------|-----------------------------------------------|
//! | `/boot/version`    | Kernel version string (static)                |
//! | `/boot/motd`       | Message of the day shown at early boot        |
//!
//! # Design
//! All file contents are `&'static [u8]` slices embedded directly in the
//! kernel image.  No heap allocation is required to store the data itself;
//! only the [`Arc`] wrappers for the [`VfsNode`] objects are heap-allocated.
//! This makes the boot filesystem available immediately after the allocator
//! is initialised, without any I/O or initialisation step.

use abi::errors::{Errno, SysResult};
use alloc::sync::Arc;

use super::{VfsDriver, VfsNode, VfsStat};

// ── Embedded file contents ────────────────────────────────────────────────────

const VERSION_DATA: &[u8] = b"Thing-OS v0.1 (janix ACT IV)\n";
const MOTD_DATA: &[u8] = b"Welcome to Thing-OS.\nBooting into path-based namespace...\n";

// ── Directory entry list ──────────────────────────────────────────────────────

/// Names of all entries visible in the `/boot` directory.
const BOOT_DIR_ENTRIES: &[u8] = b"version\0motd\0";

// ── BootFs driver ─────────────────────────────────────────────────────────────

/// The boot filesystem driver.  Mounted at `/boot` by `vfs::init`.
///
/// All files are static; no files can be created, modified, or removed.
pub struct BootFs;

impl BootFs {
    pub fn new() -> Self {
        Self
    }
}

impl Default for BootFs {
    fn default() -> Self {
        Self::new()
    }
}

impl VfsDriver for BootFs {
    fn lookup(&self, path: &str) -> SysResult<Arc<dyn VfsNode>> {
        match path {
            "" => Ok(Arc::new(BootDirNode)),
            "version" => Ok(Arc::new(StaticFileNode::new(VERSION_DATA, 10))),
            "motd" => Ok(Arc::new(StaticFileNode::new(MOTD_DATA, 11))),
            _ => Err(Errno::ENOENT),
        }
    }
    // create / mkdir / unlink all use the default EROFS implementation.
}

// ── /boot directory node ──────────────────────────────────────────────────────

struct BootDirNode;

impl VfsNode for BootDirNode {
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
            ino: 9,
        })
    }
    fn readdir(&self, _offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let n = BOOT_DIR_ENTRIES.len().min(buf.len());
        buf[..n].copy_from_slice(&BOOT_DIR_ENTRIES[..n]);
        Ok(n)
    }
}

// ── Static file node ──────────────────────────────────────────────────────────

/// A read-only file node backed by a `&'static [u8]` slice.
struct StaticFileNode {
    data: &'static [u8],
    ino: u64,
}

impl StaticFileNode {
    const fn new(data: &'static [u8], ino: u64) -> Self {
        Self { data, ino }
    }
}

impl VfsNode for StaticFileNode {
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

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use abi::errors::Errno;

    fn lookup(path: &str) -> SysResult<Arc<dyn VfsNode>> {
        BootFs::new().lookup(path)
    }

    #[test]
    fn test_lookup_root_is_dir() {
        let node = lookup("").unwrap();
        let stat = node.stat().unwrap();
        assert!(stat.is_dir());
    }

    #[test]
    fn test_readdir_lists_entries() {
        let node = lookup("").unwrap();
        let mut buf = [0u8; 64];
        let n = node.readdir(0, &mut buf).unwrap();
        assert!(n > 0);
        let s = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(s.contains("version"));
        assert!(s.contains("motd"));
    }

    #[test]
    fn test_lookup_version() {
        let node = lookup("version").unwrap();
        let stat = node.stat().unwrap();
        assert!(stat.is_reg());
        assert!(stat.size > 0);
        let mut buf = [0u8; 64];
        let n = node.read(0, &mut buf).unwrap();
        assert!(n > 0);
        assert!(
            core::str::from_utf8(&buf[..n])
                .unwrap()
                .contains("Thing-OS")
        );
    }

    #[test]
    fn test_lookup_motd() {
        let node = lookup("motd").unwrap();
        let mut buf = [0u8; 128];
        let n = node.read(0, &mut buf).unwrap();
        assert!(n > 0);
    }

    #[test]
    fn test_lookup_unknown_returns_enoent() {
        assert!(matches!(lookup("nonexistent"), Err(Errno::ENOENT)));
    }

    #[test]
    fn test_static_files_are_readonly() {
        let node = lookup("version").unwrap();
        assert!(matches!(node.write(0, b"bad"), Err(Errno::EROFS)));
    }

    #[test]
    fn test_create_returns_erofs() {
        let fs = BootFs::new();
        assert!(matches!(fs.create("newfile"), Err(Errno::EROFS)));
    }

    #[test]
    fn test_unlink_returns_erofs() {
        let fs = BootFs::new();
        assert!(matches!(fs.unlink("version"), Err(Errno::EROFS)));
    }

    #[test]
    fn test_mkdir_returns_erofs() {
        let fs = BootFs::new();
        assert!(matches!(fs.mkdir("newdir"), Err(Errno::EROFS)));
    }

    #[test]
    fn test_read_at_offset() {
        let node = lookup("version").unwrap();
        // Read just the first 5 bytes.
        let mut buf = [0u8; 5];
        let n = node.read(0, &mut buf).unwrap();
        assert_eq!(n, 5);
        // Read past end.
        let mut buf2 = [0u8; 4];
        let n2 = node.read(10000, &mut buf2).unwrap();
        assert_eq!(n2, 0);
    }
}
