//! Root/Graph syscalls

use crate::root::{self as root_svc, RootOp};
use crate::syscall::validate::validate_user_range;
use super::{copyin, copyout, read_symbol, root_call};
use abi::errors::{Errno, SysResult};
use abi::wire::{ThingId, SymbolId};
use alloc::string::String;
use core::sync::atomic::Ordering;

fn read_thing(ptr: usize) -> SysResult<ThingId> {
    validate_user_range(ptr, 16, false)?;
    let mut bytes = [0u8; 16];
    unsafe {
        copyin(&mut bytes, ptr)?;
    }
    Ok(ThingId(bytes))
}

fn read_value(ptr: usize) -> SysResult<[u8; 16]> {
    validate_user_range(ptr, 16, false)?;
    let mut bytes = [0u8; 16];
    unsafe {
        copyin(&mut bytes, ptr)?;
    }
    Ok(bytes)
}

pub fn sys_root_get_kind(id_ptr: usize, out_ptr: usize) -> SysResult<usize> {
    let id = read_thing(id_ptr)?;
    validate_user_range(out_ptr, 16, true)?;
    root_call(RootOp::GetKind { id, out_ptr: out_ptr as u64 })
}

pub fn sys_root_intern(ptr: usize, len: usize, out_ptr: usize) -> SysResult<usize> {
    if len > 256 {
        return Err(Errno::EINVAL);
    }
    validate_user_range(ptr, len, false)?;
    if out_ptr != 0 {
        validate_user_range(out_ptr, 16, true)?;
    }

    let mut buf = [0u8; 256];
    unsafe {
        copyin(&mut buf[..len], ptr)?;
    }

    let s = core::str::from_utf8(&buf[..len]).map_err(|_| Errno::EINVAL)?;
    let msg = RootOp::Intern {
        name: String::from(s),
        out_ptr: out_ptr as u64,
    };

    root_call(msg)
}

pub fn sys_root_create_node(kind_ptr: usize, out_ptr: usize) -> SysResult<usize> {
    let sym = read_symbol(kind_ptr)?;
    if out_ptr != 0 {
        validate_user_range(out_ptr, 16, true)?;
    }
    root_call(RootOp::CreateNode { kind: sym, out_ptr: out_ptr as u64 })
}

pub fn sys_root_link(src_ptr: usize, rel_ptr: usize, dst_ptr: usize) -> SysResult<usize> {
    let src = read_thing(src_ptr)?;
    let rel = read_symbol(rel_ptr)?;
    let dst = read_thing(dst_ptr)?;

    let reply = root_svc::enqueue(RootOp::Link {
        src,
        rel,
        dst,
    });
    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            let status = reply.status.load(Ordering::Relaxed);
            if status == 0 {
                return Ok(0);
            } else {
                return Err(Errno::EIO);
            }
        }
        unsafe {
            crate::task::scheduler::yield_now_current();
        }
    }
}

pub fn sys_root_prop_get(id_ptr: usize, key_ptr: usize, out_ptr: usize) -> SysResult<usize> {
    let id = read_thing(id_ptr)?;
    let sym = read_symbol(key_ptr)?;
    validate_user_range(out_ptr, 16, true)?;
    let msg = RootOp::PropGet {
        id,
        key: sym,
        out_ptr: out_ptr as u64,
    };
    root_call(msg)
}

pub fn sys_root_prop_set(id_ptr: usize, key_ptr: usize, value_ptr: usize) -> SysResult<usize> {
    let id = read_thing(id_ptr)?;
    let key = read_symbol(key_ptr)?;
    let value = read_value(value_ptr)?;
    root_call(RootOp::PropSet {
        id,
        key,
        value,
    })
}

pub fn sys_root_find(ptr_kind: usize, ptr_buf: usize, len: usize) -> SysResult<usize> {
    let sym = read_symbol(ptr_kind)?;

    validate_user_range(ptr_buf, len, true)?;

    if len > 4096 {
        return Err(Errno::EINVAL);
    }
    let mut kbuf = [0u8; 4096];

    let msg = RootOp::Find {
        kind: sym,
        buffer: kbuf.as_mut_ptr() as u64,
        len: len as u64,
    };

    let count = root_call(msg)?;

    // 16 bytes per ID now
    let bytes_to_copy = core::cmp::min(count * 16, len);
    unsafe {
        copyout(ptr_buf, &kbuf[..bytes_to_copy])?;
    }

    Ok(count)
}

