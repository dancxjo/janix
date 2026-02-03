use super::symbol::IntoSymbolRef;
use super::{ThingId, ThingKind};
use crate::errors::{errno, Errno};
use crate::syscall::syscall6;
use abi::ids::HandleId;
use abi::symbols::SymbolId;
use abi::syscall::*;

pub fn get_kind(id: ThingId) -> Result<ThingKind, Errno> {
    let ret = unsafe { syscall6(SYS_ROOT_GET_KIND, id.to_u64_lossy() as usize, 0, 0, 0, 0, 0) };
    errno(ret).map(|v| ThingKind(v as u64))
}

pub fn bytespace_create(len: usize, flags: u64, format: u64) -> Result<ThingId, Errno> {
    let ret = unsafe {
        syscall6(
            SYS_ROOT_BYTESPACE_CREATE,
            len,
            flags as usize,
            format as usize,
            0,
            0,
            0,
        )
    };
    errno(ret).map(|v| ThingId::from_u64(v as u64))
}

pub fn bytespace_read(id: ThingId, offset: usize, out: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        syscall6(
            SYS_ROOT_BYTESPACE_READ,
            id.to_u64_lossy() as usize,
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
    crate::println!(
        "STEM: watch_subscribe target={} mask={}",
        target.to_u64_lossy(),
        mask
    );
    let ret = unsafe {
        syscall6(
            SYS_ROOT_WATCH_SUBSCRIBE,
            target.to_u64_lossy() as usize,
            mask as usize,
            0,
            0,
            0,
            0,
        )
    };
    errno(ret).map(|v| ThingId::from_u64(v as u64))
}

pub fn stream_poll(stream: ThingId, out: &mut abi::types::RootWatchEvent) -> Result<usize, Errno> {
    let ret = unsafe {
        syscall6(
            SYS_ROOT_STREAM_POLL,
            stream.to_u64_lossy() as usize,
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
    let wire = key.to_wire();
    let ret = unsafe {
        syscall6(
            SYS_ROOT_PROP_SET,
            id.to_u64_lossy() as usize,
            &wire as *const _ as usize,
            value as usize,
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
    // Disabling check for v0.1.
    Err(KindMismatch)
}

#[derive(Debug)]
pub struct KindMismatch;

pub fn describe_thing(id: ThingId, out: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        syscall6(
            SYS_ROOT_DESCRIBE_THING,
            id.to_u64_lossy() as usize,
            out.as_mut_ptr() as usize,
            out.len(),
            0,
            0,
            0,
        )
    };
    errno(ret).map(|v| v as usize)
}

pub fn describe_symbol(id: SymbolId, out: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        syscall6(
            SYS_ROOT_DESCRIBE_SYMBOL,
            id as usize,
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
            src.to_u64_lossy() as usize,
            &wire as *const _ as usize,
            dst.to_u64_lossy() as usize,
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
            id.to_u64_lossy() as usize,
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
            id.to_u64_lossy() as usize,
            out.as_mut_ptr() as usize,
            out.len() * core::mem::size_of::<abi::types::Edge>(),
            0,
            0,
            0,
        )
    };
    errno(ret).map(|v| v as usize)
}

pub fn get_props(id: ThingId, out: &mut [abi::types::GraphProp]) -> Result<usize, Errno> {
    let ret = unsafe {
        syscall6(
            SYS_ROOT_GET_PROPS,
            id.to_u64_lossy() as usize,
            out.as_mut_ptr() as usize,
            out.len() * core::mem::size_of::<abi::types::GraphProp>(),
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
            src.to_u64_lossy() as usize,
            &wire as *const _ as usize,
            dst.to_u64_lossy() as usize,
            0,
            0,
            0,
        )
    };
    errno(ret).map(|_| ())
}

pub fn intern(s: &str) -> Result<SymbolId, Errno> {
    let ret = unsafe { syscall6(SYS_ROOT_INTERN, s.as_ptr() as usize, s.len(), 0, 0, 0, 0) };
    errno(ret).map(|v| v as u32)
}

pub fn prop_get<S: IntoSymbolRef>(id: ThingId, key: S) -> Result<u64, Errno> {
    let wire = key.to_wire();
    let ret = unsafe {
        syscall6(
            SYS_ROOT_PROP_GET,
            id.to_u64_lossy() as usize,
            &wire as *const _ as usize,
            0,
            0,
            0,
            0,
        )
    };
    errno(ret).map(|v| v as u64)
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
    errno(ret).map(|v| core::cmp::min(v as usize, out.len()))
}

pub fn create_node<S: IntoSymbolRef>(kind: S) -> Result<ThingId, Errno> {
    let wire = kind.to_wire();
    let ret = unsafe {
        syscall6(
            SYS_ROOT_CREATE_NODE,
            &wire as *const _ as usize,
            0,
            0,
            0,
            0,
            0,
        )
    };
    errno(ret).map(|v| ThingId::from_u64(v as u64))
}

pub fn dump_graph(limit: u64) -> Result<(u64, u64), Errno> {
    let ret = unsafe { syscall6(SYS_ROOT_DUMP_GRAPH, limit as usize, 0, 0, 0, 0, 0) };
    errno(ret).map(|_| (0, 0))
}

pub fn bytespace_write(id: ThingId, offset: usize, data: &[u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        syscall6(
            SYS_ROOT_BYTESPACE_WRITE,
            id.to_u64_lossy() as usize,
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
            id.to_u64_lossy() as usize,
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
            id.to_u64_lossy() as usize,
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
            id.to_u64_lossy() as usize,
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
            id.to_u64_lossy() as usize,
            0,
            0,
            0,
            0,
            0,
        )
    };
    errno(ret).map(|v| v as u64)
}

/// Bulk property fetch - get multiple properties for a node in one syscall
pub fn props_get_many(id: ThingId, keys: &[u32]) -> Result<abi::types::BulkPropsResponse, Errno> {
    use abi::types::{BulkPropsResponse, BULK_PROPS_MAX_KEYS};

    if keys.is_empty() || keys.len() > BULK_PROPS_MAX_KEYS {
        return Err(Errno::EINVAL);
    }

    let mut response = BulkPropsResponse::default();

    let ret = unsafe {
        syscall6(
            abi::syscall::SYS_ROOT_PROPS_GET_MANY,
            id.to_u64_lossy() as usize,
            keys.as_ptr() as usize,
            keys.len(),
            &mut response as *mut BulkPropsResponse as usize,
            0,
            0,
        )
    };

    errno(ret).map(|_| response)
}
