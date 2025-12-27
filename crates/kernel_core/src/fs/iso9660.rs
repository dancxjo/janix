use alloc::vec::Vec;
use alloc::string::String;
use thing_models::core::fs::{FileBody, FilesystemBody, FilesystemKind};

// 2048 bytes per sector
pub const SECTOR_SIZE: usize = 2048;

// PVD is usually at sector 16
pub const PVD_SECTOR: u32 = 16;

#[repr(C, packed)]
struct VolumeDescriptor {
    type_code: u8,
    id: [u8; 5],
    version: u8,
    data: [u8; 2041],
}

#[repr(C, packed)]
struct DirectoryRecord {
    len: u8,
    ext_attr_len: u8,
    extent_loc_lsb: u32,
    extent_loc_msb: u32, // Unused
    data_len_lsb: u32,
    data_len_msb: u32,   // Unused
    record_date: [u8; 7],
    flags: u8,
    file_unit_size: u8,
    interleave_gap: u8,
    vol_seq_num: u16,  // ignored
    vol_seq_num_msb: u16, // ignored
    name_len: u8,
    // Name follows...
}

pub struct Iso9660Reader<F> 
where F: FnMut(u32, &mut [u8]) -> bool 
{
    pub read_sector: F,
    pub buffer: Vec<u8>,
}

impl<F> Iso9660Reader<F> 
where F: FnMut(u32, &mut [u8]) -> bool
{
    pub fn new(read_fn: F) -> Self {
        let mut buf = Vec::with_capacity(SECTOR_SIZE);
        buf.resize(SECTOR_SIZE, 0);
        Self {
            read_sector: read_fn,
            buffer: buf,
        }
    }

    pub fn scan_root(&mut self) -> Option<(FilesystemBody, Vec<FileBody>)> {
        // 1. Read PVD
        if !(self.read_sector)(PVD_SECTOR, &mut self.buffer) {
            return None;
        }

        // Check Signature "CD001"
        if self.buffer[1..6] != *b"CD001" {
            return None;
        }

        // Root Directory Record is at offset 156
        let root_record_slice = &self.buffer[156..156+34]; // Min size 34
        
        // Parse extent
         let root_extent = u32::from_le_bytes([
            root_record_slice[2], root_record_slice[3], root_record_slice[4], root_record_slice[5]
        ]);
        
        let root_size = u32::from_le_bytes([
            root_record_slice[10], root_record_slice[11], root_record_slice[12], root_record_slice[13]
        ]);
        
        let fs_body = FilesystemBody {
            name: String::from("ISO9660-CD"),
            kind: FilesystemKind::Iso9660,
            read_only: true,
        };
        
        // Scan Root Directory
        let mut files = Vec::new();
        self.scan_dir(root_extent, root_size, &mut files, String::from(""), 0);

        Some((fs_body, files))
    }
    
    fn scan_dir(&mut self, extent: u32, size: u32, files: &mut Vec<FileBody>, prefix: String, depth: u32) {
        if depth > 5 { return; } // recursion limit
        
        let sectors = (size + 2047) / 2048;
        let mut sector_buf = Vec::with_capacity(2048);
        sector_buf.resize(2048, 0);

        for i in 0..sectors {
            if !(self.read_sector)(extent + i, &mut sector_buf) {
                break;
            }
            
            let mut offset = 0;
            while offset < 2048 {
                let len = sector_buf[offset];
                if len == 0 { break; } // Padding
                
                let rec_slice = &sector_buf[offset..(offset + len as usize)];
                let flags = rec_slice[25];
                let name_len = rec_slice[32];
                let name_bytes = &rec_slice[33..(33 + name_len as usize)];
                
                let file_extent = u32::from_le_bytes([rec_slice[2], rec_slice[3], rec_slice[4], rec_slice[5]]);
                let file_size = u32::from_le_bytes([rec_slice[10], rec_slice[11], rec_slice[12], rec_slice[13]]);

                // Handle Name
                let mut name_str = String::new();
                if name_len == 1 && name_bytes[0] == 0 {
                    name_str = String::from(".");
                } else if name_len == 1 && name_bytes[0] == 1 {
                    name_str = String::from("..");
                } else {
                     if let Ok(s) = core::str::from_utf8(name_bytes) {
                         name_str = String::from(s);
                         // Strip ;1 version suffix
                         if let Some(idx) = name_str.find(';') {
                             name_str.truncate(idx);
                         }
                     } else {
                         name_str = String::from("???");
                     }
                }

                if name_str != "." && name_str != ".." {
                    let full_path = if prefix.is_empty() {
                         name_str.clone()
                    } else {
                         alloc::format!("{}/{}", prefix, name_str)
                    };
                    
                    let is_dir = (flags & 0x02) != 0;
                    
                    files.push(FileBody {
                        name: full_path.clone(),
                        size: file_size as u64,
                        is_dir,
                        start_sector: file_extent,
                    });
                    
                    if is_dir {
                        // Recursive call
                        // Note: Limit depth to avoid stack overflow
                         self.scan_dir(file_extent, file_size, files, full_path, depth + 1);
                    }
                }
                
                offset += len as usize;
            }
        }
    }
}
