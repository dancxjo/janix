use super::symbol::IntoSymbolRef;
use super::{ThingId, ThingKind};
use crate::errors::{errno, Errno};
use crate::syscall::syscall6;
use abi::symbols::SymbolId;
use abi::syscall::*;
use abi::ids::HandleId;

pub fn get_kind(id: ThingId) -> Result<ThingKind, Errno> {
    let mut out = SymbolId::default();
    let ret = unsafe {
        syscall6(SYS_ROOT_GET_KIND,
            &id as *const _ as usize,
            &mut out as *mut _ as usize,
            0, 0, 0, 0)
    };
    // ThingKind is u64 wrapper.
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&out.0[0..8]);
    let val = u64::from_le_bytes(bytes);
    errno(ret).map(|_| ThingKind(val))
}

pub fn bytespace_create(len: usize, flags: u64, format: u64) -> Result<ThingId, Errno> {
    let mut out = ThingId::default();
    let ret = unsafe {
        syscall6(
            SYS_ROOT_BYTESPACE_CREATE,
            len,
            flags as usize,
            format as usize,
            &mut out as *mut _ as usize,
            0,
            0,
        )
    };
    errno(ret).map(|_| out)
}

pub fn bytespace_read(id: ThingId, offset: usize, out: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        syscall6(
            SYS_ROOT_BYTESPACE_READ,
            &id as *const _ as usize,
            offset,
            out.as_mut_ptr() as usize,
            out.len(),
            0,
            0,
        )
    };
    errno(ret).map(|v| v as usize)
}

pub fn watch_subscribe(target: ThingId, mask: u64) -> Result<ThingId, Errno> {
    let mut out = ThingId::default();
    let ret = unsafe {
        syscall6(
            SYS_ROOT_WATCH_SUBSCRIBE,
            &target as *const _ as usize,
            mask as usize,
            &mut out as *mut _ as usize,
            0,
            0,
            0,
        )
    };
    errno(ret).map(|_| out)
}

pub fn stream_poll(stream: ThingId, out: &mut abi::types::RootWatchEvent) -> Result<usize, Errno> {
    let ret = unsafe {
        syscall6(
            SYS_ROOT_STREAM_POLL,
            &stream as *const _ as usize,
            core::mem::size_of_val(out),
            out as *mut _ as usize,
            0,
            0,
            0,
        )
    };
    errno(ret).map(|v| v as usize)
}

pub fn prop_set<S: IntoSymbolRef>(id: ThingId, key: S, value: u64) -> Result<(), Errno> {
    let mut val_bytes = [0u8; 16];
    val_bytes[0..8].copy_from_slice(&value.to_le_bytes());
    prop_set_raw(id, key, &val_bytes)
}

pub fn prop_set_raw<S: IntoSymbolRef>(id: ThingId, key: S, value: &[u8; 16]) -> Result<(), Errno> {
    let wire = key.to_wire();
    let ret = unsafe {
        syscall6(
            SYS_ROOT_PROP_SET,
            &id as *const _ as usize,
            &wire as *const _ as usize,
            value as *const _ as usize,
            0,
            0,
            0,
        )
    };
    errno(ret).map(|_| ())
}

pub fn try_typed<T: super::Thing>(
    _id: ThingId,
) -> Result<super::ThingRef<T>, super::sys::KindMismatch> {
    Err(KindMismatch)
}

#[derive(Debug)]
pub struct KindMismatch;

pub fn describe_thing(id: ThingId, out: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        syscall6(
            SYS_ROOT_DESCRIBE_THING,
            &id as *const _ as usize,
            out.as_mut_ptr() as usize,
            out.len(),
            0,
            0,
            0,
        )
    };
    errno(ret).map(|v| v as usize)
}

pub fn describe_edge<S: IntoSymbolRef>(
    src: ThingId,
    rel: S,
    dst: ThingId,
    out: &mut [u8],
) -> Result<usize, Errno> {
    let wire = rel.to_wire();
    let ret = unsafe {
        syscall6(
            SYS_ROOT_DESCRIBE_EDGE,
            &src as *const _ as usize,
            &wire as *const _ as usize,
            &dst as *const _ as usize,
            out.as_mut_ptr() as usize,
            out.len(),
            0,
        )
    };
    errno(ret).map(|v| v as usize)
}

pub fn dump_edges(id: ThingId, out: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        syscall6(
            SYS_ROOT_DUMP_EDGES,
            &id as *const _ as usize,
            out.as_mut_ptr() as usize,
            out.len(),
            0,
            0,
            0,
        )
    };
    errno(ret).map(|v| v as usize)
}

