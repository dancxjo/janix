//! procfs — process information filesystem mounted at `/proc`.
//!
//! Provides a read-only view of running processes and system state.
//!
//! # Paths exposed
//!
//! | Path                       | Contents |
//! |----------------------------|----------|
//! | `/proc/version`            | Kernel version string |
//! | `/proc/mounts`             | Active mount table (text) |
//! | `/proc/meminfo`            | Heap memory statistics |
//! | `/proc/cpuinfo`            | CPU model and frequency |
//! | `/proc/uptime`             | Seconds since boot |
//! | `/proc/<pid>/status`       | Process state, name, ppid |
//! | `/proc/<pid>/cmdline`      | argv as null-delimited bytes |
//! | `/proc/<pid>/fd/`          | Directory of open fd targets |

use abi::errors::{Errno, SysResult};
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;

use super::{VfsDriver, VfsNode, VfsStat};

// ── ProcFs driver ─────────────────────────────────────────────────────────────

/// The process filesystem driver.  Mounted at `/proc` by `vfs::init`.
pub struct ProcFs;

impl ProcFs {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ProcFs {
    fn default() -> Self {
        Self::new()
    }
}

impl VfsDriver for ProcFs {
    fn lookup(&self, path: &str) -> SysResult<Arc<dyn VfsNode>> {
        match path {
            "" => Ok(Arc::new(ProcDirNode)),
            "version" => Ok(Arc::new(StaticTextNode::new(
                b"Thing-OS v0.1 (janix ACT VII)\n",
                2,
            ))),
            "mounts" => Ok(Arc::new(MountsNode)),
            "meminfo" => Ok(Arc::new(MemInfoNode)),
            "cpuinfo" => Ok(Arc::new(CpuInfoNode)),
            "uptime" => Ok(Arc::new(UptimeNode)),
            _ => {
                // Try to match /proc/<pid>/... paths.
                // `path` is already relative to the mount point, so it looks
                // like "42/status", "42/cmdline", "42", etc.
                let mut parts = path.splitn(2, '/');
                let pid_str = parts.next().unwrap_or("");
                let rest = parts.next().unwrap_or("");

                if let Ok(pid) = pid_str.parse::<u32>() {
                    return lookup_pid(pid, rest);
                }
                Err(Errno::ENOENT)
            }
        }
    }
}

/// Look up a node inside a per-process `/proc/<pid>/` directory.
fn lookup_pid(pid: u32, rest: &str) -> SysResult<Arc<dyn VfsNode>> {
    let procs = crate::sched::list_processes_current();
    let snap = procs
        .iter()
        .find(|p| p.pid == pid)
        .ok_or(Errno::ENOENT)?
        .clone();

    match rest {
        // /proc/<pid> — the per-process directory itself
        "" => Ok(Arc::new(ProcPidDirNode { pid })),
        "status" => {
            let state_name = match snap.state {
                crate::task::TaskState::Runnable => "R",
                crate::task::TaskState::Running => "R",
                crate::task::TaskState::Blocked => "S",
                crate::task::TaskState::Dead => "Z",
            };
            let text = alloc::format!(
                "Name:\t{}\nState:\t{}\nPid:\t{}\nPPid:\t{}\n",
                snap.name,
                state_name,
                snap.pid,
                snap.ppid,
            );
            Ok(Arc::new(DynamicTextNode::new(
                text.into_bytes(),
                300 + pid as u64 * 10 + 1,
            )))
        }
        "cmdline" => {
            // Standard Linux /proc/<pid>/cmdline format: each argument is
            // followed by a NUL byte (including the last), so the full content
            // is "arg0\0arg1\0arg2\0".
            let mut data: Vec<u8> = Vec::new();
            for arg in snap.argv.iter() {
                data.extend_from_slice(arg);
                data.push(0);
            }
            Ok(Arc::new(DynamicTextNode::new(
                data,
                300 + pid as u64 * 10 + 2,
            )))
        }
        "fd" => Ok(Arc::new(ProcPidFdDirNode { pid })),
        _ => Err(Errno::ENOENT),
    }
}

// ── /proc root directory ──────────────────────────────────────────────────────

struct ProcDirNode;

impl VfsNode for ProcDirNode {
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
            ino: 200,
        ..Default::default()
        })
    }
    fn readdir(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let mut names = alloc::vec![
            String::from("version"),
            String::from("mounts"),
            String::from("meminfo"),
            String::from("cpuinfo"),
            String::from("uptime"),
        ];
        for snap in crate::sched::list_processes_current() {
            names.push(alloc::format!("{}", snap.pid));
        }
        super::write_readdir_entries(names.iter().map(|s: &String| s.as_str()), offset, buf)
    }
}

