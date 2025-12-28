use alloc::alloc::{alloc, dealloc, Layout};
use alloc::string::String;
use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};

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

struct AlignedBuffer {
    ptr: *mut u8,
    layout: Layout,
}

impl AlignedBuffer {
    fn new(size: usize) -> Option<Self> {
        let layout = Layout::from_size_align(size, 4096).ok()?;
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr, layout })
        }
    }
}

impl Drop for AlignedBuffer {
    fn drop(&mut self) {
        unsafe { dealloc(self.ptr, self.layout) };
    }
}

impl Deref for AlignedBuffer {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.ptr, self.layout.size()) }
    }
}

impl DerefMut for AlignedBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.layout.size()) }
    }
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
        // Use aligned buffer for PVD read
        let mut buf = AlignedBuffer::new(SECTOR_SIZE)?;

        // PVD is usually at sector 16
        if !reader.read_sector(16, &mut buf) {
            return None;
        }

        // Check Standard Identifier "CD001"
        if &buf[1..6] != b"CD001" {
            return None;
        }

        // Root Directory Record is at offset 156
        // 34 bytes is min size for a dir record
        let root_entry_ptr = &buf[156..156 + 34];
        let root_lba = u32::from_le_bytes(root_entry_ptr[2..6].try_into().unwrap());
        let root_len = u32::from_le_bytes(root_entry_ptr[10..14].try_into().unwrap());

        Some(Self {
            reader,
            root_lba,
            root_len,
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

        let mut buf = AlignedBuffer::new(SECTOR_SIZE)?;
        let sector_buf = &mut buf[0..SECTOR_SIZE];

        for i in 0..num_sectors {
            if !self.reader.read_sector(dir_lba + i, sector_buf) {
                return None;
            }

            let mut offset = 0;
            while offset < SECTOR_SIZE {
                let rec_len = sector_buf[offset];
                if rec_len == 0 {
                    break;
                } // End of records in this sector

                if let Some((_lba, _len, _flags, entry_name, _entry_name_raw)) =
                    Self::parse_dir_record(sector_buf, offset)
                {
                    if entry_name == name {
                        // Found match!
                        let (lba, len, flags, name, name_raw) =
                            Self::parse_dir_record(sector_buf, offset)?;
                        return Some(DirEntry {
                            name,
                            name_raw,
                            lba,
                            size: len,
                            is_dir: (flags & 2) != 0,
                        });
                    }
                }
                offset += rec_len as usize;
            }
        }
        None
    }

    pub fn read(&self, handle: &FileHandle, offset: usize, len: usize, out: &mut [u8]) -> usize {
        // limit to 4KB (1 page) to ensure physical contiguity for AHCI
        // (allocator only guarantees contiguity within a page)
        let chunk_size = 4096;
        let sectors_per_chunk = chunk_size / SECTOR_SIZE;

        let start_sector_abs = offset / SECTOR_SIZE;
        let end_sector_abs = (offset + len + SECTOR_SIZE - 1) / SECTOR_SIZE;

        // Use local AlignedBuffer
        let mut chunk_buf = match AlignedBuffer::new(chunk_size) {
            Some(b) => b,
            None => return 0,
        };

        let mut current_sector = start_sector_abs;
        let mut read_len = 0;

        while current_sector < end_sector_abs {
            // How many sectors to read? Min(remaining in file, chunk_capacity)
            let remaining_sectors = end_sector_abs - current_sector;
            let sectors_to_read = core::cmp::min(remaining_sectors, sectors_per_chunk);
            let bytes_to_read = sectors_to_read * SECTOR_SIZE;

            // Adjust buffer size for this read
            let current_buf = &mut chunk_buf[0..bytes_to_read];

            if !self
                .reader
                .read_sector(handle.lba + current_sector as u32, current_buf)
            {
                break;
            }

            // Copy relevant bytes to output
            let _chunk_start_offset = current_sector * SECTOR_SIZE;

            for i in 0..sectors_to_read {
                let sector_idx = current_sector + i;
                let sector_file_offset = sector_idx * SECTOR_SIZE;

                let copy_start_in_sector = if sector_file_offset < offset {
                    offset - sector_file_offset
                } else {
                    0
                };

                let val_start = sector_file_offset + copy_start_in_sector;
                // How much to copy? Min(available in sector, remaining req)

                let available = SECTOR_SIZE - copy_start_in_sector;
                let need = len - read_len;
                let file_left = (handle.size as usize).saturating_sub(val_start);

                let copy = core::cmp::min(available, need);
                let copy = core::cmp::min(copy, file_left);

                if copy > 0 {
                    let src_start = i * SECTOR_SIZE + copy_start_in_sector;
                    out[read_len..read_len + copy]
                        .copy_from_slice(&current_buf[src_start..src_start + copy]);
                    read_len += copy;
                }
            }

            current_sector += sectors_to_read;
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

        let mut buf = AlignedBuffer::new(SECTOR_SIZE)?;
        let sector_buf = &mut buf[0..SECTOR_SIZE];

        for i in 0..num_sectors {
            if !self.reader.read_sector(dir_lba + i, sector_buf) {
                return None;
            }
            let mut offset = 0;
            while offset < SECTOR_SIZE {
                let rec_len = sector_buf[offset];
                if rec_len == 0 {
                    break;
                }

                if let Some((lba, size, flags, name, name_raw)) =
                    Self::parse_dir_record(sector_buf, offset)
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