pub fn get_edges(id: ThingId, out: &mut [abi::types::Edge]) -> Result<usize, Errno> {
    let ret = unsafe {
        syscall6(
            SYS_ROOT_GET_EDGES,
            &id as *const _ as usize,
            out.as_mut_ptr() as usize,
            out.len() * core::mem::size_of::<abi::types::Edge>(),
            0,
            0,
            0,
        )
    };
    errno(ret).map(|v| v as usize)
}

pub fn link<S: IntoSymbolRef>(src: ThingId, rel: S, dst: ThingId) -> Result<(), Errno> {
    let wire = rel.to_wire();
    let ret = unsafe {
        syscall6(
            SYS_ROOT_LINK,
            &src as *const _ as usize,
            &wire as *const _ as usize,
            &dst as *const _ as usize,
            0,
            0,
            0,
        )
    };
    errno(ret).map(|_| ())
}

pub fn intern(s: &str) -> Result<SymbolId, Errno> {
    let mut out = SymbolId::default();
    let ret = unsafe { syscall6(SYS_ROOT_INTERN, s.as_ptr() as usize, s.len(), &mut out as *mut _ as usize, 0, 0, 0) };
    errno(ret).map(|_| out)
}

pub fn prop_get<S: IntoSymbolRef>(id: ThingId, key: S) -> Result<u64, Errno> {
    prop_get_raw(id, key).map(|val| {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&val[0..8]);
        u64::from_le_bytes(bytes)
    })
}

pub fn prop_get_raw<S: IntoSymbolRef>(id: ThingId, key: S) -> Result<[u8; 16], Errno> {
    let wire = key.to_wire();
    let mut out_val = [0u8; 16];
    let ret = unsafe {
        syscall6(
            SYS_ROOT_PROP_GET,
            &id as *const _ as usize,
            &wire as *const _ as usize,
            &mut out_val as *mut _ as usize,
            0,
            0,
            0,
        )
    };
    errno(ret).map(|_| out_val)
}

pub fn find<S: IntoSymbolRef>(kind: S, out: &mut [ThingId]) -> Result<usize, Errno> {
    let wire = kind.to_wire();
    let ret = unsafe {
        syscall6(
            SYS_ROOT_FIND,
            &wire as *const _ as usize,
            out.as_mut_ptr() as usize,
            out.len() * core::mem::size_of::<ThingId>(),
            0,
            0,
            0,
        )
    };
    errno(ret).map(|v| v as usize)
}

pub fn create_node<S: IntoSymbolRef>(kind: S) -> Result<ThingId, Errno> {
    let wire = kind.to_wire();
    let mut out = ThingId::default();
    let ret = unsafe {
        syscall6(
            SYS_ROOT_CREATE_NODE,
            &wire as *const _ as usize,
            &mut out as *mut _ as usize,
            0,
            0,
            0,
            0,
        )
    };
    errno(ret).map(|_| out)
}

pub fn dump_graph(limit: u64) -> Result<(u64, u64), Errno> {
    let ret = unsafe { syscall6(SYS_ROOT_DUMP_GRAPH, limit as usize, 0, 0, 0, 0, 0) };
    errno(ret).map(|_| (0, 0))
}

pub fn bytespace_write(id: ThingId, offset: usize, data: &[u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        syscall6(
            SYS_ROOT_BYTESPACE_WRITE,
            &id as *const _ as usize,
            offset,
            data.as_ptr() as usize,
            data.len(),
            0,
            0,
        )
    };
    errno(ret).map(|v| v as usize)
}

pub fn bytespace_info(id: ThingId) -> Result<usize, Errno> {
    let ret = unsafe {
        syscall6(
            SYS_ROOT_BYTESPACE_INFO,
            &id as *const _ as usize,
            0,
            0,
            0,
            0,
            0,
        )
    };
    errno(ret).map(|v| v as usize)
}

pub fn bytespace_map(id: ThingId) -> Result<*mut u8, Errno> {
    let ret = unsafe {
        syscall6(
            SYS_ROOT_BYTESPACE_MAP,
            &id as *const _ as usize,
            0,
            0,
            0,
            0,
            0,
        )
    };
    errno(ret).map(|v| v as *mut u8)
}

pub fn bytespace_unmap(id: ThingId, ptr: *mut u8) -> Result<(), Errno> {
    let ret = unsafe {
        syscall6(
            SYS_ROOT_BYTESPACE_UNMAP,
            &id as *const _ as usize,
            ptr as usize,
            0,
            0,
            0,
            0,
        )
    };
    errno(ret).map(|_| ())
}

pub fn bytespace_phys(id: ThingId) -> Result<u64, Errno> {
    let ret = unsafe {
        syscall6(
            SYS_ROOT_BYTESPACE_PHYS,
            &id as *const _ as usize,
            0,
            0,
            0,
            0,
            0,
        )
    };
    errno(ret).map(|v| v as u64)
}