pub fn sys_root_query(
    plan_ptr: usize,
    plan_len: usize,
    out_ptr: usize,
    out_cap: usize,
) -> SysResult<usize> {
    use crate::root::query::PreparedStep;
    use abi::query::{QueryRow, QueryStep};

    let step_size = core::mem::size_of::<QueryStep>();
    let total_plan_bytes = plan_len * step_size;
    validate_user_range(plan_ptr, total_plan_bytes, false)?;

    if plan_len > 8 {
        return Err(Errno::EINVAL);
    }

    let mut steps = alloc::vec::Vec::with_capacity(plan_len);
    for i in 0..plan_len {
        let ptr = plan_ptr + i * step_size;
        let mut step: QueryStep = unsafe { core::mem::zeroed() };
        let slice =
            unsafe { core::slice::from_raw_parts_mut(&mut step as *mut _ as *mut u8, step_size) };
        unsafe {
            copyin(slice, ptr)?;
        }

        let sym_id = match step.symbol.tag {
            abi::symbols::SYMBOL_REF_TAG_ID => {
                let id_ptr = step.symbol.ptr_or_id as usize;
                let id = read_thing(id_ptr)?;
                SymbolId(id.0)
            },
            abi::symbols::SYMBOL_REF_TAG_STR => {
                let s_ptr = step.symbol.ptr_or_id as usize;
                let s_len = step.symbol.len as usize;
                if s_len > 256 {
                    return Err(Errno::EINVAL);
                }
                validate_user_range(s_ptr, s_len, false)?;
                let mut buf = [0u8; 256];
                unsafe {
                    copyin(&mut buf[..s_len], s_ptr)?;
                }
                let s = core::str::from_utf8(&buf[..s_len]).map_err(|_| Errno::EINVAL)?;

                // We use stack buffer for ID output
                let mut id_buf = SymbolId::default();
                let intern_msg = RootOp::Intern {
                    name: String::from(s),
                    out_ptr: &mut id_buf as *mut _ as u64,
                };
                root_call(intern_msg)?;
                id_buf
            }
            _ => return Err(Errno::EINVAL),
        };

        steps.push(PreparedStep {
            op: step.op,
            arg1: step.arg1,
            symbol: sym_id,
        });
    }

    let row_size = core::mem::size_of::<QueryRow>();
    let total_out_bytes = out_cap * row_size;
    validate_user_range(out_ptr, total_out_bytes, true)?;

    let safe_cap = core::cmp::min(out_cap, 1024);
    let mut kbuf = alloc::vec![QueryRow::default(); safe_cap];

    let msg = RootOp::Query {
        plan: steps,
        out_buffer: kbuf.as_mut_ptr() as u64,
        out_len: (safe_cap * row_size) as u64,
    };

    let count = root_call(msg)?;

    let bytes_to_copy = count * row_size;
    let src = unsafe { core::slice::from_raw_parts(kbuf.as_ptr() as *const u8, bytes_to_copy) };
    unsafe {
        copyout(out_ptr, src)?;
    }

    Ok(count)
}

pub fn sys_root_describe_thing(id_ptr: usize, out_ptr: usize, len: usize) -> SysResult<usize> {
    let id = read_thing(id_ptr)?;
    validate_user_range(out_ptr, len, true)?;
    let mut kbuf = [0u8; 256];
    let kbuf_len = core::cmp::min(len, kbuf.len());
    let reply = root_svc::enqueue(RootOp::DescribeThing {
        id,
        buffer: kbuf.as_mut_ptr() as u64,
        len: kbuf_len as u64,
    });

    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            let status = reply.status.load(Ordering::Relaxed);
            let written = reply.value.load(Ordering::Relaxed) as usize;
            if status == 0 {
                unsafe {
                    copyout(out_ptr, &kbuf[..written])?;
                }
                return Ok(written);
            } else {
                return Err(Errno::EIO);
            }
        }
        unsafe {
            crate::task::scheduler::yield_now_current();
        }
    }
}

