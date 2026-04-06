//! Root/Graph syscalls

use super::{copyin, copyout, read_symbol, root_call};
use crate::root::{self as root_svc, RootOp};
use crate::syscall::validate::validate_user_range;
use abi::errors::{Errno, SysResult};
use abi::vm::{VmBackingKind, VmMapFlags, VmProt, VmRegionInfo};
use alloc::string::String;
use core::sync::atomic::Ordering;

macro_rules! wait_reply_block {
    ($reply:expr) => {{
        let mut spins = 0;
        loop {
            let done = $reply.done.load(Ordering::Acquire);
            if done != 0 {
                break;
            }
            spins += 1;
            if spins < 100 {
                core::hint::spin_loop();
            } else {
                break;
            }
        }

        if $reply.done.load(Ordering::Acquire) == 0 {
            let my_tid = unsafe { crate::sched::current_tid_current() };
            $reply.waiting_task.store(my_tid, Ordering::SeqCst);

            loop {
                let done = $reply.done.load(Ordering::SeqCst);
                if done != 0 {
                    $reply.waiting_task.store(0, Ordering::Relaxed);
                    break;
                }
                unsafe {
                    crate::sched::block_current_erased();
                }
            }
        }
    }};
}

pub fn sys_root_get_kind(id: usize) -> SysResult<usize> {
    root_call(RootOp::GetKind { id: id as u64 })
}

pub fn sys_root_intern(ptr: usize, len: usize) -> SysResult<usize> {
    // Support up to 4KB strings for SVG path data and style attributes
    if len > 4096 {
        return Err(Errno::EINVAL);
    }
    validate_user_range(ptr, len, false)?;

    let mut buf = alloc::vec![0u8; len];
    unsafe {
        copyin(&mut buf[..len], ptr)?;
    }

    let s = core::str::from_utf8(&buf[..len]).map_err(|_| Errno::EINVAL)?;
    crate::ktrace!("ROOT_INTERN: '{}'", s);
    let msg = RootOp::Intern {
        name: String::from(s),
    };

    root_call(msg)
}

pub fn sys_root_create_node(kind_ptr: usize) -> SysResult<usize> {
    let sym = read_symbol(kind_ptr)?;
    let creator_tid = unsafe { crate::sched::current_tid_current() };
    let owner_thing_id = unsafe { crate::sched::graph_thing_for_current() };
    root_call(RootOp::CreateNode {
        kind: sym,
        creator_tid,
        owner_thing_id,
    })
}

pub fn sys_root_link(src: usize, rel_ptr: usize, dst: usize) -> SysResult<usize> {
    let rel = read_symbol(rel_ptr)?;
    let reply = root_svc::enqueue(RootOp::Link {
        src: src as u64,
        rel,
        dst: dst as u64,
    });
    wait_reply_block!(reply);
    let status = reply.status.load(Ordering::Relaxed);
    if status == 0 {
        return Ok(0);
    } else {
        return Err(Errno::EIO);
    }
}

pub fn sys_root_prop_get(id: usize, ptr: usize, _reserved: usize) -> SysResult<usize> {
    let sym = read_symbol(ptr)?;
    let msg = RootOp::PropGet {
        id: id as u64,
        key: sym,
    };
    root_call(msg)
}

pub fn sys_root_prop_set(id: usize, key_ptr: usize, value: usize) -> SysResult<usize> {
    let key = read_symbol(key_ptr)?;
    root_call(RootOp::PropSet {
        id: id as u64,
        key,
        value: value as u64,
    })
}

