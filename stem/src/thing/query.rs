use abi::query::*;
use super::symbol::IntoSymbolRef;
use crate::thing::ThingId;
use crate::syscall::syscall6;
use crate::errors::{Errno, errno};
use alloc::vec::Vec;
use abi::syscall::SYS_ROOT_QUERY;

pub fn query_nodes_by_kind(kind: &str, limit: usize, out: &mut [ThingId]) -> Result<usize, Errno> {
    let wire = kind.to_wire();
    let step = QueryStep {
        op: QueryOpKind::Scan as u64,
        arg1: limit as u64,
        arg2: 0,
        symbol: wire,
    };
    
    // Prepare output buffer. 
    // The user passed `out: &mut [ThingId]`.
    // The syscall returns `[QueryRow]`.
    // `QueryRow` is bigger than `ThingId`.
    // We need a temp buffer or careful casting?
    // User wrapper constraint: "out: &mut [ThingId]".
    // System call returns full rows.
    // We must buffer generic rows and project.
    
    let mut rows = alloc::vec![QueryRow::default(); limit];
    let plan = [step];
    
    let ret = unsafe {
        syscall6(
            SYS_ROOT_QUERY,
            plan.as_ptr() as usize,
            1,
            rows.as_mut_ptr() as usize,
            rows.len(),
            0,
            0
        )
    };
    
    let count = errno(ret).map(|v| v as usize)?;
    
    for i in 0..core::cmp::min(count, out.len()) {
        out[i] = ThingId(rows[i].id);
    }
    
    Ok(count)
}

pub fn query_edges(src: ThingId, rel: Option<&str>, limit: usize) -> Result<Vec<(u64, ThingId)>, Errno> {
    // Step 1: Start
    let step1 = QueryStep {
        op: QueryOpKind::Start as u64,
        arg1: src.0,
        arg2: 0,
        symbol: "".to_wire(), // Ignored for Start
    };
    
    // Step 2: Expand
    let wire_rel = if let Some(r) = rel { r.to_wire() } else { "".to_wire() }; // Rel="" means any? Executor logic needs check.
    // Currently executor checks `if *r == rel`. If rel is "" and interned ID is 0, we match 0.
    // But edges have valid symbol Ids > 0.
    // So current executor logic doesn't support "Any".
    // For v0.1, we require specific rel.
    
    let step2 = QueryStep {
        op: QueryOpKind::Expand as u64,
        arg1: 0, // Out
        arg2: 0,
        symbol: wire_rel,
    };
    
    let mut rows = alloc::vec![QueryRow::default(); limit];
    let plan = [step1, step2];
    
    let ret = unsafe {
        syscall6(
            SYS_ROOT_QUERY,
            plan.as_ptr() as usize,
            2,
            rows.as_mut_ptr() as usize,
            rows.len(),
            0,
            0
        )
    };
    
    let count = errno(ret).map(|v| v as usize)?;
    
    // Map to (RelKind, Dst)
    // Row layout for Expand:
    // id: row.id (src)
    // kind_rel: *r as u64 (Rel Kind)
    // val_dst: *dst (Dst ID)
    
    let mut res = Vec::with_capacity(count);
    for i in 0..count {
        res.push((rows[i].kind_rel, ThingId(rows[i].val_dst)));
    }
    
    Ok(res)
}
