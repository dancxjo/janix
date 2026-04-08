//! ramfs — volatile in-memory filesystem.
//!
//! Provides a simple, non-persistent filesystem backed entirely by kernel
//! heap memory.  Used for the VFS root (`/`) and for the transient runtime
//! state directory (`/run`).
//!
//! # Design
//! - Directories are `BTreeMap<name, Arc<RamfsEntry>>` nodes.
//! - Files are `Vec<u8>` payloads behind a `Mutex`.
//! - `RamFs` implements [`VfsDriver`] and resolves paths relative to the
//!   mount point.  Path components are split on `/`; empty components and `.`
//!   are skipped; `..` is not supported at the driver level (handled by the
//!   path resolution engine in [`super::path`]).
//! - All operations are `no_std` compatible.

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::Mutex;

use abi::errors::{Errno, SysResult};

use super::{VfsDriver, VfsNode, VfsStat};

// ── Inode counter ────────────────────────────────────────────────────────────

static NEXT_INO: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(10);

fn alloc_ino() -> u64 {
    NEXT_INO.fetch_add(1, core::sync::atomic::Ordering::Relaxed)
}

// ── Internal tree node ───────────────────────────────────────────────────────

enum RamfsEntry {
    File(Mutex<Vec<u8>>, u64 /* ino */),
    Dir(Mutex<BTreeMap<String, Arc<RamfsEntry>>>, u64 /* ino */),
}

impl RamfsEntry {
    fn new_dir() -> Arc<Self> {
        Arc::new(RamfsEntry::Dir(Mutex::new(BTreeMap::new()), alloc_ino()))
    }

    fn new_file(data: Vec<u8>) -> Arc<Self> {
        Arc::new(RamfsEntry::File(Mutex::new(data), alloc_ino()))
    }

    fn ino(&self) -> u64 {
        match self {
            RamfsEntry::File(_, ino) => *ino,
            RamfsEntry::Dir(_, ino) => *ino,
        }
    }

    /// Look up a child by name inside a directory entry.
    fn lookup_child(&self, name: &str) -> SysResult<Arc<RamfsEntry>> {
        match self {
            RamfsEntry::Dir(children, _) => children.lock().get(name).cloned().ok_or(Errno::ENOENT),
            _ => Err(Errno::ENOTDIR),
        }
    }

    /// Insert a child into a directory entry.
    fn insert_child(&self, name: &str, child: Arc<RamfsEntry>) -> SysResult<()> {
        match self {
            RamfsEntry::Dir(children, _) => {
                children.lock().insert(name.to_string(), child);
                Ok(())
            }
            _ => Err(Errno::ENOTDIR),
        }
    }
}

// ── RamfsNode — VfsNode wrapper ──────────────────────────────────────────────

struct RamfsNode(Arc<RamfsEntry>);

