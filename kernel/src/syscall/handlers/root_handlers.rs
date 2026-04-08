//! Root/Graph syscalls (DEPRECATED)
//!
//! This module previously handled the legacy "System Graph" Root service.
//! It is now decommissioned and all syscalls return ENOSYS.

use abi::errors::{Errno, SysResult};

pub fn sys_root_get_kind(_id: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_intern(_ptr: usize, _len: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_create_node(_kind_ptr: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_link(_src: usize, _rel_ptr: usize, _dst: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_prop_get(_id: usize, _ptr: usize, _reserved: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_prop_set(_id: usize, _key_ptr: usize, _value: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_find(_ptr_kind: usize, _ptr_buf: usize, _len: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_query(
    _plan_ptr: usize,
    _plan_len: usize,
    _out_ptr: usize,
    _out_cap: usize,
) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_describe_thing(_id: usize, _out_ptr: usize, _len: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_describe_symbol(_id: usize, _out_ptr: usize, _len: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_describe_edge(
    _src: usize,
    _rel_ptr: usize,
    _dst: usize,
    _out_ptr: usize,
    _len: usize,
) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_dump_edges(_id: usize, _out_ptr: usize, _len: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_get_edges(_id: usize, _out_ptr: usize, _len: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_get_props(_id: usize, _out_ptr: usize, _len: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_dump_graph(_limit: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_watch_subscribe(_target: usize, _mask: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_bytespace_map(_id: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_bytespace_unmap(_id: usize, _user_va: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_async_prop_set(_id: usize, _key_ptr: usize, _value: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_async_link(_src: usize, _rel_ptr: usize, _dst: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_async_create_node(_kind_ptr: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_async_wait(_handle: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_async_drop(_handle: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_async_status(_handle: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_bytespace_phys(_id: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}
pub fn sys_root_watch_open(_spec_ptr: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_watch_next(
    _id: usize,
    _out_seq_ptr: usize,
    _out_ptr: usize,
    _out_len: usize,
) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_watch_try_next(
    _id: usize,
    _out_seq_ptr: usize,
    _out_ptr: usize,
    _out_len: usize,
) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_apply_batch(_ptr: usize, _len: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_watch_close(_id: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_props_get_many(
    _node_id: usize,
    _keys_ptr: usize,
    _keys_len: usize,
    _out_ptr: usize,
) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_resolve_path(_ptr: usize, _len: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_bytespace_truncate(_id: usize, _new_len: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_unlink(_src: usize, _rel: usize, _dst: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_dir_list(_dir_id: usize, _out_ptr: usize, _out_len: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_orphan_thing(_thing_id: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}
