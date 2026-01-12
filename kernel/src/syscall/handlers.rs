use super::validate::{copyin, copyout, validate_user_range};
use crate::root::{self, RootOp, SymbolShell};
use abi::device::{DeviceCall, DeviceKind};
use abi::errors::{Errno, SysResult};
use abi::symbols::{SYMBOL_REF_TAG_ID, SYMBOL_REF_TAG_STR, SymbolRefWire};
use alloc::string::String;
use core::sync::atomic::Ordering;

pub fn sys_exit(code: i32) -> SysResult<usize> {
    crate::kprintln!("SYSCALL EXIT: code={}", code);
    unsafe {
        crate::task::scheduler::exit_current(code);
    }
    Ok(0)
}

pub fn sys_log_write(ptr: usize, len: usize, level_arg: usize) -> SysResult<usize> {
    let _ = validate_user_range(ptr, len, false)?;
    if len > 2048 {
        return Err(Errno::EINVAL);
    }

    let level = match level_arg {
        1 => crate::logging::LogLevel::Error,
        2 => crate::logging::LogLevel::Warn,
        3 => crate::logging::LogLevel::Info,
        4 => crate::logging::LogLevel::Debug,
        5 => crate::logging::LogLevel::Trace,
        _ => crate::logging::LogLevel::Info, // Default
    };

    // We want to log the whole message as one event if possible.
    // Allocate a vector? Or use a fixed stack buffer.
    // 256 is too small for some logs. Let's try 512.
    // If message is longer, we might split it or truncate.
    // Given we are in kernel, stack is limited.
    // Let's alloc a vec since we are in a syscall handler (interrupts enabled? yes, usually).
    // Syscalls run in kernel task context.

    let mut buf = alloc::vec![0u8; len];
    unsafe {
        copyin(&mut buf[..len], ptr)?;
    }

    match core::str::from_utf8(&buf) {
        Ok(s) => {
            let s_trimmed = s.trim_end();

            // Allow userspace to set provenance via "SOURCE: " prefix
            let (provenance, msg_body) = if let Some(idx) = s_trimmed.find(": ") {
                let (prefix, rest) = s_trimmed.split_at(idx);
                // Simple heuristic: prefix must be reasonably short and no spaces (or limited)
                if prefix.len() < 32 && !prefix.contains(char::is_whitespace) {
                    (prefix, &rest[2..])
                } else {
                    ("user.print", s_trimmed)
                }
            } else {
                ("user.print", s_trimmed)
            };

            crate::logging::_log_event(
                crate::logging::LogMetadata {
                    level,
                    file: "userspace",
                    line: 0,
                    module: "user",
                },
                provenance,
                format_args!("{}", msg_body),
                &[], // no extra fields
                &[], // no about edges
            );
        }
        Err(_) => return Err(Errno::EINVAL),
    }

    Ok(len)
}

pub fn sys_debug_write(ptr: usize, len: usize) -> SysResult<usize> {
    sys_log_write(ptr, len, 4) // Debug level
}

pub fn sys_yield() -> SysResult<usize> {
    unsafe {
        crate::task::scheduler::yield_now_current();
    }
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
        if now >= deadline {
            break;
        }
        unsafe {
            crate::task::scheduler::yield_now_current();
        }
    }
    Ok(0)
}

pub fn sys_device_call(call_ptr: usize) -> SysResult<usize> {
    let size = core::mem::size_of::<DeviceCall>();
    validate_user_range(call_ptr, size, true)?;
    let mut call: DeviceCall = unsafe { core::mem::zeroed() };
    let slice = unsafe { core::slice::from_raw_parts_mut(&mut call as *mut _ as *mut u8, size) };
    unsafe {
        copyin(slice, call_ptr)?;
    }
    match call.kind {
        DeviceKind::RtcCmos => Err(Errno::NotSupported),
        _ => Err(Errno::NotSupported),
    }
}

pub fn sys_spawn_thread(entry: usize, stack: usize) -> SysResult<usize> {
    validate_user_range(entry, 1, false)?;
    let stack_top = if stack == 0 {
        unsafe { crate::task::scheduler::alloc_user_stack_current(0) }.ok_or(Errno::ENOMEM)?
    } else {
        validate_user_range(stack, 1, true)?;
        stack
    };

    let tid = unsafe { crate::task::scheduler::spawn_user_thread_current(entry, stack_top, 0) };
    if let Some(tid) = tid {
        Ok(tid as usize)
    } else {
        Err(Errno::EAGAIN)
    }
}

