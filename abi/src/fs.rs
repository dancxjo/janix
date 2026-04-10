//! VFS file status types for the janix ABI.
//!
//! These types cross the userspace/kernel boundary and must remain stable.
//! They are used by the `SYS_FS_STAT` syscall and the `vfs_stat` stem wrapper.

/// A timestamp with nanosecond precision.
///
/// Represents a point in wall-clock time as seconds since the Unix epoch
/// (January 1, 1970 00:00:00 UTC) plus a nanosecond sub-second component.
///
/// For synthetic/virtual nodes that have no meaningful wall-clock time,
/// this is conventionally set to zero (the Unix epoch).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct Timespec {
    /// Seconds since the Unix epoch.
    pub sec: u64,
    /// Sub-second nanoseconds (0–999_999_999).
    pub nsec: u32,
    /// Reserved padding (must be zero).
    pub _pad: u32,
}

impl Timespec {
    /// The zero/epoch timestamp (1970-01-01 00:00:00 UTC).
    pub const ZERO: Self = Self {
        sec: 0,
        nsec: 0,
        _pad: 0,
    };

    /// Create a `Timespec` from seconds and nanoseconds.
    #[inline]
    pub const fn new(sec: u64, nsec: u32) -> Self {
        Self { sec, nsec, _pad: 0 }
    }
}

/// File status structure returned by the `stat`/`fstat` syscall (`SYS_FS_STAT`).
///
/// All fields are stable across the userspace/kernel ABI boundary.
///
/// # Timestamp policy
/// - `atime`: updated on meaningful data read access (coarse v1 policy).
/// - `mtime`: updated when file contents change (write, truncate).
/// - `ctime`: updated when file content or metadata changes.
/// - For synthetic/virtual nodes, timestamps are set to zero (epoch).
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct FileStat {
    /// File type and permissions bitmask (same encoding as POSIX `st_mode`).
    pub mode: u32,
    /// Reserved padding for alignment.
    pub _mode_pad: u32,
    /// File size in bytes (0 for devices/directories).
    pub size: u64,
    /// Inode-like unique identifier within the filesystem.
    pub ino: u64,
    /// Last access time.
    pub atime: Timespec,
    /// Last modification time (content changed).
    pub mtime: Timespec,
    /// Last status change time (content or metadata changed).
    pub ctime: Timespec,
}
