//! Graph Mutation Syscalls

use abi::ids::{SymbolId, ThingId};
use abi::syscall::err;
use abi::types::RelationshipRef;
use abi::wire::SyscallResult;
use graph::store;
use graph::symbols;
use crate::watch;

pub fn sys_thing_create(kind_low: u64, parent_low: u64) -> SyscallResult {
    let kind = SymbolId(kind_low);
    let parent = ThingId(parent_low as u128);

    let thing_id = store::thing_create(kind);

    let pred_contains = symbols::intern(b"predicate.contains");
    store::relationship_create(pred_contains, parent, thing_id);
    watch::graph_member_added(parent, thing_id);

    SyscallResult::new(0, thing_id.high(), thing_id.low())
}

pub fn sys_relationship_create(pred_low: u64, from_low: u64, to_low: u64) -> SyscallResult {
    let pred = SymbolId(pred_low);
    let from = ThingId(from_low as u128);
    let to = ThingId(to_low as u128);

    let rel_id = store::relationship_create(pred, from, to);
    if pred == symbols::intern(b"predicate.contains") {
        watch::graph_member_added(from, to);
    }
    SyscallResult::new(0, rel_id.high(), rel_id.low())
}

pub fn sys_relationship_delete(rel_low: u64) -> SyscallResult {
    let rel_id = ThingId(rel_low as u128);
    if let Some(rel) = store::relationship_delete(rel_id) {
        if rel.kind == symbols::intern(b"predicate.contains") {
            watch::graph_member_removed(rel.from, rel.to);
        }
        return SyscallResult::new(0, 0, 0);
    }
    SyscallResult::new(err::ENOENT, 0, 0)
}

pub fn sys_thing_get(id_low: u64, out_ptr: u64, out_len: u64) -> SyscallResult {
    let id = ThingId(id_low as u128);

    if let Some(body) = store::get_body(id) {
        let header = store::get_thing_header(id).unwrap();
        let digest = header.integrity_digest;

        if out_ptr == 0 {
            // Return digest in val0, required size in val1
            return SyscallResult::new(0, digest, body.len() as u64);
        }

        let write_len = core::cmp::min(body.len(), out_len as usize);
        unsafe {
            core::ptr::copy_nonoverlapping(body.as_ptr(), out_ptr as *mut u8, write_len);
        }

        // Return digest in val0, actual write length in val1
        SyscallResult::new(0, digest, write_len as u64)
    } else {
        SyscallResult::new(0, 0, 0)
    }
}

pub fn sys_thing_set_body(id_low: u64, buf_ptr: u64, buf_len: u64) -> SyscallResult {
    let id = ThingId(id_low as u128);
    
    if buf_ptr == 0 || buf_len == 0 {
        if let Err(e) = store::thing_set_body(id, &[]) {
            return SyscallResult::new(e as i32, 0, 0);
        }
        return SyscallResult::new(0, 0, 0);
    }

    if buf_len > 10 * 1024 * 1024 { // 10MB limit for sanity
        return SyscallResult::new(err::ENOMEM, 0, 0);
    }

    let buf = unsafe { core::slice::from_raw_parts(buf_ptr as *const u8, buf_len as usize) };

    match store::thing_set_body(id, buf) {
        Ok(()) => {
            watch::thing_updated(id);
            SyscallResult::new(0, 0, 0)
        }
        Err(e) => SyscallResult::new(e as i32, 0, 0),
    }
}

pub fn sys_relationships_from(
    id_low: u64,
    cursor: u64,
    out_ptr: u64,
    out_len: u64,
) -> SyscallResult {
    let id = ThingId(id_low as u128);

    let rel_ids = store::relationships_from(id);
    let total_rels = rel_ids.len() as u64;

    let skip = cursor as usize;
    if skip >= rel_ids.len() {
        return SyscallResult::new(0, 0, total_rels);
    }

    if out_ptr == 0 {
        return SyscallResult::new(err::EINVAL, 0, total_rels);
    }

    // Copy bytes directly to avoid alignment issues with typed slices
    if out_len == 0 {
        return SyscallResult::new(0, 0, total_rels);
    }

    let elem_size = core::mem::size_of::<RelationshipRef>();
    if out_len as usize > (isize::MAX as usize) / elem_size {
        return SyscallResult::new(err::EINVAL, 0, total_rels);
    }

    let mut count = 0u64;
    let dest_base = out_ptr as *mut u8;

    for (i, &rel_id) in rel_ids.iter().skip(skip).enumerate() {
        if i >= out_len as usize {
            break;
        }
        if let Some(rel) = store::get_relationship(rel_id) {
            let ref_data = RelationshipRef {
                id: rel_id,
                kind: rel.kind,
                target: rel.to,
            };
            // Copy bytes directly to avoid alignment issues
            unsafe {
                let src_ptr = &ref_data as *const RelationshipRef as *const u8;
                let dest_ptr = dest_base.add(i * elem_size);
                core::ptr::copy_nonoverlapping(src_ptr, dest_ptr, elem_size);
            }
            count += 1;
        }
    }

    SyscallResult::new(0, count, total_rels)
}

pub fn sys_symbol_resolve(id_low: u64, out_ptr: u64, out_len: u64) -> SyscallResult {
    let sym_id = SymbolId(id_low);
    if let Some(s) = symbols::resolve(sym_id) {
        let bytes = s.as_bytes();
        let len = bytes.len() as u64;

        if out_ptr != 0 && out_len > 0 {
            let write_len = core::cmp::min(len, out_len);
            unsafe {
                core::ptr::copy_nonoverlapping(
                    bytes.as_ptr(),
                    out_ptr as *mut u8,
                    write_len as usize,
                );
            }
            SyscallResult::new(0, write_len, len)
        } else {
            SyscallResult::new(0, 0, len)
        }
    } else {
        SyscallResult::new(err::ENOENT, 0, 0)
    }
}

pub fn sys_thing_find(name_ptr: u64, name_len: u64) -> SyscallResult {
    if name_ptr == 0 || name_len == 0 || name_len > 1024 {
        return SyscallResult::new(err::EINVAL, 0, 0);
    }

    let name_slice =
        unsafe { core::slice::from_raw_parts(name_ptr as *const u8, name_len as usize) };

    let sym_id = symbols::intern(name_slice);

    if let Some(thing_id) = store::find_thing_by_name(sym_id) {
        SyscallResult::new(0, thing_id.high(), thing_id.low())
    } else {
        SyscallResult::new(err::ENOENT, 0, 0)
    }
}

pub fn sys_thing_register_name(id_low: u64, name_ptr: u64, name_len: u64) -> SyscallResult {
    if name_ptr == 0 || name_len == 0 || name_len > 1024 {
        return SyscallResult::new(err::EINVAL, 0, 0);
    }

    let thing_id = ThingId(id_low as u128);

    let name_slice =
        unsafe { core::slice::from_raw_parts(name_ptr as *const u8, name_len as usize) };

    let sym_id = symbols::intern(name_slice);
    store::thing_register_name(thing_id, sym_id);

    SyscallResult::new(0, 0, 0)
}

pub fn sys_symbol_intern(name_ptr: u64, name_len: u64) -> SyscallResult {
    if name_ptr == 0 || name_len == 0 || name_len > 1024 {
        return SyscallResult::new(err::EINVAL, 0, 0);
    }

    let name_slice =
        unsafe { core::slice::from_raw_parts(name_ptr as *const u8, name_len as usize) };

    let sym_id = symbols::intern(name_slice);
    SyscallResult::new(0, sym_id.0, 0)
}
