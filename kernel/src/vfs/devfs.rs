//! devfs — kernel-native device filesystem mounted at `/dev`.
//!
//! Provides a minimal set of device nodes:
//!
//! | Path            | Kind     | Description                              |
//! |-----------------|----------|------------------------------------------|
//! | `/dev/console`  | char     | Writes go to the boot console; reads from the per-process console input queue |
//! | `/dev/null`     | char     | Discards writes; returns EOF on reads    |
//! | `/dev/zero`     | char     | Returns zero bytes; discards writes      |
//!
//! # Design
//! `DevFs` implements [`VfsDriver`].  Each `lookup` call returns a fresh
//! `Arc<dyn VfsNode>` so that the node itself can be stateless (or hold only
//! shared state via an `Arc`).

use alloc::sync::Arc;
use abi::errors::{Errno, SysResult};

use super::{VfsDriver, VfsNode, VfsStat};

// ── DevFs driver ─────────────────────────────────────────────────────────────

/// The device filesystem driver.  Mounted at `/dev` by `vfs::init`.
pub struct DevFs;

impl DevFs {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DevFs {
    fn default() -> Self {
        Self::new()
    }
}

impl VfsDriver for DevFs {
    fn lookup(&self, path: &str) -> SysResult<Arc<dyn VfsNode>> {
        match path {
            "console" => Ok(Arc::new(ConsoleNode)),
            "null" => Ok(Arc::new(NullNode)),
            "zero" => Ok(Arc::new(ZeroNode)),
            _ => Err(Errno::ENOENT),
        }
    }
}

// ── /dev/console ─────────────────────────────────────────────────────────────

/// Character device node for `/dev/console`.
///
/// - **write**: each byte is forwarded to the kernel's boot console via
///   [`crate::runtime_base()`].
/// - **read**: drains the calling process's `console_stdin` ring, blocking
///   (yielding) until data is available.
pub struct ConsoleNode;

impl VfsNode for ConsoleNode {
    fn read(&self, _offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        let pinfo = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        loop {
            let count = {
                let mut lock = pinfo.lock();
                let mut n = 0usize;
                while n < buf.len() {
                    match lock.console_stdin.pop_front() {
                        Some(b) => { buf[n] = b; n += 1; }
                        None => break,
                    }
                }
                n
            };
            if count > 0 {
                return Ok(count);
            }
            // No data yet — yield and retry.
            unsafe { crate::sched::yield_now_current() };
        }
    }

    fn write(&self, _offset: u64, buf: &[u8]) -> SysResult<usize> {
        let rt = crate::runtime_base();
        for &b in buf {
            rt.putchar(b);
        }
        Ok(buf.len())
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFCHR | 0o666,
            size: 0,
            ino: 1,
        })
    }
}

// ── /dev/null ────────────────────────────────────────────────────────────────

/// Character device node for `/dev/null`.
/// Reads return EOF immediately; writes silently succeed.
pub struct NullNode;

impl VfsNode for NullNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Ok(0) // EOF
    }

    fn write(&self, _offset: u64, buf: &[u8]) -> SysResult<usize> {
        Ok(buf.len()) // silently discard
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFCHR | 0o666,
            size: 0,
            ino: 2,
        })
    }
}

// ── /dev/zero ────────────────────────────────────────────────────────────────

/// Character device node for `/dev/zero`.
/// Reads fill the buffer with zero bytes; writes succeed silently.
pub struct ZeroNode;

impl VfsNode for ZeroNode {
    fn read(&self, _offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        buf.fill(0);
        Ok(buf.len())
    }

    fn write(&self, _offset: u64, buf: &[u8]) -> SysResult<usize> {
        Ok(buf.len())
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFCHR | 0o666,
            size: 0,
            ino: 3,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    fn lookup(path: &str) -> SysResult<Arc<dyn VfsNode>> {
        DevFs::new().lookup(path)
    }

    #[test]
    fn test_null_read_returns_zero() {
        let node = lookup("null").unwrap();
        let mut buf = [0xFFu8; 8];
        let n = node.read(0, &mut buf).unwrap();
        assert_eq!(n, 0);
    }

    #[test]
    fn test_null_write_succeeds() {
        let node = lookup("null").unwrap();
        let n = node.write(0, b"hello").unwrap();
        assert_eq!(n, 5);
    }

    #[test]
    fn test_zero_read_fills_zeros() {
        let node = lookup("zero").unwrap();
        let mut buf = [0xFFu8; 4];
        let n = node.read(0, &mut buf).unwrap();
        assert_eq!(n, 4);
        assert_eq!(buf, [0u8; 4]);
    }

    #[test]
    fn test_zero_write_succeeds() {
        let node = lookup("zero").unwrap();
        let n = node.write(0, b"ignored").unwrap();
        assert_eq!(n, 7);
    }

    #[test]
    fn test_lookup_console_returns_node() {
        assert!(lookup("console").is_ok());
    }

    #[test]
    fn test_lookup_unknown_returns_enoent() {
        assert!(matches!(lookup("nonexistent"), Err(Errno::ENOENT)));
    }

    #[test]
    fn test_null_stat() {
        let node = lookup("null").unwrap();
        let stat = node.stat().unwrap();
        assert!(stat.is_chr());
    }

    #[test]
    fn test_zero_stat() {
        let node = lookup("zero").unwrap();
        let stat = node.stat().unwrap();
        assert!(stat.is_chr());
    }
}