pub fn sys_root_describe_edge(
    src_ptr: usize,
    rel_ptr: usize,
    dst_ptr: usize,
    out_ptr: usize,
    len: usize,
) -> SysResult<usize> {
    validate_user_range(out_ptr, len, true)?;
    let src = read_thing(src_ptr)?;
    let rel = read_symbol(rel_ptr)?;
    let dst = read_thing(dst_ptr)?;

    let mut kbuf = [0u8; 512];
    let kbuf_len = core::cmp::min(len, kbuf.len());
    let reply = root_svc::enqueue(RootOp::DescribeEdge {
        src,
        rel,
        dst,
        buffer: kbuf.as_mut_ptr() as u64,
        len: kbuf_len as u64,
    });

    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            let status = reply.status.load(Ordering::Relaxed);
            let written = reply.value.load(Ordering::Relaxed) as usize;
            if status == 0 {
                unsafe {
                    copyout(out_ptr, &kbuf[..written])?;
                }
                return Ok(written);
            } else {
                return Err(Errno::EIO);
            }
        }
        unsafe {
            crate::task::scheduler::yield_now_current();
        }
    }
}

pub fn sys_root_dump_edges(id_ptr: usize, out_ptr: usize, len: usize) -> SysResult<usize> {
    let id = read_thing(id_ptr)?;
    validate_user_range(out_ptr, len, true)?;
    let mut kbuf = [0u8; 1024];
    let kbuf_len = core::cmp::min(len, kbuf.len());
    let reply = root_svc::enqueue(RootOp::DumpEdges {
        id,
        buffer: kbuf.as_mut_ptr() as u64,
        len: kbuf_len as u64,
    });

    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            let status = reply.status.load(Ordering::Relaxed);
            let written = reply.value.load(Ordering::Relaxed) as usize;
            if status == 0 {
                unsafe {
                    copyout(out_ptr, &kbuf[..written])?;
                }
                return Ok(written);
            } else {
                return Err(Errno::EIO);
            }
        }
        unsafe {
            crate::task::scheduler::yield_now_current();
        }
    }
}

pub fn sys_root_get_edges(id_ptr: usize, out_ptr: usize, len: usize) -> SysResult<usize> {
    let id = read_thing(id_ptr)?;
    validate_user_range(out_ptr, len, true)?;
    
    // Allocate a temporary kernel buffer to receive the edges
    let mut kbuf = [0u8; 4096]; // ~78 edges max per batch
    let kbuf_len = core::cmp::min(len, kbuf.len());
    
    let reply = root_svc::enqueue(RootOp::GetEdges {
        id,
        buffer: kbuf.as_mut_ptr() as u64,
        len: kbuf_len as u64,
    });

    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            let status = reply.status.load(Ordering::Relaxed);
            let written = reply.value.load(Ordering::Relaxed) as usize;
            if status == 0 {
                let edge_size = core::mem::size_of::<abi::types::Edge>();
                let bytes_to_copy = core::cmp::min(written * edge_size, len);
                unsafe {
                    copyout(out_ptr, &kbuf[..bytes_to_copy])?;
                }
                return Ok(written);
            } else {
                return Err(Errno::EIO);
            }
        }
        unsafe {
            crate::task::scheduler::yield_now_current();
        }
    }
}

pub fn sys_root_dump_graph(limit: usize) -> SysResult<usize> {
    root_call(RootOp::DumpGraph {
        limit: limit as u64,
    })
}

pub fn sys_root_bytespace_create(len: usize, flags: usize, format: usize, out_ptr: usize) -> SysResult<usize> {
    if out_ptr != 0 {
        validate_user_range(out_ptr, 16, true)?;
    }
    root_call(RootOp::BytespaceCreate {
        len: len as u64,
        flags: flags as u64,
        format: format as u64,
        out_ptr: out_ptr as u64,
    })
}

pub fn sys_root_bytespace_read(
    id_ptr: usize,
    offset: usize,
    ptr: usize,
    len: usize,
) -> SysResult<usize> {
    let id = read_thing(id_ptr)?;
    validate_user_range(ptr, len, true)?;

    let mut kbuf = [0u8; 4096];
    let mut total_read = 0;
    let mut curr_offset = offset;
    let mut curr_ptr = ptr;
    let mut remaining = len;

    while remaining > 0 {
        let chunk_len = core::cmp::min(remaining, kbuf.len());

        let op = RootOp::BytespaceRead {
            id,
            offset: curr_offset as u64,
            ptr: kbuf.as_mut_ptr() as u64,
            len: chunk_len as u64,
        };

        let res = root_call(op)?;
        if res == 0 {
            break;
        }

        unsafe {
            copyout(curr_ptr, &kbuf[..res])?;
        }

        curr_offset += res;
        curr_ptr += res;
        total_read += res;
        remaining -= res;

        if res < chunk_len {
            break;
        }
    }

    Ok(total_read)
}