pub fn sys_alloc_stack(pages: usize) -> SysResult<usize> {
    let top =
        unsafe { crate::task::scheduler::alloc_user_stack_current(pages) }.ok_or(Errno::ENOMEM)?;
    Ok(top)
}

pub fn sys_spawn_process(name_ptr: usize, name_len: usize, arg: usize) -> SysResult<usize> {
    if name_len > 128 {
        return Err(Errno::EINVAL);
    }
    validate_user_range(name_ptr, name_len, false)?;
    let mut buf = [0u8; 128];
    unsafe {
        copyin(&mut buf[..name_len], name_ptr)?;
    }
    let name = core::str::from_utf8(&buf[..name_len]).map_err(|_| Errno::EINVAL)?;
    let tid = unsafe { crate::task::scheduler::spawn_process_current(name, arg) };
    if let Some(tid) = tid {
        Ok(tid as usize)
    } else {
        Err(Errno::ENOENT)
    }
}

pub fn sys_time_monotonic_ns() -> SysResult<usize> {
    let rt = crate::runtime_base();
    let ticks = rt.mono_ticks();
    let freq = rt.mono_freq_hz();
    let ns = (ticks as u128 * 1_000_000_000) / (freq as u128);
    Ok(ns as usize)
}

pub fn sys_time_now() -> SysResult<usize> {
    let rt = crate::runtime_base();
    let ticks = rt.mono_ticks();
    let freq = rt.mono_freq_hz();
    let mono_ns = (ticks as u128 * 1_000_000_000) / (freq as u128);
    let sys_ns = crate::time::get_system_time_ns(mono_ns as u64);
    let sys_sec = sys_ns / 1_000_000_000;
    Ok(sys_sec as usize)
}

pub fn sys_rtc_read(_out_ptr: usize) -> SysResult<usize> {
    // RTC is now handled by userspace drivers via ioport syscalls
    Err(Errno::ENODEV)
}

pub fn sys_sleep_ns(ns: u64) -> SysResult<usize> {
    let rt = crate::runtime_base();
    let freq = rt.mono_freq_hz();
    let ticks = (ns as u128 * freq as u128) / 1_000_000_000;
    let start = rt.mono_ticks();
    let deadline = start + ticks as u64;
    loop {
        let now = rt.mono_ticks();
        if now >= deadline {
            break;
        }
        unsafe {
            crate::task::scheduler::yield_now_current();
        }
    }
    Ok(0)
}

pub fn sys_get_tid() -> SysResult<usize> {
    unsafe { Ok(crate::task::scheduler::current_tid_current() as usize) }
}

pub fn sys_task_poll(pid: usize) -> SysResult<usize> {
    use abi::types::TaskStatus;

    let status_opt = unsafe { crate::task::scheduler::task_status_current(pid as u64) };

    if let Some((state, exit_code)) = status_opt {
        let (st, code) = match state {
            crate::task::TaskState::Runnable => (TaskStatus::Runnable, 0),
            crate::task::TaskState::Running => (TaskStatus::Running, 0),
            crate::task::TaskState::Blocked => (TaskStatus::Blocked, 0),
            crate::task::TaskState::Dead => (TaskStatus::Dead, exit_code.unwrap_or(0)),
        };

        // Pack: [Status: 32][Code: 32] -> actually Code usually i32.
        // Let's put Status in low 32 bits, Code in high 32 bits.
        let val = (st as u64) | ((code as u32 as u64) << 32);
        Ok(val as usize)
    } else {
        Err(Errno::ESRCH)
    }
}

// ------ Device Capabilities ------

pub fn sys_device_claim(_id: usize) -> SysResult<usize> {
    // Stub for v0.1: just return fake success or NotSupported
    // eventually checks if caller can claim device
    Err(Errno::NotSupported)
}

pub fn sys_device_map_mmio(_id: usize, _flags: usize) -> SysResult<usize> {
    // Stub
    Err(Errno::NotSupported)
}

pub fn sys_device_irq_subscribe(_id: usize) -> SysResult<usize> {
    // Stub
    Err(Errno::NotSupported)
}

