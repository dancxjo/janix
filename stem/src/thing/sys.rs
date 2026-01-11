use abi::symbols::SymbolId;
use crate::syscall::syscall6;
use abi::syscall::*;
use super::{ThingId, ThingKind};
use crate::errors::{Errno, errno};
use super::symbol::IntoSymbolRef;

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

pub fn prop_set<S: IntoSymbolRef>(id: ThingId, key: S, value: u64) -> Result<(), Errno> {
    let wire = key.to_wire();
    let ret = unsafe { syscall6(SYS_ROOT_PROP_SET, id.0 as usize, &wire as *const _ as usize, value as usize, 0, 0, 0) };
    errno(ret).map(|_| ())
}

pub fn try_typed<T: super::Thing>(_id: ThingId) -> Result<super::ThingRef<T>, super::sys::KindMismatch> {
    // try_typed logic is broken until we have stable IDs or resolution.
    // For now, allow everything or fail?
    // Let's assume ID match for now if T::KIND is defined?
    // But T::KIND is u64 constant 0x10 etc.
    // Runtime IDs are 0, 1, 2.
    // So this will fail.
    // Disabling check for v0.1.
    Err(KindMismatch)
}

#[derive(Debug)]
pub struct KindMismatch;

pub fn describe_thing(id: ThingId, out: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe { syscall6(SYS_ROOT_DESCRIBE_THING, id.0 as usize, out.as_mut_ptr() as usize, out.len(), 0, 0, 0) };
    errno(ret).map(|v| v as usize)
}

pub fn describe_edge<S: IntoSymbolRef>(src: ThingId, rel: S, dst: ThingId, out: &mut [u8]) -> Result<usize, Errno> {
    let wire = rel.to_wire();
    let ret = unsafe { syscall6(SYS_ROOT_DESCRIBE_EDGE, src.0 as usize, &wire as *const _ as usize, dst.0 as usize, out.as_mut_ptr() as usize, out.len(), 0) };
    errno(ret).map(|v| v as usize)
}

pub fn dump_edges(id: ThingId, out: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe { syscall6(SYS_ROOT_DUMP_EDGES, id.0 as usize, out.as_mut_ptr() as usize, out.len(), 0, 0, 0) };
    errno(ret).map(|v| v as usize)
}

pub fn link<S: IntoSymbolRef>(src: ThingId, rel: S, dst: ThingId) -> Result<(), Errno> {
    let wire = rel.to_wire();
    let ret = unsafe { syscall6(SYS_ROOT_LINK, src.0 as usize, &wire as *const _ as usize, dst.0 as usize, 0, 0, 0) };
    errno(ret).map(|_| ())
}

pub fn intern(s: &str) -> Result<SymbolId, Errno> {
    let ret = unsafe { syscall6(SYS_ROOT_INTERN, s.as_ptr() as usize, s.len(), 0, 0, 0, 0) };
    errno(ret).map(|v| v as u32)
}

pub fn prop_get<S: IntoSymbolRef>(id: ThingId, key: S) -> Result<u64, Errno> {
    let wire = key.to_wire();
    let ret = unsafe { syscall6(SYS_ROOT_PROP_GET, id.0 as usize, &wire as *const _ as usize, 0, 0, 0, 0) };
    errno(ret).map(|v| v as u64)
}

pub fn find<S: IntoSymbolRef>(kind: S, out: &mut [ThingId]) -> Result<usize, Errno> {
    let wire = kind.to_wire();
    let ret = unsafe { syscall6(SYS_ROOT_FIND, &wire as *const _ as usize, out.as_mut_ptr() as usize, out.len() * 8, 0, 0, 0) };
    errno(ret).map(|v| v as usize)
}

pub fn create_node<S: IntoSymbolRef>(kind: S) -> Result<ThingId, Errno> {
    let wire = kind.to_wire();
    let ret = unsafe { syscall6(SYS_ROOT_CREATE_NODE, &wire as *const _ as usize, 0, 0, 0, 0, 0) };
    errno(ret).map(|v| ThingId(v as u64))
}