pub fn sys_root_watch_subscribe(target_ptr: usize, mask: usize, out_ptr: usize) -> SysResult<usize> {
    let target_id = read_thing(target_ptr)?;
    if out_ptr != 0 {
        validate_user_range(out_ptr, 16, true)?;
    }
    let res = root_call(RootOp::WatchSubscribe {
        target_id,
        mask: mask as u64,
        out_ptr: out_ptr as u64,
    });
    if let Err(e) = res {
        crate::kinfo!("sys_root_watch_subscribe target=? mask={} failed: {:?}", mask, e);
    }
    res
}

pub fn sys_root_stream_poll(stream_ptr: usize, max: usize, out_ptr: usize) -> SysResult<usize> {
    let stream_id = read_thing(stream_ptr)?;
    validate_user_range(out_ptr, max, true)?;

    let evt_size = core::mem::size_of::<abi::types::RootWatchEvent>();
    if max < evt_size {
        return Err(Errno::EINVAL);
    }

    let reply = root_svc::enqueue(RootOp::StreamPoll {
        stream_id,
        max,
        out_ptr: out_ptr as u64,
    });

    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            let status = reply.status.load(Ordering::Relaxed);
            let count = reply.value.load(Ordering::Relaxed);

            if status == 0 && count > 0 {
                return Ok(1);
            } else {
                return Ok(0);
            }
        }
        unsafe {
            crate::task::scheduler::yield_now_current();
        }
    }
}

pub fn sys_root_bytespace_write(
    id_ptr: usize,
    offset: usize,
    ptr: usize,
    len: usize,
) -> SysResult<usize> {
    let id = read_thing(id_ptr)?;
    validate_user_range(ptr, len, false)?;

    let mut kbuf = [0u8; 4096];
    let mut total_written = 0;
    let mut curr_offset = offset;
    let mut curr_ptr = ptr;
    let mut remaining = len;

    while remaining > 0 {
        let chunk_len = core::cmp::min(remaining, kbuf.len());

        unsafe {
            copyin(&mut kbuf[..chunk_len], curr_ptr)?;
        }

        let op = RootOp::BytespaceWrite {
            id,
            offset: curr_offset as u64,
            ptr: kbuf.as_ptr() as u64,
            len: chunk_len as u64,
        };

        let res = root_call(op)?;
        if res == 0 {
            break;
        }

        curr_offset += res;
        curr_ptr += res;
        total_written += res;
        remaining -= res;

        if res < chunk_len {
            break;
        }
    }

    Ok(total_written)
}

pub fn sys_root_bytespace_info(id_ptr: usize) -> SysResult<usize> {
    let id = read_thing(id_ptr)?;
    let reply = root_svc::enqueue(RootOp::BytespaceInfo { id });
    
    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            let status = reply.status.load(Ordering::Relaxed);
            if status == 0 {
                // Size is in value, page_count in p0, flags in p1
                let size = reply.value.load(Ordering::Relaxed);
                return Ok(size as usize);
            } else {
                return Err(Errno::ENOENT);
            }
        }
        unsafe {
            crate::task::scheduler::yield_now_current();
        }
    }
}

pub fn sys_root_bytespace_map(id_ptr: usize) -> SysResult<usize> {
    let id = read_thing(id_ptr)?;
    // Get caller's TID
    let tid = unsafe { crate::task::scheduler::current_tid_current() };
    
    let reply = root_svc::enqueue(RootOp::BytespaceMap {
        id,
        tid,
    });
    
    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            let status = reply.status.load(Ordering::Relaxed);
            if status == 0 {
                let user_va = reply.value.load(Ordering::Relaxed);
                let phys_base = reply.p0.load(Ordering::Relaxed);
                let page_count = reply.p1.load(Ordering::Relaxed) as usize;
                
                // Map pages using global mapping hook
                for i in 0..page_count {
                    let virt = user_va + (i as u64 * 4096);
                    let phys = phys_base + (i as u64 * 4096);
                    
                    unsafe {
                        crate::memory::map_user_page(virt, phys)?;
                    }
                }
                
                return Ok(user_va as usize);
            } else {
                return Err(Errno::ENOENT);
            }
        }
        unsafe {
            crate::task::scheduler::yield_now_current();
        }
    }
}

