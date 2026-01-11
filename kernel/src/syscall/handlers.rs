use abi::errors::{Errno, SysResult};
use abi::device::{DeviceCall, DeviceKind};
use super::validate::{validate_user_range, copyin, copyout};
use crate::root::{self, RootOp, SymbolShell};
use abi::symbols::{SymbolRefWire, SYMBOL_REF_TAG_ID, SYMBOL_REF_TAG_STR};
use core::sync::atomic::Ordering;
use alloc::string::String;

// ... existing syscalls ... 
// I'll just paste the whole file with updates to Root syscalls.

pub fn sys_exit(code: i32) -> SysResult<usize> {
    crate::kprintln!("SYSCALL EXIT: code={}", code);
    unsafe { crate::task::scheduler::exit_current(code); }
    Ok(0)
}

pub fn sys_debug_write(ptr: usize, len: usize) -> SysResult<usize> {
    let _ = validate_user_range(ptr, len, false)?;
    if len > 1024 { return Err(Errno::EINVAL); }
    let mut buf = [0u8; 128];
    let mut offset = 0;
    while offset < len {
        let chunk_len = core::cmp::min(len - offset, buf.len());
        unsafe { copyin(&mut buf[..chunk_len], ptr + offset)?; }
        if let Ok(s) = core::str::from_utf8(&buf[..chunk_len]) {
             crate::kprint!("{}", s);
        } else {
             crate::kprint!("<invalid utf8>");
        }
        offset += chunk_len;
    }
    Ok(len)
}

pub fn sys_yield() -> SysResult<usize> {
    unsafe { crate::task::scheduler::yield_now_current(); }
    Ok(0)
}

pub fn sys_sleep_ms(ms: u64) -> SysResult<usize> {
    let rt = crate::runtime_base();
    let freq = rt.mono_freq_hz();
    let ticks = (ms * freq) / 1000;
    let start = rt.mono_ticks();
    let deadline = start + ticks;
    loop {
        let now = rt.mono_ticks();
        if now >= deadline { break; }
        unsafe { crate::task::scheduler::yield_now_current(); }
    }
    Ok(0)
}

pub fn sys_device_call(call_ptr: usize) -> SysResult<usize> {
    let size = core::mem::size_of::<DeviceCall>();
    validate_user_range(call_ptr, size, true)?;
    let mut call: DeviceCall = unsafe { core::mem::zeroed() };
    let slice = unsafe { core::slice::from_raw_parts_mut(&mut call as *mut _ as *mut u8, size) };
    unsafe { copyin(slice, call_ptr)?; }
    match call.kind {
        DeviceKind::RtcCmos => Err(Errno::NotSupported),
        _ => Err(Errno::NotSupported)
    }
}

pub fn sys_spawn_thread(entry: usize, stack: usize) -> SysResult<usize> {
    validate_user_range(entry, 1, false)?;
    validate_user_range(stack, 1, true)?;
    let tid = unsafe { crate::task::scheduler::spawn_user_thread_current(entry, stack, 0) };
    if let Some(tid) = tid { Ok(tid as usize) } else { Err(Errno::EAGAIN) }
}

pub fn sys_spawn_process(name_ptr: usize, name_len: usize) -> SysResult<usize> {
    if name_len > 128 { return Err(Errno::EINVAL); }
    validate_user_range(name_ptr, name_len, false)?;
    let mut buf = [0u8; 128];
    unsafe { copyin(&mut buf[..name_len], name_ptr)?; }
    let name = core::str::from_utf8(&buf[..name_len]).map_err(|_| Errno::EINVAL)?;
    let tid = unsafe { crate::task::scheduler::spawn_process_current(name) };
    if let Some(tid) = tid { Ok(tid as usize) } else { Err(Errno::ENOENT) }
}

pub fn sys_time_monotonic_ns() -> SysResult<usize> {
    let rt = crate::runtime_base();
    let ticks = rt.mono_ticks();
    let freq = rt.mono_freq_hz();
    let ns = (ticks as u128 * 1_000_000_000) / (freq as u128);
    Ok(ns as usize)
}

pub fn sys_rtc_read(out_ptr: usize) -> SysResult<usize> {
    validate_user_range(out_ptr, core::mem::size_of::<abi::device::RtcTime>(), true)?;
    let rt = crate::runtime_base();
    if let Some(time) = rt.read_rtc() {
         let src = unsafe { core::slice::from_raw_parts(&time as *const _ as *const u8, core::mem::size_of::<abi::device::RtcTime>()) };
         unsafe { copyout(out_ptr, src)?; }
         Ok(0)
    } else {
         Err(Errno::ENODEV)
    }
}

