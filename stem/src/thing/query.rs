use super::symbol::IntoSymbolRef;
use crate::errors::{errno, Errno};
use crate::syscall::syscall6;
use crate::thing::ThingId;
use abi::query::*;
use abi::syscall::SYS_ROOT_QUERY;
use alloc::vec::Vec;
use abi::symbols::SymbolId;

pub fn query_nodes_by_kind(kind: &str, limit: usize, out: &mut [ThingId]) -> Result<usize, Errno> {
    let wire = kind.to_wire();
    let step = QueryStep {
        op: QueryOpKind::Scan as u64,
        arg1: limit as u64,
        arg2: 0,
        symbol: wire,
    };

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
            0,
        )
    };

    let count = errno(ret).map(|v| v as usize)?;

    for i in 0..core::cmp::min(count, out.len()) {
        out[i] = rows[i].id;
    }

    Ok(count)
}

pub fn query_edges(
    src: ThingId,
    rel: Option<&str>,
    limit: usize,
) -> Result<Vec<(SymbolId, ThingId)>, Errno> {
    let id_wire = abi::symbols::SymbolRefWire {
        tag: abi::symbols::SYMBOL_REF_TAG_ID,
        ptr_or_id: &src as *const _ as u64,
        len: 0,
    };

    let step1 = QueryStep {
        op: QueryOpKind::Start as u64,
        arg1: 0,
        arg2: 0,
        symbol: id_wire,
    };

    let wire_rel = if let Some(r) = rel {
        r.to_wire()
    } else {
        "".to_wire()
    };

    let step2 = QueryStep {
        op: QueryOpKind::Expand as u64,
        arg1: 0,
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
            0,
        )
    };

    let count = errno(ret).map(|v| v as usize)?;

    let mut res = Vec::with_capacity(count);
    for i in 0..count {
        res.push((rows[i].kind_rel, rows[i].val_dst));
    }

    Ok(res)
}
