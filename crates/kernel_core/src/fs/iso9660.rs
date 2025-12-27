use alloc::string::String;
use alloc::vec::Vec;

const SECTOR_SIZE: usize = 2048;

pub trait BlockReader {
    fn read_sector(&mut self, lba: u32, buf: &mut [u8]) -> bool;
}

impl<F> BlockReader for F
where
    F: FnMut(u32, &mut [u8]) -> bool,
{
    fn read_sector(&mut self, lba: u32, buf: &mut [u8]) -> bool {
        self(lba, buf)
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
    pub lba: u32,
    pub size: u32,
    pub is_dir: bool,
}

pub struct FileHandle {
    pub lba: u32,
    pub size: u32,
}

impl<R: BlockReader> Iso9660Reader<R> {
    pub fn new(mut reader: R) -> Option<Self> {
        let mut buf = [0u8; SECTOR_SIZE];

        // PVD is usually at sector 16
        if !reader.read_sector(16, &mut buf) {
            return None;
        }

        // Check Standard Identifier "CD001"
        if &buf[1..6] != b"CD001" {
            return None;
        }

        // Root Directory Record is at offset 156
        // Parse root record to get location/size
        let (lba, len, _flags, _name) = Self::parse_dir_record(&buf, 156)?;

        Some(Self {
            reader,
            root_lba: lba,
            root_len: len,
        })
    }

    fn parse_dir_record(buf: &[u8], offset: usize) -> Option<(u32, u32, u8, String)> {
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
        let name = if name_len == 1 && name_bytes[0] == 0 {
            String::from(".")
        } else if name_len == 1 && name_bytes[0] == 1 {
            String::from("..")
        } else {
            // Handle RockRidge / Joliet? For now assume ASCII/UTF8
            // ISO9660 usually has ";1" suffix.
            let s = core::str::from_utf8(name_bytes).ok()?;
            let clean = s.split(';').next().unwrap_or(s);
            String::from(clean)
        };

        Some((extent_lba, data_len, flags, name))
    }

    pub fn open(&mut self, path: &str) -> Option<FileHandle> {
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

    fn find_entry(&mut self, dir_lba: u32, dir_len: u32, name: &str) -> Option<DirEntry> {
        let num_sectors = (dir_len + SECTOR_SIZE as u32 - 1) / SECTOR_SIZE as u32;
        let mut buf = [0u8; SECTOR_SIZE];

        for i in 0..num_sectors {
            if !self.reader.read_sector(dir_lba + i, &mut buf) {
                return None;
            }

            let mut offset = 0;
            while offset < SECTOR_SIZE {
                let rec_len = buf[offset];
                if rec_len == 0 {
                    break;
                } // End of records in this sector

                if let Some((lba, size, flags, entry_name)) = Self::parse_dir_record(&buf, offset) {
                    if entry_name.eq_ignore_ascii_case(name) {
                        return Some(DirEntry {
                            name: entry_name,
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

    pub fn read(&mut self, handle: &FileHandle, offset: usize, len: usize, out: &mut [u8]) -> usize {
        let start_sector = offset / SECTOR_SIZE;
        let end_sector = (offset + len + SECTOR_SIZE - 1) / SECTOR_SIZE;
        let mut read_len = 0;
        let mut sector_buf = [0u8; SECTOR_SIZE];

        for i in start_sector..end_sector {
            if !self.reader.read_sector(handle.lba + i as u32, &mut sector_buf) {
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

            if copy_len == 0 { break; }

            out[read_len..read_len + copy_len]
                .copy_from_slice(&sector_buf[sector_offset..sector_offset + copy_len]);
            read_len += copy_len;
        }
        read_len
    }

    pub fn list_dir(&mut self, path: &str) -> Option<Vec<String>> {
        let handle = self.open(path)?;

        let dir_lba = handle.lba;
        let dir_len = handle.size;

        let mut names = Vec::new();
        let num_sectors = (dir_len + SECTOR_SIZE as u32 - 1) / SECTOR_SIZE as u32;
        let mut buf = [0u8; SECTOR_SIZE];

        for i in 0..num_sectors {
             if !self.reader.read_sector(dir_lba + i, &mut buf) {
                 return None;
             }
             let mut offset = 0;
             while offset < SECTOR_SIZE {
                 let rec_len = buf[offset];
                 if rec_len == 0 { break; }

                 if let Some((_, _, _, name)) = Self::parse_dir_record(&buf, offset) {
                      if name != "." && name != ".." {
                          names.push(name);
                      }
                 }
                 offset += rec_len as usize;
             }
        }
        Some(names)
    }
}
