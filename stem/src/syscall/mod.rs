mod arch;
pub mod graph;
pub mod pipe;
pub mod port;
pub mod stream;
pub mod vfs;
pub mod wait;

use abi::device::{DEVICE_IRQ_SUBSCRIBE_DEVICE, DEVICE_IRQ_SUBSCRIBE_VECTOR};
use abi::errors::Errno;
pub use abi::syscall::*;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

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

/// Reboot the system. This call does not return.
pub fn reboot() -> ! {
    unsafe {
        raw_syscall6(SYS_REBOOT, 0, 0, 0, 0, 0, 0);
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

pub fn read(fd: usize, buf: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe { raw_syscall6(SYS_READ, fd, buf.as_mut_ptr() as usize, buf.len(), 0, 0, 0) };
    abi::errors::errno(ret)
}

pub fn write(fd: usize, buf: &[u8]) -> Result<usize, Errno> {
    let ret = unsafe { raw_syscall6(SYS_WRITE, fd, buf.as_ptr() as usize, buf.len(), 0, 0, 0) };
    abi::errors::errno(ret)
}

pub use port::{
    port_close, port_create, port_recv, port_send, port_send_all, port_try_recv, port_wait,
    topic_create, topic_publish, topic_subscribe, PortHandle,
};
pub use vfs::{vfs_close, vfs_mkdir, vfs_mount, vfs_open, vfs_read, vfs_readdir, vfs_umount, vfs_unlink, vfs_write};
pub use wait::wait_many;

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

pub fn get_tid() -> Result<u64, Errno> {
    let ret = unsafe { raw_syscall6(SYS_GET_TID, 0, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|v| v as u64)
}

/// Get the current process ID.
pub fn getpid() -> u32 {
    let ret = unsafe { raw_syscall6(SYS_GETPID, 0, 0, 0, 0, 0, 0) };
    ret as u32
}

/// Get the parent process ID.
pub fn getppid() -> u32 {
    let ret = unsafe { raw_syscall6(SYS_GETPPID, 0, 0, 0, 0, 0, 0) };
    ret as u32
}

/// Retrieve the process argv into `buf`. Returns total bytes needed.
/// First call with an empty/small buffer to learn the size, then retry.
pub fn argv_get(buf: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_ARGV_GET,
            buf.as_mut_ptr() as usize,
            buf.len(),
            0,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

/// Get a single environment variable by key. Returns value length needed.
pub fn env_get(key: &[u8], val: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_ENV_GET,
            key.as_ptr() as usize,
            key.len(),
            val.as_mut_ptr() as usize,
            val.len(),
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

/// Set an environment variable.
pub fn env_set(key: &[u8], val: &[u8]) -> Result<(), Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_ENV_SET,
            key.as_ptr() as usize,
            key.len(),
            val.as_ptr() as usize,
            val.len(),
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|_| ())
}

/// Remove an environment variable.
pub fn env_unset(key: &[u8]) -> Result<(), Errno> {
    let ret = unsafe { raw_syscall6(SYS_ENV_UNSET, key.as_ptr() as usize, key.len(), 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|_| ())
}

/// List all environment variables. Returns total bytes needed.
pub fn env_list(buf: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_ENV_LIST,
            buf.as_mut_ptr() as usize,
            buf.len(),
            0,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
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

pub fn spawn_process_ex(
    name: &str,
    argv: &[&[u8]],
    env: &BTreeMap<Vec<u8>, Vec<u8>>,
    stdin_mode: u32,
    stdout_mode: u32,
    stderr_mode: u32,
) -> Result<abi::types::SpawnProcessExResp, Errno> {
    let argv_blob = serialize_argv(argv);
    let env_blob = serialize_env(env);

    let req = abi::types::SpawnProcessExReq {
        name_ptr: name.as_ptr() as u64,
        name_len: name.len() as u32,
        _pad0: 0,
        argv_ptr: argv_blob.as_ptr() as u64,
        argv_len: argv_blob.len() as u32,
        _pad1: 0,
        env_ptr: env_blob.as_ptr() as u64,
        env_len: env_blob.len() as u32,
        _pad2: 0,
        stdin_mode,
        stdout_mode,
        stderr_mode,
        _reserved: 0,
    };

    let mut resp = abi::types::SpawnProcessExResp::default();
    let ret = unsafe {
        raw_syscall6(
            SYS_SPAWN_PROCESS_EX,
            &req as *const _ as usize,
            &mut resp as *mut _ as usize,
            0,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|_| resp)
}

fn serialize_argv(argv: &[&[u8]]) -> Vec<u8> {
    let mut blob = Vec::new();
    blob.extend_from_slice(&(argv.len() as u32).to_le_bytes());
    for arg in argv {
        blob.extend_from_slice(&(arg.len() as u32).to_le_bytes());
        blob.extend_from_slice(arg);
    }
    blob
}

fn serialize_env(env: &BTreeMap<Vec<u8>, Vec<u8>>) -> Vec<u8> {
    let mut blob = Vec::new();
    blob.extend_from_slice(&(env.len() as u32).to_le_bytes());
    for (key, value) in env {
        blob.extend_from_slice(&(key.len() as u32).to_le_bytes());
        blob.extend_from_slice(key);
        blob.extend_from_slice(&(value.len() as u32).to_le_bytes());
        blob.extend_from_slice(value);
    }
    blob
}

pub fn set_priority(tid: u64, priority: usize) -> Result<(), Errno> {
    let ret = unsafe { raw_syscall6(SYS_SET_PRIORITY, tid as usize, priority, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|_| ())
}

pub fn alloc_stack(pages: usize) -> Result<usize, Errno> {
    let ret = unsafe { raw_syscall6(SYS_ALLOC_STACK, pages, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret)
}

pub fn spawn_thread(entry: usize, arg: usize, stack: &crate::stack::Stack) -> Result<u64, Errno> {
    let req = abi::types::SpawnThreadReq {
        entry,
        sp: stack.sp as usize,
        arg,
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

/// Kill a task by TID. Returns Ok(()) if the task was killed, Err(ESRCH) if not found.
pub fn task_kill(tid: u64) -> Result<(), Errno> {
    let ret = unsafe { raw_syscall6(SYS_TASK_KILL, tid as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|_| ())
}

/// Dump all task information to the kernel serial console (like `top`).
pub fn task_dump() {
    unsafe {
        raw_syscall6(SYS_TASK_DUMP, 0, 0, 0, 0, 0, 0);
    }
}

// ...
pub fn trace_read(buf: &mut [abi::trace::TraceEvent]) -> Result<usize, Errno> {
    let ret = unsafe {
        match raw_syscall6(
            abi::syscall::SYS_TRACE_READ,
            buf.as_mut_ptr() as usize,
            buf.len(),
            0,
            0,
            0,
            0,
        ) {
            r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
            r => r as usize,
        }
    };
    Ok(ret)
}

/// Disable the boot console (call when compositor takes over framebuffer)
pub fn console_disable() {
    unsafe {
        raw_syscall6(SYS_CONSOLE_DISABLE, 0, 0, 0, 0, 0, 0);
    }
}

// Device MMIO and DMA syscalls

/// Claim a device from the device registry
pub fn device_claim(graph_id: u64) -> Result<usize, Errno> {
    let ret = unsafe { raw_syscall6(SYS_DEVICE_CLAIM, graph_id as usize, 0, 0, 0, 0, 0) };
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
/// For PS/2 keyboard: vector 0x21 (IRQ1), PS/2 mouse: vector 0x2C (IRQ12)
pub fn irq_subscribe(vector: u8) -> Result<(), Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_DEVICE_IRQ_SUBSCRIBE,
            vector as usize,
            0,
            DEVICE_IRQ_SUBSCRIBE_VECTOR as usize,
            0,
            0,
            0,
        )
    };
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
    let ret = unsafe {
        raw_syscall6(
            SYS_DEVICE_IRQ_WAIT,
            vector as usize,
            0,
            DEVICE_IRQ_SUBSCRIBE_VECTOR as usize,
            0,
            0,
            0,
        )
    };
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

pub fn root_watch_open(spec: &abi::types::WatchSpec) -> Result<usize, Errno> {
    // [DEPRECATION GUARDRAIL]
    // Check for WATCH_START_LATEST usage and warn once.
    // Assuming abi::types::WATCH_START_LATEST is available.
    if spec.start_seq == abi::types::WATCH_START_LATEST {
        static WARNED: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
        if !WARNED.swap(true, core::sync::atomic::Ordering::Relaxed) {
            let _ = log_write("WARN: root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.", 2);
        }
    }

    let ret = unsafe {
        match raw_syscall6(
            SYS_ROOT_WATCH_OPEN,
            spec as *const _ as usize,
            0,
            0,
            0,
            0,
            0,
        ) {
            r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
            r => r as usize,
        }
    };
    Ok(ret)
}

/// Blocking watch read.
///
/// Parks the calling task until a matching graph commit is available, then
/// returns the number of bytes written to `out`.  Unlike `root_watch_try_next`
/// this never returns `Err(Errno::EAGAIN)` — it blocks instead.
///
/// Use `root_watch_try_next` in drain loops where the caller has already
/// established readiness via [`WaitSet::wait`](crate::wait_set::WaitSet) or
/// `SYS_WAIT_MANY`.
pub fn root_watch_next(id: usize, seq_out: &mut u64, out: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        match raw_syscall6(
            SYS_ROOT_WATCH_NEXT,
            id,
            seq_out as *mut _ as usize,
            out.as_mut_ptr() as usize,
            out.len(),
            0,
            0,
        ) {
            r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
            r => r as usize,
        }
    };
    Ok(ret)
}

/// Non-blocking watch read.
///
/// Returns `Err(Errno::EAGAIN)` immediately when no matching event is pending.
/// Prefer this inside drain loops that follow a
/// [`WaitSet::wait`](crate::wait_set::WaitSet) / `SYS_WAIT_MANY` call, so
/// the loop terminates quickly once all queued events have been consumed.
pub fn root_watch_try_next(
    id: usize,
    seq_out: &mut u64,
    out: &mut [u8],
) -> Result<usize, Errno> {
    let ret = unsafe {
        match raw_syscall6(
            SYS_ROOT_WATCH_TRY_NEXT,
            id,
            seq_out as *mut _ as usize,
            out.as_mut_ptr() as usize,
            out.len(),
            0,
            0,
        ) {
            r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
            r => r as usize,
        }
    };
    Ok(ret)
}

pub fn root_watch_close(id: usize) -> Result<(), Errno> {
    unsafe {
        match raw_syscall6(SYS_ROOT_WATCH_CLOSE, id, 0, 0, 0, 0, 0) {
            r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
            _ => {}
        }
    };
    Ok(())
}

pub fn root_bytespace_read(id: usize, offset: usize, buf: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        match raw_syscall6(
            SYS_ROOT_BYTESPACE_READ,
            id,
            offset,
            buf.as_mut_ptr() as usize,
            buf.len(),
            0,
            0,
        ) {
            r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
            r => r as usize,
        }
    };
    Ok(ret)
}

pub fn root_bytespace_write(id: usize, offset: usize, buf: &[u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        match raw_syscall6(
            SYS_ROOT_BYTESPACE_WRITE,
            id,
            offset,
            buf.as_ptr() as usize,
            buf.len(),
            0,
            0,
        ) {
            r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
            r => r as usize,
        }
    };
    Ok(ret)
}

pub fn root_bytespace_create(len: usize, flags: usize, format: usize) -> Result<usize, Errno> {
    let ret = unsafe {
        match raw_syscall6(SYS_ROOT_BYTESPACE_CREATE, len, flags, format, 0, 0, 0) {
            r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
            r => r as usize,
        }
    };
    Ok(ret)
}

pub fn root_resolve_path(path: &str) -> Result<u64, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_ROOT_RESOLVE_PATH,
            path.as_ptr() as usize,
            path.len(),
            0,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|v| v as u64)
}

pub fn root_dir_list(dir_id: u64, out: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_ROOT_DIR_LIST,
            dir_id as usize,
            out.as_mut_ptr() as usize,
            out.len(),
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

pub fn root_intern(name: &str) -> Result<usize, Errno> {
    let ret = unsafe {
        match raw_syscall6(
            SYS_ROOT_INTERN,
            name.as_ptr() as usize,
            name.len(),
            0,
            0,
            0,
            0,
        ) {
            r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
            r => r as usize,
        }
    };
    Ok(ret)
}

pub fn root_create_node(kind_ptr: usize) -> Result<usize, Errno> {
    let ret = unsafe {
        match raw_syscall6(SYS_ROOT_CREATE_NODE, kind_ptr, 0, 0, 0, 0, 0) {
            r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
            r => r as usize,
        }
    };
    Ok(ret)
}

pub fn root_prop_set(id: usize, key_ptr: usize, value: usize) -> Result<usize, Errno> {
    let ret = unsafe {
        match raw_syscall6(SYS_ROOT_PROP_SET, id, key_ptr, value, 0, 0, 0) {
            r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
            r => r as usize,
        }
    };
    Ok(ret)
}

pub fn root_link(src: usize, rel_ptr: usize, dst: usize) -> Result<usize, Errno> {
    let ret = unsafe {
        match raw_syscall6(SYS_ROOT_LINK, src, rel_ptr, dst, 0, 0, 0) {
            r if r < 0 => return Err(core::mem::transmute(-(r as i32))),
            r => r as usize,
        }
    };
    Ok(ret)
}

// ============================================================================
// Entropy / Random
// ============================================================================

/// Fill `buf` with random bytes from the kernel entropy pool.
pub fn getrandom(buf: &mut [u8]) -> Result<(), Errno> {
    let mut offset = 0;
    while offset < buf.len() {
        let chunk = &mut buf[offset..];
        let ret = unsafe {
            raw_syscall6(
                SYS_GETRANDOM,
                chunk.as_mut_ptr() as usize,
                chunk.len(),
                0,
                0,
                0,
                0,
            )
        };
        if ret < 0 {
            return Err(unsafe { core::mem::transmute(-(ret as i32)) });
        }
        // Kernel caps at 256 bytes per call, advance by the amount requested
        // (we know the kernel filled min(len, 256))
        let filled = chunk.len().min(256);
        offset += filled;
    }
    Ok(())
}
