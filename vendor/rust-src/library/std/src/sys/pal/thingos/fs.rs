
use crate::ffi::{OsString, OsStr, CStr, CString};
use crate::fmt;
use crate::io::{self, IoSlice, IoSliceMut, SeekFrom};
use crate::path::{Path, PathBuf, Component};
use crate::sys::time::SystemTime;
use crate::sys::unsupported;
use crate::os::thingos::ffi::OsStrExt;
use crate::cell::Cell;

use stem::abi::wire::ThingId;
use stem::abi::types::edge::Edge;
use stem::syscall::{
    root_intern, root_create_node, root_link, root_get_edges,
    root_bytespace_create, root_bytespace_read, root_bytespace_write, 
    root_bytespace_info, root_find
};

pub struct File {
    node_id: u64,
    offset: Cell<u64>, 
}

#[derive(Clone)]
pub struct FileAttr {
    pub size: u64,
    pub kind: u64,
    pub perm: FilePermissions,
    pub created: SystemTime,
    pub modified: SystemTime,
    pub accessed: SystemTime,
}

pub struct ReadDir {
    root: u64,
    data: crate::vec::Vec<u8>,
    idx: usize,
}

pub struct DirEntry {
    name: OsString,
    node: u64,
}

#[derive(Clone, Debug)]
pub struct OpenOptions {
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FilePermissions {
    readonly: bool,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct FileType {
    is_dir: bool,
    is_file: bool,
    is_symlink: bool,
}

pub struct DirBuilder {}

impl FileAttr {
    pub fn size(&self) -> u64 { self.size }
    pub fn perm(&self) -> FilePermissions { self.perm.clone() }
    pub fn file_type(&self) -> FileType {
        FileType {
            is_dir: false, 
            is_file: true,
            is_symlink: false,
        }
    }
    pub fn modified(&self) -> io::Result<SystemTime> { Ok(self.modified) }
    pub fn accessed(&self) -> io::Result<SystemTime> { Ok(self.accessed) }
    pub fn created(&self) -> io::Result<SystemTime> { Ok(self.created) }
}

impl FilePermissions {
    pub fn readonly(&self) -> bool { self.readonly }
    pub fn set_readonly(&mut self, readonly: bool) { self.readonly = readonly; }
}

impl FileType {
    pub fn is_dir(&self) -> bool { self.is_dir }
    pub fn is_file(&self) -> bool { self.is_file }
    pub fn is_symlink(&self) -> bool { self.is_symlink }
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;
    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        None
    }
}

impl DirEntry {
    pub fn path(&self) -> PathBuf { PathBuf::from(&self.name) }
    pub fn file_name(&self) -> OsString { self.name.clone() }
    pub fn metadata(&self) -> io::Result<FileAttr> {
        Ok(FileAttr {
            size: 0,
            kind: 0,
            perm: FilePermissions { readonly: false },
            created: SystemTime::zero(),
            modified: SystemTime::zero(),
            accessed: SystemTime::zero(),
        })
    }
    pub fn file_type(&self) -> io::Result<FileType> {
        Ok(FileType { is_dir: false, is_file: true, is_symlink: false })
    }
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions {
            read: false,
            write: false,
            append: false,
            truncate: false,
            create: false,
            create_new: false,
        }
    }

    pub fn read(&mut self, read: bool) { self.read = read; }
    pub fn write(&mut self, write: bool) { self.write = write; }
    pub fn append(&mut self, append: bool) { self.append = append; }
    pub fn truncate(&mut self, truncate: bool) { self.truncate = truncate; }
    pub fn create(&mut self, create: bool) { self.create = create; }
    pub fn create_new(&mut self, create_new: bool) { self.create_new = create_new; }
}

