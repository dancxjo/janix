//! Per-process VFS file descriptor table.
//!
//! Each process that uses VFS syscalls has an `FdTable` embedded inside its
//! `ProcessInfo`.  Kernel threads do not have an `FdTable` and all VFS
//! syscalls return `ENOENT` for them.
//!
//! File descriptors are non-negative integers starting at 0.  Descriptors 0,
//! 1, and 2 are **not** pre-allocated here — they are handled by the existing
//! `StdioBinding` mechanism so that legacy code continues to work unchanged.
//! VFS descriptors therefore start at 3.
//!
//! # Limits
//! `MAX_FDS` open files per process.  This is intentionally small for now.

use alloc::sync::Arc;
use abi::errors::{Errno, SysResult};

use super::{OpenFlags, VfsNode};

pub const MAX_FDS: usize = 256;

/// A single open-file entry in the FD table.
pub struct OpenFile {
    pub node: Arc<dyn VfsNode>,
    pub flags: OpenFlags,
    /// Current read/write position.
    pub offset: u64,
}

/// Per-process file descriptor table.
pub struct FdTable {
    entries: [Option<OpenFile>; MAX_FDS],
}

impl FdTable {
    pub fn new() -> Self {
        // Option<OpenFile> is not Copy so we can't use array initialisation
        // directly; build it via a const pointer.
        // SAFETY: None variant has no resources; it is safe to zero-initialise.
        let entries = core::array::from_fn(|_| None);
        Self { entries }
    }

    /// Insert `node` into the table and return the allocated file descriptor.
    ///
    /// Descriptors 0–2 are reserved for stdio; allocation starts at 3.
    /// Returns `EMFILE` when all slots are exhausted.
    pub fn open(&mut self, node: Arc<dyn VfsNode>, flags: OpenFlags) -> SysResult<u32> {
        for i in 3..MAX_FDS {
            if self.entries[i].is_none() {
                self.entries[i] = Some(OpenFile { node, flags, offset: 0 });
                return Ok(i as u32);
            }
        }
        Err(Errno::EMFILE)
    }

    /// Return a reference to the open-file entry for `fd`, or `EBADF`.
    pub fn get(&self, fd: u32) -> SysResult<&OpenFile> {
        let idx = fd as usize;
        if idx >= MAX_FDS {
            return Err(Errno::EBADF);
        }
        self.entries[idx].as_ref().ok_or(Errno::EBADF)
    }

    /// Return a mutable reference to the open-file entry for `fd`, or `EBADF`.
    pub fn get_mut(&mut self, fd: u32) -> SysResult<&mut OpenFile> {
        let idx = fd as usize;
        if idx >= MAX_FDS {
            return Err(Errno::EBADF);
        }
        self.entries[idx].as_mut().ok_or(Errno::EBADF)
    }

    /// Close file descriptor `fd`.  Returns `EBADF` if not open.
    pub fn close(&mut self, fd: u32) -> SysResult<()> {
        let idx = fd as usize;
        if idx >= MAX_FDS {
            return Err(Errno::EBADF);
        }
        let entry = self.entries[idx].take().ok_or(Errno::EBADF)?;
        entry.node.close();
        Ok(())
    }

    /// Close all open file descriptors (called on process exit).
    pub fn close_all(&mut self) {
        for slot in self.entries.iter_mut() {
            if let Some(entry) = slot.take() {
                entry.node.close();
            }
        }
    }
}

impl Default for FdTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::sync::Arc;
    use abi::errors::Errno;
    use crate::vfs::{VfsNode, VfsStat};

    struct NullNode;
    impl VfsNode for NullNode {
        fn read(&self, _: u64, _: &mut [u8]) -> SysResult<usize> { Ok(0) }
        fn write(&self, _: u64, buf: &[u8]) -> SysResult<usize> { Ok(buf.len()) }
        fn stat(&self) -> SysResult<VfsStat> {
            Ok(VfsStat { mode: VfsStat::S_IFCHR | 0o666, size: 0, ino: 1 })
        }
    }

    fn null_node() -> Arc<dyn VfsNode> {
        Arc::new(NullNode)
    }

    #[test]
    fn test_open_allocates_from_3() {
        let mut table = FdTable::new();
        let fd = table.open(null_node(), OpenFlags::read_only()).unwrap();
        assert_eq!(fd, 3, "first VFS fd should be 3");
    }

    #[test]
    fn test_open_sequential_fds() {
        let mut table = FdTable::new();
        let fd1 = table.open(null_node(), OpenFlags::read_only()).unwrap();
        let fd2 = table.open(null_node(), OpenFlags::read_only()).unwrap();
        assert_eq!(fd1, 3);
        assert_eq!(fd2, 4);
    }

    #[test]
    fn test_get_unknown_fd_returns_ebadf() {
        let table = FdTable::new();
        assert!(matches!(table.get(10), Err(Errno::EBADF)));
    }

    #[test]
    fn test_close_frees_slot() {
        let mut table = FdTable::new();
        let fd = table.open(null_node(), OpenFlags::read_only()).unwrap();
        table.close(fd).unwrap();
        assert!(matches!(table.get(fd), Err(Errno::EBADF)));
    }

    #[test]
    fn test_close_reuses_slot() {
        let mut table = FdTable::new();
        let fd1 = table.open(null_node(), OpenFlags::read_only()).unwrap();
        table.close(fd1).unwrap();
        let fd2 = table.open(null_node(), OpenFlags::read_only()).unwrap();
        // Slot 3 was freed, so it should be reused.
        assert_eq!(fd2, 3);
    }

    #[test]
    fn test_close_ebadf_for_not_open() {
        let mut table = FdTable::new();
        assert!(matches!(table.close(99), Err(Errno::EBADF)));
    }

    #[test]
    fn test_stdio_fds_not_allocated() {
        // Open MAX_FDS-3 files and ensure none get fd 0, 1, 2.
        let mut table = FdTable::new();
        let limit = MAX_FDS - 3;
        let mut fds = alloc::vec::Vec::new();
        for _ in 0..limit {
            match table.open(null_node(), OpenFlags::read_only()) {
                Ok(fd) => {
                    assert!(fd >= 3, "fd must be >= 3, got {}", fd);
                    fds.push(fd);
                }
                Err(Errno::EMFILE) => break,
                Err(e) => panic!("unexpected error {:?}", e),
            }
        }
        // Next open should fail with EMFILE.
        assert_eq!(
            table.open(null_node(), OpenFlags::read_only()).unwrap_err(),
            Errno::EMFILE
        );
    }
}
