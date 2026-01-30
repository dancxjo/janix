//! ISO9660 Filesystem Parser
//!
//! Minimal, read-only ISO9660 parser for reading files from CD-ROM boot media.
//! Supports Level 1/2 interchange without Rock Ridge or Joliet extensions.

#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;
use stem::block::{BlockDevice, BlockError};

/// ISO9660 sector size (logical block size).
pub const ISO_SECTOR_SIZE: u64 = 2048;

/// Primary Volume Descriptor location (sector 16).
const PVD_SECTOR: u64 = 16;

/// Volume descriptor type codes.
const VD_TYPE_PRIMARY: u8 = 1;
#[allow(dead_code)]
const VD_TYPE_TERMINATOR: u8 = 255;

/// Parsed Primary Volume Descriptor.
#[derive(Debug)]
pub struct PrimaryVolumeDescriptor {
    pub system_id: [u8; 32],
    pub volume_id: [u8; 32],
    pub volume_space_size: u32,
    pub root_dir_extent: u32,
    pub root_dir_size: u32,
    pub logical_block_size: u16,
}

/// Performance counters for ISO operations.
#[cfg(feature = "perf")]
#[derive(Debug, Default)]
pub struct PerfCounters {
    pub dir_parses: u64,
    pub cache_hits: u64,
    pub path_resolution_steps: u64,
    pub bytes_read: u64,
}

#[cfg(not(feature = "perf"))]
#[derive(Debug, Default)]
pub struct PerfCounters;

/// Directory cache key: uniquely identifies a directory extent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct DirCacheKey {
    extent_lba: u32,
    data_length: u32,
}

/// Cached directory index with parsed entries.
#[derive(Debug, Clone)]
struct DirIndex {
    entries: Vec<IsoDirEntry>,
}

/// Directory entry from the ISO9660 filesystem.
#[derive(Debug, Clone)]
pub struct IsoDirEntry {
    pub name: String,
    pub extent_lba: u32,
    pub size: u32,
    pub is_directory: bool,
}

/// An open file handle with extent information.
#[derive(Debug)]
pub struct IsoFile {
    pub extent_lba: u32,
    pub size: u32,
}

/// Mounted ISO9660 filesystem.
pub struct IsoFs {
    pub pvd: PrimaryVolumeDescriptor,
    dir_cache: RefCell<BTreeMap<DirCacheKey, DirIndex>>,
    #[cfg(feature = "perf")]
    pub perf: RefCell<PerfCounters>,
}

impl IsoFs {
    /// Probe a block device for ISO9660 filesystem.
    ///
    /// Returns `Some(IsoFs)` if a valid ISO9660 Primary Volume Descriptor is found.
    pub fn probe(dev: &dyn BlockDevice) -> Option<Self> {
        let mut buf = [0u8; 2048];

        // Read PVD sector
        if dev.read_sectors(PVD_SECTOR, 1, &mut buf).is_err() {
            return None;
        }

        // Check for "CD001" signature at offset 1
        if &buf[1..6] != b"CD001" {
            return None;
        }

        // Check volume descriptor type
        if buf[0] != VD_TYPE_PRIMARY {
            return None;
        }

        // Check version
        if buf[6] != 1 {
            return None;
        }

        // Parse PVD fields
        let mut system_id = [0u8; 32];
        system_id.copy_from_slice(&buf[8..40]);

        let mut volume_id = [0u8; 32];
        volume_id.copy_from_slice(&buf[40..72]);

        // Volume space size (little-endian at offset 80)
        let volume_space_size = u32::from_le_bytes([buf[80], buf[81], buf[82], buf[83]]);

        // Logical block size (little-endian at offset 128)
        let logical_block_size = u16::from_le_bytes([buf[128], buf[129]]);

        // Root directory record is at offset 156
        let root_dir_extent = u32::from_le_bytes([buf[158], buf[159], buf[160], buf[161]]);
        let root_dir_size = u32::from_le_bytes([buf[166], buf[167], buf[168], buf[169]]);

        Some(IsoFs {
            pvd: PrimaryVolumeDescriptor {
                system_id,
                volume_id,
                volume_space_size,
                root_dir_extent,
                root_dir_size,
                logical_block_size,
            },
            dir_cache: RefCell::new(BTreeMap::new()),
            #[cfg(feature = "perf")]
            perf: RefCell::new(PerfCounters::default()),
        })
    }

    /// ASCII case-insensitive string comparison without allocation.
    /// Handles ISO9660 version suffixes (e.g., ";1").
    fn ascii_eq_ignore_case(a: &str, b: &str) -> bool {
        // Strip version suffix from both strings
        let a_clean = if let Some(pos) = a.find(';') {
            &a[..pos]
        } else {
            a
        };
        let b_clean = if let Some(pos) = b.find(';') {
            &b[..pos]
        } else {
            b
        };

        if a_clean.len() != b_clean.len() {
            return false;
        }

        a_clean.bytes().zip(b_clean.bytes()).all(|(a_byte, b_byte)| {
            a_byte.to_ascii_lowercase() == b_byte.to_ascii_lowercase()
        })
    }