// ── /proc/<pid>/ directory ────────────────────────────────────────────────────

struct ProcPidDirNode {
    pid: u32,
}

impl VfsNode for ProcPidDirNode {
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
            ino: 300 + self.pid as u64 * 10,
        ..Default::default()
        })
    }
    fn readdir(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let entries = ["status", "cmdline", "fd"];
        super::write_readdir_entries(entries.into_iter(), offset, buf)
    }
}

// ── /proc/<pid>/fd/ directory ─────────────────────────────────────────────────

struct ProcPidFdDirNode {
    #[allow(dead_code)]
    pid: u32,
}

impl VfsNode for ProcPidFdDirNode {
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
            ino: 300 + self.pid as u64 * 10 + 3,
        ..Default::default()
        })
    }
    fn readdir(&self, _offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        // Stub: empty directory.
        let _ = buf;
        Ok(0)
    }
}

// ── Static text node ──────────────────────────────────────────────────────────

/// Returns a fixed byte slice on read.
struct StaticTextNode {
    data: &'static [u8],
    ino: u64,
}

impl StaticTextNode {
    const fn new(data: &'static [u8], ino: u64) -> Self {
        Self { data, ino }
    }
}

impl VfsNode for StaticTextNode {
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
        ..Default::default()
        })
    }
}

// ── /proc/mounts ─────────────────────────────────────────────────────────────

/// A dynamic node that renders the current mount table as text.
///
/// The output mirrors a simplified `/proc/mounts` format:
/// ```text
/// <mount_point> ramfs rw 0 0
/// ```
struct MountsNode;

impl VfsNode for MountsNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        // Build the mounts listing dynamically from the global mount table.
        let listing = super::mount::mounts_text();
        let data = listing.as_bytes();
        let off = offset as usize;
        if off >= data.len() {
            return Ok(0);
        }
        let avail = &data[off..];
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
            size: 0, // dynamic — size not known until read
            ino: 201,
        ..Default::default()
        })
    }
}

// ── Dynamic text node ─────────────────────────────────────────────────────────

/// Returns an owned byte vector on read.  Used for dynamically-generated
/// per-process text nodes such as `/proc/<pid>/status`.
struct DynamicTextNode {
    data: Vec<u8>,
    ino: u64,
}

impl DynamicTextNode {
    fn new(data: Vec<u8>, ino: u64) -> Self {
        Self { data, ino }
    }
}

impl VfsNode for DynamicTextNode {
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
        ..Default::default()
        })
    }
}

// ── /proc/meminfo ─────────────────────────────────────────────────────────────

/// Reports kernel heap statistics in a simplified `/proc/meminfo` format.
struct MemInfoNode;

impl VfsNode for MemInfoNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        // We report the static heap reservation size; detailed used/free
        // accounting is not yet tracked in the global allocator.
        let total_kb = (crate::memory::layout::KHEAP_SIZE / 1024) as u64;
        let text = alloc::format!(
            "MemTotal:    {:8} kB\nMemFree:     {:8} kB\n",
            total_kb,
            0u64,
        );
        let data = text.as_bytes();
        let off = offset as usize;
        if off >= data.len() {
            return Ok(0);
        }
        let n = (data.len() - off).min(buf.len());
        buf[..n].copy_from_slice(&data[off..off + n]);
        Ok(n)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EROFS)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFREG | 0o444,
            size: 0,
            ino: 202,
        ..Default::default()
        })
    }
}

// ── /proc/cpuinfo ─────────────────────────────────────────────────────────────

/// Reports a minimal CPU description.
struct CpuInfoNode;

