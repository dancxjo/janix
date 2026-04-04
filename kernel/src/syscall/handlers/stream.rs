//! Stream IPC syscalls
//!
//! Implements SYS_STREAM_OPEN, LISTEN, READ, POLL.
//! Uses Ports underneath.

use super::port::sys_port_recv;
use crate::ipc::{get_port, Handle, HandleMode, GLOBAL_HANDLE_TABLE};
use abi::errors::{Errno, SysResult};
use alloc::collections::BTreeMap;
use spin::Mutex;

/// Registry of listeners: NodeId -> PortWriteHandle (global handle table index)
static STREAM_LISTENERS: Mutex<BTreeMap<u64, Handle>> = Mutex::new(BTreeMap::new());

pub fn sys_stream_listen(node_id: usize, port_handle: usize) -> SysResult<usize> {
    let handle = Handle(port_handle as u32);

    // Verify handle is valid and writable
    {
        let table = GLOBAL_HANDLE_TABLE.lock();
        let _ = table.get(handle, HandleMode::Write).ok_or(Errno::EBADF)?;
    }

    let mut listeners = STREAM_LISTENERS.lock();
    listeners.insert(node_id as u64, handle);

    Ok(0)
}

pub fn sys_stream_open(node_id: usize) -> SysResult<usize> {
    // 1. Find listener
    let listener_handle = {
        let listeners = STREAM_LISTENERS.lock();
        *listeners.get(&(node_id as u64)).ok_or(Errno::ENOENT)?
    };

    // 2. Create new port pair for the connection
    // sys_port_create returns (write << 16) | read
    // But we need TWO pairs for full duplex?
    // User prompts imply "Streams". Usually bidirectional.
    // Port is unidirectional.
    // "Stream API" usually implies one handle for R/W?
    // If I use a single Port, it's one way.
    // If connection is Request/Reply, we need 2 ports.

    // Simpler: `stream_open` returns a handle to a bidirectional object?
    // Our Ports are SPSC.
    // If `fontd` architecture uses `(msg_read, msg_write)`, it expects 2 ports.
    // But `sys_stream_open` returns ONE handle.
    // Does the handle represent a "Socket"?
    // If I implement a Socket in kernel, it wraps 2 ports.
    // But `SYS_STREAM_READ` takes a handle. `SYS_PORT_SEND` takes a handle.
    // If I reuse `SYS_PORT_SEND` on a Stream Handle, it must map to the Write Port.
    // If I use `SYS_STREAM_READ` on a Stream Handle, it maps to the Read Port.

    // Solution: `StreamHandle` is a virtual handle pointing to a `Stream` object,
    // which contains `read_port_id` and `write_port_id`.
    // OR: We exploit the fact that `PortHandle` is just an index.
    // Can we pack two indices into one usize? No, `read(handle)`.

    // Let's implement `Stream` in IPC.
    // Or simpler: `stream_open` returns PACKED handles (like `port_create`).
    // `(write_handle << 16) | read_handle`.
    // Userspace separates them.
    // Then `stream_read` uses the read handle, `port_send` uses write handle.
    // This leaks implementation details but fits "Start Simple" and existing `port_create` pattern.

    // Let's do that: bidirectional capability.
    // Client gets (ClientWrite, ClientRead).
    // Server gets (ServerWrite, ServerRead).
    // ClientWrite -> ServerRead.
    // ServerWrite -> ClientRead.

    // 1. Create C->S port.
    let cs_port_id = crate::ipc::create_port(4096);
    // 2. Create S->C port.
    let sc_port_id = crate::ipc::create_port(4096);

    let mut table = GLOBAL_HANDLE_TABLE.lock();

    // Client handles
    let c_write = table
        .alloc(cs_port_id, HandleMode::Write)
        .ok_or(Errno::ENOMEM)?;
    let c_read = table
        .alloc(sc_port_id, HandleMode::Read)
        .ok_or(Errno::ENOMEM)?;

    // Server handles
    let s_read = table
        .alloc(cs_port_id, HandleMode::Read)
        .ok_or(Errno::ENOMEM)?;
    let s_write = table
        .alloc(sc_port_id, HandleMode::Write)
        .ok_or(Errno::ENOMEM)?;

    drop(table); // unlock

    // 3. Send "Accept" message to Listener.
    // Message format: [Magic(u64), Mode(u64), HandleRead(u64), HandleWrite(u64)]
    // 32 bytes.
    // Magic: "STREAM_CONN" ascii bytes or similar
    // 0x53545245414D434E (STREAMCN)
    let magic: u64 = 0x53545245414D434E;

    // We need to write to `listener_handle`.
    // We can use `sys_port_send` logic but we have the handle.
    // We need the PORT from the handle.

    let l_port = {
        let table = GLOBAL_HANDLE_TABLE.lock();
        let entry = table
            .get(listener_handle, HandleMode::Write)
            .ok_or(Errno::EBADF)?;
        get_port(entry.port_id).ok_or(Errno::EBADF)?
    };

    let msg = [
        magic.to_le_bytes(),
        (s_read.0 as u64).to_le_bytes(),  // Read handle for server
        (s_write.0 as u64).to_le_bytes(), // Write handle for server
        (0u64).to_le_bytes(),             // Padding
    ]
    .concat();

    let written = l_port.send(&msg);
    if written != msg.len() {
        // Failed to send full message (listener buffer full?)
        // Cleanup? For v0, we leak or return error.
        return Err(Errno::EAGAIN);
    }

    // Return Client Packed Handles
    let packed = ((c_write.0 as usize) << 16) | (c_read.0 as usize);
    Ok(packed)
}

pub fn sys_stream_read(handle: usize, ptr: usize, len: usize) -> SysResult<usize> {
    sys_port_recv(handle, ptr, len)
}

pub fn sys_stream_poll(handle: usize, _events: usize, _timeout: usize) -> SysResult<usize> {
    // Basic poll: check if data available
    let table = GLOBAL_HANDLE_TABLE.lock();
    let entry = table
        .get(Handle(handle as u32), HandleMode::Read)
        .ok_or(Errno::EBADF)?;
    let port = get_port(entry.port_id).ok_or(Errno::EBADF)?;
    drop(table);

    if !port.is_empty() {
        Ok(1) // Readable
    } else {
        Ok(0)
    }
}
