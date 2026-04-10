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
pub mod port_node;
pub mod procfs;
pub mod provider;
pub mod ramfs;
pub mod sysfs;
pub mod union;
pub mod watch;

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

/// Kernel-internal file status, analogous to a subset of POSIX `struct stat`.
///
/// All VFS node implementations return this type from their `stat()` method.
/// The syscall handler converts it into the ABI-stable [`abi::fs::FileStat`]
/// before copying to userspace.
///
/// # Timestamp policy (v1)
/// - `atime`: updated on meaningful data read access (coarse policy).
/// - `mtime`: updated when file contents change (write, truncate).
/// - `ctime`: updated when file content or metadata changes.
/// - Synthetic/virtual nodes return zero (epoch) timestamps.
#[derive(Clone, Copy, Debug, Default)]
pub struct VfsStat {
    /// File type and permissions bitmask (same encoding as POSIX st_mode).
    pub mode: u32,
    /// Size in bytes (for regular files; 0 for devices/directories).
    pub size: u64,
    /// Inode-like unique identifier within the filesystem.
    pub ino: u64,
    /// Last access time — seconds since Unix epoch.
    pub atime_sec: u64,
    /// Last access time — nanosecond component (0–999_999_999).
    pub atime_nsec: u32,
    /// Last modification time — seconds since Unix epoch.
    pub mtime_sec: u64,
    /// Last modification time — nanosecond component (0–999_999_999).
    pub mtime_nsec: u32,
    /// Last status-change time — seconds since Unix epoch.
    pub ctime_sec: u64,
    /// Last status-change time — nanosecond component (0–999_999_999).
    pub ctime_nsec: u32,
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

