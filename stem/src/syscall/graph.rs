use abi::syscall::*;
use abi::errors::{Errno, errno};
use super::raw_syscall6;

pub fn intern(name: &str) -> Result<u64, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_ROOT_INTERN,
            name.as_ptr() as usize,
            name.len(),
            0, 0, 0, 0
        )
    };
    if ret < 0 { errno(ret).map(|_| 0) } else { Ok(ret as u64) }
}

pub fn create_node(kind: u64) -> Result<u64, Errno> {
    let ret = unsafe {
        raw_syscall6(SYS_ROOT_CREATE_NODE, kind as usize, 0, 0, 0, 0, 0)
    };
    if ret < 0 { errno(ret).map(|_| 0) } else { Ok(ret as u64) }
}

pub fn link(src: u64, rel: u64, dst: u64) -> Result<(), Errno> {
    let ret = unsafe {
        raw_syscall6(SYS_ROOT_LINK, src as usize, rel as usize, dst as usize, 0, 0, 0)
    };
    errno(ret).map(|_| ())
}

pub fn prop_set(node: u64, key: u64, value: u64) -> Result<(), Errno> {
    let ret = unsafe {
        raw_syscall6(SYS_ROOT_PROP_SET, node as usize, key as usize, value as usize, 0, 0, 0)
    };
    errno(ret).map(|_| ())
}

pub fn prop_get(node: u64, key: u64) -> Result<u64, Errno> {
    let ret = unsafe {
        raw_syscall6(SYS_ROOT_PROP_GET, node as usize, key as usize, 0, 0, 0, 0)
    };
    if ret < 0 { errno(ret).map(|_| 0) } else { Ok(ret as u64) }
}

/// Low-level query execution. 
/// `plan_buf` contains serialized `PreparedStep`s.
/// `out_buf` receives `QueryRow`s.
pub fn query(plan_buf: &[u8], out_buf: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_ROOT_QUERY,
            plan_buf.as_ptr() as usize,
            plan_buf.len(),
            out_buf.as_mut_ptr() as usize,
            out_buf.len(),
            0,
            0
        )
    };
    errno(ret).map(|r| r as usize)
}

pub fn find(kind: u64, out_ids: &mut [u64]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_ROOT_FIND,
            kind as usize,
            out_ids.as_mut_ptr() as usize,
            out_ids.len() * 8, // bytes
            0, 0, 0
        )
    };
    errno(ret).map(|r| r as usize)
}

pub fn get_kind(node: u64) -> Result<u64, Errno> {
    let ret = unsafe {
        raw_syscall6(SYS_ROOT_GET_KIND, node as usize, 0, 0, 0, 0, 0)
    };
    if ret < 0 { errno(ret).map(|_| 0) } else { Ok(ret as u64) }
}
