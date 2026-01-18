mod arch;
pub mod graph;
pub mod port;
pub mod stream;

use abi::device::{DEVICE_IRQ_SUBSCRIBE_DEVICE, DEVICE_IRQ_SUBSCRIBE_VECTOR};
use abi::errors::Errno;
pub use abi::syscall::*;
use abi::wire::{ThingId, SymbolId};

use arch::raw_syscall6;

/// Helper to expose raw syscalls safely to other modules if needed.
#[inline(always)]
pub unsafe fn syscall6(
    n: u32,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> isize {
    unsafe { raw_syscall6(n, a0, a1, a2, a3, a4, a5) }
}

// Low-level wrappers

pub fn exit(code: i32) -> ! {
    unsafe {
        raw_syscall6(SYS_EXIT, code as usize, 0, 0, 0, 0, 0);
        core::hint::unreachable_unchecked();
    }
}

pub fn log_write(msg: &str, level: usize) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_LOG_WRITE,
            msg.as_ptr() as usize,
            msg.len(),
            level,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

pub use log_write as debug_write;
pub use port::{PortHandle, port_create, port_send, port_recv, port_close, port_wait};

pub fn yield_now() {
    unsafe {
        raw_syscall6(SYS_YIELD, 0, 0, 0, 0, 0, 0);
    }
}

pub fn sleep_ns(ns: u64) {
    unsafe {
        raw_syscall6(SYS_SLEEP_NS, ns as usize, 0, 0, 0, 0, 0);
    }
}

pub fn sleep_ms(ms: u64) {
    // Legacy support, or use ns
    sleep_ns(ms * 1_000_000);
}

pub fn monotonic_ns() -> u64 {
    let ret = unsafe { raw_syscall6(SYS_TIME_MONOTONIC, 0, 0, 0, 0, 0, 0) };
    if ret < 0 {
        0
    } else {
        ret as u64
    }
}

pub fn spawn_process(name: &str, arg: usize) -> Result<u64, abi::errors::Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_SPAWN_PROCESS,
            name.as_ptr() as usize,
            name.len(),
            arg,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|v| v as u64)
}

pub fn set_priority(tid: u64, priority: usize) -> Result<(), Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_SET_PRIORITY,
            tid as usize,
            priority,
            0,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|_| ())
}