impl VfsNode for RamfsNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        match &*self.0 {
            RamfsEntry::File(data, _) => {
                let data = data.lock();
                let off = offset as usize;
                if off >= data.len() {
                    return Ok(0);
                }
                let avail = &data[off..];
                let n = avail.len().min(buf.len());
                buf[..n].copy_from_slice(&avail[..n]);
                Ok(n)
            }
            RamfsEntry::Dir(_, _) => Err(Errno::EISDIR),
        }
    }

    fn write(&self, offset: u64, buf: &[u8]) -> SysResult<usize> {
        match &*self.0 {
            RamfsEntry::File(data, _) => {
                let mut data = data.lock();
                let off = offset as usize;
                let end = off + buf.len();
                if end > data.len() {
                    data.resize(end, 0);
                }
                data[off..end].copy_from_slice(buf);
                Ok(buf.len())
            }
            RamfsEntry::Dir(_, _) => Err(Errno::EISDIR),
        }
    }

    fn stat(&self) -> SysResult<VfsStat> {
        match &*self.0 {
            RamfsEntry::File(data, ino) => Ok(VfsStat {
                mode: VfsStat::S_IFREG | 0o644,
                size: data.lock().len() as u64,
                ino: *ino,
            }),
            RamfsEntry::Dir(_, ino) => Ok(VfsStat {
                mode: VfsStat::S_IFDIR | 0o755,
                size: 0,
                ino: *ino,
            }),
        }
    }

    fn truncate(&self, new_size: u64) -> SysResult<()> {
        match &*self.0 {
            RamfsEntry::File(data, _) => {
                let mut data = data.lock();
                data.resize(new_size as usize, 0);
                Ok(())
            }
            RamfsEntry::Dir(_, _) => Err(Errno::EISDIR),
        }
    }

    fn readdir(&self, _offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        match &*self.0 {
            RamfsEntry::Dir(children, _) => {
                // Encode directory entries as NUL-terminated names.
                let lock = children.lock();
                let mut written = 0usize;
                for name in lock.keys() {
                    let bytes = name.as_bytes();
                    if written + bytes.len() + 1 > buf.len() {
                        break;
                    }
                    buf[written..written + bytes.len()].copy_from_slice(bytes);
                    written += bytes.len();
                    buf[written] = 0;
                    written += 1;
                }
                Ok(written)
            }
            _ => Err(Errno::ENOTDIR),
        }
    }

    fn poll(&self) -> u16 {
        use abi::syscall::poll_flags::*;
        match &*self.0 {
            RamfsEntry::File(_, _) => POLLIN | POLLOUT,
            RamfsEntry::Dir(_, _) => POLLIN | POLLOUT,
        }
    }
}

// ── RamFs — VfsDriver ────────────────────────────────────────────────────────

/// In-memory filesystem driver.
///
/// Call [`RamFs::new`] to create an empty root directory.  Files and
/// directories can be pre-populated with [`RamFs::mkdir`] and
/// [`RamFs::create_file`].
pub struct RamFs {
    root: Arc<RamfsEntry>,
}

impl RamFs {
    /// Create a new, empty ramfs with a root directory.
    pub fn new() -> Self {
        Self {
            root: RamfsEntry::new_dir(),
        }
    }

    /// Create a subdirectory `path` (relative to this filesystem's root).
    ///
    /// Intermediate directories are created as needed.  It is not an error if
    /// the directory already exists.
    pub fn mkdir(&self, path: &str) -> SysResult<()> {
        let mut current = self.root.clone();
        for component in path.split('/').filter(|c| !c.is_empty()) {
            let next = match current.lookup_child(component) {
                Ok(child) => child,
                Err(Errno::ENOENT) => {
                    let new_dir = RamfsEntry::new_dir();
                    current.insert_child(component, new_dir.clone())?;
                    new_dir
                }
                Err(e) => return Err(e),
            };
            current = next;
        }
        Ok(())
    }

    /// Create or overwrite a file at `path` with `data`.
    pub fn create_file(&self, path: &str, data: Vec<u8>) -> SysResult<()> {
        let (dir_path, file_name) = split_last(path).ok_or(Errno::EINVAL)?;
        let dir = self.resolve_entry(dir_path)?;
        let file_entry = RamfsEntry::new_file(data);
        dir.insert_child(file_name, file_entry)
    }

    /// Resolve `path` to its `RamfsEntry`, walking the tree.
    fn resolve_entry(&self, path: &str) -> SysResult<Arc<RamfsEntry>> {
        let mut current = self.root.clone();
        for component in path.split('/').filter(|c| !c.is_empty()) {
            current = current.lookup_child(component)?;
        }
        Ok(current)
    }
}

impl Default for RamFs {
    fn default() -> Self {
        Self::new()
    }
}

impl VfsDriver for RamFs {
    /// Look up `path` (relative to the mount point) in this filesystem.
    fn lookup(&self, path: &str) -> SysResult<Arc<dyn VfsNode>> {
        let entry = self.resolve_entry(path)?;
        Ok(Arc::new(RamfsNode(entry)))
    }

