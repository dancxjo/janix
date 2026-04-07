//! devfs — kernel-native device filesystem mounted at `/dev`.
//!
//! Provides a minimal set of built-in device nodes plus a **runtime
//! registration** mechanism so that kernel subsystems and drivers can add
//! new device entries without modifying this file.
//!
//! # Built-in nodes
//!
//! | Path            | Kind     | Description                              |
//! |-----------------|----------|------------------------------------------|
//! | `/dev/console`  | char     | Writes go to the boot console; reads from the per-process console input queue |
//! | `/dev/null`     | char     | Discards writes; returns EOF on reads    |
//! | `/dev/zero`     | char     | Returns zero bytes; discards writes      |
//!
//! # Extensibility
//! Additional device nodes can be registered at runtime via the global
//! [`register`] function:
//!
//! ```ignore
//! devfs::register("ttyS0", Arc::new(my_uart_node));
//! ```
//!
//! Registered nodes are consulted **before** the built-in match, so they can
//! shadow built-in names when needed (last registration wins).  The global
//! registry is protected by a spin-lock.

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use abi::errors::{Errno, SysResult};
use spin::Mutex;

use super::{VfsDriver, VfsNode, VfsStat};

// ── Global device registry ────────────────────────────────────────────────────

/// Global table of dynamically registered `/dev` entries.
///
/// Keys are bare device names (no leading `/dev/`).  The table is consulted
/// *after* the built-in match, so built-in names (`null`, `zero`, `console`)
/// can still be overridden if needed.
static DEVICE_REGISTRY: Mutex<BTreeMap<String, Arc<dyn VfsNode>>> =
    Mutex::new(BTreeMap::new());

/// Register a device node under the name `name` in `/dev`.
///
/// The `name` must be the bare device name, e.g. `"ttyS0"` (not `/dev/ttyS0`).
/// If a node with the same name was previously registered, it is replaced.
///
/// # Example
/// ```ignore
/// use kernel::vfs::devfs;
/// devfs::register("ttyS0", Arc::new(UartNode::new()));
/// ```
pub fn register(name: &str, node: Arc<dyn VfsNode>) {
    DEVICE_REGISTRY.lock().insert(name.to_string(), node);
}

/// Remove a previously registered device node.
///
/// Returns `true` if a node was found and removed, `false` if the name was
/// not registered.
pub fn unregister(name: &str) -> bool {
    DEVICE_REGISTRY.lock().remove(name).is_some()
}

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
        // Empty path → the /dev directory node itself.
        if path.is_empty() {
            return Ok(Arc::new(DevDirNode));
        }

        // Check the dynamic registry first; registered nodes take precedence
        // over built-in names, allowing callers to override defaults.
        {
            let reg = DEVICE_REGISTRY.lock();
            if let Some(node) = reg.get(path) {
                return Ok(node.clone());
            }
        }

        // Fall back to built-in nodes.
        match path {
            "console" => Ok(Arc::new(ConsoleNode)),
            "null" => Ok(Arc::new(NullNode)),
            "zero" => Ok(Arc::new(ZeroNode)),
            _ => Err(Errno::ENOENT),
        }
    }
}

// ── /dev directory node ───────────────────────────────────────────────────────

/// Directory node for `/dev` itself.
struct DevDirNode;

impl VfsNode for DevDirNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFDIR | 0o755,
            size: 0,
            ino: 100,
        })
    }
    fn readdir(&self, _offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        // Enumerate built-in names plus dynamically registered ones.
        let mut entries = alloc::vec::Vec::<u8>::new();
        for name in &["console", "null", "zero"] {
            entries.extend_from_slice(name.as_bytes());
            entries.push(0);
        }
        {
            let reg = DEVICE_REGISTRY.lock();
            for name in reg.keys() {
                // Avoid duplicating names already listed above.
                if !matches!(name.as_str(), "console" | "null" | "zero") {
                    entries.extend_from_slice(name.as_bytes());
                    entries.push(0);
                }
            }
        }
        let n = entries.len().min(buf.len());
        buf[..n].copy_from_slice(&entries[..n]);
        Ok(n)
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

    #[test]
    fn test_lookup_root_is_dir() {
        let node = lookup("").unwrap();
        let stat = node.stat().unwrap();
        assert!(stat.is_dir());
    }

    #[test]
    fn test_readdir_lists_builtin_nodes() {
        let node = lookup("").unwrap();
        let mut buf = [0u8; 64];
        let n = node.readdir(0, &mut buf).unwrap();
        assert!(n > 0);
        let s = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(s.contains("console"));
        assert!(s.contains("null"));
        assert!(s.contains("zero"));
    }

    #[test]
    fn test_register_and_lookup_dynamic_device() {
        struct TestDev;
        impl VfsNode for TestDev {
            fn read(&self, _: u64, _: &mut [u8]) -> SysResult<usize> { Ok(0) }
            fn write(&self, _: u64, buf: &[u8]) -> SysResult<usize> { Ok(buf.len()) }
            fn stat(&self) -> SysResult<VfsStat> {
                Ok(VfsStat { mode: VfsStat::S_IFCHR | 0o666, size: 0, ino: 999 })
            }
        }

        register("test_unique_dev_42", Arc::new(TestDev));
        let node = DevFs::new().lookup("test_unique_dev_42").unwrap();
        assert!(node.stat().unwrap().is_chr());
        // Clean up.
        unregister("test_unique_dev_42");
    }

    #[test]
    fn test_unregister_removes_device() {
        struct TestDev2;
        impl VfsNode for TestDev2 {
            fn read(&self, _: u64, _: &mut [u8]) -> SysResult<usize> { Ok(0) }
            fn write(&self, _: u64, buf: &[u8]) -> SysResult<usize> { Ok(buf.len()) }
            fn stat(&self) -> SysResult<VfsStat> {
                Ok(VfsStat { mode: VfsStat::S_IFCHR | 0o666, size: 0, ino: 998 })
            }
        }

        register("test_unique_dev_99", Arc::new(TestDev2));
        assert!(DevFs::new().lookup("test_unique_dev_99").is_ok());
        let removed = unregister("test_unique_dev_99");
        assert!(removed);
        assert!(matches!(DevFs::new().lookup("test_unique_dev_99"), Err(Errno::ENOENT)));
    }
}
