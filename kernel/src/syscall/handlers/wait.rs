use super::root_call;
use crate::syscall::validate::validate_user_range;
use abi::errors::{Errno, SysResult};
use abi::wait::{self, WaitKind, WaitResult, WaitSpec};
use core::mem::size_of;

#[derive(Clone, Copy)]
enum Registration {
    PortRead(crate::ipc::PortId),
    PortWrite(crate::ipc::PortId),
    Watch(u64),
    TaskExit(u64),
}

pub fn sys_wait_many(
    specs_ptr: usize,
    spec_count: usize,
    results_ptr: usize,
    results_cap: usize,
    timeout_ns: u64,
) -> SysResult<usize> {
    if spec_count == 0 || spec_count > wait::WAIT_MANY_MAX_ITEMS {
        return Err(Errno::EINVAL);
    }
    if results_cap == 0 || results_cap > wait::WAIT_MANY_MAX_ITEMS {
        return Err(Errno::EINVAL);
    }

    validate_user_range(specs_ptr, spec_count * size_of::<WaitSpec>(), false)?;
    validate_user_range(results_ptr, results_cap * size_of::<WaitResult>(), true)?;

    let mut specs = [WaitSpec::default(); wait::WAIT_MANY_MAX_ITEMS];
    unsafe {
        let dst = core::slice::from_raw_parts_mut(
            specs.as_mut_ptr() as *mut u8,
            spec_count * size_of::<WaitSpec>(),
        );
        super::copyin(dst, specs_ptr)?;
    }
    let specs = &specs[..spec_count];

    for spec in specs {
        if WaitKind::from_u32(spec.kind).is_none() {
            return Err(Errno::EINVAL);
        }
    }

    let tid = unsafe { crate::sched::current_tid_current() };
    let timeout_tick = if timeout_ns == u64::MAX {
        None
    } else {
        let ticks = timeout_ns.saturating_add(9_999_999) / 10_000_000;
        Some(crate::sched::TICK_COUNT.load(core::sync::atomic::Ordering::Relaxed) + ticks)
    };

    loop {
        let mut results = [WaitResult::default(); wait::WAIT_MANY_MAX_ITEMS];
        let ready = collect_ready(specs, &mut results[..results_cap])?;
        if ready > 0 {
            unsafe {
                let src = core::slice::from_raw_parts(
                    results.as_ptr() as *const u8,
                    ready * size_of::<WaitResult>(),
                );
                super::copyout(results_ptr, src)?;
            }
            return Ok(ready);
        }

        if timeout_expired(timeout_tick) {
            let result = WaitResult {
                kind: WaitKind::Timeout as u32,
                flags: wait::ready::TIMEOUT,
                object: 0,
                token: 0,
                value: 0,
                reserved: 0,
            };
            unsafe {
                let src = core::slice::from_raw_parts(
                    &result as *const _ as *const u8,
                    size_of::<WaitResult>(),
                );
                super::copyout(results_ptr, src)?;
            }
            return Ok(1);
        }

        let regs = register_all(specs, tid)?;
        if let Some(deadline) = timeout_tick {
            crate::sched::register_timeout_wake_current(tid, deadline);
        }

        let mut results = [WaitResult::default(); wait::WAIT_MANY_MAX_ITEMS];
        let ready = collect_ready(specs, &mut results[..results_cap])?;
        if ready > 0 || timeout_expired(timeout_tick) {
            cleanup_all(&regs, tid, timeout_tick)?;
            let count = if ready > 0 {
                ready
            } else {
                results[0] = WaitResult {
                    kind: WaitKind::Timeout as u32,
                    flags: wait::ready::TIMEOUT,
                    object: 0,
                    token: 0,
                    value: 0,
                    reserved: 0,
                };
                1
            };
            unsafe {
                let src = core::slice::from_raw_parts(
                    results.as_ptr() as *const u8,
                    count * size_of::<WaitResult>(),
                );
                super::copyout(results_ptr, src)?;
            }
            return Ok(count);
        }

        unsafe {
            crate::sched::block_current_erased();
        }
        cleanup_all(&regs, tid, timeout_tick)?;
    }
}

fn timeout_expired(timeout_tick: Option<u64>) -> bool {
    match timeout_tick {
        Some(deadline) => {
            crate::sched::TICK_COUNT.load(core::sync::atomic::Ordering::Relaxed) >= deadline
        }
        None => false,
    }
}

fn collect_ready(specs: &[WaitSpec], out: &mut [WaitResult]) -> SysResult<usize> {
    let mut count = 0usize;
    for spec in specs {
        if count >= out.len() {
            break;
        }
        if let Some(result) = poll_spec(spec)? {
            out[count] = result;
            count += 1;
        }
    }
    Ok(count)
}

fn poll_spec(spec: &WaitSpec) -> SysResult<Option<WaitResult>> {
    match WaitKind::from_u32(spec.kind).ok_or(Errno::EINVAL)? {
        WaitKind::Port => poll_port(spec),
        WaitKind::RootWatch => poll_watch(spec),
        WaitKind::TaskExit => poll_task_exit(spec),
        WaitKind::Irq => Ok(poll_irq(spec)),
        WaitKind::Timeout => Err(Errno::EINVAL),
    }
}

