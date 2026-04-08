//! UnionNode — layered (union) mount support.
//!
//! Allows multiple [`VfsDriver`] providers to be stacked at the same path
//! prefix.  Resolution order: **last mounted wins** unless [`UnionNode`] is
//! created with `fallthrough = true`, in which case the first provider that
//! returns a node (i.e. does *not* return `ENOENT`) wins.
//!
//! # Example
//! ```text
//! /bin = base_bin_provider + user_bin_provider
//! ```
//! With `fallthrough = false` (default): `user_bin_provider` is tried first;
//! only if it returns `ENOENT` is `base_bin_provider` tried (last → first).
//!
//! # Use cases
//! - Overlay user tools over system tools.
//! - Compose testing environments (mock `/services/net` overlaid for a test
//!   process).
//! - Fallback chains without graph merging.
//!
//! [`VfsDriver`]: super::VfsDriver

use abi::errors::{Errno, SysResult};
use alloc::sync::Arc;
use alloc::vec::Vec;

use super::{VfsDriver, VfsNode};

// ── UnionNode ─────────────────────────────────────────────────────────────────

/// A [`VfsDriver`] that layers multiple drivers at the same mount point.
///
/// Drivers are stored in the order they were added.  Lookup walks the list
/// from the **end** (most-recently added) toward the beginning, returning the
/// first successful result.  If all drivers return `ENOENT` the union returns
/// `ENOENT`.  Any other error from a driver is propagated immediately.
pub struct UnionFs {
    /// Ordered list of backing drivers (append-only in normal usage).
    layers: Vec<Arc<dyn VfsDriver>>,
    /// When `true`, lookup falls through to the next layer even on success
    /// (layers are merged; later layers take precedence for duplicate names).
    ///
    /// When `false` (the default), the first successful lookup wins.
    pub fallthrough: bool,
}

impl UnionFs {
    /// Create an empty union with no layers.
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            fallthrough: false,
        }
    }

    /// Create a union with `fallthrough` behaviour enabled.
    pub fn new_fallthrough() -> Self {
        Self {
            layers: Vec::new(),
            fallthrough: true,
        }
    }

    /// Add `driver` as the **topmost** (highest-priority) layer.
    ///
    /// Returns `&mut self` so calls can be chained.
    pub fn push(&mut self, driver: Arc<dyn VfsDriver>) -> &mut Self {
        self.layers.push(driver);
        self
    }

    /// Number of layers currently in this union.
    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }
}

impl Default for UnionFs {
    fn default() -> Self {
        Self::new()
    }
}

impl VfsDriver for UnionFs {
    /// Resolve `path` by trying layers from most-recently added to oldest.
    ///
    /// Returns the first successful node, or `ENOENT` if all layers miss.
    fn lookup(&self, path: &str) -> SysResult<Arc<dyn VfsNode>> {
        for driver in self.layers.iter().rev() {
            match driver.lookup(path) {
                Ok(node) => return Ok(node),
                Err(Errno::ENOENT) => continue, // try lower layer
                Err(e) => return Err(e),        // hard error — stop
            }
        }
        Err(Errno::ENOENT)
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vfs::{VfsNode, VfsStat};
    use abi::errors::Errno;

    // ── Helpers ─────────────────────────────────────────────────────────────

    struct SingleFileFs {
        name: &'static str,
        content: &'static [u8],
        ino: u64,
    }

    impl SingleFileFs {
        fn new(name: &'static str, content: &'static [u8], ino: u64) -> Arc<Self> {
            Arc::new(Self { name, content, ino })
        }
    }

    impl VfsDriver for SingleFileFs {
        fn lookup(&self, path: &str) -> SysResult<Arc<dyn VfsNode>> {
            if path == self.name {
                Ok(Arc::new(StaticNode {
                    content: self.content,
                    ino: self.ino,
                }))
            } else {
                Err(Errno::ENOENT)
            }
        }
    }

    struct StaticNode {
        content: &'static [u8],
        ino: u64,
    }

    impl VfsNode for StaticNode {
        fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
            let off = offset as usize;
            if off >= self.content.len() {
                return Ok(0);
            }
            let avail = &self.content[off..];
            let n = avail.len().min(buf.len());
            buf[..n].copy_from_slice(&avail[..n]);
            Ok(n)
        }
        fn write(&self, _: u64, _: &[u8]) -> SysResult<usize> {
            Err(Errno::EROFS)
        }
        fn stat(&self) -> SysResult<VfsStat> {
            Ok(VfsStat {
                mode: VfsStat::S_IFREG | 0o444,
                size: self.content.len() as u64,
                ino: self.ino,
            })
        }
    }

    // Error-producing driver.
    struct ErrFs(Errno);
    impl VfsDriver for ErrFs {
        fn lookup(&self, _: &str) -> SysResult<Arc<dyn VfsNode>> {
            Err(self.0)
        }
    }

    // ── Tests ────────────────────────────────────────────────────────────────

    #[test]
    fn test_empty_union_returns_enoent() {
        let union = UnionFs::new();
        assert!(matches!(union.lookup("anything"), Err(Errno::ENOENT)));
    }

    #[test]
    fn test_single_layer_hit() {
        let mut union = UnionFs::new();
        union.push(SingleFileFs::new("hello", b"world", 1));
        let node = union.lookup("hello").unwrap();
        let mut buf = [0u8; 5];
        node.read(0, &mut buf).unwrap();
        assert_eq!(&buf, b"world");
    }

    #[test]
    fn test_single_layer_miss() {
        let mut union = UnionFs::new();
        union.push(SingleFileFs::new("hello", b"world", 1));
        assert!(matches!(union.lookup("nope"), Err(Errno::ENOENT)));
    }

    #[test]
    fn test_upper_layer_shadows_lower() {
        let mut union = UnionFs::new();
        // base layer: "tool" → "base"
        union.push(SingleFileFs::new("tool", b"base", 1));
        // upper layer: "tool" → "user" (added last = highest priority)
        union.push(SingleFileFs::new("tool", b"user", 2));

        let node = union.lookup("tool").unwrap();
        let stat = node.stat().unwrap();
        // Should come from the upper (ino=2).
        assert_eq!(stat.ino, 2);
        let mut buf = [0u8; 4];
        node.read(0, &mut buf).unwrap();
        assert_eq!(&buf, b"user");
    }

    #[test]
    fn test_fallthrough_to_lower_when_upper_misses() {
        let mut union = UnionFs::new();
        // base layer has "base_tool"
        union.push(SingleFileFs::new("base_tool", b"bt", 10));
        // upper layer has only "upper_tool"
        union.push(SingleFileFs::new("upper_tool", b"ut", 20));

        // "base_tool" not in upper → should fall through to base.
        let node = union.lookup("base_tool").unwrap();
        let stat = node.stat().unwrap();
        assert_eq!(stat.ino, 10);
    }

    #[test]
    fn test_hard_error_propagates_without_falling_through() {
        let mut union = UnionFs::new();
        union.push(Arc::new(ErrFs(Errno::EIO)));
        // Hard error should propagate, not fall to lower layers.
        assert!(matches!(union.lookup("x"), Err(Errno::EIO)));
    }

    #[test]
    fn test_layer_count() {
        let mut union = UnionFs::new();
        assert_eq!(union.layer_count(), 0);
        union.push(SingleFileFs::new("a", b"a", 1));
        assert_eq!(union.layer_count(), 1);
        union.push(SingleFileFs::new("b", b"b", 2));
        assert_eq!(union.layer_count(), 2);
    }
}