    /// Create a new empty regular file at `path`.
    ///
    /// Intermediate directories must already exist.  Returns the new node
    /// as an open file descriptor suitable for immediate writing.
    fn create(&self, path: &str) -> SysResult<Arc<dyn VfsNode>> {
        let (dir_path, file_name) = split_last(path).ok_or(Errno::EINVAL)?;
        let dir = self.resolve_entry(dir_path)?;
        let file_entry = RamfsEntry::new_file(alloc::vec::Vec::new());
        dir.insert_child(file_name, file_entry.clone())?;
        Ok(Arc::new(RamfsNode(file_entry)))
    }

    /// Create a directory at `path`.
    ///
    /// Intermediate directories are created as needed (like `mkdir -p`).
    fn mkdir(&self, path: &str) -> SysResult<()> {
        // Inline the inherent mkdir logic to avoid ambiguous self.mkdir() dispatch.
        let mut current = self.root.clone();
        for component in path.split('/').filter(|c| !c.is_empty()) {
            let next = match current.lookup_child(component) {
                Ok(child) => child,
                Err(Errno::ENOENT) => {
                    let new_dir = RamfsEntry::new_dir();
                    current.insert_child(component, new_dir.clone())?;
                    new_dir
                }
                Err(e) => return Err(e),
            };
            current = next;
        }
        Ok(())
    }

    /// Remove the file or empty directory at `path`.
    fn unlink(&self, path: &str) -> SysResult<()> {
        let (dir_path, file_name) = split_last(path).ok_or(Errno::EINVAL)?;
        let dir = self.resolve_entry(dir_path)?;
        match &*dir {
            RamfsEntry::Dir(children, _) => {
                let mut lock = children.lock();
                if lock.remove(file_name).is_some() {
                    Ok(())
                } else {
                    Err(Errno::ENOENT)
                }
            }
            _ => Err(Errno::ENOTDIR),
        }
    }

