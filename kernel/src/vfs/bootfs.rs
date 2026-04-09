//! bootfs — minimal static boot filesystem.
//!
//! Provides a read-only filesystem whose contents are compiled into the kernel
//! binary at link time, or passed as boot modules by the loader.
//!
//! The boot filesystem is mounted at `/boot` by [`crate::vfs::init`].

use abi::errors::{Errno, SysResult};
use alloc::sync::Arc;
use crate::BootModuleDesc;

use super::{VfsDriver, VfsNode, VfsStat};

// ── Embedded file contents ────────────────────────────────────────────────────

const VERSION_DATA: &[u8] = b"Thing-OS v0.1 (janix ACT IV)\n";
const MOTD_DATA: &[u8] = b"Welcome to Thing-OS.\nBooting into path-based namespace...\n";

// ── BootFs driver ─────────────────────────────────────────────────────────────

/// The boot filesystem driver.  Mounted at `/boot` by `vfs::init`.
pub struct BootFs {
    modules: &'static [BootModuleDesc],
}

impl BootFs {
    pub fn new(modules: &'static [BootModuleDesc]) -> Self {
        Self { modules }
    }
}

impl VfsDriver for BootFs {
    fn lookup(&self, path: &str) -> SysResult<Arc<dyn VfsNode>> {
        crate::kinfo!("BootFs: lookup path='{}'", path);
        if path.is_empty() {
            return Ok(Arc::new(BootDirNode { modules: self.modules }));
        }

        if path == "version" {
            return Ok(Arc::new(StaticFileNode::new(VERSION_DATA, 10)));
        }
        if path == "motd" {
            return Ok(Arc::new(StaticFileNode::new(MOTD_DATA, 11)));
        }

        // Search in modules
        for (i, m) in self.modules.iter().enumerate() {
            let m_name = m.name.trim_matches('\0').trim();
            let name = m_name.strip_prefix("/boot/").unwrap_or(m_name);
            
            if name == path {
                crate::kinfo!("BootFs: EXACT match for '{}' at index {}", path, i);
                return Ok(Arc::new(StaticFileNode::new(m.bytes, 100 + i as u64)));
            }

            // Also try matching basename (e.g. "/assets/fonts/unifont.hex" matches "unifont.hex")
            if let Some(slash_idx) = m_name.rfind('/') {
                let basename = &m_name[slash_idx + 1..];
                if basename == path {
                    crate::kinfo!("BootFs: BASENAME match for '{}' at index {}", path, i);
                    return Ok(Arc::new(StaticFileNode::new(m.bytes, 100 + i as u64)));
                }
            }
        }

        crate::kerror!("BootFs: ENOENT for '{}'", path);
        Err(Errno::ENOENT)
    }
}

// ── /boot directory node ──────────────────────────────────────────────────────

struct BootDirNode {
    modules: &'static [BootModuleDesc],
}

impl VfsNode for BootDirNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFDIR | 0o555,
            size: 0,
            ino: 9,
        })
    }
    fn readdir(&self, _offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        use alloc::vec::Vec;
        let mut entries: Vec<u8> = Vec::new();
        entries.extend_from_slice(b"version\0motd\0");
        
        for m in self.modules {
            let name = m.name.strip_prefix("/boot/").unwrap_or(m.name);
            let final_name = if let Some(slash_idx) = name.rfind('/') {
                &name[slash_idx + 1..]
            } else {
                name
            };

            // Skip entries that are already hardcoded
            if final_name == "version" || final_name == "motd" {
                continue;
            }
            entries.extend_from_slice(final_name.as_bytes());
            entries.push(0);
        }

        let n = entries.len().min(buf.len());
        buf[..n].copy_from_slice(&entries[..n]);
        Ok(n)
    }
}

// ── Static file node ──────────────────────────────────────────────────────────

struct StaticFileNode {
    data: &'static [u8],
    ino: u64,
}

impl StaticFileNode {
    const fn new(data: &'static [u8], ino: u64) -> Self {
        Self { data, ino }
    }
}

impl VfsNode for StaticFileNode {
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
            mode: VfsStat::S_IFREG | 0o444,
            size: self.data.len() as u64,
            ino: self.ino,
        })
    }
}
