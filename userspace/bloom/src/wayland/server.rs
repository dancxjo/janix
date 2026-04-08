use crate::wayland::protocol::{decode_header, MessageBuilder};
use abi::vfs_rpc::{DirentWire, VfsRpcOp, VfsRpcReqHeader, VFS_RPC_MAX_REQ, VFS_RPC_MAX_RESP};
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};
use stem::syscall::{port_create, port_send, port_try_recv, vfs_mount, PortHandle};

static NEXT_HANDLE: AtomicU32 = AtomicU32::new(10);

#[derive(Clone, Debug, PartialEq)]
pub enum WaylandObject {
    Display,
    Registry,
    Compositor,
    Shm,
    ShmPool {
        bs_id: u64,
        size: u32,
    },
    Buffer {
        pool_id: u32,
        offset: u32,
        width: u32,
        height: u32,
        stride: u32,
        format: u32,
    },
    Surface {
        buffer_id: Option<u32>,
    },
}

pub struct ClientConnection {
    pub handle: u64,
    pub pending_read_port: Option<u32>,
    pub out_buf: Vec<u8>,
    pub in_buf: Vec<u8>,
    pub objects: BTreeMap<u32, WaylandObject>,
}

impl ClientConnection {
    pub fn new(handle: u64) -> Self {
        let mut objects = BTreeMap::new();
        objects.insert(1, WaylandObject::Display); // object 1 is always the display
        Self {
            handle,
            pending_read_port: None,
            out_buf: Vec::new(),
            in_buf: Vec::new(),
            objects,
        }
    }

    pub fn flush_read_if_pending(&mut self) {
        if self.out_buf.is_empty() {
            return;
        }
        if let Some(resp_port) = self.pending_read_port.take() {
            let mut resp = Vec::with_capacity(1 + 4 + self.out_buf.len());
            resp.push(0); // OK status
            resp.extend_from_slice(&(self.out_buf.len() as u32).to_le_bytes());
            resp.extend_from_slice(&self.out_buf);
            self.out_buf.clear();
            let _ = port_send(resp_port, &resp);
        }
    }
}

pub struct WaylandServer {
    pub req_port: PortHandle,
    pub clients: BTreeMap<u64, ClientConnection>,
    // For bloom compositor integration: surfaces mapped by buffer IDs
    pub committed_surfaces: Vec<crate::wayland::server::WaylandSurfaceCommit>,
}

pub struct WaylandSurfaceCommit {
    pub client_handle: u64,
    pub surface_id: u32,
    pub bs_id: u64,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
}

impl WaylandServer {
    pub fn new() -> Option<Self> {
        let (req_write, req_read) = port_create(65536).ok()?;
        stem::syscall::vfs_mount(req_write, "/run/wayland-0").ok()?;

        Some(Self {
            req_port: req_read,
            clients: BTreeMap::new(),
            committed_surfaces: Vec::new(),
        })
    }

    pub fn pump(&mut self) {
        let mut buf = [0u8; VFS_RPC_MAX_REQ];
        while let Ok(len) = port_try_recv(self.req_port, &mut buf) {
            if len < core::mem::size_of::<VfsRpcReqHeader>() {
                continue;
            }
            let mut hdr_bytes = [0u8; core::mem::size_of::<VfsRpcReqHeader>()];
            hdr_bytes.copy_from_slice(&buf[..core::mem::size_of::<VfsRpcReqHeader>()]);
            let hdr: VfsRpcReqHeader = unsafe { core::mem::transmute(hdr_bytes) };
            let payload = &buf[core::mem::size_of::<VfsRpcReqHeader>()..len];

            self.handle_vfs_rpc(hdr, payload);
        }
    }