pub fn sys_device_ioport(port: usize, val: usize, write: bool, width: usize) -> SysResult<usize> {
    // x86 only implementation example
    #[cfg(target_arch = "x86_64")]
    unsafe {
        if write {
            match width {
                1 => core::arch::asm!("out dx, al", in("dx") port as u16, in("al") val as u8),
                2 => core::arch::asm!("out dx, ax", in("dx") port as u16, in("ax") val as u16),
                4 => core::arch::asm!("out dx, eax", in("dx") port as u16, in("eax") val as u32),
                _ => return Err(Errno::EINVAL),
            }
            return Ok(0);
        } else {
            let mut ret: usize = 0;
            match width {
                1 => {
                    let v: u8;
                    core::arch::asm!("in al, dx", out("al") v, in("dx") port as u16);
                    ret = v as usize;
                }
                2 => {
                    let v: u16;
                    core::arch::asm!("in ax, dx", out("ax") v, in("dx") port as u16);
                    ret = v as usize;
                }
                4 => {
                    let v: u32;
                    core::arch::asm!("in eax, dx", out("eax") v, in("dx") port as u16);
                    ret = v as usize;
                }
                _ => return Err(Errno::EINVAL),
            }
            return Ok(ret);
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = (port, val, write, width);
        Err(Errno::ENOSYS)
    }
}

// ------ Root Syscalls ------

fn root_call(op: RootOp) -> SysResult<usize> {
    let reply = root::enqueue(op);
    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            let status = reply.status.load(Ordering::Relaxed);
            let value = reply.value.load(Ordering::Relaxed);

            // Debug trace for garbage values
            // if value > 1000000 {
            crate::ktrace!("ROOT_CALL_DEBUG: status={} value={:x}", status, value);
            // }

            return if status == 0 {
                Ok(value as usize)
            } else {
                Err(Errno::EIO)
            };
        }
        unsafe {
            crate::task::scheduler::yield_now_current();
        }
    }
}

fn read_symbol(ptr: usize) -> SysResult<SymbolShell> {
    let size = core::mem::size_of::<SymbolRefWire>();
    validate_user_range(ptr, size, false)?;

    let mut wire: SymbolRefWire = unsafe { core::mem::zeroed() };
    let slice = unsafe { core::slice::from_raw_parts_mut(&mut wire as *mut _ as *mut u8, size) };
    unsafe {
        copyin(slice, ptr)?;
    }

    // crate::kprintln!("SYSCALL: read_symbol wire: tag={} ptr={:x} len={}", wire.tag, wire.ptr_or_id, wire.len); // DEBUG

    match wire.tag {
        SYMBOL_REF_TAG_ID => Ok(SymbolShell::Id(wire.ptr_or_id as u32)),
        SYMBOL_REF_TAG_STR => {
            let s_ptr = wire.ptr_or_id as usize;
            let s_len = wire.len as usize;
            if s_len > 256 {
                crate::kprintln!("SYSCALL: Symbol string too long: {}", s_len);
                return Err(Errno::EINVAL);
            }

            if let Err(e) = validate_user_range(s_ptr, s_len, false) {
                crate::kprintln!(
                    "SYSCALL: Symbol string ptr {:x} len {} validation failed: {:?}",
                    s_ptr,
                    s_len,
                    e
                );
                return Err(e);
            }
            let mut buf = [0u8; 256];
            unsafe {
                copyin(&mut buf[..s_len], s_ptr)?;
            }

            match core::str::from_utf8(&buf[..s_len]) {
                Ok(s) => Ok(SymbolShell::Str(String::from(s))),
                Err(_) => {
                    crate::kprintln!("SYSCALL: Symbol string invalid utf8");
                    Err(Errno::EINVAL)
                }
            }
        }
        _ => {
            crate::kprintln!("SYSCALL: Unknown symbol tag: {}", wire.tag);
            Err(Errno::EINVAL)
        }
    }
}

pub fn sys_root_get_kind(id: usize) -> SysResult<usize> {
    root_call(RootOp::GetKind { id: id as u64 })
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

    // Chunked read because we use a stack buffer for copyout
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
        } // EOF or error

        // Copyout
        unsafe {
            copyout(curr_ptr, &kbuf[..res])?;
        }

        curr_offset += res;
        curr_ptr += res;
        total_read += res;
        remaining -= res;

        if res < chunk_len {
            break;
        } // Partial read implies end
    }

    Ok(total_read)
}