pub fn sys_root_find(ptr_kind: usize, ptr_buf: usize, len: usize) -> SysResult<usize> {
    let sym = read_symbol(ptr_kind)?;

    validate_user_range(ptr_buf, len, true)?;

    // Support up to 64KB of results (4096 nodes)
    const MAX_FIND_BYTES: usize = 64 * 1024;
    let kbuf_len = core::cmp::min(len, MAX_FIND_BYTES);
    let mut kbuf = alloc::vec![0u8; kbuf_len];

    let msg = RootOp::Find {
        kind: sym,
        buffer: kbuf.as_mut_ptr() as u64,
        len: kbuf_len as u64,
    };

    let count = root_call(msg)?;

    let entries_found = count;
    let entries_to_copy = core::cmp::min(entries_found, kbuf_len / 16);
    let bytes_to_copy = entries_to_copy * 16;

    unsafe {
        copyout(ptr_buf, &kbuf[..bytes_to_copy])?;
    }

    Ok(entries_to_copy)
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
            abi::symbols::SYMBOL_REF_TAG_ID => step.symbol.ptr_or_id as u32,
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
                let intern_msg = RootOp::Intern {
                    name: String::from(s),
                };
                let id = root_call(intern_msg)?;
                id as u32
            }
            _ => return Err(Errno::EINVAL),
        };

        steps.push(PreparedStep {
            op: step.op,
            arg1: step.arg1,
            arg2: step.arg2,
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

    let rows_found = count;
    let rows_to_copy = core::cmp::min(rows_found, safe_cap);
    let bytes_to_copy = rows_to_copy * row_size;

    let src = unsafe { core::slice::from_raw_parts(kbuf.as_ptr() as *const u8, bytes_to_copy) };
    unsafe {
        copyout(out_ptr, src)?;
    }

    Ok(rows_to_copy)
}

pub fn sys_root_describe_thing(id: usize, out_ptr: usize, len: usize) -> SysResult<usize> {
    validate_user_range(out_ptr, len, true)?;
    let mut kbuf = [0u8; 256];
    let kbuf_len = core::cmp::min(len, kbuf.len());
    let reply = root_svc::enqueue(RootOp::DescribeThing {
        id: id as u64,
        buffer: kbuf.as_mut_ptr() as u64,
        len: kbuf_len as u64,
    });

    wait_reply_block!(reply);
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

pub fn sys_root_describe_symbol(id: usize, out_ptr: usize, len: usize) -> SysResult<usize> {
    validate_user_range(out_ptr, len, true)?;
    // Support up to 4KB for long SVG path data and style attributes
    let kbuf_len = core::cmp::min(len, 4096);
    let mut kbuf = alloc::vec![0u8; kbuf_len];
    let reply = root_svc::enqueue(RootOp::DescribeSymbol {
        id: id as u32,
        buffer: kbuf.as_mut_ptr() as u64,
        len: kbuf_len as u64,
    });

    wait_reply_block!(reply);
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

pub fn sys_root_describe_edge(
    src: usize,
    rel_ptr: usize,
    dst: usize,
    out_ptr: usize,
    len: usize,
) -> SysResult<usize> {
    validate_user_range(out_ptr, len, true)?;
    let rel = read_symbol(rel_ptr)?;

    let mut kbuf = [0u8; 512];
    let kbuf_len = core::cmp::min(len, kbuf.len());
    let reply = root_svc::enqueue(RootOp::DescribeEdge {
        src: src as u64,
        rel,
        dst: dst as u64,
        buffer: kbuf.as_mut_ptr() as u64,
        len: kbuf_len as u64,
    });

    wait_reply_block!(reply);
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

pub fn sys_root_dump_edges(id: usize, out_ptr: usize, len: usize) -> SysResult<usize> {
    validate_user_range(out_ptr, len, true)?;
    let mut kbuf = [0u8; 1024];
    let kbuf_len = core::cmp::min(len, kbuf.len());
    let reply = root_svc::enqueue(RootOp::DumpEdges {
        id: id as u64,
        buffer: kbuf.as_mut_ptr() as u64,
        len: kbuf_len as u64,
    });

    wait_reply_block!(reply);
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

pub fn sys_root_get_edges(id: usize, out_ptr: usize, len: usize) -> SysResult<usize> {
    validate_user_range(out_ptr, len, true)?;

    // Allocate a temporary kernel buffer to receive the edges
    // Must be large enough to hold some edges, but not too large for stack
    // GraphEdge is now abi::types::Edge (52 bytes).
    let mut kbuf = [0u8; 4096]; // ~78 edges max per batch
    let kbuf_len = core::cmp::min(len, kbuf.len());

    let reply = root_svc::enqueue(RootOp::GetEdges {
        id: id as u64,
        buffer: kbuf.as_mut_ptr() as u64,
        len: kbuf_len as u64,
    });

    wait_reply_block!(reply);
    let status = reply.status.load(Ordering::Relaxed);
    let written = reply.value.load(Ordering::Relaxed) as usize;
    if status == 0 {
        let edge_size = core::mem::size_of::<abi::types::Edge>();
        let actual_count = core::cmp::min(written, kbuf_len / edge_size);
        let bytes_to_copy = actual_count * edge_size;
        unsafe {
            copyout(out_ptr, &kbuf[..bytes_to_copy])?;
        }
        return Ok(actual_count);
    } else {
        return Err(Errno::EIO);
    }
}

pub fn sys_root_get_props(id: usize, out_ptr: usize, len: usize) -> SysResult<usize> {
    validate_user_range(out_ptr, len, true)?;

    let entry_size = core::mem::size_of::<abi::types::GraphProp>();
    if entry_size == 0 || len < entry_size {
        return Ok(0);
    }

    const MAX_PROP_BYTES: usize = 64 * 1024;
    let kbuf_len = core::cmp::min(len, MAX_PROP_BYTES);
    let mut kbuf = alloc::vec![0u8; kbuf_len];

    let reply = root_svc::enqueue(RootOp::GetProps {
        id: id as u64,
        buffer: kbuf.as_mut_ptr() as u64,
        len: kbuf_len as u64,
    });

    wait_reply_block!(reply);
    let status = reply.status.load(Ordering::Relaxed);
    let written = reply.value.load(Ordering::Relaxed) as usize;
    if status == 0 {
        let max_entries = kbuf_len / entry_size;
        let actual_count = core::cmp::min(written, max_entries);
        let bytes_to_copy = actual_count * entry_size;
        unsafe {
            copyout(out_ptr, &kbuf[..bytes_to_copy])?;
        }
        return Ok(actual_count);
    } else {
        return Err(Errno::EIO);
    }
}

pub fn sys_root_dump_graph(limit: usize) -> SysResult<usize> {
    root_call(RootOp::DumpGraph {
        limit: limit as u64,
    })
}

pub fn sys_root_bytespace_create(len: usize, flags: usize, format: usize) -> SysResult<usize> {
    root_call(RootOp::BytespaceCreate {
        len: len as u64,
        flags: flags as u64,
        format: format as u64,
    })
}

pub fn sys_root_bytespace_read(
    id: usize,
    offset: usize,
    ptr: usize,
    len: usize,
) -> SysResult<usize> {
    validate_user_range(ptr, len, true)?;

    let mut kbuf = [0u8; 4096];
    let mut total_read = 0;
    let mut curr_offset = offset;
    let mut curr_ptr = ptr;
    let mut remaining = len;

    while remaining > 0 {
        let chunk_len = core::cmp::min(remaining, kbuf.len());

        let op = RootOp::BytespaceRead {
            id: id as u64,
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

pub fn sys_root_watch_subscribe(target: usize, mask: usize) -> SysResult<usize> {
    let res = root_call(RootOp::WatchSubscribe {
        target_id: target as u64,
        mask: mask as u64,
    });
    if let Err(e) = res {
        crate::kinfo!(
            "FLAT: sys_root_watch_subscribe target={} mask={} failed: {:?}",
            target,
            mask,
            e
        );
    }
    res
}

pub fn sys_root_stream_poll(stream: usize, max: usize, out_ptr: usize) -> SysResult<usize> {
    validate_user_range(out_ptr, max, true)?;

    let evt_size = core::mem::size_of::<abi::types::RootWatchEvent>();
    if max < evt_size {
        return Err(Errno::EINVAL);
    }

    let reply = root_svc::enqueue(RootOp::StreamPoll {
        stream_id: stream as u64,
        max,
        out_ptr: out_ptr as u64,
    });

    wait_reply_block!(reply);
    let status = reply.status.load(Ordering::Relaxed);
    let count = reply.value.load(Ordering::Relaxed);

    if status == 0 && count > 0 {
        let p0 = reply.p0.load(Ordering::Relaxed);
        let p1 = reply.p1.load(Ordering::Relaxed);
        let p2 = reply.p2.load(Ordering::Relaxed);

        let evt = abi::types::RootWatchEvent {
            target: p0,
            key: p1,
            value: p2,
        };

        let src = unsafe { core::slice::from_raw_parts(&evt as *const _ as *const u8, evt_size) };
        unsafe {
            copyout(out_ptr, src)?;
        }

        return Ok(1);
    } else {
        return Ok(0);
    }
}

pub fn sys_root_bytespace_write(
    id: usize,
    offset: usize,
    ptr: usize,
    len: usize,
) -> SysResult<usize> {
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
            id: id as u64,
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

pub fn sys_root_bytespace_info(id: usize) -> SysResult<usize> {
    let reply = root_svc::enqueue(RootOp::BytespaceInfo { id: id as u64 });

    wait_reply_block!(reply);
    let status = reply.status.load(Ordering::Relaxed);
    if status == 0 {
        // Size is in value, page_count in p0, flags in p1
        let size = reply.value.load(Ordering::Relaxed);
        return Ok(size as usize);
    } else {
        return Err(Errno::ENOENT);
    }
}

pub fn sys_root_bytespace_map(id: usize) -> SysResult<usize> {
    // Get caller's TID
    let tid = unsafe { crate::sched::current_tid_current() };

    let reply = root_svc::enqueue(RootOp::BytespaceMap { id: id as u64, tid });

    wait_reply_block!(reply);
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

        // Add mapping
        let region = VmRegionInfo {
            start: user_va as usize,
            end: (user_va as usize) + page_count * 4096,
            prot: VmProt::USER | VmProt::READ | VmProt::WRITE, // Assuming RW
            flags: VmMapFlags::SHARED,                         // Assuming shared
            backing_kind: VmBackingKind::Unknown,
            _reserved: [0; 7],
        };
        unsafe {
            crate::sched::add_user_mapping_current(region).ok();
        }

        return Ok(user_va as usize);
    } else {
        return Err(Errno::ENOENT);
    }
}

pub fn sys_root_bytespace_unmap(id: usize, user_va: usize) -> SysResult<usize> {
    let tid = unsafe { crate::sched::current_tid_current() };

    let reply = root_svc::enqueue(RootOp::BytespaceUnmap {
        id: id as u64,
        user_va: user_va as u64,
        tid,
    });

    wait_reply_block!(reply);
    let status = reply.status.load(Ordering::Relaxed);
    if status == 0 {
        // Unmap pages
        if let Some(region) = unsafe { crate::sched::get_user_mapping_at_current(user_va) } {
            let len = region.end - region.start;
            unsafe {
                if let Ok(removed) = crate::sched::remove_user_mappings_current(user_va, len) {
                    for (start, end) in removed {
                        let mut virt = start as u64;
                        let end_virt = end as u64;
                        while virt < end_virt {
                            let _ = crate::memory::unmap_user_page(virt);
                            virt += 4096;
                        }
                    }
                }
            }
        }
        return Ok(0);
    } else {
        return Err(Errno::ENOENT);
    }
}

pub fn sys_root_async_prop_set(id: usize, key_ptr: usize, value: usize) -> SysResult<usize> {
    let key = read_symbol(key_ptr)?;
    let cell = root_svc::enqueue(RootOp::PropSet {
        id: id as u64,
        key,
        value: value as u64,
    });
    crate::root::async_ops::alloc_handle(cell)
        .map(|h| h as usize)
        .ok_or(Errno::ENOMEM)
}

pub fn sys_root_async_link(src: usize, rel_ptr: usize, dst: usize) -> SysResult<usize> {
    let rel = read_symbol(rel_ptr)?;
    let cell = root_svc::enqueue(RootOp::Link {
        src: src as u64,
        rel,
        dst: dst as u64,
    });
    crate::root::async_ops::alloc_handle(cell)
        .map(|h| h as usize)
        .ok_or(Errno::ENOMEM)
}

pub fn sys_root_async_create_node(kind_ptr: usize) -> SysResult<usize> {
    let kind = read_symbol(kind_ptr)?;
    let creator_tid = unsafe { crate::sched::current_tid_current() };
    let owner_thing_id = unsafe { crate::sched::graph_thing_for_current() };
    let cell = root_svc::enqueue(RootOp::CreateNode {
        kind,
        creator_tid,
        owner_thing_id,
    });
    crate::root::async_ops::alloc_handle(cell)
        .map(|h| h as usize)
        .ok_or(Errno::ENOMEM)
}

pub fn sys_root_async_wait(handle: usize) -> SysResult<usize> {
    let cell = crate::root::async_ops::get_cell(handle as u64).ok_or(Errno::ENOENT)?;
    wait_reply_block!(cell);
    let status = cell.status.load(Ordering::Relaxed);
    let val = cell.value.load(Ordering::Relaxed) as usize;
    crate::root::async_ops::free_handle(handle as u64);
    if status == 0 {
        Ok(val)
    } else {
        Err(abi::errors::errno(status as isize).unwrap_err())
    }
}

pub fn sys_root_async_drop(handle: usize) -> SysResult<usize> {
    crate::root::async_ops::free_handle(handle as u64);
    Ok(0)
}

pub fn sys_root_async_status(handle: usize) -> SysResult<usize> {
    let cell = crate::root::async_ops::get_cell(handle as u64).ok_or(Errno::ENOENT)?;
    if cell.done.load(Ordering::Acquire) != 0 {
        let status = cell.status.load(Ordering::Relaxed);
        if status == 0 {
            Ok(1) // 1 = Done
        } else {
            Ok(2) // 2 = Error
        }
    } else {
        Ok(0) // 0 = Pending
    }
}

pub fn sys_root_bytespace_phys(id: usize) -> SysResult<usize> {
    let reply = root_svc::enqueue(RootOp::BytespacePhys { id: id as u64 });

    wait_reply_block!(reply);
    let status = reply.status.load(Ordering::Relaxed);
    if status == 0 {
        let phys_base = reply.value.load(Ordering::Relaxed);
        return Ok(phys_base as usize);
    } else {
        return Err(Errno::ENOENT);
    }
}
pub fn sys_root_watch_open(spec_ptr: usize) -> SysResult<usize> {
    use crate::root::graph::WatchFilter;
    use crate::root::query::PreparedStep;
    use abi::query::QueryStep;
    use abi::root::RootWatchFilter;
    use abi::types::WatchSpec;

    crate::kdebug!("sys_root_watch_open: ptr={:#x}", spec_ptr);
    let mut spec = WatchSpec::default();
    let spec_slice = unsafe {
        core::slice::from_raw_parts_mut(
            &mut spec as *mut _ as *mut u8,
            core::mem::size_of::<WatchSpec>(),
        )
    };
    crate::kdebug!(
        "sys_root_watch_open: validating range len={}",
        spec_slice.len()
    );
    validate_user_range(spec_ptr, spec_slice.len(), false)?;
    unsafe { copyin(spec_slice, spec_ptr)? };
    crate::kdebug!(
        "sys_root_watch_open: copyin success. mode={} start_seq={}",
        spec.mode,
        spec.start_seq
    );

    // Decode filter if present for logging
    if spec.filter_ptr != 0 && spec.filter_len >= core::mem::size_of::<RootWatchFilter>() as u64 {
        // We already validated and copied it later, but let's peek for logging
        let filter_ptr = spec.filter_ptr as usize;
        let mut abi_filter = RootWatchFilter::default();
        if validate_user_range(filter_ptr, core::mem::size_of::<RootWatchFilter>(), false).is_ok() {
            let filter_slice = unsafe {
                core::slice::from_raw_parts_mut(
                    &mut abi_filter as *mut _ as *mut u8,
                    core::mem::size_of::<RootWatchFilter>(),
                )
            };
            if unsafe { copyin(filter_slice, filter_ptr) }.is_ok() {
                crate::kdebug!(
                    "sys_root_watch_open: DECODED FILTER: flags={:#x} kind={} pred={} subj_lo={}",
                    abi_filter.flags,
                    abi_filter.kind_id,
                    abi_filter.predicate_id,
                    abi_filter.subject_lo
                );
            }
        }
    } else {
        crate::kdebug!(
            "sys_root_watch_open: NO FILTER (filter_ptr={:#x} filter_len={})",
            spec.filter_ptr,
            spec.filter_len
        );
    }

    // Validate mode enum (must be 0=QueryThenStream or 1=StreamOnly)
    if abi::types::WatchMode::from_u32(spec.mode).is_none() {
        crate::kdebug!("sys_root_watch_open: invalid mode={}", spec.mode);
        return Err(Errno::EINVAL);
    }

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
        let slice =
            unsafe { core::slice::from_raw_parts_mut(&mut step as *mut _ as *mut u8, step_size) };
        unsafe { copyin(slice, ptr)? };

        let sym_id = match step.symbol.tag {
            abi::symbols::SYMBOL_REF_TAG_ID => step.symbol.ptr_or_id as u32,
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
                let intern_msg = RootOp::Intern {
                    name: String::from(s),
                };
                let id = root_call(intern_msg)?;
                id as u32
            }
            _ => return Err(Errno::EINVAL),
        };

        steps.push(PreparedStep {
            op: step.op,
            arg1: step.arg1,
            arg2: step.arg2,
            symbol: sym_id,
        });
    }

    // Read filter if provided
    let filter = if spec.filter_ptr != 0
        && spec.filter_len >= core::mem::size_of::<RootWatchFilter>() as u64
    {
        let filter_ptr = spec.filter_ptr as usize;
        validate_user_range(filter_ptr, core::mem::size_of::<RootWatchFilter>(), false)?;
        let mut abi_filter = RootWatchFilter::default();
        let filter_slice = unsafe {
            core::slice::from_raw_parts_mut(
                &mut abi_filter as *mut _ as *mut u8,
                core::mem::size_of::<RootWatchFilter>(),
            )
        };
        unsafe { copyin(filter_slice, filter_ptr)? };

        // Validate filter flags - reject unknown bits
        if (abi_filter.flags & !abi::root::WATCH_F_KNOWN_MASK) != 0 {
            crate::kdebug!(
                "sys_root_watch_open: unknown filter flags={:#x}",
                abi_filter.flags
            );
            return Err(Errno::EINVAL);
        }

        // Convert ABI filter to kernel filter
        WatchFilter {
            flags: abi_filter.flags,
            kind_id: abi_filter.kind_id,
            predicate_id: abi_filter.predicate_id,
            subject_lo: abi_filter.subject_lo,
        }
    } else {
        WatchFilter::default() // flags=0 means match all
    };

    let msg = RootOp::WatchOpen {
        mode: spec.mode,
        start_seq: spec.start_seq,
        query: steps,
        filter,
    };
    root_call(msg)
}

pub fn sys_root_watch_next(
    id: usize,
    out_seq_ptr: usize,
    out_ptr: usize,
    out_len: usize,
) -> SysResult<usize> {
    validate_user_range(out_seq_ptr, core::mem::size_of::<u64>(), true)?;
    validate_user_range(out_ptr, out_len, true)?;

    let cap = core::cmp::min(out_len, abi::watch::MAX_WATCH_PAYLOAD_BYTES);

    // Hybrid allocation: use stack for small requests, heap for large ones.
    // 1024 bytes covers ~20 small events (49 bytes each), sufficient for most polls.
    let mut stack_buf = [0u8; 1024];
    let mut heap_buf = alloc::vec::Vec::new();

    let buf_ptr = if cap <= stack_buf.len() {
        stack_buf.as_mut_ptr()
    } else {
        heap_buf.resize(cap, 0);
        heap_buf.as_mut_ptr()
    };

    let reply = root_svc::enqueue(RootOp::WatchNext {
        id: id as u64,
        out_seq_ptr: 0,
        out_ptr: buf_ptr as u64,
        out_len: cap as u64,
    });

    wait_reply_block!(reply);
    let status = reply.status.load(Ordering::Relaxed);

    if status >= 0 {
        let bytes_read = reply.value.load(Ordering::Relaxed) as usize;

        if status == 0 {
            // Copy data
            unsafe {
                let src = core::slice::from_raw_parts(buf_ptr, bytes_read);
                copyout(out_ptr, src)?;
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
            -22 => return Err(Errno::EINVAL), // Invalid handle
            -9 => return Err(Errno::EBADF),   // Bad/stale watch descriptor
            _ => {
                // Log unexpected status for debugging
                let tid = unsafe { crate::sched::current_tid_current() };
                crate::kinfo!(
                    "watch_next: UNEXPECTED status={} wid={} tid={}",
                    status,
                    id,
                    tid
                );
                return Err(Errno::EIO);
            }
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

    let reply = root_svc::enqueue(RootOp::ApplyBatch { batch });

    wait_reply_block!(reply);
    let status = reply.status.load(Ordering::Relaxed);
    let val = reply.value.load(Ordering::Relaxed);
    if status == 0 {
        return Ok(val as usize); // Returns seq
    } else {
        return Err(Errno::EINVAL);
    }
}

pub fn sys_root_watch_close(id: usize) -> SysResult<usize> {
    root_call(RootOp::WatchClose { id: id as u64 })
}

/// Bulk property fetch syscall - get multiple properties in one call
pub fn sys_root_props_get_many(
    node_id: usize,
    keys_ptr: usize,
    keys_len: usize,
    out_ptr: usize,
) -> SysResult<usize> {
    use abi::types::{BulkPropsResponse, BULK_PROPS_MAX_KEYS};

    // Validate key count
    if keys_len == 0 || keys_len > BULK_PROPS_MAX_KEYS {
        return Err(Errno::EINVAL);
    }

    // Validate input buffer (keys array)
    let keys_bytes = keys_len * core::mem::size_of::<u32>();
    validate_user_range(keys_ptr, keys_bytes, false)?;

    // Validate output buffer
    validate_user_range(out_ptr, core::mem::size_of::<BulkPropsResponse>(), true)?;

    // Copy keys from userspace
    let mut keys = alloc::vec::Vec::with_capacity(keys_len);
    for i in 0..keys_len {
        let key_ptr = keys_ptr + i * core::mem::size_of::<u32>();
        let mut key_val: u32 = 0;
        let key_slice =
            unsafe { core::slice::from_raw_parts_mut(&mut key_val as *mut u32 as *mut u8, 4) };
        unsafe { copyin(key_slice, key_ptr)? };
        keys.push(key_val);
    }

    // Allocate kernel buffer for response
    let mut kbuf = BulkPropsResponse::default();

    // Enqueue the operation with kernel buffer pointer
    let reply = root_svc::enqueue(RootOp::PropsGetMany {
        id: node_id as u64,
        keys,
        kbuf_ptr: &mut kbuf as *mut BulkPropsResponse as u64,
    });

    wait_reply_block!(reply);
    let status = reply.status.load(Ordering::Relaxed);
    let value = reply.value.load(Ordering::Relaxed);
    if status == 0 {
        // Copy response from kernel buffer to user buffer
        let src = unsafe {
            core::slice::from_raw_parts(
                &kbuf as *const BulkPropsResponse as *const u8,
                core::mem::size_of::<BulkPropsResponse>(),
            )
        };
        unsafe { copyout(out_ptr, src)? };
        return Ok(value as usize);
    } else {
        return Err(Errno::EIO);
    }
}

/// Resolve a filesystem path to a ThingId.
/// Args: path_ptr, path_len
pub fn sys_root_resolve_path(ptr: usize, len: usize) -> SysResult<usize> {
    if len > 4096 {
        return Err(Errno::EINVAL);
    }
    validate_user_range(ptr, len, false)?;

    let mut buf = alloc::vec![0u8; len];
    unsafe {
        copyin(&mut buf[..len], ptr)?;
    }

    let s = core::str::from_utf8(&buf[..len]).map_err(|_| Errno::EINVAL)?;
    let msg = RootOp::ResolvePath {
        path: String::from(s),
    };

    root_call(msg)
}

/// Truncate a bytespace to a new logical length.
/// Args: bytespace_id, new_len
pub fn sys_root_bytespace_truncate(id: usize, new_len: usize) -> SysResult<usize> {
    root_call(RootOp::BytespaceTruncate {
        id: id as u64,
        new_len: new_len as u64,
    })
}

/// Unlink an edge between two nodes.
/// Args: src_id, rel_sym, dst_id
pub fn sys_root_unlink(src: usize, rel: usize, dst: usize) -> SysResult<usize> {
    root_call(RootOp::Unlink {
        src: src as u64,
        rel: rel as u64,
        dst: dst as u64,
    })
}

/// List directory entries for a directory node.
/// Args: dir_id, out_ptr (userspace buffer), out_len (buffer size in bytes)
/// Returns: number of entries written
pub fn sys_root_dir_list(dir_id: usize, out_ptr: usize, out_len: usize) -> SysResult<usize> {
    validate_user_range(out_ptr, out_len, true)?;

    // DirEntryWire is 264 bytes. Allocate kernel buffer, capped at 4KB.
    let kbuf_len = core::cmp::min(out_len, 4096);
    let mut kbuf = alloc::vec![0u8; kbuf_len];

    let reply = root_svc::enqueue(RootOp::DirList {
        id: dir_id as u64,
        out_ptr: kbuf.as_mut_ptr() as u64,
        out_len: kbuf_len as u64,
    });

    wait_reply_block!(reply);
    let status = reply.status.load(Ordering::Relaxed);
    let count = reply.value.load(Ordering::Relaxed) as usize;
    if status == 0 {
        // Copy filled entries to userspace
        let entry_size = 264usize; // DIR_ENTRY_WIRE_SIZE
        let bytes_to_copy = count * entry_size;
        if bytes_to_copy > 0 {
            unsafe {
                copyout(out_ptr, &kbuf[..bytes_to_copy])?;
            }
        }
        return Ok(count);
    } else {
        return Err(Errno::EIO);
    }
}

pub fn sys_root_orphan_thing(thing_id: usize) -> SysResult<usize> {
    root_call(RootOp::OrphanThing {
        thing_id: thing_id as u64,
    })
}