    fn handle_vfs_rpc(&mut self, hdr: VfsRpcReqHeader, payload: &[u8]) {
        match VfsRpcOp::from_u8(hdr.op) {
            Some(VfsRpcOp::Lookup) => {
                // Return a new client handle
                let handle = NEXT_HANDLE.fetch_add(1, Ordering::SeqCst) as u64;
                self.clients.insert(handle, ClientConnection::new(handle));

                let mut resp = alloc::vec![0u8; 9];
                resp[0] = 0; // OK
                resp[1..9].copy_from_slice(&handle.to_le_bytes());
                let _ = port_send(hdr.resp_port, &resp);
                stem::info!("Wayland: New client connected. Handle={}", handle);
            }
            Some(VfsRpcOp::Read) => {
                if payload.len() < 12 {
                    return;
                }
                let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());

                if let Some(client) = self.clients.get_mut(&handle) {
                    if client.out_buf.is_empty() {
                        // Pend the read
                        client.pending_read_port = Some(hdr.resp_port);
                    } else {
                        // Fulfill immediately
                        let mut resp = Vec::with_capacity(1 + 4 + client.out_buf.len());
                        resp.push(0); // OK
                        resp.extend_from_slice(&(client.out_buf.len() as u32).to_le_bytes());
                        resp.extend_from_slice(&client.out_buf);
                        client.out_buf.clear();
                        let _ = port_send(hdr.resp_port, &resp);
                    }
                } else {
                    let _ = port_send(hdr.resp_port, &[abi::errors::Errno::EBADF as u8]);
                }
            }
            Some(VfsRpcOp::Write) => {
                if payload.len() < 12 {
                    return;
                }
                let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());
                // let offset = u64::from_le_bytes(payload[8..16].try_into().unwrap());
                let data_len = u32::from_le_bytes(payload[16..20].try_into().unwrap()) as usize;

                if payload.len() < 20 + data_len {
                    return;
                }
                let data = &payload[20..20 + data_len];

                // Send response early (write successful)
                let mut resp = alloc::vec![0u8; 5];
                resp[0] = 0; // OK
                resp[1..5].copy_from_slice(&(data.len() as u32).to_le_bytes());
                let _ = port_send(hdr.resp_port, &resp);

                if let Some(client) = self.clients.get_mut(&handle) {
                    client.in_buf.extend_from_slice(data);
                    self.process_client_input(handle);
                }
            }
            Some(VfsRpcOp::Stat) => {
                if payload.len() < 8 {
                    return;
                }
                let mut resp = alloc::vec![0u8; 21];
                let mode: u32 = 0o020000 | 0o666; // S_IFCHR
                resp[0] = 0; // OK
                resp[1..5].copy_from_slice(&mode.to_le_bytes());
                resp[5..13].copy_from_slice(&0u64.to_le_bytes()); // size
                resp[13..21].copy_from_slice(&1u64.to_le_bytes()); // ino
                let _ = port_send(hdr.resp_port, &resp);
            }
            Some(VfsRpcOp::Close) => {
                if payload.len() < 8 {
                    return;
                }
                let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());
                self.clients.remove(&handle);
                let _ = port_send(hdr.resp_port, &[0u8]);
                stem::info!("Wayland: Client {} disconnected", handle);
            }
            _ => {
                let _ = port_send(hdr.resp_port, &[abi::errors::Errno::ENOSYS as u8]);
            }
        }
    }

    fn process_client_input(&mut self, handle: u64) {
        let mut consumed = 0;
        let mut surfaces_to_commit = Vec::new();

        {
            let client = self.clients.get_mut(&handle).unwrap();
            while client.in_buf.len() - consumed >= 8 {
                let buf = &client.in_buf[consumed..];
                let (obj_id, opcode, size) = decode_header(buf);
                if buf.len() < size as usize {
                    break;
                }

                let payload = &buf[8..size as usize];
                stem::info!(
                    "Wayland: Client {} Obj={} Op={} Size={}",
                    handle,
                    obj_id,
                    opcode,
                    size
                );

                let obj_clone = client.objects.get(&obj_id).cloned();

                match obj_clone {
                    Some(WaylandObject::Display) => {
                        if opcode == 1 {
                            // get_registry (new_id)
                            let new_id = u32::from_ne_bytes(payload[0..4].try_into().unwrap());
                            client.objects.insert(new_id, WaylandObject::Registry);

                            // Send .global event for compositor
                            let mut mb = MessageBuilder::new(new_id, 0); // opcode 0: global
                            mb.push_u32(1); // name
                            mb.push_string("wl_compositor");
                            mb.push_u32(1); // version
                            client.out_buf.extend_from_slice(&mb.build());

                            // Send .global event for shm
                            let mut mb2 = MessageBuilder::new(new_id, 0);
                            mb2.push_u32(2); // name
                            mb2.push_string("wl_shm");
                            mb2.push_u32(1); // version
                            client.out_buf.extend_from_slice(&mb2.build());
                        }
                    }
                    Some(WaylandObject::Registry) => {
                        if opcode == 0 {
                            // bind (name, string, version, new_id)
                            let name = u32::from_ne_bytes(payload[0..4].try_into().unwrap());
                            // Simple parsing: skip string, read new_id at end
                            let new_id_offset = payload.len() - 4;
                            let new_id =
                                u32::from_ne_bytes(payload[new_id_offset..].try_into().unwrap());

                            if name == 1 {
                                client.objects.insert(new_id, WaylandObject::Compositor);
                            } else if name == 2 {
                                client.objects.insert(new_id, WaylandObject::Shm);
                                // Emit shm.format (0 = ARGB8888, 1 = XRGB8888)
                                let mut mb = MessageBuilder::new(new_id, 0); // format event
                                mb.push_u32(0); // ARGB8888
                                client.out_buf.extend_from_slice(&mb.build());
                            }
                        }
                    }
                    Some(WaylandObject::Compositor) => {
                        if opcode == 0 {
                            // create_surface (new_id)
                            let new_id = u32::from_ne_bytes(payload[0..4].try_into().unwrap());
                            client
                                .objects
                                .insert(new_id, WaylandObject::Surface { buffer_id: None });
                        }
                    }
                    Some(WaylandObject::Shm) => {
                        if opcode == 0 {
                            // create_pool (new_id, fd, size)
                            let new_id = u32::from_ne_bytes(payload[0..4].try_into().unwrap());
                            let fd = u32::from_ne_bytes(payload[4..8].try_into().unwrap()); // fd is bs_id!
                            let size = u32::from_ne_bytes(payload[8..12].try_into().unwrap());
                            client.objects.insert(
                                new_id,
                                WaylandObject::ShmPool {
                                    bs_id: fd as u64,
                                    size,
                                },
                            );
                            stem::info!(
                                "Wayland: created shm_pool={}, bs_id={}, size={}",
                                new_id,
                                fd,
                                size
                            );
                        }
                    }
                    Some(WaylandObject::ShmPool { bs_id: _, size: _ }) => {
                        if opcode == 0 {
                            // create_buffer (new_id, offset, w, h, stride, format)
                            let new_id = u32::from_ne_bytes(payload[0..4].try_into().unwrap());
                            let offset = u32::from_ne_bytes(payload[4..8].try_into().unwrap());
                            let width = u32::from_ne_bytes(payload[8..12].try_into().unwrap());
                            let height = u32::from_ne_bytes(payload[12..16].try_into().unwrap());
                            let stride = u32::from_ne_bytes(payload[16..20].try_into().unwrap());
                            let format = u32::from_ne_bytes(payload[20..24].try_into().unwrap());

                            client.objects.insert(
                                new_id,
                                WaylandObject::Buffer {
                                    pool_id: obj_id,
                                    offset,
                                    width,
                                    height,
                                    stride,
                                    format,
                                },
                            );
                        }
                    }
                    _ => {}
                }

                // Handle mutating surface state separately
                if let Some(WaylandObject::Surface { ref mut buffer_id }) =
                    client.objects.get_mut(&obj_id)
                {
                    if opcode == 1 {
                        // attach (buffer_id, x, y)
                        let bid = u32::from_ne_bytes(payload[0..4].try_into().unwrap());
                        if bid != 0 {
                            *buffer_id = Some(bid);
                        }
                    } else if opcode == 6 {
                        // commit
                        if let Some(bid) = *buffer_id {
                            if let Some(WaylandObject::Buffer {
                                pool_id,
                                width,
                                height,
                                stride,
                                format,
                                ..
                            }) = client.objects.get(&bid).cloned()
                            {
                                if let Some(WaylandObject::ShmPool { bs_id, .. }) =
                                    client.objects.get(&pool_id).cloned()
                                {
                                    surfaces_to_commit.push(WaylandSurfaceCommit {
                                        client_handle: handle,
                                        surface_id: obj_id,
                                        bs_id,
                                        width,
                                        height,
                                        stride,
                                        format,
                                    });
                                }
                            }
                        }
                    }
                }

                consumed += size as usize;
            }

            client.in_buf.drain(0..consumed);
            client.flush_read_if_pending();
        }

        self.committed_surfaces.extend(surfaces_to_commit);
    }
}