/// Find the FS root.
fn find_root() -> io::Result<u64> {
     let root_kind_sym = "FilesystemRoot";
     let root_kind = root_intern(root_kind_sym).map_err(|e| io::Error::from_raw_os_error(e as i32))?;
     
     let mut ids = [0u8; 128]; // Buffer for ThingIds (16 bytes each)
     let count = root_find(root_kind, ids.as_mut_ptr(), ids.len())
         .map_err(|e| io::Error::from_raw_os_error(e as i32))?;
     
     if count > 0 {
         // Assuming first 16 bytes is the ThingId
         let mut id_bytes = [0u8; 16];
         id_bytes.copy_from_slice(&ids[0..16]);
         let tid = ThingId(id_bytes);
         Ok(tid.to_u64_lossy())
     } else {
         let id = root_create_node(root_kind as usize).map_err(|e| io::Error::from_raw_os_error(e as i32))?;
         Ok(id as u64)
     }
}

impl File {
    pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
        let mut curr = find_root()?;
        
        let components: crate::vec::Vec<_> = path.components().collect();
        
        for (i, comp) in components.iter().enumerate() {
            match comp {
                Component::RootDir => continue,
                Component::CurDir => continue,
                Component::ParentDir => return Err(io::Error::UNSUPPORTED_PLATFORM), // TODO
                Component::Normal(name) => {
                    let name_str = name.to_str().ok_or(io::Error::other("invalid filename"))?;
                    let pred_val = root_intern(name_str).map_err(|e| io::Error::from_raw_os_error(e as i32))?;
                    let pred_id = pred_val as u64; // Handle or ID? root_intern returns usize handle
                    
                    let mut buf = [0u8; 1024]; 
                    let count_res = root_get_edges(curr as usize, buf.as_mut_ptr(), buf.len());
                    let count = count_res.map_err(|e| io::Error::from_raw_os_error(e as i32))?;
                    
                    let edges_ptr = buf.as_ptr() as *const Edge;
                    let mut found = None;
                    
                    for j in 0..count {
                        let edge = unsafe { &*edges_ptr.add(j) };
                        if edge.predicate.to_u64_lossy() == pred_id {
                             found = Some(edge.to.to_u64_lossy());
                             break;
                        }
                    }
                    
                    if let Some(next) = found {
                        curr = next;
                    } else {
                        if i == components.len() - 1 && (opts.create || opts.create_new) {
                             // Create bytespace (which includes the node)
                             let bs = root_bytespace_create(4096, 0, 0)
                                 .map_err(|e| io::Error::from_raw_os_error(e as i32))?;
                             
                             let res = root_link(curr as usize, pred_id as usize, bs as usize)
                                 .map_err(|e| io::Error::from_raw_os_error(e as i32))?;
                                 
                             curr = bs as u64;
                        } else {
                             return Err(io::Error::from_raw_os_error(2)); // ENOENT
                        }
                    }
                },
                _ => {}
            }
        }
        
        Ok(File { node_id: curr, offset: Cell::new(0) })
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        let len_res = root_bytespace_info(self.node_id as usize);
        
        let size = match len_res {
            Ok(len) => len as u64,
            Err(_) => 0,
        };