impl VfsNode for CpuInfoNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let text = b"model name\t: Thing-OS virtual CPU\nprocessor\t: 0\n";
        let off = offset as usize;
        if off >= text.len() {
            return Ok(0);
        }
        let n = (text.len() - off).min(buf.len());
        buf[..n].copy_from_slice(&text[off..off + n]);
        Ok(n)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EROFS)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFREG | 0o444,
            size: 0,
            ino: 203,
        ..Default::default()
        })
    }
}

// ── /proc/uptime ──────────────────────────────────────────────────────────────

/// Returns seconds since boot as a decimal string.
struct UptimeNode;

impl VfsNode for UptimeNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let secs = uptime_secs();
        let text = alloc::format!("{}.00 {}.00\n", secs, secs);
        let data = text.as_bytes();
        let off = offset as usize;
        if off >= data.len() {
            return Ok(0);
        }
        let n = (data.len() - off).min(buf.len());
        buf[..n].copy_from_slice(&data[off..off + n]);
        Ok(n)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EROFS)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFREG | 0o444,
            size: 0,
            ino: 204,
        ..Default::default()
        })
    }
}

/// Return the number of seconds elapsed since boot using the runtime's
/// monotonic clock.  Returns 0 in test environments where the runtime hook
/// is not installed.
fn uptime_secs() -> u64 {
    // `runtime_base()` panics in test builds if the hook is not set up;
    // guard with a cfg flag so unit tests still pass.
    #[cfg(not(test))]
    {
        let rt = crate::runtime_base();
        let ticks = rt.mono_ticks();
        let freq = rt.mono_freq_hz();
        if freq == 0 {
            return 0;
        }
        ticks / freq
    }
    #[cfg(test)]
    {
        0
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn lookup(path: &str) -> SysResult<Arc<dyn VfsNode>> {
        ProcFs::new().lookup(path)
    }

    #[test]
    fn test_lookup_root_is_dir() {
        let node = lookup("").unwrap();
        let stat = node.stat().unwrap();
        assert!(stat.is_dir());
    }

    #[test]
    fn test_lookup_version() {
        let node = lookup("version").unwrap();
        let mut buf = [0u8; 64];
        let n = node.read(0, &mut buf).unwrap();
        assert!(n > 0);
        assert!(
            core::str::from_utf8(&buf[..n])
                .unwrap()
                .contains("Thing-OS")
        );
    }

    #[test]
    fn test_version_is_readonly() {
        let node = lookup("version").unwrap();
        assert!(matches!(node.write(0, b"hack"), Err(Errno::EROFS)));
    }

    #[test]
    fn test_lookup_unknown_returns_enoent() {
        assert!(matches!(lookup("doesnotexist"), Err(Errno::ENOENT)));
    }

    #[test]
    fn test_readdir_root_lists_entries() {
        let node = lookup("").unwrap();
        let mut buf = [0u8; 256];
        let n = node.readdir(0, &mut buf).unwrap();
        assert!(n > 0);
        let s = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(s.contains("version"));
        assert!(s.contains("mounts"));
        assert!(s.contains("meminfo"));
        assert!(s.contains("cpuinfo"));
        assert!(s.contains("uptime"));
    }

    #[test]
    fn test_lookup_meminfo() {
        let node = lookup("meminfo").unwrap();
        let mut buf = [0u8; 128];
        let n = node.read(0, &mut buf).unwrap();
        assert!(n > 0);
        let s = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(s.contains("MemTotal"));
    }

    #[test]
    fn test_lookup_cpuinfo() {
        let node = lookup("cpuinfo").unwrap();
        let mut buf = [0u8; 128];
        let n = node.read(0, &mut buf).unwrap();
        assert!(n > 0);
        let s = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(s.contains("processor"));
    }

    #[test]
    fn test_lookup_uptime() {
        let node = lookup("uptime").unwrap();
        let mut buf = [0u8; 64];
        let n = node.read(0, &mut buf).unwrap();
        assert!(n > 0);
    }

    #[test]
    fn test_lookup_pid_enoent_when_no_processes() {
        // In test environment there are no real processes so any PID should
        // return ENOENT.
        assert!(matches!(lookup("1/status"), Err(Errno::ENOENT)));
    }

    #[test]
    fn test_lookup_non_numeric_pid_returns_enoent() {
        assert!(matches!(lookup("notapid"), Err(Errno::ENOENT)));
    }
}
