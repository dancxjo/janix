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

use abi::errors::{Errno, SysResult};
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use spin::Mutex;

use super::{VfsDriver, VfsNode, VfsStat};

// ── Global device registry ────────────────────────────────────────────────────

/// Global table of dynamically registered `/dev` entries.
///
/// Keys are bare device names (no leading `/dev/`).  The table is consulted
/// *after* the built-in match, so built-in names (`null`, `zero`, `console`)
/// can still be overridden if needed.
static DEVICE_REGISTRY: Mutex<BTreeMap<String, Arc<dyn VfsNode>>> = Mutex::new(BTreeMap::new());
static BOOT_FB_INFO: Mutex<Option<(crate::FramebufferInfo, u64)>> = Mutex::new(None);

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

pub fn set_boot_fb(fb: crate::FramebufferInfo, graph_id: u64) {
    *BOOT_FB_INFO.lock() = Some((fb, graph_id));
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
            "fb0" => {
                if let Some((fb, graph_id)) = *BOOT_FB_INFO.lock() {
                    Ok(Arc::new(FbNode::new(fb, graph_id)))
                } else {
                    Err(Errno::ENOENT)
                }
            }
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
        if BOOT_FB_INFO.lock().is_some() {
            entries.extend_from_slice(b"fb0");
            entries.push(0);
        }
        {
            let reg = DEVICE_REGISTRY.lock();
            for name in reg.keys() {
                // Avoid duplicating names already listed above.
                if !matches!(name.as_str(), "console" | "null" | "zero" | "fb0") {
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

static CONSOLE_BUF: Mutex<alloc::collections::VecDeque<u8>> =
    Mutex::new(alloc::collections::VecDeque::new());

/// Character device node for `/dev/console`.
///
/// - **write**: each byte is forwarded to the kernel's boot console via
///   [`crate::runtime_base()`].
/// - **read**: reads from the boot console, applying a canonical line discipline.
///   Blocks (yields) until a complete line is available.
pub struct ConsoleNode;

impl VfsNode for ConsoleNode {
    fn read(&self, _offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        let rt = crate::runtime_base();
        let mut read_bytes = 0;
        loop {
            // Drain hardware
            while let Some(c) = rt.getchar() {
                let mut cb = CONSOLE_BUF.lock();
                match c {
                    b'\r' | b'\n' => {
                        rt.putchar(b'\r');
                        rt.putchar(b'\n');
                        cb.push_back(b'\n');
                    }
                    0x08 | 0x7f => {
                        if !cb.is_empty() && *cb.back().unwrap() != b'\n' {
                            cb.pop_back();
                            rt.putchar(0x08);
                            rt.putchar(b' ');
                            rt.putchar(0x08);
                        }
                    }
                    0x20..=0x7e => {
                        cb.push_back(c);
                        rt.putchar(c);
                    }
                    0x03 => {
                        rt.putchar(b'^');
                        rt.putchar(b'C');
                        rt.putchar(b'\r');
                        rt.putchar(b'\n');
                        cb.clear();
                        cb.push_back(0x03);
                    }
                    _ => {}
                }
            }

            let mut cb = CONSOLE_BUF.lock();
            let has_line = cb.iter().any(|&b| b == b'\n' || b == 0x03);
            if has_line || cb.len() >= buf.len() {
                while read_bytes < buf.len() {
                    if let Some(b) = cb.pop_front() {
                        buf[read_bytes] = b;
                        read_bytes += 1;
                        if b == b'\n' || b == 0x03 {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                return Ok(read_bytes);
            }
            drop(cb);

            unsafe { crate::sched::yield_now_current() };
        }
    }

    fn write(&self, _offset: u64, buf: &[u8]) -> SysResult<usize> {
        let rt = crate::runtime_base();
        for &b in buf {
            if b == b'\n' {
                rt.putchar(b'\r');
            }
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

// ── /dev/fb0 ─────────────────────────────────────────────────────────────────

pub struct FbNode {
    fb: crate::FramebufferInfo,
    graph_id: u64,
}

impl FbNode {
    pub const fn new(fb: crate::FramebufferInfo, graph_id: u64) -> Self {
        Self { fb, graph_id }
    }
}

impl VfsNode for FbNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        use abi::display_driver_protocol::{FB_INFO_PAYLOAD_SIZE, FbInfoPayload};

        let payload = FbInfoPayload {
            graph_id: self.graph_id,
            width: self.fb.width,
            height: self.fb.height,
            stride: self.fb.pitch,
            bpp: self.fb.bpp as u32,
            format: self.fb.format as u32,
        };

        let slice = unsafe {
            core::slice::from_raw_parts(&payload as *const _ as *const u8, FB_INFO_PAYLOAD_SIZE)
        };

        let off = offset as usize;
        if off >= slice.len() {
            return Ok(0);
        }

        let avail = &slice[off..];
        let n = avail.len().min(buf.len());
        buf[..n].copy_from_slice(&avail[..n]);
        Ok(n)
    }

    fn write(&self, offset: u64, buf: &[u8]) -> SysResult<usize> {
        let off = offset as usize;
        if off >= self.fb.byte_len {
            return Ok(0);
        }

        let n = buf.len().min(self.fb.byte_len.saturating_sub(off));
        if n == 0 {
            return Ok(0);
        }

        unsafe {
            core::ptr::copy_nonoverlapping(
                buf.as_ptr(),
                (self.fb.addr as usize + off) as *mut u8,
                n,
            );
        }
        Ok(n)
    }

    fn stat(&self) -> SysResult<VfsStat> {
        use abi::display_driver_protocol::FB_INFO_PAYLOAD_SIZE;
        Ok(VfsStat {
            mode: VfsStat::S_IFCHR | 0o666,
            size: FB_INFO_PAYLOAD_SIZE as u64,
            ino: 4,
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
            fn read(&self, _: u64, _: &mut [u8]) -> SysResult<usize> {
                Ok(0)
            }
            fn write(&self, _: u64, buf: &[u8]) -> SysResult<usize> {
                Ok(buf.len())
            }
            fn stat(&self) -> SysResult<VfsStat> {
                Ok(VfsStat {
                    mode: VfsStat::S_IFCHR | 0o666,
                    size: 0,
                    ino: 999,
                })
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
            fn read(&self, _: u64, _: &mut [u8]) -> SysResult<usize> {
                Ok(0)
            }
            fn write(&self, _: u64, buf: &[u8]) -> SysResult<usize> {
                Ok(buf.len())
            }
            fn stat(&self) -> SysResult<VfsStat> {
                Ok(VfsStat {
                    mode: VfsStat::S_IFCHR | 0o666,
                    size: 0,
                    ino: 998,
                })
            }
        }

        register("test_unique_dev_99", Arc::new(TestDev2));
        assert!(DevFs::new().lookup("test_unique_dev_99").is_ok());
        let removed = unregister("test_unique_dev_99");
        assert!(removed);
        assert!(matches!(
            DevFs::new().lookup("test_unique_dev_99"),
            Err(Errno::ENOENT)
        ));
    }
}
