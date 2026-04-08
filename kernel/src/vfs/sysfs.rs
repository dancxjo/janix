//! sysfs — read-only kernel device discovery metadata mounted at `/sys`.
//!
//! The initial implementation exposes claimable PCI devices from the kernel
//! device registry under `/sys/devices`. This is enough for `devd` to discover
//! hardware, match a userspace driver, and decide whether a restart is valid.

use abi::errors::{Errno, SysResult};
use alloc::format;
use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::device_registry::{DeviceEntry, REGISTRY};

use super::{VfsDriver, VfsNode, VfsStat};

pub struct SysFs;

impl SysFs {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SysFs {
    fn default() -> Self {
        Self::new()
    }
}

impl VfsDriver for SysFs {
    fn lookup(&self, path: &str) -> SysResult<Arc<dyn VfsNode>> {
        match SysPath::parse(path)? {
            SysPath::Root => Ok(Arc::new(StaticDirNode::new(300, &["devices", "firmware"]))),
            SysPath::Devices => Ok(Arc::new(DevicesDirNode)),
            SysPath::DeviceDir(name) => {
                let (_, entry) = find_device_by_slot(name)?;
                Ok(Arc::new(DeviceDirNode::new(entry)))
            }
            SysPath::DeviceFile(name, file) => {
                let (_, entry) = find_device_by_slot(name)?;
                let node = lookup_device_file(entry, file)?;
                Ok(Arc::new(node))
            }
            SysPath::Firmware => Ok(Arc::new(StaticDirNode::new(302, &["acpi", "dtb"]))),
            SysPath::FirmwareFile("acpi") => {
                if let Some(rsdp) = crate::boot_info::get().acpi_rsdp {
                    let text = format!("0x{:016x}\n", rsdp);
                    Ok(Arc::new(StaticTextNode::new(text.into_bytes(), 303)))
                } else {
                    Err(Errno::ENOENT)
                }
            }
            SysPath::FirmwareFile("dtb") => {
                if let Some(dtb) = crate::boot_info::get().dtb_ptr {
                    let text = format!("0x{:016x}\n", dtb);
                    Ok(Arc::new(StaticTextNode::new(text.into_bytes(), 304)))
                } else {
                    Err(Errno::ENOENT)
                }
            }
            SysPath::FirmwareFile(_) => Err(Errno::ENOENT),
        }
    }
}

enum SysPath<'a> {
    Root,
    Devices,
    DeviceDir(&'a str),
    DeviceFile(&'a str, &'a str),
    Firmware,
    FirmwareFile(&'a str),
}

impl<'a> SysPath<'a> {
    fn parse(path: &'a str) -> SysResult<Self> {
        if path.is_empty() {
            return Ok(Self::Root);
        }

        let mut parts = path.split('/').filter(|part| !part.is_empty());
        match (parts.next(), parts.next(), parts.next(), parts.next()) {
            (Some("devices"), None, None, None) => Ok(Self::Devices),
            (Some("devices"), Some(dev), None, None) => Ok(Self::DeviceDir(dev)),
            (Some("devices"), Some(dev), Some(file), None) => Ok(Self::DeviceFile(dev, file)),
            (Some("firmware"), None, None, None) => Ok(Self::Firmware),
            (Some("firmware"), Some(file), None, None) => Ok(Self::FirmwareFile(file)),
            _ => Err(Errno::ENOENT),
        }
    }
}

struct StaticDirNode {
    ino: u64,
    entries: &'static [&'static str],
}

impl StaticDirNode {
    const fn new(ino: u64, entries: &'static [&'static str]) -> Self {
        Self { ino, entries }
    }
}

impl VfsNode for StaticDirNode {
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
            ino: self.ino,
        })
    }

    fn readdir(&self, _offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        write_dir_entries(self.entries.iter().copied(), buf)
    }
}

struct DevicesDirNode;

impl VfsNode for DevicesDirNode {
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
            ino: 301,
        })
    }

    fn readdir(&self, _offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let slots = pci_slot_names();
        write_dir_entries(slots.iter().map(|s| s.as_str()), buf)
    }
}

struct DeviceDirNode {
    ino: u64,
}

impl DeviceDirNode {
    fn new(entry: DeviceEntry) -> Self {
        Self {
            ino: 0x1000 + entry.graph_id,
        }
    }
}

impl VfsNode for DeviceDirNode {
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
            ino: self.ino,
        })
    }

    fn readdir(&self, _offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        write_dir_entries(
            ["vendor", "device", "class", "status", "handle"].into_iter(),
            buf,
        )
    }
}

struct StaticTextNode {
    data: Vec<u8>,
    ino: u64,
}

impl StaticTextNode {
    fn new(data: Vec<u8>, ino: u64) -> Self {
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
        })
    }
}

fn pci_slot_names() -> Vec<alloc::string::String> {
    let reg = REGISTRY.lock();
    let mut out = Vec::new();
    for index in 0..reg.len() {
        let Some(entry) = reg.entry_copy(index) else {
            continue;
        };
        if entry.pci_location.is_some() {
            out.push(slot_name(entry));
        }
    }
    out
}

fn find_device_by_slot(slot: &str) -> SysResult<(usize, DeviceEntry)> {
    let reg = REGISTRY.lock();
    for index in 0..reg.len() {
        let Some(entry) = reg.entry_copy(index) else {
            continue;
        };
        if entry.pci_location.is_some() && slot_name(entry) == slot {
            return Ok((index, entry));
        }
    }
    Err(Errno::ENOENT)
}

fn slot_name(entry: DeviceEntry) -> alloc::string::String {
    let loc = entry.pci_location.expect("slot_name requires pci_location");
    format!("pci-0000:{:02x}:{:02x}.{}", loc.bus, loc.dev, loc.func)
}

fn lookup_device_file(entry: DeviceEntry, file: &str) -> SysResult<StaticTextNode> {
    let ino_base = 0x2000 + entry.graph_id * 8;
    let text = match file {
        "vendor" => format!("0x{:04x}\n", entry.vendor_id),
        "device" => format!("0x{:04x}\n", entry.device_id),
        "class" => format!(
            "0x{:02x}{:02x}{:02x}\n",
            entry.class_code, entry.subclass, entry.prog_if
        ),
        "status" => "present\n".into(),
        "handle" => format!("{}\n", entry.graph_id),
        _ => return Err(Errno::ENOENT),
    };
    Ok(StaticTextNode::new(text.into_bytes(), ino_base))
}

fn write_dir_entries<'a>(
    entries: impl IntoIterator<Item = &'a str>,
    buf: &mut [u8],
) -> SysResult<usize> {
    let mut written = 0usize;
    for entry in entries {
        let bytes = entry.as_bytes();
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