    /// Rename a file or directory from `old_path` to `new_path`.
    fn rename(&self, old_path: &str, new_path: &str) -> SysResult<()> {
        let (old_dir_path, old_name) = split_last(old_path).ok_or(Errno::EINVAL)?;
        let (new_dir_path, new_name) = split_last(new_path).ok_or(Errno::EINVAL)?;

        let old_dir = self.resolve_entry(old_dir_path)?;
        let new_dir = self.resolve_entry(new_dir_path)?;

        let old_children = match &*old_dir {
            RamfsEntry::Dir(c, _) => c,
            _ => return Err(Errno::ENOTDIR),
        };
        let new_children = match &*new_dir {
            RamfsEntry::Dir(c, _) => c,
            _ => return Err(Errno::ENOTDIR),
        };

        if Arc::ptr_eq(&old_dir, &new_dir) {
            let mut lock = old_children.lock();
            let entry = lock.remove(old_name).ok_or(Errno::ENOENT)?;
            lock.insert(new_name.to_string(), entry);
        } else {
            // Cross-directory rename within the same ramfs instance.
            // Lock in a stable order by pointer address to avoid deadlocks.
            let ptr_old = old_children as *const _ as usize;
            let ptr_new = new_children as *const _ as usize;

            if ptr_old < ptr_new {
                let mut lock_old = old_children.lock();
                let mut lock_new = new_children.lock();
                let entry = lock_old.remove(old_name).ok_or(Errno::ENOENT)?;
                lock_new.insert(new_name.to_string(), entry);
            } else {
                let mut lock_new = new_children.lock();
                let mut lock_old = old_children.lock();
                let entry = lock_old.remove(old_name).ok_or(Errno::ENOENT)?;
                lock_new.insert(new_name.to_string(), entry);
            }
        }

        Ok(())
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Split a path into `(parent_dir, last_component)`.
/// Returns `None` if `path` is empty or contains no filename.
fn split_last(path: &str) -> Option<(&str, &str)> {
    let trimmed = path.trim_matches('/');
    if trimmed.is_empty() {
        return None;
    }
    match trimmed.rfind('/') {
        Some(idx) => {
            let parent = &trimmed[..idx]; // parent dir (no trailing slash)
            let name = &trimmed[idx + 1..];
            if name.is_empty() {
                None
            } else {
                Some((parent, name))
            }
        }
        None => Some(("", trimmed)),
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_lookup_root_is_dir() {
        let fs = RamFs::new();
        let node = fs.lookup("").unwrap();
        let stat = node.stat().unwrap();
        assert!(stat.is_dir());
    }

    #[test]
    fn test_lookup_nonexistent_returns_enoent() {
        let fs = RamFs::new();
        assert!(matches!(fs.lookup("missing"), Err(Errno::ENOENT)));
    }

    #[test]
    fn test_create_and_read_file() {
        let fs = RamFs::new();
        fs.create_file("hello.txt", b"hello".to_vec()).unwrap();
        let node = fs.lookup("hello.txt").unwrap();
        let mut buf = [0u8; 5];
        let n = node.read(0, &mut buf).unwrap();
        assert_eq!(n, 5);
        assert_eq!(&buf, b"hello");
    }

    #[test]
    fn test_write_and_read_back() {
        let fs = RamFs::new();
        fs.create_file("data.bin", vec![]).unwrap();
        let node = fs.lookup("data.bin").unwrap();
        let n = node.write(0, b"test").unwrap();
        assert_eq!(n, 4);
        let mut buf = [0u8; 4];
        let n = node.read(0, &mut buf).unwrap();
        assert_eq!(n, 4);
        assert_eq!(&buf, b"test");
    }

    #[test]
    fn test_mkdir_and_lookup_subdir() {
        let fs = RamFs::new();
        fs.mkdir("sub").unwrap();
        let node = fs.lookup("sub").unwrap();
        let stat = node.stat().unwrap();
        assert!(stat.is_dir());
    }

    #[test]
    fn test_file_in_subdir() {
        let fs = RamFs::new();
        fs.mkdir("sub").unwrap();
        fs.create_file("sub/file.txt", b"data".to_vec()).unwrap();
        let node = fs.lookup("sub/file.txt").unwrap();
        let mut buf = [0u8; 4];
        node.read(0, &mut buf).unwrap();
        assert_eq!(&buf, b"data");
    }

    #[test]
    fn test_read_dir_returns_entries() {
        let fs = RamFs::new();
        fs.mkdir("alpha").unwrap();
        fs.mkdir("beta").unwrap();
        let root = fs.lookup("").unwrap();
        let mut buf = [0u8; 64];
        let n = root.readdir(0, &mut buf).unwrap();
        assert!(n > 0, "readdir should return some bytes");
        let content = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(content.contains("alpha"));
        assert!(content.contains("beta"));
    }

    #[test]
    fn test_write_extends_file() {
        let fs = RamFs::new();
        fs.create_file("grow.bin", vec![0u8; 4]).unwrap();
        let node = fs.lookup("grow.bin").unwrap();
        // Write past original end.
        node.write(4, b"more").unwrap();
        let stat = node.stat().unwrap();
        assert_eq!(stat.size, 8);
    }

    // ── VfsDriver trait methods ──────────────────────────────────────────────

    #[test]
    fn test_driver_create_makes_empty_file() {
        let fs = RamFs::new();
        let node = fs.create("newfile.txt").unwrap();
        let stat = node.stat().unwrap();
        assert!(stat.is_reg());
        assert_eq!(stat.size, 0);
    }

    #[test]
    fn test_driver_create_file_is_writable() {
        let fs = RamFs::new();
        let node = fs.create("write_me.txt").unwrap();
        let n = node.write(0, b"hello").unwrap();
        assert_eq!(n, 5);
        // The file must also be retrievable via lookup.
        let node2 = fs.lookup("write_me.txt").unwrap();
        let mut buf = [0u8; 5];
        let n = node2.read(0, &mut buf).unwrap();
        assert_eq!(n, 5);
        assert_eq!(&buf, b"hello");
    }

    #[test]
    fn test_driver_create_invalid_empty_path() {
        let fs = RamFs::new();
        // An empty path has no file name component → EINVAL.
        assert!(matches!(fs.create(""), Err(Errno::EINVAL)));
    }

    #[test]
    fn test_driver_mkdir_creates_directory() {
        let fs = RamFs::new();
        fs.mkdir("newdir").unwrap();
        let node = fs.lookup("newdir").unwrap();
        assert!(node.stat().unwrap().is_dir());
    }

    #[test]
    fn test_driver_mkdir_nested() {
        let fs = RamFs::new();
        fs.mkdir("a/b/c").unwrap();
        let node = fs.lookup("a/b/c").unwrap();
        assert!(node.stat().unwrap().is_dir());
    }

    #[test]
    fn test_driver_unlink_removes_file() {
        let fs = RamFs::new();
        fs.create_file("to_delete.txt", b"bye".to_vec()).unwrap();
        // File exists.
        assert!(fs.lookup("to_delete.txt").is_ok());
        // Unlink it.
        fs.unlink("to_delete.txt").unwrap();
        // File is gone.
        assert!(matches!(fs.lookup("to_delete.txt"), Err(Errno::ENOENT)));
    }

    #[test]
    fn test_driver_unlink_nonexistent_returns_enoent() {
        let fs = RamFs::new();
        assert!(matches!(fs.unlink("ghost.txt"), Err(Errno::ENOENT)));
    }

    #[test]
    fn test_driver_unlink_empty_path_returns_einval() {
        let fs = RamFs::new();
        assert!(matches!(fs.unlink(""), Err(Errno::EINVAL)));
    }

    // ── truncate ────────────────────────────────────────────────────────────

    #[test]
    fn test_truncate_shrinks_file() {
        let fs = RamFs::new();
        fs.create_file("shrink.bin", b"hello".to_vec()).unwrap();
        let node = fs.lookup("shrink.bin").unwrap();
        node.truncate(2).unwrap();
        assert_eq!(node.stat().unwrap().size, 2);
        let mut buf = [0u8; 5];
        let n = node.read(0, &mut buf).unwrap();
        assert_eq!(n, 2);
        assert_eq!(&buf[..2], b"he");
    }

    #[test]
    fn test_truncate_extends_file_with_zeros() {
        let fs = RamFs::new();
        fs.create_file("grow.bin", b"hi".to_vec()).unwrap();
        let node = fs.lookup("grow.bin").unwrap();
        node.truncate(5).unwrap();
        assert_eq!(node.stat().unwrap().size, 5);
        let mut buf = [0xFFu8; 5];
        let n = node.read(0, &mut buf).unwrap();
        assert_eq!(n, 5);
        assert_eq!(&buf, b"hi\0\0\0");
    }

    #[test]
    fn test_truncate_to_zero() {
        let fs = RamFs::new();
        fs.create_file("zero.bin", b"data".to_vec()).unwrap();
        let node = fs.lookup("zero.bin").unwrap();
        node.truncate(0).unwrap();
        assert_eq!(node.stat().unwrap().size, 0);
        let mut buf = [0u8; 4];
        let n = node.read(0, &mut buf).unwrap();
        assert_eq!(n, 0);
    }

    #[test]
    fn test_truncate_on_dir_returns_eisdir() {
        let fs = RamFs::new();
        fs.mkdir("adir").unwrap();
        let node = fs.lookup("adir").unwrap();
        assert!(matches!(node.truncate(0), Err(Errno::EISDIR)));
    }
}