pub fn sys_sleep_ns(ns: u64) -> SysResult<usize> {
    let rt = crate::runtime_base();
    let freq = rt.mono_freq_hz();
    let ticks = (ns as u128 * freq as u128) / 1_000_000_000;
    let start = rt.mono_ticks();
    let deadline = start + ticks as u64;
    loop {
         let now = rt.mono_ticks();
         if now >= deadline { break; }
         unsafe { crate::task::scheduler::yield_now_current(); }
    }
    Ok(0)
}

pub fn sys_get_tid() -> SysResult<usize> {
    unsafe { Ok(crate::task::scheduler::current_tid_current() as usize) }
}

// ------ Root Syscalls ------

fn root_call(op: RootOp) -> SysResult<usize> {
    let reply = root::enqueue(op);
    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            let status = reply.status.load(Ordering::Relaxed);
            let value = reply.value.load(Ordering::Relaxed);
            return if status == 0 {
                Ok(value as usize)
            } else {
                Err(Errno::EIO) 
            };
        }
        unsafe { crate::task::scheduler::yield_now_current(); }
    }
}

fn read_symbol(ptr: usize) -> SysResult<SymbolShell> {
    let size = core::mem::size_of::<SymbolRefWire>();
    validate_user_range(ptr, size, false)?;
    
    let mut wire: SymbolRefWire = unsafe { core::mem::zeroed() };
    let slice = unsafe { core::slice::from_raw_parts_mut(&mut wire as *mut _ as *mut u8, size) };
    unsafe { copyin(slice, ptr)?; }
    
    match wire.tag {
        SYMBOL_REF_TAG_ID => Ok(SymbolShell::Id(wire.ptr_or_id as u32)),
        SYMBOL_REF_TAG_STR => {
            let s_ptr = wire.ptr_or_id as usize;
            let s_len = wire.len as usize;
            if s_len > 256 { return Err(Errno::EINVAL); } // Max len
            
            validate_user_range(s_ptr, s_len, false)?;
            let mut buf = [0u8; 256]; // Stack buffer for copy
            unsafe { copyin(&mut buf[..s_len], s_ptr)?; }
            
            let s = core::str::from_utf8(&buf[..s_len]).map_err(|_| Errno::EINVAL)?;
            Ok(SymbolShell::Str(String::from(s)))
        },
        _ => Err(Errno::EINVAL),
    }
}

pub fn sys_root_get_kind(id: usize) -> SysResult<usize> {
    root_call(RootOp::GetKind { id: id as u64 })
}

pub fn sys_root_bytespace_create(len: usize, flags: usize, format: usize) -> SysResult<usize> {
    root_call(RootOp::BytespaceCreate { len: len as u64, flags: flags as u64, format: format as u64 })
}

pub fn sys_root_watch_subscribe(target: usize, mask: usize) -> SysResult<usize> {
    root_call(RootOp::WatchSubscribe { target_id: target as u64, mask: mask as u64 })
}

pub fn sys_root_stream_poll(stream: usize, max: usize, out_ptr: usize) -> SysResult<usize> {
    validate_user_range(out_ptr, max, true)?;
    
    let evt_size = core::mem::size_of::<abi::types::RootWatchEvent>();
    if max < evt_size {
        return Err(Errno::EINVAL);
    }
    
    let reply = root::enqueue(RootOp::StreamPoll { stream_id: stream as u64, max, out_ptr: out_ptr as u64 });
    
    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
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
                 unsafe { copyout(out_ptr, src)?; }
                 
                 return Ok(1);
            } else {
                 return Ok(0);
            }
        }
        unsafe { crate::task::scheduler::yield_now_current(); }
    }
}

pub fn sys_root_prop_set(id: usize, key_ptr: usize, value: usize) -> SysResult<usize> {
    let key = read_symbol(key_ptr)?;
    root_call(RootOp::PropSet { id: id as u64, key, value: value as u64 })
}

pub fn sys_root_describe_thing(id: usize, out_ptr: usize, len: usize) -> SysResult<usize> {
    validate_user_range(out_ptr, len, true)?;
    let mut kbuf = [0u8; 256];
    let kbuf_len = core::cmp::min(len, kbuf.len());
    let reply = root::enqueue(RootOp::DescribeThing { 
        id: id as u64, 
        buffer: kbuf.as_mut_ptr() as u64, 
        len: kbuf_len as u64 
    });
    
    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            let status = reply.status.load(Ordering::Relaxed);
            let written = reply.value.load(Ordering::Relaxed) as usize;
            if status == 0 {
                unsafe { copyout(out_ptr, &kbuf[..written])?; }
                return Ok(written);
            } else {
                 return Err(Errno::EIO);
            }
        }
        unsafe { crate::task::scheduler::yield_now_current(); }
    }
}