fn error_result(spec: &WaitSpec, errno: Errno) -> WaitResult {
    WaitResult {
        kind: spec.kind,
        flags: wait::ready::ERROR,
        object: spec.object,
        token: spec.token,
        value: errno as i64,
        reserved: 0,
    }
}

fn poll_port(spec: &WaitSpec) -> SysResult<Option<WaitResult>> {
    let handle = crate::ipc::Handle(spec.object as u32);
    let table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
    let mut ready_flags = 0u32;
    let mut value = 0i64;

    if (spec.flags & wait::interest::READABLE) != 0 {
        match table.get(handle, crate::ipc::HandleMode::Read).copied() {
            Some(entry) => {
                if let Some(port) = crate::ipc::get_port(entry.port_id) {
                    if !port.is_empty() {
                        ready_flags |= wait::ready::READABLE;
                        value = port.len() as i64;
                    } else if !port.has_writers() {
                        ready_flags |= wait::ready::HANGUP;
                    }
                } else {
                    return Ok(Some(error_result(spec, Errno::EBADF)));
                }
            }
            None => return Ok(Some(error_result(spec, Errno::EBADF))),
        }
    }

    if (spec.flags & wait::interest::WRITABLE) != 0 {
        match table.get(handle, crate::ipc::HandleMode::Write).copied() {
            Some(entry) => {
                if let Some(port) = crate::ipc::get_port(entry.port_id) {
                    if !port.is_full() {
                        ready_flags |= wait::ready::WRITABLE;
                        value = port.available() as i64;
                    } else if !port.has_readers() {
                        ready_flags |= wait::ready::HANGUP;
                    }
                } else {
                    return Ok(Some(error_result(spec, Errno::EBADF)));
                }
            }
            None => return Ok(Some(error_result(spec, Errno::EBADF))),
        }
    }

    if ready_flags == 0 {
        Ok(None)
    } else {
        Ok(Some(WaitResult {
            kind: spec.kind,
            flags: ready_flags,
            object: spec.object,
            token: spec.token,
            value,
            reserved: 0,
        }))
    }
}

fn poll_watch(spec: &WaitSpec) -> SysResult<Option<WaitResult>> {
    let value = root_call(crate::root::RootOp::WatchPoll { id: spec.object })?;
    let mut flags = 0u32;
    if (value & 1) != 0 {
        flags |= wait::ready::READABLE;
    }
    if (value & 2) != 0 {
        flags |= wait::ready::OVERFLOW;
    }
    if flags == 0 {
        Ok(None)
    } else {
        Ok(Some(WaitResult {
            kind: spec.kind,
            flags,
            object: spec.object,
            token: spec.token,
            value: 0,
            reserved: 0,
        }))
    }
}

fn poll_task_exit(spec: &WaitSpec) -> SysResult<Option<WaitResult>> {
    match unsafe { crate::sched::poll_task_exit_current(spec.object) } {
        Ok(Some(code)) => Ok(Some(WaitResult {
            kind: spec.kind,
            flags: wait::ready::EXITED,
            object: spec.object,
            token: spec.token,
            value: code as i64,
            reserved: 0,
        })),
        Ok(None) => Ok(None),
        Err(err) => Ok(Some(error_result(spec, err))),
    }
}

fn poll_irq(spec: &WaitSpec) -> Option<WaitResult> {
    if spec.object > u8::MAX as u64 {
        return Some(error_result(spec, Errno::EINVAL));
    }
    match crate::irq::poll(spec.object as u8) {
        Some(0) => None,
        Some(count) => Some(WaitResult {
            kind: spec.kind,
            flags: wait::ready::IRQ,
            object: spec.object,
            token: spec.token,
            value: count as i64,
            reserved: 0,
        }),
        None => Some(error_result(spec, Errno::ENODEV)),
    }
}

fn register_all(specs: &[WaitSpec], tid: u64) -> SysResult<alloc::vec::Vec<Registration>> {
    let mut regs = alloc::vec::Vec::new();
    for spec in specs {
        match WaitKind::from_u32(spec.kind).ok_or(Errno::EINVAL)? {
            WaitKind::Port => {
                let handle = crate::ipc::Handle(spec.object as u32);
                let table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
                if (spec.flags & wait::interest::READABLE) != 0 {
                    if let Some(entry) = table.get(handle, crate::ipc::HandleMode::Read).copied() {
                        if let Some(port) = crate::ipc::get_port(entry.port_id) {
                            port.add_waiter_read(tid);
                            regs.push(Registration::PortRead(entry.port_id));
                        }
                    }
                }
                if (spec.flags & wait::interest::WRITABLE) != 0 {
                    if let Some(entry) = table.get(handle, crate::ipc::HandleMode::Write).copied() {
                        if let Some(port) = crate::ipc::get_port(entry.port_id) {
                            port.add_waiter_write(tid);
                            regs.push(Registration::PortWrite(entry.port_id));
                        }
                    }
                }
            }
            WaitKind::RootWatch => {
                root_call(crate::root::RootOp::WatchRegisterWaiter {
                    id: spec.object,
                    tid,
                })?;
                regs.push(Registration::Watch(spec.object));
            }
            WaitKind::TaskExit => {
                match unsafe { crate::sched::register_task_exit_waiter_current(spec.object, tid) } {
                    Ok(Some(_)) | Ok(None) => regs.push(Registration::TaskExit(spec.object)),
                    Err(_) => {}
                }
            }
            WaitKind::Irq => {}
            WaitKind::Timeout => return Err(Errno::EINVAL),
        }
    }
    Ok(regs)
}

