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

pub mod bootfs;
pub mod devfs;
pub mod fd_table;
pub mod memfd;
pub mod mount;
pub mod path;
pub mod procfs;
pub mod provider;
pub mod ramfs;
pub mod sysfs;
pub mod union;

use abi::errors::{Errno, SysResult};
use alloc::sync::Arc;

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
    pub const S_IFIFO: u32 = 0o010000;

    pub fn is_dir(self) -> bool {
        self.mode & Self::S_IFMT == Self::S_IFDIR
    }
    pub fn is_chr(self) -> bool {
        self.mode & Self::S_IFMT == Self::S_IFCHR
    }
    pub fn is_reg(self) -> bool {
        self.mode & Self::S_IFMT == Self::S_IFREG
    }
    pub fn is_fifo(self) -> bool {
        self.mode & Self::S_IFMT == Self::S_IFIFO
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

    /// Truncate the file to `new_size` bytes.
    ///
    /// If `new_size` is less than the current size, the extra data is discarded.
    /// If `new_size` is greater, the file is extended with zero bytes.
    /// Default: returns `EROFS` (read-only / non-truncatable).
    fn truncate(&self, _new_size: u64) -> SysResult<()> {
        Err(abi::errors::Errno::EROFS)
    }

    /// Read directory entries into `buf` starting at `offset`.
    /// Returns bytes written into `buf`, or 0 when exhausted.
    /// Only meaningful for directory nodes; regular files return `ENOTDIR`.
    fn readdir(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::ENOTDIR)
    }

    /// Return the exact physical memory backing this node, if it is directly memory-mapped.
    /// Used for zero-copy userspace memory mapping of devices and shm buffers.
    /// Returns physical base address and length in bytes.
    fn phys_region(&self) -> SysResult<(u64, usize)> {
        Err(Errno::ENOSYS)
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

    /// Create a new regular file at `path` and return it as an open node.
    ///
    /// The default implementation returns `EROFS`, indicating a read-only
    /// filesystem.  Writable filesystems (ramfs/tmpfs) should override this.
    fn create(&self, _path: &str) -> SysResult<Arc<dyn VfsNode>> {
        Err(abi::errors::Errno::EROFS)
    }

    /// Create a directory at `path`.
    ///
    /// The default implementation returns `EROFS`.
    fn mkdir(&self, _path: &str) -> SysResult<()> {
        Err(abi::errors::Errno::EROFS)
    }

    /// Remove the file or empty directory at `path`.
    ///
    /// The default implementation returns `EROFS`.
    fn unlink(&self, _path: &str) -> SysResult<()> {
        Err(abi::errors::Errno::EROFS)
    }
}

// ── Namespace ────────────────────────────────────────────────────────────────

/// A reference to the VFS namespace (mount table view) for a process.
///
/// **Design stub for ACT III**: all processes share a single global namespace.
/// Per-process namespace divergence (sandboxing, containers) will be
/// introduced in a later act once the process registry is wired in.
///
/// Carrying this type in [`crate::task::ProcessInfo`] now makes it possible
/// to plumb per-process namespaces without changing the call sites later.
#[derive(Clone, Debug, Default)]
pub struct NamespaceRef;

impl NamespaceRef {
    /// Return the shared (global) namespace reference.
    pub fn global() -> Self {
        Self
    }
}

// ── Global init ──────────────────────────────────────────────────────────────

/// Initialise the VFS subsystem and mount built-in filesystems.
///
/// Called once from `kernel::start` during early boot, *before* any user
/// processes are spawned.
///
/// Boot mounts:
/// - `/`         ← boot filesystem (static read-only initramfs)
/// - `/`         ← root tmpfs (layered over bootfs via union)
/// - `/dev`      ← device filesystem
/// - `/proc`     ← process info (stub)
/// - `/sys`      ← kernel device discovery metadata
/// - `/tmp`      ← temporary filesystem (writable, volatile)
/// - `/run`      ← transient runtime state (tmpfs)
/// - `/services` ← populated by userland daemons (tmpfs stub for now)
pub fn init() {
    mount::init();

    // Boot filesystem — minimal static tree available before anything else.
    mount::mount("/boot", Arc::new(bootfs::BootFs::new()));
    crate::kinfo!("vfs: mounted bootfs at /boot");

    // Root filesystem (tmpfs) — writable, volatile.
    mount::mount("/", Arc::new(ramfs::RamFs::new()));
    crate::kinfo!("vfs: mounted tmpfs at /");

    // Device filesystem
    mount::mount("/dev", Arc::new(devfs::DevFs::new()));
    crate::kinfo!("vfs: mounted devfs at /dev");

    // Process info filesystem
    mount::mount("/proc", Arc::new(procfs::ProcFs::new()));
    crate::kinfo!("vfs: mounted procfs at /proc");

    // Kernel device metadata
    mount::mount("/sys", Arc::new(sysfs::SysFs::new()));
    crate::kinfo!("vfs: mounted sysfs at /sys");

    // Temporary filesystem — scratch space for userland.
    mount::mount("/tmp", Arc::new(ramfs::RamFs::new()));
    crate::kinfo!("vfs: mounted tmpfs at /tmp");

    // Transient runtime state
    mount::mount("/run", Arc::new(ramfs::RamFs::new()));
    crate::kinfo!("vfs: mounted tmpfs at /run");

    // Service namespace — populated by userland daemons (ACT V)
    mount::mount("/services", Arc::new(ramfs::RamFs::new()));
    crate::kinfo!("vfs: mounted tmpfs at /services");
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
        let dir = VfsStat {
            mode: VfsStat::S_IFDIR | 0o755,
            size: 0,
            ino: 1,
        };
        assert!(dir.is_dir());
        assert!(!dir.is_reg());
        assert!(!dir.is_chr());

        let chr = VfsStat {
            mode: VfsStat::S_IFCHR | 0o666,
            size: 0,
            ino: 2,
        };
        assert!(chr.is_chr());
        assert!(!chr.is_dir());

        let reg = VfsStat {
            mode: VfsStat::S_IFREG | 0o644,
            size: 42,
            ino: 3,
        };
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
        let node = MemNode {
            data: vec![],
            mode: VfsStat::S_IFREG | 0o444,
        };
        let mut buf = [0u8; 4];
        let n = node.read(0, &mut buf).unwrap();
        assert_eq!(n, 0);
    }
}