    /// Convert this kernel-internal stat into the ABI-stable [`abi::fs::FileStat`]
    /// suitable for copying to userspace.
    pub fn to_abi_stat(self) -> abi::fs::FileStat {
        abi::fs::FileStat {
            mode: self.mode,
            _mode_pad: 0,
            size: self.size,
            ino: self.ino,
            atime: abi::fs::Timespec::new(self.atime_sec, self.atime_nsec),
            mtime: abi::fs::Timespec::new(self.mtime_sec, self.mtime_nsec),
            ctime: abi::fs::Timespec::new(self.ctime_sec, self.ctime_nsec),
        }
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

    /// Poll this node for readiness.
    /// Returns the current readiness mask (using [`abi::syscall::poll_flags`]).
    fn poll(&self) -> u16 {
        abi::syscall::poll_flags::POLLIN | abi::syscall::poll_flags::POLLOUT
    }

    /// If this node is a port, returns the underlying port.
    fn as_port(&self) -> Option<Arc<crate::ipc::Port>> {
        None
    }

    /// Device-specific control call (ioctl).
    fn device_call(&self, _call: &abi::device::DeviceCall) -> SysResult<usize> {
        Err(abi::errors::Errno::ENOSYS)
    }

    /// Flush any pending writes to the backing store.
    ///
    /// For RAM-backed filesystems this is a no-op that always succeeds.
    /// Drivers with real backing storage should override this to drain their
    /// write buffers and ensure durability.
    fn sync(&self) -> SysResult<()> {
        Ok(())
    }

    /// Add a task to the wait queue for this node.
    fn add_waiter(&self, _tid: u64) {}

    /// Remove a task from the wait queue for this node.
    fn remove_waiter(&self, _tid: u64) {}
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

    /// Rename a file or directory from `old_path` to `new_path`.
    ///
    /// The default implementation returns `EROFS`.
    fn rename(&self, _old_path: &str, _new_path: &str) -> SysResult<()> {
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
pub fn init(modules: &'static [crate::BootModuleDesc]) {
    mount::init();

    // Create the root filesystem (tmpfs) — writable, volatile.
    let root_fs = Arc::new(ramfs::RamFs::new());

    // Pre-populate mount point directories in the root filesystem so they appear in readdir("/")
    let _ = root_fs.mkdir("boot");
    let _ = root_fs.mkdir("bin");
    let _ = root_fs.mkdir("etc");
    let _ = root_fs.mkdir("share");
    let _ = root_fs.mkdir("mnt");
    let _ = root_fs.mkdir("dev");
    let _ = root_fs.mkdir("dev/display");
    crate::kdebug!("VFS: Created /dev/display directory");
    let _ = root_fs.mkdir("dev/input");
    let _ = root_fs.mkdir("proc");
    let _ = root_fs.mkdir("sys");
    let _ = root_fs.mkdir("tmp");
    let _ = root_fs.mkdir("run");
    let _ = root_fs.mkdir("services");
    let _ = root_fs.mkdir("session");

    // Create the root union filesystem.
    let mut root_union = union::UnionFs::new_fallthrough();
    root_union.push(Arc::new(bootfs::BootFs::new(modules))); // Layer 0: Read-only boot modules
    root_union.push(root_fs); // Layer 1: Writable RAM overlay

    mount::mount("/", Arc::new(root_union));
    crate::kdebug!("vfs: mounted union filesystem at / (root)");

    // Device filesystem
    mount::mount("/dev", Arc::new(devfs::DevFs::new()));
    crate::kdebug!("vfs: mounted devfs at /dev");

    // Process info filesystem
    mount::mount("/proc", Arc::new(procfs::ProcFs::new()));
    crate::kdebug!("vfs: mounted procfs at /proc");

    // Kernel device metadata
    mount::mount("/sys", Arc::new(sysfs::SysFs::new()));
    crate::kdebug!("vfs: mounted sysfs at /sys");

    // Temporary filesystem — scratch space for userland.
    mount::mount("/tmp", Arc::new(ramfs::RamFs::new()));
    crate::kdebug!("vfs: mounted tmpfs at /tmp");

    // Transient runtime state
    mount::mount("/run", Arc::new(ramfs::RamFs::new()));
    crate::kdebug!("vfs: mounted tmpfs at /run");

    // Service namespace
    mount::mount("/services", Arc::new(ramfs::RamFs::new()));
    crate::kdebug!("vfs: mounted tmpfs at /services");

    // Session namespace — filesystem-native GUI objects live here.
    mount::mount("/session", Arc::new(ramfs::RamFs::new()));
    crate::kdebug!("vfs: mounted tmpfs at /session");
}

/// Helper for filesystem drivers to implement `readdir`.
///
/// Encodes directory entries as NUL-terminated names into `buf`.  Only entries
/// (or parts of entries) that fall within the virtual byte stream range
/// starting at `offset` are included.
pub fn write_readdir_entries<'a>(
    entries: impl IntoIterator<Item = &'a str>,
    offset: u64,
    buf: &mut [u8],
) -> SysResult<usize> {
    let mut written = 0usize;
    let mut virtual_pos = 0u64;

    for entry in entries {
        let name = entry.as_bytes();
        let entry_full_len = (name.len() + 1) as u64;

        let entry_start = virtual_pos;
        let entry_end = virtual_pos + entry_full_len;

        if entry_end > offset {
            // This entry (or part of it) is within the requested range.
            let start_in_entry = if offset > entry_start {
                (offset - entry_start) as usize
            } else {
                0
            };

            if start_in_entry < name.len() {
                let copy_from_entry = &name[start_in_entry..];
                let n = copy_from_entry.len().min(buf.len() - written);
                buf[written..written + n].copy_from_slice(&copy_from_entry[..n]);
                written += n;

                if n == copy_from_entry.len() && written < buf.len() {
                    buf[written] = 0;
                    written += 1;
                }
            } else if start_in_entry == name.len() && written < buf.len() {
                // Offset requested exactly the NUL byte of this entry.
                buf[written] = 0;
                written += 1;
            }

            if written == buf.len() {
                break;
            }
        }

        virtual_pos = entry_end;
    }

    Ok(written)
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
                ..Default::default()
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
            ..Default::default()
        };
        assert!(dir.is_dir());
        assert!(!dir.is_reg());
        assert!(!dir.is_chr());

        let chr = VfsStat {
            mode: VfsStat::S_IFCHR | 0o666,
            size: 0,
            ino: 2,
            ..Default::default()
        };
        assert!(chr.is_chr());
        assert!(!chr.is_dir());

        let reg = VfsStat {
            mode: VfsStat::S_IFREG | 0o644,
            size: 42,
            ino: 3,
            ..Default::default()
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