pub fn alloc_stack(pages: usize) -> Result<usize, Errno> {
    let ret = unsafe { raw_syscall6(SYS_ALLOC_STACK, pages, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret)
}

pub fn spawn_thread(entry: extern "C" fn() -> !, stack: &crate::stack::Stack) -> Result<u64, Errno> {
    let req = abi::types::SpawnThreadReq {
        entry: entry as usize,
        sp: stack.sp as usize,
        stack: stack.info,
    };
    let ret = unsafe {
        raw_syscall6(
            SYS_SPAWN_THREAD,
            &req as *const abi::types::SpawnThreadReq as usize,
            0,
            0,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|v| v as u64)
}

pub fn task_poll(pid: u64) -> Result<(abi::types::TaskStatus, i32), Errno> {
    let ret = unsafe { raw_syscall6(abi::syscall::SYS_TASK_POLL, pid as usize, 0, 0, 0, 0, 0) };
    if ret < 0 {
        abi::errors::errno(ret).map(|_| (abi::types::TaskStatus::Unknown, 0))
    } else {
        let val = ret as u64;
        let status_val = val & 0xFFFFFFFF;
        let code_val = (val >> 32) as i32;
        let status = match status_val {
            0 => abi::types::TaskStatus::Unknown,
            1 => abi::types::TaskStatus::Runnable,
            2 => abi::types::TaskStatus::Running,
            3 => abi::types::TaskStatus::Blocked,
            4 => abi::types::TaskStatus::Dead,
            _ => abi::types::TaskStatus::Unknown,
        };
        Ok((status, code_val))
    }
}

pub fn time_anchor(unix_secs: u64) {
    unsafe {
        raw_syscall6(SYS_TIME_ANCHOR, unix_secs as usize, 0, 0, 0, 0, 0);
    }
}

pub fn ioport_read(port: usize, width: usize) -> usize {
    let ret = unsafe { raw_syscall6(SYS_DEVICE_IOPORT_READ, port, width, 0, 0, 0, 0) };
    if ret < 0 {
        0
    } else {
        ret as usize
    }
}

pub fn ioport_write(port: usize, value: usize, width: usize) {
    unsafe {
        raw_syscall6(SYS_DEVICE_IOPORT_WRITE, port, value, width, 0, 0, 0);
    }
}


pub fn task_wait(tid: u64) -> Result<i32, Errno> {
    let ret = unsafe { raw_syscall6(SYS_TASK_WAIT, tid as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|v| v as i32)
}

// ...
pub fn trace_read(buf: &mut [abi::trace::TraceEvent]) -> Result<usize, Errno> {
    let ret = unsafe { match raw_syscall6(
        abi::syscall::SYS_TRACE_READ,
        buf.as_mut_ptr() as usize,
        buf.len(),
        0, 0, 0, 0
    ) {
         r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
         r => r as usize
    }};
    Ok(ret)
}

// Device MMIO and DMA syscalls

/// Claim a device from the device registry
pub fn device_claim(graph_id: &ThingId) -> Result<usize, Errno> {
    let ret = unsafe { raw_syscall6(SYS_DEVICE_CLAIM, graph_id as *const _ as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret)
}

/// Map a device MMIO BAR into memory
/// Returns the virtual address where the BAR is mapped
pub fn device_map_mmio(claim_handle: usize, bar_index: usize) -> Result<u64, Errno> {
    let ret = unsafe { raw_syscall6(SYS_DEVICE_MAP_MMIO, claim_handle, bar_index, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|v| v as u64)
}

/// Allocate DMA-safe memory for a device
/// Returns the virtual address of the allocated buffer
pub fn device_alloc_dma(claim_handle: usize, page_count: usize) -> Result<u64, Errno> {
    let ret = unsafe { raw_syscall6(SYS_DEVICE_ALLOC_DMA, claim_handle, page_count, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|v| v as u64)
}

/// Get the physical address for a DMA virtual address
pub fn device_dma_phys(virt_addr: u64) -> Result<u64, Errno> {
    let ret = unsafe { raw_syscall6(SYS_DEVICE_DMA_PHYS, virt_addr as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|v| v as u64)
}

// ============================================================================
// IRQ Subscription and Waiting
// ============================================================================

/// Subscribe the current task to receive interrupts for a given vector.
pub fn irq_subscribe(vector: u8) -> Result<(), Errno> {
    let ret = unsafe { raw_syscall6(SYS_DEVICE_IRQ_SUBSCRIBE, vector as usize, 0, DEVICE_IRQ_SUBSCRIBE_VECTOR as usize, 0, 0, 0) };
    abi::errors::errno(ret).map(|_| ())
}

pub fn device_irq_subscribe(claim_handle: usize, irq_index: u8) -> Result<(), Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_DEVICE_IRQ_SUBSCRIBE,
            claim_handle,
            irq_index as usize,
            DEVICE_IRQ_SUBSCRIBE_DEVICE as usize,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|_| ())
}

/// Wait for an interrupt on the given vector. Blocks until interrupt fires.
/// Returns the number of pending interrupts since last wait.
pub fn irq_wait(vector: u8) -> Result<u32, Errno> {
    let ret = unsafe { raw_syscall6(SYS_DEVICE_IRQ_WAIT, vector as usize, 0, DEVICE_IRQ_SUBSCRIBE_VECTOR as usize, 0, 0, 0) };
    abi::errors::errno(ret).map(|v| v as u32)
}

pub fn device_irq_wait(claim_handle: usize, irq_index: u8) -> Result<u32, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_DEVICE_IRQ_WAIT,
            claim_handle,
            irq_index as usize,
            DEVICE_IRQ_SUBSCRIBE_DEVICE as usize,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|v| v as u32)
}

// --- Root / Graph Wrappers ---

pub fn root_watch_open(spec: &abi::types::WatchSpec, out_id: &mut ThingId) -> Result<usize, Errno> {
    let ret = unsafe { match raw_syscall6(
        SYS_ROOT_WATCH_OPEN,
        spec as *const _ as usize,
        out_id as *mut _ as usize,
        0, 0, 0, 0
    ) {
        r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
        r => r as usize
    }};
    Ok(ret)
}

pub fn root_watch_next(id: &ThingId, out_seq: &mut u64, buf: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe { match raw_syscall6(
        SYS_ROOT_WATCH_NEXT,
        id as *const _ as usize,
        out_seq as *mut _ as usize,
        buf.as_mut_ptr() as usize,
        buf.len(),
        0, 0
    ) {
        r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
        r => r as usize
    }};
    Ok(ret)
}

pub fn root_watch_close(id: &ThingId) -> Result<(), Errno> {
    unsafe { match raw_syscall6(SYS_ROOT_WATCH_CLOSE, id as *const _ as usize, 0, 0, 0, 0, 0) {
        r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
        _ => {}
    }};
    Ok(())
}

pub fn root_bytespace_read(id: &ThingId, offset: usize, buf: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe { match raw_syscall6(
        SYS_ROOT_BYTESPACE_READ,
        id as *const _ as usize,
        offset,
        buf.as_mut_ptr() as usize,
        buf.len(),
        0, 0
    ) {
        r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
        r => r as usize
    }};
    Ok(ret)
}

pub fn root_bytespace_write(id: &ThingId, offset: usize, buf: &[u8]) -> Result<usize, Errno> {
    let ret = unsafe { match raw_syscall6(
        SYS_ROOT_BYTESPACE_WRITE,
        id as *const _ as usize,
        offset,
        buf.as_ptr() as usize,
        buf.len(),
        0, 0
    ) {
        r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
        r => r as usize
    }};
    Ok(ret)
}

pub fn root_bytespace_create(len: usize, flags: usize, format: usize, out_id: &mut ThingId) -> Result<usize, Errno> {
    let ret = unsafe { match raw_syscall6(
        SYS_ROOT_BYTESPACE_CREATE,
        len, flags, format,
        out_id as *mut _ as usize,
        0, 0
    ) {
        r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
        r => r as usize
    }};
    Ok(ret)
}

pub fn root_intern(name: &str, out_id: &mut SymbolId) -> Result<usize, Errno> {
    let ret = unsafe { match raw_syscall6(
        SYS_ROOT_INTERN,
        name.as_ptr() as usize,
        name.len(),
        out_id as *mut _ as usize,
        0, 0, 0
    ) {
        r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
        r => r as usize
    }};
    Ok(ret)
}

pub fn root_create_node(kind_ptr: usize, out_id: &mut ThingId) -> Result<usize, Errno> {
    let ret = unsafe { match raw_syscall6(
        SYS_ROOT_CREATE_NODE,
        kind_ptr,
        out_id as *mut _ as usize,
        0, 0, 0, 0
    ) {
        r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
        r => r as usize
    }};
    Ok(ret)
}

pub fn root_prop_set(id: &ThingId, key_ptr: usize, value: &[u8; 16]) -> Result<usize, Errno> {
    let ret = unsafe { match raw_syscall6(
        SYS_ROOT_PROP_SET,
        id as *const _ as usize,
        key_ptr,
        value as *const _ as usize,
        0, 0, 0
    ) {
        r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
        r => r as usize
    }};
    Ok(ret)
}

pub fn root_link(src: &ThingId, rel_ptr: usize, dst: &ThingId) -> Result<usize, Errno> {
    let ret = unsafe { match raw_syscall6(
        SYS_ROOT_LINK,
        src as *const _ as usize,
        rel_ptr,
        dst as *const _ as usize,
        0, 0, 0
    ) {
        r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
        r => r as usize
    }};
    Ok(ret)
}

pub fn root_query(plan_buf: &[u8], out_buf: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe { match raw_syscall6(
        SYS_ROOT_QUERY,
        plan_buf.as_ptr() as usize,
        plan_buf.len(),
        out_buf.as_mut_ptr() as usize,
        out_buf.len(),
        0, 0
    ) {
        r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
        r => r as usize
    }};
    Ok(ret)
}

pub fn root_find(kind_ptr: usize, out_buf: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe { match raw_syscall6(
        SYS_ROOT_FIND,
        kind_ptr,
        out_buf.as_mut_ptr() as usize,
        out_buf.len(),
        0, 0, 0
    ) {
        r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
        r => r as usize
    }};
    Ok(ret)
}

pub fn root_get_kind(id: &ThingId, out_kind: &mut SymbolId) -> Result<usize, Errno> {
    let ret = unsafe { match raw_syscall6(
        SYS_ROOT_GET_KIND,
        id as *const _ as usize,
        out_kind as *mut _ as usize,
        0, 0, 0, 0
    ) {
        r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
        r => r as usize
    }};
    Ok(ret)
}

pub fn root_prop_get(id: &ThingId, key_ptr: usize, out_val: &mut [u8; 16]) -> Result<usize, Errno> {
    let ret = unsafe { match raw_syscall6(
        SYS_ROOT_PROP_GET,
        id as *const _ as usize,
        key_ptr,
        out_val as *mut _ as usize,
        0, 0, 0
    ) {
        r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
        r => r as usize
    }};
    Ok(ret)
}
