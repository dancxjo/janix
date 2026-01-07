//! Graph Mutation Syscalls

use abi::ids::{SymbolId, ThingId};
use abi::syscall::err;
use abi::types::RelationshipRef;
use abi::wire::SyscallResult;
use graph::store;
use graph::symbols;
use crate::watch;
use crate::syscall::user_mem;
use abi::cap::CapOp;

pub fn sys_thing_create(kind_low: u64, parent_low: u64) -> SyscallResult {
    let kind = SymbolId(kind_low);
    let parent = ThingId(parent_low as u128);

    if let Err(code) = user_mem::require_current_cap(CapOp::GraphCreate, Some(parent)) {
        return SyscallResult::new(code, 0, 0);
    }

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

    if let Err(code) = user_mem::require_current_cap(CapOp::GraphLink, Some(from)) {
        return SyscallResult::new(code, 0, 0);
    }

    let rel_id = store::relationship_create(pred, from, to);
    if pred == symbols::intern(b"predicate.contains") {
        watch::graph_member_added(from, to);
    }
    SyscallResult::new(0, rel_id.high(), rel_id.low())
}

pub fn sys_relationship_delete(rel_low: u64) -> SyscallResult {
    let rel_id = ThingId(rel_low as u128);
    let Some(rel) = store::get_relationship(rel_id) else {
        return SyscallResult::new(err::ENOENT, 0, 0);
    };

    if let Err(code) = user_mem::require_current_cap(CapOp::GraphUnlink, Some(rel.from)) {
        return SyscallResult::new(code, 0, 0);
    }

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

    if let Err(code) = user_mem::require_current_cap(CapOp::GraphRead, Some(id)) {
        return SyscallResult::new(code, 0, 0);
    }

    if let Some(body) = store::get_body(id) {
        let header = store::get_thing_header(id).unwrap();
        let digest = header.integrity_digest;

        if out_ptr == 0 {
            // Return digest in val0, required size in val1
            return SyscallResult::new(0, digest, body.len() as u64);
        }

        let out_len = match usize::try_from(out_len) {
            Ok(len) => len,
            Err(_) => return SyscallResult::new(err::EINVAL, 0, 0),
        };
        let write_len = core::cmp::min(body.len(), out_len);
        if let Err(code) = user_mem::copy_to_user(out_ptr, &body, write_len) {
            return SyscallResult::new(code, 0, 0);
        }

        // Return digest in val0, actual write length in val1
        SyscallResult::new(0, digest, write_len as u64)
    } else {
        SyscallResult::new(0, 0, 0)
    }
}

pub fn sys_thing_set_body(id_low: u64, buf_ptr: u64, buf_len: u64) -> SyscallResult {
    let id = ThingId(id_low as u128);

    if let Err(code) = user_mem::require_current_cap(CapOp::GraphWrite, Some(id)) {
        return SyscallResult::new(code, 0, 0);
    }
    
    let buf_len = match usize::try_from(buf_len) {
        Ok(len) => len,
        Err(_) => return SyscallResult::new(err::EINVAL, 0, 0),
    };

    if buf_ptr == 0 || buf_len == 0 {
        if let Err(e) = store::thing_set_body(id, &[]) {
            return SyscallResult::new(e as i32, 0, 0);
        }
        return SyscallResult::new(0, 0, 0);
    }

    if buf_len > 10 * 1024 * 1024 { // 10MB limit for sanity
        return SyscallResult::new(err::ENOMEM, 0, 0);
    }

    let mut buf = alloc::vec![0u8; buf_len];
    if let Err(code) = user_mem::copy_from_user(&mut buf, buf_ptr, buf_len) {
        return SyscallResult::new(code, 0, 0);
    }

    match store::thing_set_body(id, &buf) {
        Ok(()) => {
            watch::thing_updated(id);
            SyscallResult::new(0, 0, 0)
        }
        Err(e) => SyscallResult::new(e as i32, 0, 0),
    }
}

pub fn sys_relationships_by_kind(
    id_low: u64,
    kind_low: u64,
    out_ptr: u64,
    out_len: u64,
) -> SyscallResult {
    let id = ThingId(id_low as u128);
    let kind = SymbolId(kind_low);

    if let Err(code) = user_mem::require_current_cap(CapOp::GraphRead, Some(id)) {
        return SyscallResult::new(code, 0, 0);
    }

    let targets = store::relationships_by_kind(id, kind);
    let total = targets.len() as u64;

    if out_ptr == 0 {
        return SyscallResult::new(0, 0, total);
    }

    if out_len == 0 {
        return SyscallResult::new(0, 0, total);
    }

    let elem_size = core::mem::size_of::<ThingId>();

    // out_len is in bytes. Calculate capacity in elements.
    let max_elems = (out_len as usize) / elem_size;

    // Determine how many items we can write (min of capacity and available)
    let write_count = core::cmp::min(max_elems, targets.len());
    let write_bytes = write_count * elem_size;

    if write_bytes > 0 {
        let bytes = unsafe {
            core::slice::from_raw_parts(targets.as_ptr() as *const u8, write_bytes)
        };
        if let Err(code) = user_mem::copy_to_user(out_ptr, bytes, write_bytes) {
            return SyscallResult::new(code, 0, total);
        }
    }

    SyscallResult::new(0, write_count as u64, total)
}