pub fn sys_root_watch_subscribe(target: usize, mask: usize) -> SysResult<usize> {
    root_call(RootOp::WatchSubscribe {
        target_id: target as u64,
        mask: mask as u64,
    })
}

pub fn sys_root_stream_poll(stream: usize, max: usize, out_ptr: usize) -> SysResult<usize> {
    validate_user_range(out_ptr, max, true)?;

    let evt_size = core::mem::size_of::<abi::types::RootWatchEvent>();
    if max < evt_size {
        return Err(Errno::EINVAL);
    }

    let reply = root::enqueue(RootOp::StreamPoll {
        stream_id: stream as u64,
        max,
        out_ptr: out_ptr as u64,
    });

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

                let src =
                    unsafe { core::slice::from_raw_parts(&evt as *const _ as *const u8, evt_size) };
                unsafe {
                    copyout(out_ptr, src)?;
                }

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

pub fn sys_root_prop_set(id: usize, key_ptr: usize, value: usize) -> SysResult<usize> {
    let key = read_symbol(key_ptr)?;
    root_call(RootOp::PropSet {
        id: id as u64,
        key,
        value: value as u64,
    })
}

pub fn sys_root_describe_thing(id: usize, out_ptr: usize, len: usize) -> SysResult<usize> {
    validate_user_range(out_ptr, len, true)?;
    let mut kbuf = [0u8; 256];
    let kbuf_len = core::cmp::min(len, kbuf.len());
    let reply = root::enqueue(RootOp::DescribeThing {
        id: id as u64,
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
    let reply = root::enqueue(RootOp::DescribeEdge {
        src: src as u64,
        rel,
        dst: dst as u64,
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

pub fn sys_root_link(src: usize, rel_ptr: usize, dst: usize) -> SysResult<usize> {
    let rel = read_symbol(rel_ptr)?;
    let reply = root::enqueue(RootOp::Link {
        src: src as u64,
        rel,
        dst: dst as u64,
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

pub fn sys_root_dump_edges(id: usize, out_ptr: usize, len: usize) -> SysResult<usize> {
    validate_user_range(out_ptr, len, true)?;
    let mut kbuf = [0u8; 1024];
    let kbuf_len = core::cmp::min(len, kbuf.len());
    let reply = root::enqueue(RootOp::DumpEdges {
        id: id as u64,
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

pub fn sys_root_intern(ptr: usize, len: usize) -> SysResult<usize> {
    if len > 256 {
        return Err(Errno::EINVAL);
    }
    validate_user_range(ptr, len, false)?;

    let mut buf = [0u8; 256];
    unsafe {
        copyin(&mut buf[..len], ptr)?;
    }

    let s = core::str::from_utf8(&buf[..len]).map_err(|_| Errno::EINVAL)?;
    let msg = RootOp::Intern {
        name: alloc::string::String::from(s),
    };

    root_call(msg)
}

pub fn sys_root_prop_get(id: usize, ptr: usize, _reserved: usize) -> SysResult<usize> {
    let sym = read_symbol(ptr)?;
    let msg = RootOp::PropGet {
        id: id as u64,
        key: sym,
    };
    root_call(msg)
}

pub fn sys_root_find(ptr_kind: usize, ptr_buf: usize, len: usize) -> SysResult<usize> {
    let sym = read_symbol(ptr_kind)?;

    validate_user_range(ptr_buf, len, true)?;

    // Limit max buffer size to avoid OOM
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

    let bytes_to_copy = core::cmp::min(count * 8, len);
    unsafe {
        copyout(ptr_buf, &kbuf[..bytes_to_copy])?;
    }

    Ok(count)
}

pub fn sys_root_create_node(kind_ptr: usize) -> SysResult<usize> {
    let sym = read_symbol(kind_ptr)?;
    root_call(RootOp::CreateNode { kind: sym })
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
                    name: alloc::string::String::from(s),
                };
                let id = root_call(intern_msg)?;
                id as u32
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

pub fn sys_root_dump_graph(limit: usize) -> SysResult<usize> {
    root_call(RootOp::DumpGraph {
        limit: limit as u64,
    })
}

pub fn sys_time_anchor(unix_secs: u64) -> SysResult<usize> {
    let rt = crate::runtime_base();
    let ticks = rt.mono_ticks();
    let freq = rt.mono_freq_hz();
    let mono_ns = (ticks as u128 * 1_000_000_000) / (freq as u128);
    crate::time::anchor_system_clock(unix_secs, mono_ns as u64);
    Ok(0)
}

// ---- Blocking Wait Syscalls ----

pub fn sys_task_wait(tid: usize) -> SysResult<usize> {
    use abi::types::TaskStatus;
    
    // Blocking wait for task to exit
    // Returns: exit code on success (low 32 bits), ECHILD if not found
    loop {
        let status_opt = unsafe { crate::task::scheduler::task_status_current(tid as u64) };
        
        match status_opt {
            Some((state, exit_code)) => {
                if state == crate::task::TaskState::Dead {
                    // Task has exited, return its exit code
                    return Ok(exit_code.unwrap_or(0) as usize);
                }
                // Task still running, yield and try again
                unsafe {
                    crate::task::scheduler::yield_now_current();
                }
            }
            None => {
                // Task not found
                return Err(abi::errors::Errno::ECHILD);
            }
        }
    }
}

// ------ Port IPC Syscalls ------

/// Global handle tables per-process (simplified: single global table for v0)
/// TODO: Move to per-process handle tables in task/scheduler
static PORT_HANDLE_TABLE: spin::Mutex<crate::ipc::HandleTable> = 
    spin::Mutex::new(crate::ipc::HandleTable::new());

/// Create a new port (sprout-only for v0)
pub fn sys_port_create(capacity: usize) -> SysResult<usize> {
    // TODO: restrict to sprout via capability check
    let capacity = capacity.min(65536).max(64);
    let port_id = crate::ipc::create_port(capacity);
    
    // Allocate both read and write handles for this port
    let mut table = PORT_HANDLE_TABLE.lock();
    let write_handle = table.alloc(port_id, crate::ipc::HandleMode::Write)
        .ok_or(Errno::ENOMEM)?;
    let read_handle = table.alloc(port_id, crate::ipc::HandleMode::Read)
        .ok_or(Errno::ENOMEM)?;
    
    // Return packed: (write_handle << 16) | read_handle
    let packed = ((write_handle.0 as usize) << 16) | (read_handle.0 as usize);
    Ok(packed)
}

/// Send bytes to a port via handle
pub fn sys_port_send(handle: usize, ptr: usize, len: usize) -> SysResult<usize> {
    let len = len.min(4096);
    if len == 0 {
        return Ok(0);
    }
    
    validate_user_range(ptr, len, false)?;
    
    let handle = crate::ipc::Handle(handle as u32);
    let entry = {
        let table = PORT_HANDLE_TABLE.lock();
        table.get(handle, crate::ipc::HandleMode::Write)
            .copied()
            .ok_or(Errno::EBADF)?
    };
    
    let port = crate::ipc::get_port(entry.port_id)
        .ok_or(Errno::EBADF)?;
    
    // Copy data from userspace
    let mut buf = [0u8; 4096];
    unsafe {
        copyin(&mut buf[..len], ptr)?;
    }
    
    let written = port.send(&buf[..len]);
    Ok(written)
}

/// Receive bytes from a port via handle
pub fn sys_port_recv(handle: usize, ptr: usize, len: usize) -> SysResult<usize> {
    let len = len.min(4096);
    if len == 0 {
        return Ok(0);
    }
    
    validate_user_range(ptr, len, true)?;
    
    let handle = crate::ipc::Handle(handle as u32);
    let entry = {
        let table = PORT_HANDLE_TABLE.lock();
        table.get(handle, crate::ipc::HandleMode::Read)
            .copied()
            .ok_or(Errno::EBADF)?
    };
    
    let port = crate::ipc::get_port(entry.port_id)
        .ok_or(Errno::EBADF)?;
    
    let mut buf = [0u8; 4096];
    let read = port.recv(&mut buf[..len]);
    
    if read > 0 {
        unsafe {
            copyout(ptr, &buf[..read])?;
        }
    }
    
    Ok(read)
}

/// Close a port handle
pub fn sys_port_close(handle: usize) -> SysResult<usize> {
    let handle = crate::ipc::Handle(handle as u32);
    let mut table = PORT_HANDLE_TABLE.lock();
    if table.close(handle) {
        Ok(0)
    } else {
        Err(Errno::EBADF)
    }
}