pub fn sys_root_describe_edge(src: usize, rel_ptr: usize, dst: usize, out_ptr: usize, len: usize) -> SysResult<usize> {
    validate_user_range(out_ptr, len, true)?;
    let rel = read_symbol(rel_ptr)?;

    let mut kbuf = [0u8; 512];
    let kbuf_len = core::cmp::min(len, kbuf.len());
    let reply = root::enqueue(RootOp::DescribeEdge { 
        src: src as u64, 
        rel, 
        dst: dst as u64,
        buffer: kbuf.as_mut_ptr() as u64, 
        len: kbuf_len as u64 
    });
    
    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            let status = reply.status.load(Ordering::Relaxed);
            let written = reply.value.load(Ordering::Relaxed) as usize;
            if status == 0 {
                unsafe { copyout(out_ptr, &kbuf[..written])?; }
                return Ok(written);
            } else {
                 return Err(Errno::EIO);
            }
        }
        unsafe { crate::task::scheduler::yield_now_current(); }
    }
}

pub fn sys_root_link(src: usize, rel_ptr: usize, dst: usize) -> SysResult<usize> {
     let rel = read_symbol(rel_ptr)?;
     let reply = root::enqueue(RootOp::Link { 
        src: src as u64, 
        rel, 
        dst: dst as u64
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
        unsafe { crate::task::scheduler::yield_now_current(); }
    }
}

pub fn sys_root_dump_edges(id: usize, out_ptr: usize, len: usize) -> SysResult<usize> {
    validate_user_range(out_ptr, len, true)?;
    let mut kbuf = [0u8; 1024];
    let kbuf_len = core::cmp::min(len, kbuf.len());
    let reply = root::enqueue(RootOp::DumpEdges { 
        id: id as u64,
        buffer: kbuf.as_mut_ptr() as u64, 
        len: kbuf_len as u64 
    });
    
    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            let status = reply.status.load(Ordering::Relaxed);
            let written = reply.value.load(Ordering::Relaxed) as usize;
            if status == 0 {
                unsafe { copyout(out_ptr, &kbuf[..written])?; }
                return Ok(written);
            } else {
                 return Err(Errno::EIO);
            }
        }
        unsafe { crate::task::scheduler::yield_now_current(); }
    }
}

pub fn sys_root_intern(ptr: usize, len: usize) -> SysResult<usize> {
    if len > 256 { return Err(Errno::EINVAL); }
    validate_user_range(ptr, len, false)?;
    
    let mut buf = [0u8; 256];
    unsafe { copyin(&mut buf[..len], ptr)?; }
    
    let s = core::str::from_utf8(&buf[..len]).map_err(|_| Errno::EINVAL)?;
    let msg = RootOp::Intern { name: alloc::string::String::from(s) };
    
    root_call(msg)
}


pub fn sys_root_prop_get(id: usize, ptr: usize, _reserved: usize) -> SysResult<usize> {
    // id: ThingId
    // ptr: *const SymbolRefWire
    let sym = read_symbol(ptr)?;
    // prop_get returns value in result
    // RootOp::PropGet returns (0, value) or (-1, 0)
    let msg = RootOp::PropGet { id: id as u64, key: sym };
    
    // root_call returns Result<usize, Errno>
    // We want the value directly.
    // root_call maps (0, val) to Ok(val as usize)
    // and (-1, _) to Err(Errno::ENOENT)
    
    // Wait, root_call implementation:
    // Ok(reply_value as usize)
    // Err(Errno::EIO) if status != 0
    // I should check root_call in handlers.rs or service interaction?
    // root_call is in handlers.rs I think? Or generic helper?
    // Let's rely on standard logic.
    root_call(msg)
}