    /// Parse directory entries from extent and cache them.
    /// Returns the cached directory entries.
    fn parse_and_cache_dir(
        &self,
        dev: &dyn BlockDevice,
        extent_lba: u32,
        size: u32,
    ) -> Vec<IsoDirEntry> {
        let key = DirCacheKey {
            extent_lba,
            data_length: size,
        };

        // Check if already cached
        {
            let cache = self.dir_cache.borrow();
            if let Some(index) = cache.get(&key) {
                #[cfg(feature = "perf")]
                {
                    self.perf.borrow_mut().cache_hits += 1;
                }
                return index.entries.clone();
            }
        }

        // Not cached, parse it
        #[cfg(feature = "perf")]
        {
            self.perf.borrow_mut().dir_parses += 1;
        }

        let entries = self.parse_dir_entries(dev, extent_lba, size);
        
        // Cache the parsed entries and return a clone from the cache
        self.dir_cache.borrow_mut().insert(key, DirIndex { entries });
        
        // Return a clone of the cached entries
        self.dir_cache.borrow().get(&key).unwrap().entries.clone()
    }

    /// Parse directory entries from an extent (internal implementation).
    fn parse_dir_entries(
        &self,
        dev: &dyn BlockDevice,
        extent_lba: u32,
        size: u32,
    ) -> Vec<IsoDirEntry> {
        let mut entries = Vec::new();
        let sectors_needed = (size as u64 + ISO_SECTOR_SIZE - 1) / ISO_SECTOR_SIZE;

        // Allocate buffer for directory data
        let buf_size = (sectors_needed * ISO_SECTOR_SIZE) as usize;
        let mut buf = alloc::vec![0u8; buf_size];

        if dev
            .read_sectors(extent_lba as u64, sectors_needed, &mut buf)
            .is_err()
        {
            return entries;
        }

        // Track bytes read only after successful read
        #[cfg(feature = "perf")]
        {
            self.perf.borrow_mut().bytes_read += sectors_needed * ISO_SECTOR_SIZE;
        }

        // Parse directory records
        let mut offset = 0usize;
        while offset < size as usize {
            let record_len = buf[offset] as usize;
            if record_len == 0 {
                // Padding to next sector
                let next_sector =
                    ((offset / ISO_SECTOR_SIZE as usize) + 1) * ISO_SECTOR_SIZE as usize;
                if next_sector >= size as usize {
                    break;
                }
                offset = next_sector;
                continue;
            }

            if offset + record_len > buf.len() {
                break;
            }

            let extent = u32::from_le_bytes([
                buf[offset + 2],
                buf[offset + 3],
                buf[offset + 4],
                buf[offset + 5],
            ]);
            let file_size = u32::from_le_bytes([
                buf[offset + 10],
                buf[offset + 11],
                buf[offset + 12],
                buf[offset + 13],
            ]);
            let flags = buf[offset + 25];
            let name_len = buf[offset + 32] as usize;

            if name_len > 0 && offset + 33 + name_len <= buf.len() {
                let name_bytes = &buf[offset + 33..offset + 33 + name_len];

                // Skip . and .. entries
                if name_bytes == [0x00] || name_bytes == [0x01] {
                    offset += record_len;
                    continue;
                }

                // Convert name to string, strip version suffix
                let name_str = core::str::from_utf8(name_bytes).unwrap_or("");
                let mut name = if let Some(pos) = name_str.find(';') {
                    String::from(&name_str[..pos])
                } else {
                    String::from(name_str)
                };

                // Parse System Use Area for Rock Ridge NM (Alternate Name)
                let mut sys_use_offset = offset + 33 + name_len;
                if name_len % 2 == 0 {
                    sys_use_offset += 1;
                }

                let mut rock_ridge_name = String::new();
                let mut found_nm = false;

                while sys_use_offset + 4 <= offset + record_len {
                    let sig = &buf[sys_use_offset..sys_use_offset + 2];
                    let len = buf[sys_use_offset + 2] as usize;
                    let _ver = buf[sys_use_offset + 3];

                    if len < 4 || sys_use_offset + len > offset + record_len {
                        break;
                    }

                    if sig == b"NM" {
                        let _flags = buf[sys_use_offset + 4];
                        let name_start = sys_use_offset + 5;
                        let name_end = sys_use_offset + len;
                        
                        if name_end > name_start {
                             if let Ok(nm_part) = core::str::from_utf8(&buf[name_start..name_end]) {
                                 rock_ridge_name.push_str(nm_part);
                                 found_nm = true;
                             }
                        }
                        
                        // If CONTINUE bit (0) or others are not set, we might be done, 
                        // but NM entries can be split. We just append them all.
                    } else if sig == b"CE" {
                        // Continuation Area (implied TODO: simple NM parsing normally resides in the record itself)
                    } else if sig == b"ST" {
                        // Terminator
                        break;
                    }

                    sys_use_offset += len;
                }

                if found_nm {
                    name = rock_ridge_name;
                }

                entries.push(IsoDirEntry {
                    name,
                    extent_lba: extent,
                    size: file_size,
                    is_directory: (flags & 0x02) != 0,
                });
            }

            offset += record_len;
        }

        entries
    }