pub fn sys_relationships_from(
    id_low: u64,
    cursor: u64,
    out_ptr: u64,
    out_len: u64,
) -> SyscallResult {
    let id = ThingId(id_low as u128);

    if let Err(code) = user_mem::require_current_cap(CapOp::GraphRead, Some(id)) {
        return SyscallResult::new(code, 0, 0);
    }

    if out_ptr == 0 {
        // If out_ptr is 0, we can't return data, but we might want total count.
        // But the previous implementation required out_ptr != 0 for success unless cursor out of bounds?
        // Actually the previous implementation returned EINVAL if out_ptr == 0.
        // Let's preserve that, but fetch total count first to match error return style.
        // Wait, store access is what we want to optimize.
        // Let's call paged with 0 length if we just need total.
        let (_, total) = store::relationships_from_paged(id, 0, 0);
        return SyscallResult::new(err::EINVAL, 0, total as u64);
    }

    let elem_size = core::mem::size_of::<RelationshipRef>();
    let out_len_usize = match usize::try_from(out_len) {
        Ok(len) => len,
        Err(_) => {
            let (_, total) = store::relationships_from_paged(id, 0, 0);
            return SyscallResult::new(err::EINVAL, 0, total as u64);
        }
    };

    // Check for excessive size request to prevent DOS/overflow issues
    if out_len_usize > (isize::MAX as usize) / elem_size {
         let (_, total) = store::relationships_from_paged(id, 0, 0);
         return SyscallResult::new(err::EINVAL, 0, total as u64);
    }

    let (rels, total_rels) = store::relationships_from_paged(id, cursor as usize, out_len_usize);
    let total_rels = total_rels as u64;

    if out_len == 0 {
        return SyscallResult::new(0, 0, total_rels);
    }

    let mut count = 0u64;
    for (i, rel) in rels.into_iter().enumerate() {
        let ref_data = RelationshipRef {
            id: rel.id,
            kind: rel.kind,
            target: rel.to,
        };

        // Write item-by-item to avoid allocating a large scratch buffer.
        // Using copy_to_user with slice::from_ref
        // Calculate destination address
        let dest_addr = match out_ptr.checked_add((i * elem_size) as u64) {
            Some(addr) => addr,
            None => break, // Should not happen given bounds checks above
        };

        let src_slice = unsafe {
            core::slice::from_raw_parts(&ref_data as *const _ as *const u8, elem_size)
        };

        if let Err(code) = user_mem::copy_to_user(dest_addr, src_slice, elem_size) {
            // If copy fails, we return the error code.
            // Note: Partial writes are possible if we return error here.
            // The original code failed entirely before writing anything if scratch allocation failed,
            // or failed entirely if copy_to_user (bulk) failed.
            return SyscallResult::new(code, count, total_rels);
        }
        count += 1;
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
            if let Err(code) = user_mem::copy_to_user(out_ptr, bytes, write_len as usize) {
                return SyscallResult::new(code, 0, 0);
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
    if let Err(code) = user_mem::require_current_cap(CapOp::GraphRead, None) {
        return SyscallResult::new(code, 0, 0);
    }

    if name_ptr == 0 || name_len == 0 || name_len > 1024 {
        return SyscallResult::new(err::EINVAL, 0, 0);
    }

    let name_len = match usize::try_from(name_len) {
        Ok(len) => len,
        Err(_) => return SyscallResult::new(err::EINVAL, 0, 0),
    };
    let mut name_buf = alloc::vec![0u8; name_len];
    if let Err(code) = user_mem::copy_from_user(&mut name_buf, name_ptr, name_len) {
        return SyscallResult::new(code, 0, 0);
    }
    let name_slice = name_buf.as_slice();

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

    if let Err(code) = user_mem::require_current_cap(CapOp::GraphWrite, Some(thing_id)) {
        return SyscallResult::new(code, 0, 0);
    }

    let name_len = match usize::try_from(name_len) {
        Ok(len) => len,
        Err(_) => return SyscallResult::new(err::EINVAL, 0, 0),
    };
    let mut name_buf = alloc::vec![0u8; name_len];
    if let Err(code) = user_mem::copy_from_user(&mut name_buf, name_ptr, name_len) {
        return SyscallResult::new(code, 0, 0);
    }
    let name_slice = name_buf.as_slice();

    let sym_id = symbols::intern(name_slice);
    store::thing_register_name(thing_id, sym_id);

    SyscallResult::new(0, 0, 0)
}

pub fn sys_symbol_intern(name_ptr: u64, name_len: u64) -> SyscallResult {
    if name_ptr == 0 || name_len == 0 || name_len > 1024 {
        return SyscallResult::new(err::EINVAL, 0, 0);
    }

    let name_len = match usize::try_from(name_len) {
        Ok(len) => len,
        Err(_) => return SyscallResult::new(err::EINVAL, 0, 0),
    };
    let mut name_buf = alloc::vec![0u8; name_len];
    if let Err(code) = user_mem::copy_from_user(&mut name_buf, name_ptr, name_len) {
        return SyscallResult::new(code, 0, 0);
    }
    let name_slice = name_buf.as_slice();

    let sym_id = symbols::intern(name_slice);
    SyscallResult::new(0, sym_id.0, 0)
}