        Ok(FileAttr {
            size,
            kind: 0,
            perm: FilePermissions { readonly: false },
            created: SystemTime::zero(),
            modified: SystemTime::zero(),
            accessed: SystemTime::zero(),
        })
    }

    pub fn fsync(&self) -> io::Result<()> { Ok(()) }
    pub fn datasync(&self) -> io::Result<()> { Ok(()) }
    pub fn truncate(&self, size: u64) -> io::Result<()> { Ok(()) }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        let off = self.offset.get();
        let res = root_bytespace_read(self.node_id as usize, off as usize, buf)
            .map_err(|e| io::Error::from_raw_os_error(e as i32))?;
        let n = res as usize;
        self.offset.set(off + n as u64);
        Ok(n)
    }

    pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        crate::io::default_read_vectored(|b| self.read(b), bufs)
    }

    pub fn is_read_vectored(&self) -> bool { false }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let off = self.offset.get();
        let res = root_bytespace_write(self.node_id as usize, off as usize, buf)
            .map_err(|e| io::Error::from_raw_os_error(e as i32))?;
        let n = res as usize;
        self.offset.set(off + n as u64);
        Ok(n)
    }

    pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        crate::io::default_write_vectored(|b| self.write(b), bufs)
    }

    pub fn is_write_vectored(&self) -> bool { false }
    pub fn flush(&self) -> io::Result<()> { Ok(()) }
    pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
        let curr = self.offset.get();
        let new_pos = match pos {
            SeekFrom::Start(p) => p,
            SeekFrom::End(_) => return Err(io::Error::UNSUPPORTED_PLATFORM), 
            SeekFrom::Current(d) => {
                if d >= 0 {
                    curr.checked_add(d as u64).ok_or(io::Error::other("seek overflow"))?
                } else {
                    curr.checked_sub((-d) as u64).ok_or(io::Error::other("seek underflow"))?
                }
            }
        };
        self.offset.set(new_pos);
        Ok(new_pos)
    }
    
    pub fn duplicate(&self) -> io::Result<File> {
        Ok(File { node_id: self.node_id, offset: self.offset.clone() })
    }

    pub fn set_permissions(&self, _perm: FilePermissions) -> io::Result<()> { Ok(()) }
    pub fn set_times(&self, _times: crate::fs::FileTimes) -> io::Result<()> { Ok(()) }
}

pub struct Dir(u64);

impl Dir {
    pub fn new(root: u64) -> Dir { Dir(root) }
}

impl Iterator for Dir {
    type Item = io::Result<DirEntry>;
    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        None
    }
}

pub fn readdir(p: &Path) -> io::Result<ReadDir> {
    Ok(ReadDir { root: 0, data: crate::vec::Vec::new(), idx: 0 })
}

pub fn unlink(p: &Path) -> io::Result<()> {
    Err(io::Error::UNSUPPORTED_PLATFORM)
}

pub fn rename(old: &Path, new: &Path) -> io::Result<()> {
    Err(io::Error::UNSUPPORTED_PLATFORM)
}

pub fn set_perm(p: &Path, perm: FilePermissions) -> io::Result<()> {
    Ok(())
}

pub fn rmdir(p: &Path) -> io::Result<()> {
    Err(io::Error::UNSUPPORTED_PLATFORM)
}

pub fn remove_dir_all(p: &Path) -> io::Result<()> {
    Err(io::Error::UNSUPPORTED_PLATFORM)
}

pub fn readlink(p: &Path) -> io::Result<PathBuf> {
    Err(io::Error::UNSUPPORTED_PLATFORM)
}

pub fn symlink(original: &Path, link: &Path) -> io::Result<()> {
    Err(io::Error::UNSUPPORTED_PLATFORM)
}

pub fn link(original: &Path, link: &Path) -> io::Result<()> {
    Err(io::Error::UNSUPPORTED_PLATFORM)
}

pub fn stat(p: &Path) -> io::Result<FileAttr> {
    let f = File::open(p, &OpenOptions::new())?;
    f.file_attr()
}

pub fn lstat(p: &Path) -> io::Result<FileAttr> {
    stat(p)
}

pub fn canonicalize(p: &Path) -> io::Result<PathBuf> {
    Ok(p.to_path_buf())
}

pub fn copy(from: &Path, to: &Path) -> io::Result<u64> {
    let reader = File::open(from, &OpenOptions::new())?; 
    let mut opts = OpenOptions::new();
    opts.write(true);
    opts.create(true);
    opts.truncate(true);
    let writer = File::open(to, &opts)?; 
    
    let mut buf = [0u8; 4096];
    let mut total = 0;
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 { break; }
        let written = writer.write(&buf[..n])?;
        total += written as u64;
    }
    Ok(total)
}