    /// List entries in a directory.
    pub fn list_dir(&self, dev: &dyn BlockDevice, extent_lba: u32, size: u32) -> Vec<IsoDirEntry> {
        // Use the cache to avoid re-parsing directories
        self.parse_and_cache_dir(dev, extent_lba, size)
    }

    /// List root directory entries.
    pub fn list_root(&self, dev: &dyn BlockDevice) -> Vec<IsoDirEntry> {
        self.list_dir(dev, self.pvd.root_dir_extent, self.pvd.root_dir_size)
    }

    /// Resolve a path and return the directory entry.
    ///
    /// Path should be absolute, e.g., "/ASSETS/CURSOR.SVG".
    pub fn open_path(&self, dev: &dyn BlockDevice, path: &str) -> Option<IsoFile> {
        let path = path.trim_start_matches('/');
        if path.is_empty() {
            return None;
        }

        let parts: Vec<&str> = path.split('/').collect();
        let mut current_extent = self.pvd.root_dir_extent;
        let mut current_size = self.pvd.root_dir_size;

        for (i, part) in parts.iter().enumerate() {
            #[cfg(feature = "perf")]
            {
                self.perf.borrow_mut().path_resolution_steps += 1;
            }

            let is_last = i == parts.len() - 1;
            let entries = self.list_dir(dev, current_extent, current_size);

            // Use allocation-free case-insensitive comparison
            let entry = entries
                .iter()
                .find(|e| Self::ascii_eq_ignore_case(&e.name, part))?;

            if is_last {
                return Some(IsoFile {
                    extent_lba: entry.extent_lba,
                    size: entry.size,
                });
            } else {
                if !entry.is_directory {
                    return None;
                }
                current_extent = entry.extent_lba;
                current_size = entry.size;
            }
        }

        None
    }
}

impl IsoFile {
    /// Read the entire file contents.
    pub fn read_all(&self, dev: &dyn BlockDevice) -> Result<Vec<u8>, BlockError> {
        let sectors_needed = (self.size as u64 + ISO_SECTOR_SIZE - 1) / ISO_SECTOR_SIZE;
        let buf_size = (sectors_needed * ISO_SECTOR_SIZE) as usize;
        let mut buf = alloc::vec![0u8; buf_size];

        dev.read_sectors(self.extent_lba as u64, sectors_needed, &mut buf)?;

        // Truncate to actual file size
        buf.truncate(self.size as usize);
        Ok(buf)
    }

    /// Read a range of bytes from the file.
    pub fn read_range(
        &self,
        dev: &dyn BlockDevice,
        offset: u64,
        length: usize,
    ) -> Result<Vec<u8>, BlockError> {
        if offset >= self.size as u64 {
            return Ok(Vec::new());
        }

        let actual_len = core::cmp::min(length, (self.size as u64 - offset) as usize);
        let start_sector = offset / ISO_SECTOR_SIZE;
        let end_offset = offset + actual_len as u64;
        let end_sector = (end_offset + ISO_SECTOR_SIZE - 1) / ISO_SECTOR_SIZE;
        let sectors_to_read = end_sector - start_sector;

        let buf_size = (sectors_to_read * ISO_SECTOR_SIZE) as usize;
        let mut buf = alloc::vec![0u8; buf_size];

        dev.read_sectors(
            self.extent_lba as u64 + start_sector,
            sectors_to_read,
            &mut buf,
        )?;

        let start_in_buf = (offset % ISO_SECTOR_SIZE) as usize;
        Ok(buf[start_in_buf..start_in_buf + actual_len].to_vec())
    }
}

/// Helper to convert volume ID to a trimmed string.
pub fn volume_id_str(pvd: &PrimaryVolumeDescriptor) -> &str {
    core::str::from_utf8(&pvd.volume_id).unwrap_or("").trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_eq_ignore_case() {
        // Basic case-insensitive comparison
        assert!(IsoFs::ascii_eq_ignore_case("HELLO", "hello"));
        assert!(IsoFs::ascii_eq_ignore_case("hello", "HELLO"));
        assert!(IsoFs::ascii_eq_ignore_case("MixedCase", "mixedcase"));

        // Version suffix handling
        assert!(IsoFs::ascii_eq_ignore_case("FILE.TXT;1", "file.txt"));
        assert!(IsoFs::ascii_eq_ignore_case("FILE.TXT", "file.txt;1"));
        // Version suffixes are completely stripped, so any version matches
        assert!(IsoFs::ascii_eq_ignore_case("README;1", "readme;2"));

        // Different strings
        assert!(!IsoFs::ascii_eq_ignore_case("hello", "world"));
        assert!(!IsoFs::ascii_eq_ignore_case("FILE", "FILES"));

        // Empty strings
        assert!(IsoFs::ascii_eq_ignore_case("", ""));
        assert!(!IsoFs::ascii_eq_ignore_case("", "hello"));
    }
}
