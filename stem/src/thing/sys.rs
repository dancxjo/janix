use crate::syscall::syscall6;
use abi::syscall::*;
use super::{ThingId, ThingKind};
use crate::errors::{Errno, errno};

// We don't have Result<T> in stem prelude? stem::lib.rs is no_std.
// We should use core::result::Result.

pub fn get_kind(id: ThingId) -> Result<ThingKind, Errno> {
    let ret = unsafe { syscall6(SYS_ROOT_GET_KIND, id.0 as usize, 0, 0, 0, 0, 0) };
    errno(ret).map(|v| ThingKind(v as u64))
}

pub fn bytespace_create(len: usize, flags: u64, format: u64) -> Result<ThingId, Errno> {
    let ret = unsafe { syscall6(SYS_ROOT_BYTESPACE_CREATE, len, flags as usize, format as usize, 0, 0, 0) };
    errno(ret).map(|v| ThingId(v as u64))
}

pub fn watch_subscribe(target: ThingId, mask: u64) -> Result<ThingId, Errno> {
    let ret = unsafe { syscall6(SYS_ROOT_WATCH_SUBSCRIBE, target.0 as usize, mask as usize, 0, 0, 0, 0) };
    errno(ret).map(|v| ThingId(v as u64))
}

pub fn stream_poll(stream: ThingId, out: &mut abi::types::RootWatchEvent) -> Result<usize, Errno> {
    let ret = unsafe { syscall6(SYS_ROOT_STREAM_POLL, stream.0 as usize, core::mem::size_of_val(out), out as *mut _ as usize, 0, 0, 0) };
    errno(ret).map(|v| v as usize)
}

pub fn prop_set(id: ThingId, key: u64, value: u64) -> Result<(), Errno> {
    let ret = unsafe { syscall6(SYS_ROOT_PROP_SET, id.0 as usize, key as usize, value as usize, 0, 0, 0) };
    errno(ret).map(|_| ())
}

#[derive(Debug)]
pub struct KindMismatch;

pub fn try_typed<T: super::Thing>(id: ThingId) -> Result<super::ThingRef<T>, KindMismatch> {
    // If sys::get_kind fails, map to KindMismatch or panic?
    // For now simple map.
    let k = get_kind(id).map_err(|_| KindMismatch)?;
    if k == T::KIND {
        unsafe { Ok(super::ThingRef::new(id)) }
    } else {
        Err(KindMismatch)
    }
}

pub fn describe_thing(id: ThingId, out: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe { syscall6(SYS_ROOT_DESCRIBE_THING, id.0 as usize, out.as_mut_ptr() as usize, out.len(), 0, 0, 0) };
    errno(ret).map(|v| v as usize)
}

pub fn describe_edge(src: ThingId, rel: u64, dst: ThingId, out: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe { syscall6(SYS_ROOT_DESCRIBE_EDGE, src.0 as usize, rel as usize, dst.0 as usize, out.as_mut_ptr() as usize, out.len(), 0) };
    errno(ret).map(|v| v as usize)
}

pub fn dump_edges(id: ThingId, out: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe { syscall6(SYS_ROOT_DUMP_EDGES, id.0 as usize, out.as_mut_ptr() as usize, out.len(), 0, 0, 0) };
    errno(ret).map(|v| v as usize)
}

pub fn link(src: ThingId, rel: u64, dst: ThingId) -> Result<(), Errno> {
    let ret = unsafe { syscall6(SYS_ROOT_LINK, src.0 as usize, rel as usize, dst.0 as usize, 0, 0, 0) };
    errno(ret).map(|_| ())
}