pub fn sys_root_find(ptr_kind: usize, ptr_buf: usize, len: usize) -> SysResult<usize> {
    // ptr_kind: *const SymbolRefWire
    // ptr_buf: *mut ThingId (u64 array)
    // len: bytes length of buffer
    let sym = read_symbol(ptr_kind)?;
    validate_user_range(ptr_buf, len, true)?;
    
    // We need a kernel buffer for output? Or can Root service write directly to user memory?
    // Root service runs in kernel thread. It can access physical memory if mapped.
    // But generic RootOp usually writes to kernel virtual address provided in op.
    // So we need copyout.
    
    if len > 4096 { return Err(Errno::EINVAL); }
    let mut kbuf = [0u8; 4096];
    
    let msg = RootOp::Find { kind: sym, buffer: kbuf.as_mut_ptr() as u64, len: len as u64 };
    
    // root_call returns count of found items
    let count = root_call(msg)?;
    
    // Copy back
    // count is number of items found.
    // But we only wrote up to min(count, buffer_capacity) items into kbuf.
    // We should copy min(count * 8, len).
    let bytes_to_copy = core::cmp::min(count * 8, len);
    unsafe { copyout(ptr_buf, &kbuf[..bytes_to_copy])?; }
    
    Ok(count)
}

pub fn sys_root_create_node(kind_ptr: usize) -> SysResult<usize> {
    let sym = read_symbol(kind_ptr)?;
    root_call(RootOp::CreateNode { kind: sym })
}

pub fn sys_root_query(plan_ptr: usize, plan_len: usize, out_ptr: usize, out_cap: usize) -> SysResult<usize> {
    use abi::query::{QueryStep, QueryRow};
    use crate::root::query::PreparedStep;
    
    // Bounds check plan
    let step_size = core::mem::size_of::<QueryStep>();
    let total_plan_bytes = plan_len * step_size;
    validate_user_range(plan_ptr, total_plan_bytes, false)?;
    
    // Copy in plan
    // We limit plan size (e.g. 8 steps)
    if plan_len > 8 { return Err(Errno::EINVAL); }
    
    let mut steps = alloc::vec::Vec::with_capacity(plan_len);
    for i in 0..plan_len {
        let ptr = plan_ptr + i * step_size;
        let mut step: QueryStep = unsafe { core::mem::zeroed() };
        let slice = unsafe { core::slice::from_raw_parts_mut(&mut step as *mut _ as *mut u8, step_size) };
        unsafe { copyin(slice, ptr)?; }
        
        // Resolve symbols NOW (in user context)
        // Similar to read_symbol logic but specialized
        let sym_id = match step.symbol.tag {
            abi::symbols::SYMBOL_REF_TAG_ID => step.symbol.ptr_or_id as u32,
            abi::symbols::SYMBOL_REF_TAG_STR => {
                 let s_ptr = step.symbol.ptr_or_id as usize;
                 let s_len = step.symbol.len as usize;
                 if s_len > 256 { return Err(Errno::EINVAL); }
                 validate_user_range(s_ptr, s_len, false)?;
                 let mut buf = [0u8; 256];
                 unsafe { copyin(&mut buf[..s_len], s_ptr)?; }
                 let s = core::str::from_utf8(&buf[..s_len]).map_err(|_| Errno::EINVAL)?;
                 // Syscall must Intern?
                 // But Intern is a RootOp. We can't easily intern synchronously inside a syscall 
                 // without sending a message (which we are doing).
                 // So we can send Intern op first? Or support String in PreparedStep?
                 // RootOp::Query takes PreparedStep with SymbolId.
                 // So we MUST have an ID.
                 // Hack: Trigger Intern for each string first.
                 let intern_msg = RootOp::Intern { name: alloc::string::String::from(s) };
                 // We call root_call(intern) -> ID
                 let id = root_call(intern_msg)?;
                 id as u32
            },
            _ => return Err(Errno::EINVAL),
        };
        
        steps.push(PreparedStep {
            op: step.op,
            arg1: step.arg1,
            symbol: sym_id,
        });
    }

    // Validate out buffer
    let row_size = core::mem::size_of::<QueryRow>();
    let total_out_bytes = out_cap * row_size;
    validate_user_range(out_ptr, total_out_bytes, true)?;
    
    // Allocate kernel buffer for result
    // We cap output size (e.g. 4KB or 1024 rows)
    let safe_cap = core::cmp::min(out_cap, 1024);
    let mut kbuf = alloc::vec![QueryRow::default(); safe_cap];
    
    let msg = RootOp::Query { 
        plan: steps, 
        out_buffer: kbuf.as_mut_ptr() as u64, 
        out_len: (safe_cap * row_size) as u64 
    };
    
    let count = root_call(msg)?;
    
    // Copy out
    let bytes_to_copy = count * row_size;
    let src = unsafe { core::slice::from_raw_parts(kbuf.as_ptr() as *const u8, bytes_to_copy) };
    unsafe { copyout(out_ptr, src)?; }
    
    Ok(count)
}