fn cleanup_all(regs: &[Registration], tid: u64, timeout_tick: Option<u64>) -> SysResult<()> {
    for reg in regs {
        match *reg {
            Registration::PortRead(port_id) => {
                if let Some(port) = crate::ipc::get_port(port_id) {
                    port.remove_waiter_read(tid);
                }
            }
            Registration::PortWrite(port_id) => {
                if let Some(port) = crate::ipc::get_port(port_id) {
                    port.remove_waiter_write(tid);
                }
            }
            Registration::Watch(id) => {
                let _ = root_call(crate::root::RootOp::WatchUnregisterWaiter { id, tid });
            }
            Registration::TaskExit(target) => {
                let _ = unsafe { crate::sched::unregister_task_exit_waiter_current(target, tid) };
            }
        }
    }
    if timeout_tick.is_some() {
        crate::sched::unregister_timeout_wake_current(tid);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn alloc_port_pair(capacity: usize) -> (u32, u32) {
        let port_id = crate::ipc::create_port(capacity);
        let mut table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
        let write = table
            .alloc(port_id, crate::ipc::HandleMode::Write)
            .expect("write handle");
        let read = table
            .alloc(port_id, crate::ipc::HandleMode::Read)
            .expect("read handle");
        (write.0, read.0)
    }

    #[test]
    fn poll_port_reports_readable_and_hangup() {
        let (write_handle, read_handle) = alloc_port_pair(64);
        let port = {
            let table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
            let entry = table
                .get(
                    crate::ipc::Handle(write_handle),
                    crate::ipc::HandleMode::Write,
                )
                .copied()
                .expect("entry");
            crate::ipc::get_port(entry.port_id).expect("port")
        };

        assert!(port.send_all(b"abc"));
        let readable = poll_spec(&WaitSpec {
            kind: WaitKind::Port as u32,
            flags: wait::interest::READABLE,
            object: read_handle as u64,
            token: 11,
        })
        .expect("poll")
        .expect("ready");
        assert_ne!(readable.flags & wait::ready::READABLE, 0);
        assert_eq!(readable.token, 11);

        assert!(!port.close_writer());
        let hangup = poll_spec(&WaitSpec {
            kind: WaitKind::Port as u32,
            flags: wait::interest::READABLE,
            object: read_handle as u64,
            token: 12,
        })
        .expect("poll")
        .expect("ready");
        assert_ne!(hangup.flags & wait::ready::READABLE, 0);

        let mut drain = [0u8; 8];
        assert_eq!(port.try_recv(&mut drain), 3);

        let hangup_only = poll_spec(&WaitSpec {
            kind: WaitKind::Port as u32,
            flags: wait::interest::READABLE,
            object: read_handle as u64,
            token: 13,
        })
        .expect("poll")
        .expect("ready");
        assert_eq!(hangup_only.flags, wait::ready::HANGUP);
    }

    #[test]
    fn collect_ready_returns_multiple_ports() {
        let (write_a, read_a) = alloc_port_pair(64);
        let (write_b, read_b) = alloc_port_pair(64);

        let port_a = {
            let table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
            let entry = table
                .get(crate::ipc::Handle(write_a), crate::ipc::HandleMode::Write)
                .copied()
                .expect("entry a");
            crate::ipc::get_port(entry.port_id).expect("port a")
        };
        let port_b = {
            let table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
            let entry = table
                .get(crate::ipc::Handle(write_b), crate::ipc::HandleMode::Write)
                .copied()
                .expect("entry b");
            crate::ipc::get_port(entry.port_id).expect("port b")
        };

        assert!(port_a.send_all(b"a"));
        assert!(port_b.send_all(b"bb"));

        let specs = [
            WaitSpec {
                kind: WaitKind::Port as u32,
                flags: wait::interest::READABLE,
                object: read_a as u64,
                token: 1,
            },
            WaitSpec {
                kind: WaitKind::Port as u32,
                flags: wait::interest::READABLE,
                object: read_b as u64,
                token: 2,
            },
        ];
        let mut results = [WaitResult::default(); 2];
        let ready = collect_ready(&specs, &mut results).expect("collect");
        assert_eq!(ready, 2);
        assert_eq!(results[0].token, 1);
        assert_eq!(results[1].token, 2);
    }
}