pub fn sys_root_bytespace_unmap(id_ptr: usize, user_va: usize) -> SysResult<usize> {
    let id = read_thing(id_ptr)?;
    let tid = unsafe { crate::task::scheduler::current_tid_current() };
    
    let reply = root_svc::enqueue(RootOp::BytespaceUnmap {
        id,
        user_va: user_va as u64,
        tid,
    });
    
    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            let status = reply.status.load(Ordering::Relaxed);
            if status == 0 {
                return Ok(0);
            } else {
                return Err(Errno::ENOENT);
            }
        }
        unsafe {
            crate::task::scheduler::yield_now_current();
        }
    }
}

pub fn sys_root_bytespace_phys(id_ptr: usize) -> SysResult<usize> {
    let id = read_thing(id_ptr)?;
    let reply = root_svc::enqueue(RootOp::BytespacePhys { id });

    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            let status = reply.status.load(Ordering::Relaxed);
            if status == 0 {
                let phys_base = reply.value.load(Ordering::Relaxed);
                return Ok(phys_base as usize);
            } else {
                return Err(Errno::ENOENT);
            }
        }
        unsafe {
            crate::task::scheduler::yield_now_current();
        }
    }
}

use crate::kinfo;

pub fn sys_root_watch_open(spec_ptr: usize, out_ptr: usize) -> SysResult<usize> {
    use abi::types::WatchSpec;
    use abi::query::QueryStep;
    use abi::root::RootWatchFilter;
    use crate::root::query::PreparedStep;
    use crate::root::graph::WatchFilter;

    if out_ptr != 0 {
        validate_user_range(out_ptr, 16, true)?;
    }

    kinfo!("sys_root_watch_open: ptr={:#x}", spec_ptr);
    let mut spec = WatchSpec::default();
    let spec_slice = unsafe { 
        core::slice::from_raw_parts_mut(&mut spec as *mut _ as *mut u8, core::mem::size_of::<WatchSpec>()) 
    };
    kinfo!("sys_root_watch_open: validating range len={}", spec_slice.len());
    validate_user_range(spec_ptr, spec_slice.len(), false)?;
    unsafe { copyin(spec_slice, spec_ptr)? };
    kinfo!("sys_root_watch_open: copyin success. mode={} start_seq={}", spec.mode, spec.start_seq);

    let plan_ptr = spec.query_ptr as usize;
    let plan_len = spec.query_len as usize;
    let step_size = core::mem::size_of::<QueryStep>();
    let total_plan_bytes = plan_len * step_size;

    if plan_len > 8 {
        return Err(Errno::EINVAL);
    }
    if plan_len > 0 {
        validate_user_range(plan_ptr, total_plan_bytes, false)?;
    }

    let mut steps = alloc::vec::Vec::with_capacity(plan_len);
    for i in 0..plan_len {
        let ptr = plan_ptr + i * step_size;
        let mut step: QueryStep = unsafe { core::mem::zeroed() };
        let slice = unsafe { 
            core::slice::from_raw_parts_mut(&mut step as *mut _ as *mut u8, step_size) 
        };
        unsafe { copyin(slice, ptr)? };

        let sym_id = match step.symbol.tag {
            abi::symbols::SYMBOL_REF_TAG_ID => {
                let id_ptr = step.symbol.ptr_or_id as usize;
                let id = read_thing(id_ptr)?;
                SymbolId(id.0)
            },
            abi::symbols::SYMBOL_REF_TAG_STR => {
                let s_ptr = step.symbol.ptr_or_id as usize;
                let s_len = step.symbol.len as usize;
                if s_len > 256 {
                    return Err(Errno::EINVAL);
                }
                validate_user_range(s_ptr, s_len, false)?;
                let mut buf = [0u8; 256];
                unsafe { copyin(&mut buf[..s_len], s_ptr)? };
                let s = core::str::from_utf8(&buf[..s_len]).map_err(|_| Errno::EINVAL)?;
                // We use stack buffer for ID output
                let mut id_buf = SymbolId::default();
                let intern_msg = RootOp::Intern {
                    name: String::from(s),
                    out_ptr: &mut id_buf as *mut _ as u64,
                };
                root_call(intern_msg)?;
                id_buf
            }
            _ => return Err(Errno::EINVAL),
        };

        steps.push(PreparedStep {
            op: step.op,
            arg1: step.arg1,
            symbol: sym_id,
        });
    }

    // Read filter if provided
    let filter = if spec.filter_ptr != 0 && spec.filter_len >= core::mem::size_of::<RootWatchFilter>() as u64 {
        let filter_ptr = spec.filter_ptr as usize;
        validate_user_range(filter_ptr, core::mem::size_of::<RootWatchFilter>(), false)?;
        let mut abi_filter = RootWatchFilter::default();
        let filter_slice = unsafe {
            core::slice::from_raw_parts_mut(&mut abi_filter as *mut _ as *mut u8, 
                core::mem::size_of::<RootWatchFilter>())
        };
        unsafe { copyin(filter_slice, filter_ptr)? };
        
        // Convert ABI filter to kernel filter
        WatchFilter {
            flags: abi_filter.flags,
            kind_id: abi_filter.kind_id,
            predicate_id: abi_filter.predicate_id,
            subject: abi_filter.subject,
        }
    } else {
        WatchFilter::default()
    };

    let msg = RootOp::WatchOpen {
        mode: spec.mode,
        start_seq: spec.start_seq,
        query: steps,
        filter,
        out_ptr: out_ptr as u64,
    };
    root_call(msg)
}

