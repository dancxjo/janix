//! VFS mount table.
//!
//! Maintains a list of (`mount_point`, `Arc<dyn VfsDriver>`) pairs sorted
//! longest-prefix-first so that the most specific mount is tried first.
//!
//! # Thread safety
//! The table is protected by a spin-lock.  Mounts happen once at boot; reads
//! happen on every `open(2)` syscall.

use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::Mutex;

use super::VfsDriver;
use abi::errors::{Errno, SysResult};

struct MountEntry {
    /// The canonical mount point, e.g. `"/dev"` (no trailing slash).
    prefix: String,
    driver: Arc<dyn VfsDriver>,
}

static MOUNT_TABLE: Mutex<Vec<MountEntry>> = Mutex::new(Vec::new());
static INIT_DONE: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

/// Initialise the mount table storage.  Must be called once before any
/// [`mount`] or [`lookup`] call.
pub fn init() {
    // The static Mutex<Vec<_>> is already valid; mark init complete.
    INIT_DONE.store(true, core::sync::atomic::Ordering::SeqCst);
}

/// Mount a filesystem driver at `mount_point` (e.g. `"/dev"`).
///
/// Replaces any existing mount at the same point.  Thread-safe.
pub fn mount(mount_point: &str, driver: Arc<dyn VfsDriver>) {
    let prefix = normalise(mount_point);
    let mut table = MOUNT_TABLE.lock();
    // Remove duplicate.
    table.retain(|e| e.prefix != prefix);
    table.push(MountEntry { prefix, driver });
    // Keep longest-prefix first so that `/dev/pts` beats `/dev`.
    table.sort_by(|a, b| b.prefix.len().cmp(&a.prefix.len()));
}

/// Unmount the filesystem at `mount_point`.
///
/// Returns `Err(ENOENT)` if nothing is mounted there.
pub fn umount(mount_point: &str) -> SysResult<()> {
    let prefix = normalise(mount_point);
    let mut table = MOUNT_TABLE.lock();
    let before = table.len();
    table.retain(|e| e.prefix != prefix);
    if table.len() == before {
        Err(Errno::ENOENT)
    } else {
        Ok(())
    }
}

/// Resolve `path` to a VFS node by finding the best-matching mount and
/// calling its `lookup` with the remaining path component(s).
///
/// `path` must be absolute (start with `/`).
pub fn lookup(path: &str) -> SysResult<alloc::sync::Arc<dyn super::VfsNode>> {
    if !path.starts_with('/') {
        return Err(Errno::ENOENT);
    }
    if path == "/dev/fb0" {
        crate::kinfo!("mount::lookup: entering path='{}'", path);
    }
    let table = MOUNT_TABLE.lock();
    for entry in table.iter() {
        if let Some(rel) = strip_prefix(path, &entry.prefix) {
            if path == "/dev/fb0" {
                crate::kinfo!(
                    "mount::lookup: matched prefix='{}' rel='{}'",
                    entry.prefix,
                    rel
                );
            }
            match entry.driver.lookup(rel) {
                Ok(node) => return Ok(node),
                Err(Errno::ENOENT) if path == "/dev/fb0" && entry.prefix == "/dev" => {
                    crate::kwarn!(
                        "mount::lookup: mounted /dev driver returned ENOENT for fb0, trying builtin devfs fallback"
                    );
                    match crate::vfs::devfs::DevFs::new().lookup(rel) {
                        Ok(node) => {
                            crate::kinfo!("mount::lookup: builtin devfs fallback resolved fb0");
                            return Ok(node);
                        }
                        Err(err) => {
                            crate::kwarn!(
                                "mount::lookup: builtin devfs fallback failed for fb0: {:?}",
                                err
                            );
                            return Err(err);
                        }
                    }
                }
                Err(err) => return Err(err),
            }
        }
    }
    if path == "/dev/fb0" {
        crate::kwarn!("mount::lookup: no match for '{}'", path);
    }
    Err(Errno::ENOENT)
}

/// Create a new regular file at `path` by finding the best-matching mount.
///
/// `path` must be absolute.  Returns the new open node on success.
pub fn create(path: &str) -> SysResult<alloc::sync::Arc<dyn super::VfsNode>> {
    if !path.starts_with('/') {
        return Err(Errno::ENOENT);
    }
    let table = MOUNT_TABLE.lock();
    for entry in table.iter() {
        if let Some(rel) = strip_prefix(path, &entry.prefix) {
            return entry.driver.create(rel);
        }
    }
    Err(Errno::ENOENT)
}

/// Create a directory at `path` by finding the best-matching mount.
///
/// `path` must be absolute.
pub fn mkdir(path: &str) -> SysResult<()> {
    if !path.starts_with('/') {
        return Err(Errno::ENOENT);
    }
    let table = MOUNT_TABLE.lock();
    for entry in table.iter() {
        if let Some(rel) = strip_prefix(path, &entry.prefix) {
            return entry.driver.mkdir(rel);
        }
    }
    Err(Errno::ENOENT)
}

/// Remove the file or empty directory at `path`.
///
/// `path` must be absolute.
pub fn unlink(path: &str) -> SysResult<()> {
    if !path.starts_with('/') {
        return Err(Errno::ENOENT);
    }
    let table = MOUNT_TABLE.lock();
    for entry in table.iter() {
        if let Some(rel) = strip_prefix(path, &entry.prefix) {
            return entry.driver.unlink(rel);
        }
    }
    Err(Errno::ENOENT)
}

