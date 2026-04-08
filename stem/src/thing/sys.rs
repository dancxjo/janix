use super::symbol::IntoSymbolRef;
use super::{HandleId, ThingId, ThingKind};
use crate::errors::Errno;
use crate::syscall::arch::raw_syscall6;
use abi::symbols::SymbolId;
use abi::syscall::*;

pub fn get_kind(_id: ThingId) -> Result<ThingKind, Errno> {
    // Legacy graph kinds are gone.
    Ok(ThingKind(0))
}

pub fn prop_get<S: IntoSymbolRef>(_id: ThingId, _key: S) -> Result<u64, Errno> {
    Err(Errno::ENOSYS)
}

pub fn prop_set<S: IntoSymbolRef>(_id: ThingId, _key: S, _value: u64) -> Result<(), Errno> {
    Err(Errno::ENOSYS)
}

pub fn describe_thing(_id: ThingId, _out: &mut [u8]) -> Result<usize, Errno> {
    Err(Errno::ENOSYS)
}

pub fn link<S: IntoSymbolRef>(_src: ThingId, _rel: S, _dst: ThingId) -> Result<(), Errno> {
    Err(Errno::ENOSYS)
}

pub fn intern(_s: &str) -> Result<SymbolId, Errno> {
    Err(Errno::ENOSYS)
}

pub fn create_node<S: IntoSymbolRef>(_kind: S) -> Result<ThingId, Errno> {
    Err(Errno::ENOSYS)
}

#[derive(Debug)]
pub struct KindMismatch(pub usize);

pub fn try_typed<T: super::Thing>(id: ThingId) -> Result<super::ThingRef<T>, KindMismatch> {
    let kind = get_kind(id).map_err(|_| KindMismatch(0))?;
    if kind == T::KIND {
        Ok(unsafe { super::ThingRef::new(id) })
    } else {
        Err(KindMismatch(0))
    }
}

pub fn find(_kind: &str, _out: &mut [ThingId]) -> Result<usize, Errno> {
    Ok(0)
}

pub use crate::syscall::vfs::{
    vfs_close as close, vfs_open as open, vfs_read as read, vfs_seek as seek, vfs_stat as stat,
    vfs_write as write,
};

pub fn prop_get_fd<S: IntoSymbolRef>(fd: u32, key: S) -> Result<u64, Errno> {
    prop_get(ThingId::from_u64(fd as u64), key)
}

pub fn watch_subscribe(_id: u32, _flags: u32) -> Result<u32, Errno> {
    Err(Errno::ENOSYS)
}

pub fn props_get_many(_id: ThingId, _keys: &[u32]) -> Result<abi::types::BulkPropsResponse, Errno> {
    Err(Errno::ENOSYS)
}

pub fn describe_symbol(_sym: SymbolId, _out: &mut [u8]) -> Result<usize, Errno> {
    Err(Errno::ENOSYS)
}

pub fn root_stream_poll(
    _handle: u32,
    _out: &mut abi::types::RootWatchEvent,
) -> Result<usize, Errno> {
    Err(Errno::ENOSYS)
}

pub use crate::syscall::{memfd_create, memfd_phys, vm_map};