pub fn sys_root_watch_next(
    id_ptr: usize,
    out_seq_ptr: usize,
    out_ptr: usize,
    out_len: usize,
) -> SysResult<usize> {
    let id = read_thing(id_ptr)?;
    validate_user_range(out_seq_ptr, core::mem::size_of::<u64>(), true)?;
    validate_user_range(out_ptr, out_len, true)?;

    const MAX_WATCH_PAYLOAD: usize = 256 * 1024;
    let cap = core::cmp::min(out_len, MAX_WATCH_PAYLOAD);
    let mut kbuf = alloc::vec![0u8; cap];

    let reply = root_svc::enqueue(RootOp::WatchNext {
        id,
        out_seq_ptr: 0,
        out_ptr: kbuf.as_mut_ptr() as u64,
        out_len: out_len as u64,
    });

    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            let status = reply.status.load(Ordering::Relaxed);

            if status >= 0 {
                let bytes_read = reply.value.load(Ordering::Relaxed) as usize;

                if status == 0 {
                    // Copy data
                    unsafe {
                        copyout(out_ptr, &kbuf[..bytes_read])?;
                    }
                    // Copy seq
                    let seq = reply.p0.load(Ordering::Relaxed);
                    unsafe {
                        copyout(out_seq_ptr, &seq.to_le_bytes())?;
                    }
                    return Ok(bytes_read);
                }
                return Ok(bytes_read);
            } else {
                match status {
                    -75 => return Err(Errno::EOVERFLOW),
                    -28 => return Err(Errno::ENOSPC),
                    -11 => return Err(Errno::EAGAIN),
                    -22 => return Err(Errno::EINVAL),
                    -9 => return Err(Errno::EBADF),
                    _ => {
                        let tid = unsafe { crate::task::scheduler::current_tid_current() };
                        crate::kinfo!(
                            "watch_next: UNEXPECTED status={} wid={:?} tid={}",
                            status,
                            id,
                            tid
                        );
                        return Err(Errno::EIO);
                    }
                }
            }
        }
        unsafe {
            crate::task::scheduler::yield_now_current();
        }
    }
}

pub fn sys_root_apply_batch(ptr: usize, len: usize) -> SysResult<usize> {
    validate_user_range(ptr, len, false)?;
    
    // Validated above
    let mut batch = alloc::vec::Vec::with_capacity(len);
    unsafe {
        batch.set_len(len);
        copyin(&mut batch, ptr)?;
    }

    let reply = root_svc::enqueue(RootOp::ApplyBatch {
        batch,
    });

    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            let status = reply.status.load(Ordering::Relaxed);
            let val = reply.value.load(Ordering::Relaxed);
            if status == 0 {
                return Ok(val as usize); // Returns seq
            } else {
                return Err(Errno::EINVAL);
            }
        }
        unsafe {
            crate::task::scheduler::yield_now_current();
        }
    }
}

pub fn sys_root_watch_close(id_ptr: usize) -> SysResult<usize> {
    let id = read_thing(id_ptr)?;
    root_call(RootOp::WatchClose { id })
}