/// Rename a file or directory from `old_path` to `new_path`.
///
/// Both paths must be absolute and within the same mount point.
pub fn rename(old_path: &str, new_path: &str) -> SysResult<()> {
    if !old_path.starts_with('/') || !new_path.starts_with('/') {
        return Err(Errno::ENOENT);
    }
    let table = MOUNT_TABLE.lock();
    for entry in table.iter() {
        if let Some(old_rel) = strip_prefix(old_path, &entry.prefix) {
            if let Some(new_rel) = strip_prefix(new_path, &entry.prefix) {
                return entry.driver.rename(old_rel, new_rel);
            } else {
                // Cross-mount renaming is not supported.
                return Err(Errno::EXDEV);
            }
        }
    }
    Err(Errno::ENOENT)
}

/// Return a human-readable text listing of all active mount points.
///
/// Format:
/// ```text
/// <mount_point> <type> rw 0 0
/// ```
/// Used by `/proc/mounts`.
pub fn mounts_text() -> alloc::string::String {
    let table = MOUNT_TABLE.lock();
    let mut out = alloc::string::String::new();
    for entry in table.iter() {
        out.push_str(&entry.prefix);
        out.push_str(" vfs rw 0 0\n");
    }
    out
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn normalise(p: &str) -> String {
    // Strip trailing slash unless it is the root itself.
    let s = p.trim_end_matches('/');
    if s.is_empty() {
        String::from("/")
    } else {
        String::from(s)
    }
}

/// Returns the relative portion of `path` after `prefix`, if `path` starts
/// with that prefix followed by `/` or is exactly equal.
fn strip_prefix<'a>(path: &'a str, prefix: &str) -> Option<&'a str> {
    if prefix == "/" {
        // Root mount: everything after the leading slash.
        return Some(&path[1..]);
    }
    if path == prefix {
        return Some("");
    }
    // Avoid allocation: check if path starts with prefix followed by '/'.
    let prefix_with_slash = prefix.trim_end_matches('/');
    let rest = path.strip_prefix(prefix_with_slash)?;
    rest.strip_prefix('/')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vfs::{VfsDriver, VfsNode, VfsStat};
    use abi::errors::Errno;
    use alloc::sync::Arc;
    use alloc::vec;

    struct DummyNode;
    impl VfsNode for DummyNode {
        fn read(&self, _: u64, _: &mut [u8]) -> abi::errors::SysResult<usize> {
            Ok(0)
        }
        fn write(&self, _: u64, _: &[u8]) -> abi::errors::SysResult<usize> {
            Ok(0)
        }
        fn stat(&self) -> abi::errors::SysResult<VfsStat> {
            Ok(VfsStat {
                mode: VfsStat::S_IFCHR | 0o666,
                size: 0,
                ino: 99,
            })
        }
    }

    struct DummyFs;
    impl VfsDriver for DummyFs {
        fn lookup(&self, path: &str) -> abi::errors::SysResult<Arc<dyn VfsNode>> {
            if path == "thing" {
                Ok(Arc::new(DummyNode))
            } else {
                Err(Errno::ENOENT)
            }
        }
    }

    fn fresh_table() {
        MOUNT_TABLE.lock().clear();
        INIT_DONE.store(true, core::sync::atomic::Ordering::SeqCst);
    }

    #[test]
    fn test_mount_and_lookup() {
        fresh_table();
        mount("/test", Arc::new(DummyFs));
        assert!(lookup("/test/thing").is_ok());
    }

    #[test]
    fn test_lookup_unknown_path_returns_enoent() {
        fresh_table();
        mount("/test", Arc::new(DummyFs));
        assert!(matches!(lookup("/other/thing"), Err(Errno::ENOENT)));
    }

    #[test]
    fn test_umount_removes_mount() {
        fresh_table();
        mount("/rm", Arc::new(DummyFs));
        assert!(lookup("/rm/thing").is_ok());
        umount("/rm").unwrap();
        assert!(matches!(lookup("/rm/thing"), Err(Errno::ENOENT)));
    }

    #[test]
    fn test_longer_prefix_wins() {
        fresh_table();

        struct Short;
        impl VfsDriver for Short {
            fn lookup(&self, _: &str) -> abi::errors::SysResult<Arc<dyn VfsNode>> {
                Err(Errno::EIO)
            }
        }
        mount("/a", Arc::new(Short));
        mount("/a/b", Arc::new(DummyFs));

        // "/a/b/thing" should match the longer prefix "/a/b".
        assert!(lookup("/a/b/thing").is_ok());
        // "/a/thing" hits the short driver which returns EIO (not ENOENT).
        assert!(matches!(lookup("/a/thing"), Err(Errno::EIO)));
    }

    #[test]
    fn test_strip_prefix_helper() {
        assert_eq!(strip_prefix("/dev/console", "/dev"), Some("console"));
        assert_eq!(strip_prefix("/dev", "/dev"), Some(""));
        assert_eq!(strip_prefix("/other/path", "/dev"), None);
        assert_eq!(strip_prefix("/hello", "/"), Some("hello"));
    }
}
