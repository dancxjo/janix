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
    let _ = (plan_ptr, plan_len, out_ptr, out_cap);
    Err(Errno::ENOSYS)
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
    Err(Errno::ENOSYS)
}

pub fn sys_root_get_edges(id: usize, out_ptr: usize, len: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
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

pub fn sys_root_watch_subscribe(target: usize, mask: usize) -> SysResult<usize> {
    let _ = (target, mask);
    Err(Errno::ENOSYS)
}

pub fn sys_root_bytespace_map(id: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

pub fn sys_root_bytespace_unmap(id: usize, user_va: usize) -> SysResult<usize> {
    let _ = (id, user_va);
    Err(Errno::ENOSYS)
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
    let _ = id;
    Err(Errno::ENOSYS)
}
pub fn sys_root_watch_open(spec_ptr: usize) -> SysResult<usize> {
    let _ = spec_ptr;
    Err(Errno::ENOSYS)
}

/// Blocking watch read.  Parks the calling task on the watch's wait-queue until
/// a matching graph commit arrives, then copies the event payload to userspace.
///
/// Returns:
/// - `Ok(n)`: `n` bytes written to `out_ptr`; sequence number written to `out_seq_ptr`.
/// - `Err(EOVERFLOW)`: some commits were missed; caller should resync.
/// - `Err(ENOSPC)`: the provided buffer is too small for the pending event.
/// - `Err(EBADF)`: the watch handle is invalid or has been closed.
pub fn sys_root_watch_next(
    id: usize,
    out_seq_ptr: usize,
    out_ptr: usize,
    out_len: usize,
) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}

/// Non-blocking watch read.  Returns `Err(EAGAIN)` immediately when no
/// matching event is pending.  Prefer this in drain loops that follow a
/// `WaitSet::wait` / `SYS_WAIT_MANY` call that has already confirmed
/// readiness.
pub fn sys_root_watch_try_next(
    id: usize,
    out_seq_ptr: usize,
    out_ptr: usize,
    out_len: usize,
) -> SysResult<usize> {
    Err(Errno::ENOSYS)
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
    let _ = id;
    Err(Errno::ENOSYS)
}

/// Bulk property fetch syscall - get multiple properties in one call
pub fn sys_root_props_get_many(
    node_id: usize,
    keys_ptr: usize,
    keys_len: usize,
    out_ptr: usize,
) -> SysResult<usize> {
    use abi::types::{BULK_PROPS_MAX_KEYS, BulkPropsResponse};

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
    let _ = (id, new_len);
    Err(Errno::ENOSYS)
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
