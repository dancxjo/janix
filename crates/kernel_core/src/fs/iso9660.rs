use alloc::string::String;
use alloc::vec::Vec;
use alloc::alloc::{alloc, Layout};

const SECTOR_SIZE: usize = 2048;

pub trait BlockReader {
    fn read_sector(&self, lba: u32, buf: &mut [u8]) -> bool;
}

impl<F> BlockReader for F
where
    F: Fn(u32, &mut [u8]) -> bool,
{
    fn read_sector(&self, lba: u32, buf: &mut [u8]) -> bool {
        self(lba, buf)
    }
}

impl BlockReader for alloc::boxed::Box<dyn BlockReader + Send + Sync> {
    fn read_sector(&self, lba: u32, buf: &mut [u8]) -> bool {
        (**self).read_sector(lba, buf)
    }
}

pub struct Iso9660Reader<R: BlockReader> {
    reader: R,
    root_lba: u32,
    root_len: u32,
}

#[derive(Debug, Clone)]
pub struct DirEntry {
    pub name: String,
    pub name_raw: String,
    pub lba: u32,
    pub size: u32,
    pub is_dir: bool,
}

pub struct FileHandle {
    pub lba: u32,
    pub size: u32,
}

impl<R: BlockReader> Iso9660Reader<R> {
    pub fn new(reader: R) -> Option<Self> {
        // Allocate aligned buffer manually
        let layout = Layout::from_size_align(SECTOR_SIZE, 4096).ok()?;
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() { return None; }
        let mut buf_backing = unsafe { Vec::from_raw_parts(ptr, SECTOR_SIZE, SECTOR_SIZE) };
        let buf = &mut buf_backing;

        // PVD is usually at sector 16
        if !reader.read_sector(16, buf) {
            return None;
        }

        // Check Standard Identifier "CD001"
        if &buf[1..6] != b"CD001" {
            return None;
        }

        // Root Directory Record is at offset 156
        // Parse root record to get location/size
        let (lba, len, _flags, _name, _name_raw) = Self::parse_dir_record(&buf, 156)?;

        Some(Self {
            reader,
            root_lba: lba,
            root_len: len,
        })
    }

    fn parse_dir_record(buf: &[u8], offset: usize) -> Option<(u32, u32, u8, String, String)> {
        if offset + 33 > buf.len() {
            return None;
        }
        let len = buf[offset];
        if len == 0 {
            return None;
        } // Padding or end

        let extent_lba = u32::from_le_bytes(buf[offset + 2..offset + 6].try_into().ok()?);
        let data_len = u32::from_le_bytes(buf[offset + 10..offset + 14].try_into().ok()?);
        let flags = buf[offset + 25];
        let name_len = buf[offset + 32] as usize;

        // Check bounds
        if offset + 33 + name_len > buf.len() {
            return None;
        }

        let name_bytes = &buf[offset + 33..offset + 33 + name_len];
        let (name, name_raw) = if name_len == 1 && name_bytes[0] == 0 {
            (String::from("."), String::from("."))
        } else if name_len == 1 && name_bytes[0] == 1 {
            (String::from(".."), String::from(".."))
        } else {
            // ISO9660 usually has ";1" suffix.
            let s = core::str::from_utf8(name_bytes).ok()?;
            let clean = s.split(';').next().unwrap_or(s).to_ascii_lowercase();
            (clean, String::from(s))
        };

        Some((extent_lba, data_len, flags, name, name_raw))
    }

    pub fn open(&self, path: &str) -> Option<FileHandle> {
        // Start at root
        let mut current_lba = self.root_lba;
        let mut current_len = self.root_len;

        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        // If path is empty or just "/", return root as handle?
        if parts.is_empty() {
            return Some(FileHandle {
                lba: current_lba,
                size: current_len,
            });
        }

        for (i, part) in parts.iter().enumerate() {
            // Search in current directory
            let entry = self.find_entry(current_lba, current_len, part)?;
            if entry.is_dir {
                current_lba = entry.lba;
                current_len = entry.size;
            } else {
                // If it's the last part, return handle
                if i == parts.len() - 1 {
                    return Some(FileHandle {
                        lba: entry.lba,
                        size: entry.size,
                    });
                } else {
                    return None; // File used as dir
                }
            }
        }

        // If we processed all parts and ended on a dir
        Some(FileHandle {
            lba: current_lba,
            size: current_len,
        })
    }

