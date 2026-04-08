use super::{HandleId, ThingId, ThingKind};
use crate::errors::Errno;
use super::symbol::IntoSymbolRef;
use abi::symbols::SymbolId;
use crate::syscall::arch::raw_syscall6;
use abi::syscall::*;

pub fn get_kind(id: ThingId) -> Result<ThingKind, Errno> {
    let ret = unsafe { raw_syscall6(SYS_ROOT_GET_KIND, id.to_u64_lossy() as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|v| unsafe { core::mem::transmute(v as u64) })
}

pub fn prop_set<S: IntoSymbolRef>(id: ThingId, key: S, value: u64) -> Result<(), Errno> {
    let key_wire = key.to_wire();
    let ret = unsafe {
        raw_syscall6(
            SYS_ROOT_PROP_SET,
            id.to_u64_lossy() as usize,
            &key_wire as *const _ as usize,
            value as usize,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|_| ())
}

pub fn prop_get<S: IntoSymbolRef>(id: ThingId, key: S) -> Result<u64, Errno> {
    let key_wire = key.to_wire();
    let ret = unsafe {
        raw_syscall6(
            SYS_ROOT_PROP_GET,
            id.to_u64_lossy() as usize,
            &key_wire as *const _ as usize,
            0,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|v| v as u64)
}

pub fn describe_thing(id: ThingId, out: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_ROOT_DESCRIBE,
            id.to_u64_lossy() as usize,
            out.as_mut_ptr() as usize,
            out.len(),
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

pub fn link<S: IntoSymbolRef>(src: ThingId, rel: S, dst: ThingId) -> Result<(), Errno> {
    let rel_wire = rel.to_wire();
    let ret = unsafe {
        raw_syscall6(
            SYS_ROOT_LINK,
            src.to_u64_lossy() as usize,
            &rel_wire as *const _ as usize,
            dst.to_u64_lossy() as usize,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|_| ())
}

pub fn intern(s: &str) -> Result<SymbolId, Errno> {
    let ret = unsafe { raw_syscall6(SYS_ROOT_INTERN, s.as_ptr() as usize, s.len(), 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|v| v as u32)
}

pub fn create_node<S: IntoSymbolRef>(kind: S) -> Result<ThingId, Errno> {
    let kind_wire = kind.to_wire();
    let ret = unsafe { raw_syscall6(SYS_ROOT_CREATE_NODE, &kind_wire as *const _ as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|v| ThingId::from_u64(v as u64))
}

#[derive(Debug)]
pub struct KindMismatch(pub usize);

pub fn try_typed<T: super::Thing>(
    id: ThingId,
) -> Result<super::ThingRef<T>, KindMismatch> {
    let kind = get_kind(id).map_err(|_| KindMismatch(0))?;
    if kind == T::KIND {
        Ok(unsafe { super::ThingRef::new(id) })
    } else {
        Err(KindMismatch(0))
    }
}

pub fn find(kind: &str, out: &mut [ThingId]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_ROOT_FIND,
            kind.as_ptr() as usize,
            kind.len(),
            out.as_mut_ptr() as usize,
            out.len(),
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

pub use crate::syscall::vfs::{
    vfs_close as close, vfs_open as open, vfs_read as read, vfs_seek as seek, vfs_stat as stat,
    vfs_write as write,
};

pub fn prop_get_fd<S: IntoSymbolRef>(fd: u32, key: S) -> Result<u64, Errno> {
    prop_get(ThingId::from_u64(fd as u64), key)
}

pub fn watch_subscribe(id: u32, flags: u32) -> Result<u32, Errno> {
    let ret = unsafe { raw_syscall6(SYS_ROOT_WATCH_SUBSCRIBE, id as usize, flags as usize, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|v| v as u32)
}

pub fn props_get_many(id: ThingId, keys: &[u32]) -> Result<abi::types::BulkPropsResponse, Errno> {
    let mut resp = abi::types::BulkPropsResponse::default();
    let mut req = abi::types::BulkPropsRequest::default();
    req.node_id = id.to_u64_lossy();
    req.key_count = keys.len().min(abi::types::BULK_PROPS_MAX_KEYS) as u8;
    for i in 0..req.key_count as usize {
        req.keys[i] = keys[i];
    }

    let ret = unsafe {
        raw_syscall6(
            SYS_ROOT_PROPS_GET_MANY,
            &req as *const _ as usize,
            &mut resp as *mut _ as usize,
            0,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|_| resp)
}

pub fn describe_symbol(sym: SymbolId, out: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_ROOT_DESCRIBE_SYMBOL,
            sym as usize,
            out.as_mut_ptr() as usize,
            out.len(),
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

pub fn root_stream_poll(handle: u32, out: &mut abi::types::RootWatchEvent) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_ROOT_STREAM_POLL,
            handle as usize,
            out as *mut _ as usize,
            0,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

pub use crate::syscall::{memfd_create, memfd_phys, vm_map, stream::stream_poll};
