//! VFS (Virtual Filesystem) core for janix.
//!
//! Implements Act III of the de-graphing migration: a minimal path-based
//! kernel VFS. Provides:
//!
//! - [`VfsNode`]: trait for files/devices/directories
//! - [`VfsDriver`]: trait for filesystem backends
//! - [`VfsStat`]: file metadata
//! - [`OpenFlags`]: open(2) flags
//! - Global mount table (see [`mount`])
//! - Per-process file descriptor table (see [`fd_table`])
//! - Built-in devfs backend (see [`devfs`])
//!
//! # North Star
//! A component is *integrated* when it is reachable via a path, can be
//! opened, and can be read, written, or polled. Nothing else is required.

pub mod devfs;
pub mod fd_table;
pub mod mount;

use alloc::sync::Arc;
use abi::errors::{Errno, SysResult};

// ── Open flags ─────────────────────────────────────────────────────────────

/// Subset of POSIX open(2) flags understood by the VFS.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OpenFlags(pub u32);

impl OpenFlags {
    pub fn read_only() -> Self {
        Self(abi::syscall::vfs_flags::O_RDONLY)
    }
    pub fn write_only() -> Self {
        Self(abi::syscall::vfs_flags::O_WRONLY)
    }
    pub fn read_write() -> Self {
        Self(abi::syscall::vfs_flags::O_RDWR)
    }

    pub fn is_readable(self) -> bool {
        let access = self.0 & 0x3;
        access == abi::syscall::vfs_flags::O_RDONLY || access == abi::syscall::vfs_flags::O_RDWR
    }
    pub fn is_writable(self) -> bool {
        let access = self.0 & 0x3;
        access == abi::syscall::vfs_flags::O_WRONLY || access == abi::syscall::vfs_flags::O_RDWR
    }
    pub fn is_nonblock(self) -> bool {
        self.0 & abi::syscall::vfs_flags::O_NONBLOCK != 0
    }
    pub fn is_append(self) -> bool {
        self.0 & abi::syscall::vfs_flags::O_APPEND != 0
    }
}

// ── File metadata ───────────────────────────────────────────────────────────

/// Minimal file status, analogous to a subset of POSIX `struct stat`.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct VfsStat {
    /// File type and permissions bitmask (same encoding as POSIX st_mode).
    pub mode: u32,
    /// Size in bytes (for regular files; 0 for devices/directories).
    pub size: u64,
    /// Inode-like unique identifier within the filesystem.
    pub ino: u64,
}

impl VfsStat {
    pub const S_IFMT: u32 = 0o170000;
    pub const S_IFREG: u32 = 0o100000;
    pub const S_IFDIR: u32 = 0o040000;
    pub const S_IFCHR: u32 = 0o020000;

    pub fn is_dir(self) -> bool {
        self.mode & Self::S_IFMT == Self::S_IFDIR
    }
    pub fn is_chr(self) -> bool {
        self.mode & Self::S_IFMT == Self::S_IFCHR
    }
    pub fn is_reg(self) -> bool {
        self.mode & Self::S_IFMT == Self::S_IFREG
    }
}

// ── VfsNode ─────────────────────────────────────────────────────────────────

/// A single open file or device inside the VFS.
///
/// Implementations are expected to be `Send + Sync` so they can be stored
/// in the kernel FD table (which may be accessed from any CPU).
pub trait VfsNode: Send + Sync {
    /// Read up to `buf.len()` bytes starting at `offset` into `buf`.
    /// Returns the number of bytes read, or 0 at EOF.
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize>;

    /// Write `buf` starting at `offset`.
    /// Returns the number of bytes written.
    fn write(&self, offset: u64, buf: &[u8]) -> SysResult<usize>;

    /// Return metadata for this node.
    fn stat(&self) -> SysResult<VfsStat>;

    /// Called when the last reference to an open file is dropped.
    /// Default: no-op.
    fn close(&self) {}

    /// Read directory entries into `buf` starting at `offset`.
    /// Returns bytes written into `buf`, or 0 when exhausted.
    /// Only meaningful for directory nodes; regular files return `ENOTDIR`.
    fn readdir(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::ENOTDIR)
    }
}

// ── VfsDriver ───────────────────────────────────────────────────────────────

/// A filesystem backend that can resolve path components to [`VfsNode`]s.
///
/// `VfsDriver::lookup` is called with the path *relative to the mount point*.
/// For example, if `/dev` is mounted and the user opens `/dev/console`, the
/// driver receives `"console"`.
pub trait VfsDriver: Send + Sync {
    /// Look up `path` within this filesystem and return an open node.
    fn lookup(&self, path: &str) -> SysResult<Arc<dyn VfsNode>>;
}

// ── Global init ──────────────────────────────────────────────────────────────

/// Initialise the VFS subsystem and mount built-in filesystems.
///
/// Called once from `kernel::start` during early boot, *before* any user
/// processes are spawned.
pub fn init() {
    mount::init();
    // Mount devfs at /dev
    mount::mount(
        "/dev",
        Arc::new(devfs::DevFs::new()),
    );
    crate::kinfo!("vfs: mounted devfs at /dev");
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;
    use alloc::vec;

    // A trivial in-memory node for unit-testing the VFS layer.
    struct MemNode {
        data: alloc::vec::Vec<u8>,
        mode: u32,
    }

    impl VfsNode for MemNode {
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
                mode: self.mode,
                size: self.data.len() as u64,
                ino: 1,
            })
        }
    }

    #[test]
    fn test_open_flags_access() {
        let ro = OpenFlags::read_only();
        assert!(ro.is_readable());
        assert!(!ro.is_writable());

        let wo = OpenFlags::write_only();
        assert!(!wo.is_readable());
        assert!(wo.is_writable());

        let rw = OpenFlags::read_write();
        assert!(rw.is_readable());
        assert!(rw.is_writable());
    }

    #[test]
    fn test_vfs_stat_type_bits() {
        let dir = VfsStat { mode: VfsStat::S_IFDIR | 0o755, size: 0, ino: 1 };
        assert!(dir.is_dir());
        assert!(!dir.is_reg());
        assert!(!dir.is_chr());

        let chr = VfsStat { mode: VfsStat::S_IFCHR | 0o666, size: 0, ino: 2 };
        assert!(chr.is_chr());
        assert!(!chr.is_dir());

        let reg = VfsStat { mode: VfsStat::S_IFREG | 0o644, size: 42, ino: 3 };
        assert!(reg.is_reg());
    }

    #[test]
    fn test_mem_node_read_partial() {
        let node = MemNode {
            data: vec![1, 2, 3, 4, 5],
            mode: VfsStat::S_IFREG | 0o444,
        };
        let mut buf = [0u8; 3];
        let n = node.read(0, &mut buf).unwrap();
        assert_eq!(n, 3);
        assert_eq!(&buf, &[1, 2, 3]);
    }

    #[test]
    fn test_mem_node_read_at_offset() {
        let node = MemNode {
            data: vec![10, 20, 30],
            mode: VfsStat::S_IFREG | 0o444,
        };
        let mut buf = [0u8; 2];
        let n = node.read(1, &mut buf).unwrap();
        assert_eq!(n, 2);
        assert_eq!(&buf, &[20, 30]);
    }

    #[test]
    fn test_mem_node_read_eof() {
        let node = MemNode { data: vec![], mode: VfsStat::S_IFREG | 0o444 };
        let mut buf = [0u8; 4];
        let n = node.read(0, &mut buf).unwrap();
        assert_eq!(n, 0);
    }
}