    fn find_entry(&self, dir_lba: u32, dir_len: u32, name: &str) -> Option<DirEntry> {
        let num_sectors = (dir_len + SECTOR_SIZE as u32 - 1) / SECTOR_SIZE as u32;
        
        let layout = Layout::from_size_align(SECTOR_SIZE, 4096).ok()?;
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() { return None; }
        let mut buf_backing = unsafe { Vec::from_raw_parts(ptr, SECTOR_SIZE, SECTOR_SIZE) };
        let buf = &mut buf_backing;

        for i in 0..num_sectors {
            if !self.reader.read_sector(dir_lba + i, buf) {
                return None;
            }

            let mut offset = 0;
            while offset < SECTOR_SIZE {
                let rec_len = buf[offset];
                if rec_len == 0 {
                    break;
                } // End of records in this sector

                if let Some((lba, size, flags, entry_name, name_raw)) =
                    Self::parse_dir_record(&buf, offset)
                {
                    if entry_name.eq_ignore_ascii_case(name) {
                        return Some(DirEntry {
                            name: entry_name,
                            name_raw,
                            lba,
                            size,
                            is_dir: (flags & 2) != 0,
                        });
                    }
                }
                offset += rec_len as usize;
            }
        }
        None
    }

    pub fn read(
        &self,
        handle: &FileHandle,
        offset: usize,
        len: usize,
        out: &mut [u8],
    ) -> usize {
        let start_sector = offset / SECTOR_SIZE;
        let end_sector = (offset + len + SECTOR_SIZE - 1) / SECTOR_SIZE;
        let mut read_len = 0;
        let layout = Layout::from_size_align(SECTOR_SIZE, 4096).unwrap();
        let ptr = unsafe { alloc(layout) };
        // If alloc fails, we panic (read returns usize, can't easily return error without change signature)
        // Ideally should handle error, but for now wrap in Vec
        let mut buf_backing = unsafe { Vec::from_raw_parts(ptr, SECTOR_SIZE, SECTOR_SIZE) };
        let sector_buf = &mut buf_backing;

        for i in start_sector..end_sector {
            if !self
                .reader
                .read_sector(handle.lba + i as u32, sector_buf)
            {
                break;
            }

            let sector_offset = if i == start_sector {
                offset % SECTOR_SIZE
            } else {
                0
            };

            let remaining_req = len - read_len;
            let available_in_sector = SECTOR_SIZE - sector_offset;
            // Also limit by file size?
            // Handle size check
            let file_remaining = (handle.size as usize).saturating_sub(offset + read_len);

            let copy_len = core::cmp::min(remaining_req, available_in_sector);
            let copy_len = core::cmp::min(copy_len, file_remaining);

            if copy_len == 0 {
                break;
            }

            out[read_len..read_len + copy_len]
                .copy_from_slice(&sector_buf[sector_offset..sector_offset + copy_len]);
            read_len += copy_len;
        }
        read_len
    }

    pub fn read_dir(&self, path: &str) -> Option<Vec<DirEntry>> {
        let handle = self.open(path)?;
        if handle.size == 0 {
            return Some(Vec::new());
        }

        let dir_lba = handle.lba;
        let dir_len = handle.size;

        let mut entries = Vec::new();
        let num_sectors = (dir_len + SECTOR_SIZE as u32 - 1) / SECTOR_SIZE as u32;
        
        let layout = Layout::from_size_align(SECTOR_SIZE, 4096).ok()?;
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() { return None; }
        let mut buf_backing = unsafe { Vec::from_raw_parts(ptr, SECTOR_SIZE, SECTOR_SIZE) };
        let buf = &mut buf_backing;

        for i in 0..num_sectors {
            if !self.reader.read_sector(dir_lba + i, buf) {
                return None;
            }
            let mut offset = 0;
            while offset < SECTOR_SIZE {
                let rec_len = buf[offset];
                if rec_len == 0 {
                    break;
                }

                if let Some((lba, size, flags, name, name_raw)) =
                    Self::parse_dir_record(&buf, offset)
                {
                    if name != "." && name != ".." {
                        entries.push(DirEntry {
                            name,
                            name_raw,
                            lba,
                            size,
                            is_dir: (flags & 2) != 0,
                        });
                    }
                }
                offset += rec_len as usize;
            }
        }
        Some(entries)
    }

    pub fn list_dir(&self, path: &str) -> Option<Vec<String>> {
        self.read_dir(path)
            .map(|entries| entries.into_iter().map(|e| e.name).collect())
    }
}
