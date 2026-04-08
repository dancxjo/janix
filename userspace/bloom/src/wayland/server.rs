use crate::wayland::protocol::{decode_header, MessageBuilder};
use abi::vfs_rpc::{VfsRpcOp, VfsRpcReqHeader, VFS_RPC_MAX_REQ};
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};
use stem::syscall::{port_create, port_send, port_try_recv, PortHandle};
use stem::thing::HandleId;
use stem::thing::ThingId;

static NEXT_HANDLE: AtomicU32 = AtomicU32::new(10);
static NEXT_SERIAL: AtomicU32 = AtomicU32::new(1);

const GLOBAL_WL_COMPOSITOR: u32 = 1;
const GLOBAL_WL_SHM: u32 = 2;
const GLOBAL_XDG_WM_BASE: u32 = 3;

const XDG_TOPLEVEL_STATE_MAXIMIZED: u32 = 1;
const XDG_TOPLEVEL_STATE_FULLSCREEN: u32 = 2;
const XDG_TOPLEVEL_STATE_RESIZING: u32 = 3;
const XDG_TOPLEVEL_STATE_ACTIVATED: u32 = 4;

pub const XDG_TOPLEVEL_BORDER: i32 = 2;
pub const XDG_TOPLEVEL_TITLEBAR: i32 = 28;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BufferMeta {
    pub bs_id: u64,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Geometry {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WaylandWindowSnapshot {
    pub scene_id: ThingId,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub content_x: i32,
    pub content_y: i32,
    pub content_width: i32,
    pub content_height: i32,
    pub z_index: i32,
    pub decorated: bool,
    pub is_popup: bool,
    pub title: String,
    pub app_id: String,
    pub maximized: bool,
    pub fullscreen: bool,
    pub buffer: Option<BufferMeta>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WaylandServerEvent {
    WindowChanged(ThingId),
    WindowRemoved(ThingId),
}

#[derive(Clone, Debug, PartialEq)]
enum WaylandObject {
    Display,
    Registry,
    Compositor,
    Shm,
    ShmPool {
        bs_id: u64,
        size: u32,
    },
    Buffer(BufferMeta),
    Surface(SurfaceState),
    XdgWmBase,
    XdgPositioner(PositionerState),
    XdgSurface(XdgSurfaceState),
    XdgToplevel(XdgToplevelState),
    XdgPopup(XdgPopupState),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SurfaceState {
    buffer_id: Option<u32>,
    xdg_surface_id: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum XdgRole {
    None,
    Toplevel(u32),
    Popup(u32),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct XdgSurfaceState {
    wl_surface_id: u32,
    role: XdgRole,
    pending_configure: Option<u32>,
    last_acked_configure: Option<u32>,
    applied_configure: Option<u32>,
    window_geometry: Option<Geometry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct XdgToplevelState {
    xdg_surface_id: u32,
    scene_id: ThingId,
    title: String,
    app_id: String,
    min_width: i32,
    min_height: i32,
    max_width: i32,
    max_height: i32,
    x: i32,
    y: i32,
    content_width: u32,
    content_height: u32,
    pending_width: u32,
    pending_height: u32,
    pending_serial: Option<u32>,
    mapped: bool,
    maximized: bool,
    fullscreen: bool,
    minimized: bool,
    z_index: i32,
    buffer: Option<BufferMeta>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct XdgPopupState {
    xdg_surface_id: u32,
    scene_id: ThingId,
    parent_xdg_surface_id: u32,
    positioner_id: u32,
    rel_x: i32,
    rel_y: i32,
    width: u32,
    height: u32,
    pending_rel_x: i32,
    pending_rel_y: i32,
    pending_width: u32,
    pending_height: u32,
    pending_serial: Option<u32>,
    grab: bool,
    mapped: bool,
    z_index: i32,
    buffer: Option<BufferMeta>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct PositionerState {
    width: u32,
    height: u32,
    anchor_rect: Geometry,
    anchor: u32,
    gravity: u32,
    constraint_adjustment: u32,
    offset_x: i32,
    offset_y: i32,
}

pub struct ClientConnection {
    pub handle: u64,
    pub pending_read_port: Option<u32>,
    pub out_buf: Vec<u8>,
    pub in_buf: Vec<u8>,
    pub objects: BTreeMap<u32, WaylandObject>,
    pub last_ping_ns: u64,
    pub pending_ping_serial: Option<u32>,
}

impl ClientConnection {
    pub fn new(handle: u64) -> Self {
        let mut objects = BTreeMap::new();
        objects.insert(1, WaylandObject::Display);
        Self {
            handle,
            pending_read_port: None,
            out_buf: Vec::new(),
            in_buf: Vec::new(),
            objects,
            last_ping_ns: 0,
            pending_ping_serial: None,
        }
    }

    pub fn flush_read_if_pending(&mut self) {
        if self.out_buf.is_empty() {
            return;
        }
        if let Some(resp_port) = self.pending_read_port.take() {
            let mut resp = Vec::with_capacity(1 + 4 + self.out_buf.len());
            resp.push(0);
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
    scene_index: BTreeMap<ThingId, (u64, u32, bool)>,
    events: Vec<WaylandServerEvent>,
}

impl WaylandServer {
    pub fn new() -> Option<Self> {
        let (req_write, req_read) = port_create(65536).ok()?;
        stem::syscall::vfs_mount(req_write, "/run/wayland-0").ok()?;

        Some(Self {
            req_port: req_read,
            clients: BTreeMap::new(),
            scene_index: BTreeMap::new(),
            events: Vec::new(),
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

    pub fn tick(&mut self, now_ns: u64) {
        let handles: Vec<u64> = self.clients.keys().copied().collect();
        for handle in handles {
            let Some(client) = self.clients.get_mut(&handle) else {
                continue;
            };
            if client.pending_ping_serial.is_some() && now_ns.saturating_sub(client.last_ping_ns) < 5_000_000_000
            {
                continue;
            }
            if now_ns.saturating_sub(client.last_ping_ns) < 2_000_000_000 {
                continue;
            }

            let Some(wm_base_id) = client.objects.iter().find_map(|(id, obj)| {
                if matches!(obj, WaylandObject::XdgWmBase) {
                    Some(*id)
                } else {
                    None
                }
            }) else {
                continue;
            };

            let serial = next_serial();
            let mut mb = MessageBuilder::new(wm_base_id, 0);
            mb.push_u32(serial);
            client.out_buf.extend_from_slice(&mb.build());
            client.pending_ping_serial = Some(serial);
            client.last_ping_ns = now_ns;
            client.flush_read_if_pending();
        }
    }

    pub fn drain_events(&mut self) -> Vec<WaylandServerEvent> {
        let mut out = Vec::new();
        core::mem::swap(&mut out, &mut self.events);
        out
    }

    pub fn is_scene_surface(&self, scene_id: ThingId) -> bool {
        self.scene_index.contains_key(&scene_id)
    }

    pub fn move_surface(&mut self, scene_id: ThingId, x: i32, y: i32) -> bool {
        let Some((handle, object_id, is_popup)) = self.scene_index.get(&scene_id).copied() else {
            return false;
        };
        if is_popup {
            return false;
        }
        let Some(client) = self.clients.get_mut(&handle) else {
            return false;
        };
        let Some(WaylandObject::XdgToplevel(mut toplevel)) = client.objects.get(&object_id).cloned()
        else {
            return false;
        };
        let parent_xdg_surface_id = toplevel.xdg_surface_id;
        toplevel.x = x;
        toplevel.y = y;
        client
            .objects
            .insert(object_id, WaylandObject::XdgToplevel(toplevel));
        self.events.push(WaylandServerEvent::WindowChanged(scene_id));
        self.update_popups_for_parent(handle, parent_xdg_surface_id);
        true
    }

    pub fn resize_toplevel(&mut self, scene_id: ThingId, width: i32, height: i32) -> bool {
        let Some((handle, object_id, is_popup)) = self.scene_index.get(&scene_id).copied() else {
            return false;
        };
        if is_popup {
            return false;
        }
        let Some(client) = self.clients.get_mut(&handle) else {
            return false;
        };
        let Some(WaylandObject::XdgToplevel(toplevel)) = client.objects.get(&object_id).cloned()
        else {
            return false;
        };
        let w = width.max(64) as u32;
        let h = height.max(48) as u32;
        Self::send_toplevel_configure(client, object_id, w, h, true, toplevel.maximized, toplevel.fullscreen);
        true
    }

    pub fn toggle_maximized(&mut self, scene_id: ThingId, screen_w: i32, screen_h: i32) -> bool {
        let Some((handle, object_id, is_popup)) = self.scene_index.get(&scene_id).copied() else {
            return false;
        };
        if is_popup {
            return false;
        }
        let Some(client) = self.clients.get_mut(&handle) else {
            return false;
        };
        let Some(WaylandObject::XdgToplevel(mut toplevel)) = client.objects.get(&object_id).cloned()
        else {
            return false;
        };
        toplevel.maximized = !toplevel.maximized;
        if toplevel.maximized {
            toplevel.x = 0;
            toplevel.y = 0;
        }
        let width = if toplevel.maximized {
            (screen_w - XDG_TOPLEVEL_BORDER * 2).max(64) as u32
        } else if toplevel.content_width > 0 {
            toplevel.content_width
        } else {
            480
        };
        let height = if toplevel.maximized {
            (screen_h - XDG_TOPLEVEL_TITLEBAR - XDG_TOPLEVEL_BORDER).max(48) as u32
        } else if toplevel.content_height > 0 {
            toplevel.content_height
        } else {
            320
        };
        client
            .objects
            .insert(object_id, WaylandObject::XdgToplevel(toplevel.clone()));
        Self::send_toplevel_configure(client, object_id, width, height, false, toplevel.maximized, toplevel.fullscreen);
        true
    }

    pub fn raise_surface(&mut self, scene_id: ThingId) -> bool {
        let Some((handle, object_id, is_popup)) = self.scene_index.get(&scene_id).copied() else {
            return false;
        };
        let next_z = self
            .snapshots()
            .iter()
            .map(|w| w.z_index)
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        let Some(client) = self.clients.get_mut(&handle) else {
            return false;
        };
        if is_popup {
            if let Some(WaylandObject::XdgPopup(mut popup)) = client.objects.get(&object_id).cloned() {
                popup.z_index = next_z;
                client.objects.insert(object_id, WaylandObject::XdgPopup(popup));
                self.events.push(WaylandServerEvent::WindowChanged(scene_id));
                return true;
            }
        } else if let Some(WaylandObject::XdgToplevel(mut top)) = client.objects.get(&object_id).cloned() {
            top.z_index = next_z;
            client.objects.insert(object_id, WaylandObject::XdgToplevel(top));
            self.events.push(WaylandServerEvent::WindowChanged(scene_id));
            return true;
        }
        false
    }

    pub fn snapshots(&self) -> Vec<WaylandWindowSnapshot> {
        let mut out = Vec::new();
        for client in self.clients.values() {
            for obj in client.objects.values() {
                match obj {
                    WaylandObject::XdgToplevel(top) if top.mapped => {
                        let decorated = !top.fullscreen;
                        let content_x = if decorated { XDG_TOPLEVEL_BORDER } else { 0 };
                        let content_y = if decorated { XDG_TOPLEVEL_TITLEBAR } else { 0 };
                        let width = top.content_width as i32 + if decorated { XDG_TOPLEVEL_BORDER * 2 } else { 0 };
                        let height = top.content_height as i32
                            + if decorated {
                                XDG_TOPLEVEL_TITLEBAR + XDG_TOPLEVEL_BORDER
                            } else {
                                0
                            };
                        out.push(WaylandWindowSnapshot {
                            scene_id: top.scene_id,
                            x: top.x,
                            y: top.y,
                            width,
                            height,
                            content_x,
                            content_y,
                            content_width: top.content_width as i32,
                            content_height: top.content_height as i32,
                            z_index: top.z_index,
                            decorated,
                            is_popup: false,
                            title: top.title.clone(),
                            app_id: top.app_id.clone(),
                            maximized: top.maximized,
                            fullscreen: top.fullscreen,
                            buffer: top.buffer,
                        });
                    }
                    WaylandObject::XdgPopup(popup) if popup.mapped => {
                        let (x, y) = Self::popup_absolute_position(client, popup);
                        out.push(WaylandWindowSnapshot {
                            scene_id: popup.scene_id,
                            x,
                            y,
                            width: popup.width as i32,
                            height: popup.height as i32,
                            content_x: 0,
                            content_y: 0,
                            content_width: popup.width as i32,
                            content_height: popup.height as i32,
                            z_index: popup.z_index,
                            decorated: false,
                            is_popup: true,
                            title: String::new(),
                            app_id: String::new(),
                            maximized: false,
                            fullscreen: false,
                            buffer: popup.buffer,
                        });
                    }
                    _ => {}
                }
            }
        }
        out.sort_by(|a, b| a.z_index.cmp(&b.z_index));
        out
    }

    fn handle_vfs_rpc(&mut self, hdr: VfsRpcReqHeader, payload: &[u8]) {
        match VfsRpcOp::from_u8(hdr.op) {
            Some(VfsRpcOp::Lookup) => {
                let handle = NEXT_HANDLE.fetch_add(1, Ordering::SeqCst) as u64;
                self.clients.insert(handle, ClientConnection::new(handle));
                let mut resp = alloc::vec![0u8; 9];
                resp[0] = 0;
                resp[1..9].copy_from_slice(&handle.to_le_bytes());
                let _ = port_send(hdr.resp_port, &resp);
            }
            Some(VfsRpcOp::Read) => {
                if payload.len() < 12 {
                    return;
                }
                let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());
                if let Some(client) = self.clients.get_mut(&handle) {
                    if client.out_buf.is_empty() {
                        client.pending_read_port = Some(hdr.resp_port);
                    } else {
                        let mut resp = Vec::with_capacity(1 + 4 + client.out_buf.len());
                        resp.push(0);
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
                if payload.len() < 20 {
                    return;
                }
                let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());
                let data_len = u32::from_le_bytes(payload[16..20].try_into().unwrap()) as usize;
                if payload.len() < 20 + data_len {
                    return;
                }
                let data = &payload[20..20 + data_len];

                let mut resp = alloc::vec![0u8; 5];
                resp[0] = 0;
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
                let mode: u32 = 0o020000 | 0o666;
                resp[0] = 0;
                resp[1..5].copy_from_slice(&mode.to_le_bytes());
                resp[5..13].copy_from_slice(&0u64.to_le_bytes());
                resp[13..21].copy_from_slice(&1u64.to_le_bytes());
                let _ = port_send(hdr.resp_port, &resp);
            }
            Some(VfsRpcOp::Close) => {
                if payload.len() < 8 {
                    return;
                }
                let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());
                self.remove_client(handle);
                let _ = port_send(hdr.resp_port, &[0u8]);
            }
            _ => {
                let _ = port_send(hdr.resp_port, &[abi::errors::Errno::ENOSYS as u8]);
            }
        }
    }

    fn remove_client(&mut self, handle: u64) {
        let Some(client) = self.clients.remove(&handle) else {
            return;
        };
        for obj in client.objects.values() {
            match obj {
                WaylandObject::XdgToplevel(top) => {
                    self.scene_index.remove(&top.scene_id);
                    self.events.push(WaylandServerEvent::WindowRemoved(top.scene_id));
                }
                WaylandObject::XdgPopup(popup) => {
                    self.scene_index.remove(&popup.scene_id);
                    self.events.push(WaylandServerEvent::WindowRemoved(popup.scene_id));
                }
                _ => {}
            }
        }
    }

    fn process_client_input(&mut self, handle: u64) {
        let mut consumed = 0usize;
        let mut deferred_events = Vec::new();

        let Some(mut client) = self.clients.remove(&handle) else {
            return;
        };
        while client.in_buf.len().saturating_sub(consumed) >= 8 {
                let buf = &client.in_buf[consumed..];
                let (obj_id, opcode, size) = decode_header(buf);
                if size < 8 || buf.len() < size as usize {
                    break;
                }
                let payload = &buf[8..size as usize];
                let obj = client.objects.get(&obj_id).cloned();
                match obj {
                    Some(WaylandObject::Display) => {
                        if opcode == 1 && payload.len() >= 4 {
                            let new_id = read_u32(payload, 0);
                            client.objects.insert(new_id, WaylandObject::Registry);

                            let mut compositor = MessageBuilder::new(new_id, 0);
                            compositor.push_u32(GLOBAL_WL_COMPOSITOR);
                            compositor.push_string("wl_compositor");
                            compositor.push_u32(4);
                            client.out_buf.extend_from_slice(&compositor.build());

                            let mut shm = MessageBuilder::new(new_id, 0);
                            shm.push_u32(GLOBAL_WL_SHM);
                            shm.push_string("wl_shm");
                            shm.push_u32(1);
                            client.out_buf.extend_from_slice(&shm.build());

                            let mut xdg = MessageBuilder::new(new_id, 0);
                            xdg.push_u32(GLOBAL_XDG_WM_BASE);
                            xdg.push_string("xdg_wm_base");
                            xdg.push_u32(1);
                            client.out_buf.extend_from_slice(&xdg.build());
                        }
                    }
                    Some(WaylandObject::Registry) => {
                        if opcode == 0 && payload.len() >= 12 {
                            let name = read_u32(payload, 0);
                            let new_id = read_u32(payload, payload.len() - 4);
                            match name {
                                GLOBAL_WL_COMPOSITOR => {
                                    client.objects.insert(new_id, WaylandObject::Compositor);
                                }
                                GLOBAL_WL_SHM => {
                                    client.objects.insert(new_id, WaylandObject::Shm);
                                    let mut mb = MessageBuilder::new(new_id, 0);
                                    mb.push_u32(0);
                                    client.out_buf.extend_from_slice(&mb.build());
                                }
                                GLOBAL_XDG_WM_BASE => {
                                    client.objects.insert(new_id, WaylandObject::XdgWmBase);
                                }
                                _ => {}
                            }
                        }
                    }
                    Some(WaylandObject::Compositor) => {
                        if opcode == 0 && payload.len() >= 4 {
                            let new_id = read_u32(payload, 0);
                            client.objects.insert(
                                new_id,
                                WaylandObject::Surface(SurfaceState {
                                    buffer_id: None,
                                    xdg_surface_id: None,
                                }),
                            );
                        }
                    }
                    Some(WaylandObject::Shm) => {
                        if opcode == 0 && payload.len() >= 12 {
                            let new_id = read_u32(payload, 0);
                            let bs_id = read_u32(payload, 4) as u64;
                            let size = read_u32(payload, 8);
                            client
                                .objects
                                .insert(new_id, WaylandObject::ShmPool { bs_id, size });
                        }
                    }
                    Some(WaylandObject::ShmPool { bs_id, .. }) => {
                        if opcode == 0 && payload.len() >= 24 {
                            let new_id = read_u32(payload, 0);
                            let meta = BufferMeta {
                                bs_id,
                                width: read_u32(payload, 8),
                                height: read_u32(payload, 12),
                                stride: read_u32(payload, 16),
                                format: read_u32(payload, 20),
                            };
                            client.objects.insert(new_id, WaylandObject::Buffer(meta));
                        }
                    }
                    Some(WaylandObject::Surface(mut surface)) => match opcode {
                        0 => {
                            client.objects.remove(&obj_id);
                        }
                        1 if payload.len() >= 4 => {
                            let buffer_id = read_u32(payload, 0);
                            surface.buffer_id = if buffer_id == 0 { None } else { Some(buffer_id) };
                            client.objects.insert(obj_id, WaylandObject::Surface(surface));
                        }
                        6 => {
                            client.objects.insert(obj_id, WaylandObject::Surface(surface));
                            self.handle_surface_commit(handle, &mut client, obj_id, &mut deferred_events);
                        }
                        _ => {}
                    },
                    Some(WaylandObject::XdgWmBase) => match opcode {
                        1 if payload.len() >= 4 => {
                            let new_id = read_u32(payload, 0);
                            client.objects.insert(new_id, WaylandObject::XdgPositioner(PositionerState::default()));
                        }
                        2 if payload.len() >= 8 => {
                            let new_id = read_u32(payload, 0);
                            let wl_surface_id = read_u32(payload, 4);
                            if let Some(WaylandObject::Surface(mut surface)) =
                                client.objects.get(&wl_surface_id).cloned()
                            {
                                surface.xdg_surface_id = Some(new_id);
                                client.objects.insert(wl_surface_id, WaylandObject::Surface(surface));
                                client.objects.insert(
                                    new_id,
                                    WaylandObject::XdgSurface(XdgSurfaceState {
                                        wl_surface_id,
                                        role: XdgRole::None,
                                        pending_configure: None,
                                        last_acked_configure: None,
                                        applied_configure: None,
                                        window_geometry: None,
                                    }),
                                );
                            }
                        }
                        3 if payload.len() >= 4 => {
                            let serial = read_u32(payload, 0);
                            if client.pending_ping_serial == Some(serial) {
                                client.pending_ping_serial = None;
                            }
                        }
                        _ => {}
                    },
                    Some(WaylandObject::XdgPositioner(mut pos)) => {
                        match opcode {
                            0 => {
                                client.objects.remove(&obj_id);
                            }
                            1 if payload.len() >= 8 => {
                                pos.width = read_i32(payload, 0).max(1) as u32;
                                pos.height = read_i32(payload, 4).max(1) as u32;
                                client.objects.insert(obj_id, WaylandObject::XdgPositioner(pos));
                            }
                            2 if payload.len() >= 16 => {
                                pos.anchor_rect = Geometry {
                                    x: read_i32(payload, 0),
                                    y: read_i32(payload, 4),
                                    width: read_i32(payload, 8),
                                    height: read_i32(payload, 12),
                                };
                                client.objects.insert(obj_id, WaylandObject::XdgPositioner(pos));
                            }
                            3 if payload.len() >= 4 => {
                                pos.anchor = read_u32(payload, 0);
                                client.objects.insert(obj_id, WaylandObject::XdgPositioner(pos));
                            }
                            4 if payload.len() >= 4 => {
                                pos.gravity = read_u32(payload, 0);
                                client.objects.insert(obj_id, WaylandObject::XdgPositioner(pos));
                            }
                            5 if payload.len() >= 4 => {
                                pos.constraint_adjustment = read_u32(payload, 0);
                                client.objects.insert(obj_id, WaylandObject::XdgPositioner(pos));
                            }
                            6 if payload.len() >= 8 => {
                                pos.offset_x = read_i32(payload, 0);
                                pos.offset_y = read_i32(payload, 4);
                                client.objects.insert(obj_id, WaylandObject::XdgPositioner(pos));
                            }
                            _ => {}
                        }
                    }
                    Some(WaylandObject::XdgSurface(mut xdg_surface)) => match opcode {
                        0 => {
                            self.destroy_xdg_surface(handle, &mut client, obj_id, &mut deferred_events);
                        }
                        1 if payload.len() >= 4 => {
                            let new_id = read_u32(payload, 0);
                            if matches!(xdg_surface.role, XdgRole::None) {
                                let scene_id = scene_id_for(handle, new_id);
                                let toplevel = XdgToplevelState {
                                    xdg_surface_id: obj_id,
                                    scene_id,
                                    title: "Wayland Window".to_string(),
                                    app_id: String::new(),
                                    min_width: 0,
                                    min_height: 0,
                                    max_width: 0,
                                    max_height: 0,
                                    x: 80,
                                    y: 80,
                                    content_width: 480,
                                    content_height: 320,
                                    pending_width: 480,
                                    pending_height: 320,
                                    pending_serial: None,
                                    mapped: false,
                                    maximized: false,
                                    fullscreen: false,
                                    minimized: false,
                                    z_index: 100,
                                    buffer: None,
                                };
                                xdg_surface.role = XdgRole::Toplevel(new_id);
                                client.objects.insert(obj_id, WaylandObject::XdgSurface(xdg_surface));
                                client.objects.insert(new_id, WaylandObject::XdgToplevel(toplevel));
                                self.scene_index.insert(scene_id, (handle, new_id, false));
                                Self::send_toplevel_configure(&mut client, new_id, 480, 320, false, false, false);
                            }
                        }
                        2 if payload.len() >= 12 => {
                            let new_id = read_u32(payload, 0);
                            let parent_xdg_surface_id = read_u32(payload, 4);
                            let positioner_id = read_u32(payload, 8);
                            if matches!(xdg_surface.role, XdgRole::None) {
                                let scene_id = scene_id_for(handle, new_id);
                                let (rel_x, rel_y, width, height) =
                                    Self::compute_popup_geometry(&client, parent_xdg_surface_id, positioner_id);
                                let popup = XdgPopupState {
                                    xdg_surface_id: obj_id,
                                    scene_id,
                                    parent_xdg_surface_id,
                                    positioner_id,
                                    rel_x,
                                    rel_y,
                                    width,
                                    height,
                                    pending_rel_x: rel_x,
                                    pending_rel_y: rel_y,
                                    pending_width: width,
                                    pending_height: height,
                                    pending_serial: None,
                                    grab: false,
                                    mapped: false,
                                    z_index: 200,
                                    buffer: None,
                                };
                                xdg_surface.role = XdgRole::Popup(new_id);
                                client.objects.insert(obj_id, WaylandObject::XdgSurface(xdg_surface));
                                client.objects.insert(new_id, WaylandObject::XdgPopup(popup));
                                self.scene_index.insert(scene_id, (handle, new_id, true));
                                Self::send_popup_configure(&mut client, new_id, rel_x, rel_y, width, height);
                            }
                        }
                        3 if payload.len() >= 16 => {
                            xdg_surface.window_geometry = Some(Geometry {
                                x: read_i32(payload, 0),
                                y: read_i32(payload, 4),
                                width: read_i32(payload, 8),
                                height: read_i32(payload, 12),
                            });
                            client.objects.insert(obj_id, WaylandObject::XdgSurface(xdg_surface));
                        }
                        4 if payload.len() >= 4 => {
                            xdg_surface.last_acked_configure = Some(read_u32(payload, 0));
                            client.objects.insert(obj_id, WaylandObject::XdgSurface(xdg_surface));
                        }
                        _ => {}
                    },
                    Some(WaylandObject::XdgToplevel(mut top)) => match opcode {
                        0 => {
                            self.destroy_role_object(handle, &mut client, obj_id, false, &mut deferred_events);
                        }
                        2 => {
                            top.title = read_string(payload);
                            let scene_id = top.scene_id;
                            client.objects.insert(obj_id, WaylandObject::XdgToplevel(top));
                            deferred_events.push(WaylandServerEvent::WindowChanged(scene_id));
                        }
                        3 => {
                            top.app_id = read_string(payload);
                            let scene_id = top.scene_id;
                            client.objects.insert(obj_id, WaylandObject::XdgToplevel(top));
                            deferred_events.push(WaylandServerEvent::WindowChanged(scene_id));
                        }
                        7 if payload.len() >= 8 => {
                            top.max_width = read_i32(payload, 0);
                            top.max_height = read_i32(payload, 4);
                            client.objects.insert(obj_id, WaylandObject::XdgToplevel(top));
                        }
                        8 if payload.len() >= 8 => {
                            top.min_width = read_i32(payload, 0);
                            top.min_height = read_i32(payload, 4);
                            client.objects.insert(obj_id, WaylandObject::XdgToplevel(top));
                        }
                        9 => {
                            top.maximized = true;
                            client.objects.insert(obj_id, WaylandObject::XdgToplevel(top.clone()));
                            Self::send_toplevel_configure(&mut client, obj_id, top.content_width.max(480), top.content_height.max(320), false, true, top.fullscreen);
                        }
                        10 => {
                            top.maximized = false;
                            client.objects.insert(obj_id, WaylandObject::XdgToplevel(top.clone()));
                            Self::send_toplevel_configure(&mut client, obj_id, top.content_width.max(480), top.content_height.max(320), false, false, top.fullscreen);
                        }
                        11 => {
                            top.fullscreen = true;
                            client.objects.insert(obj_id, WaylandObject::XdgToplevel(top.clone()));
                            Self::send_toplevel_configure(&mut client, obj_id, top.content_width.max(640), top.content_height.max(360), false, top.maximized, true);
                        }
                        12 => {
                            top.fullscreen = false;
                            client.objects.insert(obj_id, WaylandObject::XdgToplevel(top.clone()));
                            Self::send_toplevel_configure(&mut client, obj_id, top.content_width.max(480), top.content_height.max(320), false, top.maximized, false);
                        }
                        13 => {
                            top.minimized = true;
                            client.objects.insert(obj_id, WaylandObject::XdgToplevel(top));
                        }
                        _ => {}
                    },
                    Some(WaylandObject::XdgPopup(mut popup)) => match opcode {
                        0 => {
                            self.destroy_role_object(handle, &mut client, obj_id, true, &mut deferred_events);
                        }
                        1 if payload.len() >= 4 => {
                            let _seat = read_u32(payload, 0);
                            popup.grab = true;
                            client.objects.insert(obj_id, WaylandObject::XdgPopup(popup));
                        }
                        _ => {}
                    },
                    _ => {}
                }

                consumed += size as usize;
        }
        client.in_buf.drain(0..consumed);
        client.flush_read_if_pending();
        self.clients.insert(handle, client);

        self.events.extend(deferred_events);
    }

    fn handle_surface_commit(
        &mut self,
        handle: u64,
        client: &mut ClientConnection,
        wl_surface_id: u32,
        deferred_events: &mut Vec<WaylandServerEvent>,
    ) {
        let Some(WaylandObject::Surface(surface)) = client.objects.get(&wl_surface_id).cloned() else {
            return;
        };
        let Some(xdg_surface_id) = surface.xdg_surface_id else {
            return;
        };
        let Some(WaylandObject::XdgSurface(mut xdg_surface)) =
            client.objects.get(&xdg_surface_id).cloned()
        else {
            return;
        };
        let Some(buffer_id) = surface.buffer_id else {
            return;
        };
        let Some(WaylandObject::Buffer(buffer)) = client.objects.get(&buffer_id).cloned() else {
            return;
        };

        if let Some(serial) = xdg_surface.pending_configure {
            if xdg_surface.last_acked_configure != Some(serial) {
                return;
            }
            xdg_surface.applied_configure = Some(serial);
            xdg_surface.pending_configure = None;
            client
                .objects
                .insert(xdg_surface_id, WaylandObject::XdgSurface(xdg_surface.clone()));
        }

        match xdg_surface.role {
            XdgRole::Toplevel(toplevel_id) => {
                let Some(WaylandObject::XdgToplevel(mut top)) =
                    client.objects.get(&toplevel_id).cloned()
                else {
                    return;
                };
                if let Some(serial) = top.pending_serial {
                    if Some(serial) == xdg_surface.applied_configure {
                        top.content_width = top.pending_width.max(1);
                        top.content_height = top.pending_height.max(1);
                        top.pending_serial = None;
                    }
                }
                if let Some(geometry) = xdg_surface.window_geometry {
                    if geometry.width > 0 {
                        top.content_width = geometry.width as u32;
                    } else if buffer.width > 0 {
                        top.content_width = buffer.width;
                    }
                    if geometry.height > 0 {
                        top.content_height = geometry.height as u32;
                    } else if buffer.height > 0 {
                        top.content_height = buffer.height;
                    }
                } else {
                    top.content_width = buffer.width.max(1);
                    top.content_height = buffer.height.max(1);
                }
                top.buffer = Some(buffer);
                top.mapped = true;
                let scene_id = top.scene_id;
                client
                    .objects
                    .insert(toplevel_id, WaylandObject::XdgToplevel(top));
                deferred_events.push(WaylandServerEvent::WindowChanged(scene_id));
                self.update_popups_for_parent_in_client(handle, client, xdg_surface_id);
            }
            XdgRole::Popup(popup_id) => {
                let Some(WaylandObject::XdgPopup(mut popup)) =
                    client.objects.get(&popup_id).cloned()
                else {
                    return;
                };
                if let Some(serial) = popup.pending_serial {
                    if Some(serial) == xdg_surface.applied_configure {
                        popup.rel_x = popup.pending_rel_x;
                        popup.rel_y = popup.pending_rel_y;
                        popup.width = popup.pending_width.max(1);
                        popup.height = popup.pending_height.max(1);
                        popup.pending_serial = None;
                    }
                }
                popup.buffer = Some(buffer);
                popup.mapped = true;
                if buffer.width > 0 {
                    popup.width = buffer.width;
                }
                if buffer.height > 0 {
                    popup.height = buffer.height;
                }
                let scene_id = popup.scene_id;
                client
                    .objects
                    .insert(popup_id, WaylandObject::XdgPopup(popup));
                deferred_events.push(WaylandServerEvent::WindowChanged(scene_id));
            }
            XdgRole::None => {}
        }
    }

    fn destroy_xdg_surface(
        &mut self,
        handle: u64,
        client: &mut ClientConnection,
        xdg_surface_id: u32,
        deferred_events: &mut Vec<WaylandServerEvent>,
    ) {
        let Some(WaylandObject::XdgSurface(surface)) = client.objects.get(&xdg_surface_id).cloned()
        else {
            return;
        };
        match surface.role {
            XdgRole::Toplevel(role_id) => self.destroy_role_object(handle, client, role_id, false, deferred_events),
            XdgRole::Popup(role_id) => self.destroy_role_object(handle, client, role_id, true, deferred_events),
            XdgRole::None => {}
        }
        if let Some(WaylandObject::Surface(mut wl_surface)) =
            client.objects.get(&surface.wl_surface_id).cloned()
        {
            wl_surface.xdg_surface_id = None;
            client
                .objects
                .insert(surface.wl_surface_id, WaylandObject::Surface(wl_surface));
        }
        client.objects.remove(&xdg_surface_id);
    }

    fn destroy_role_object(
        &mut self,
        handle: u64,
        client: &mut ClientConnection,
        role_id: u32,
        is_popup: bool,
        deferred_events: &mut Vec<WaylandServerEvent>,
    ) {
        let removed = client.objects.remove(&role_id);
        match removed {
            Some(WaylandObject::XdgToplevel(top)) => {
                self.scene_index.remove(&top.scene_id);
                deferred_events.push(WaylandServerEvent::WindowRemoved(top.scene_id));
                if let Some(WaylandObject::XdgSurface(mut surface)) =
                    client.objects.get(&top.xdg_surface_id).cloned()
                {
                    surface.role = XdgRole::None;
                    client
                        .objects
                        .insert(top.xdg_surface_id, WaylandObject::XdgSurface(surface));
                }
            }
            Some(WaylandObject::XdgPopup(popup)) => {
                self.scene_index.remove(&popup.scene_id);
                deferred_events.push(WaylandServerEvent::WindowRemoved(popup.scene_id));
                if !is_popup {
                    let _ = is_popup;
                }
                if let Some(WaylandObject::XdgSurface(mut surface)) =
                    client.objects.get(&popup.xdg_surface_id).cloned()
                {
                    surface.role = XdgRole::None;
                    client
                        .objects
                        .insert(popup.xdg_surface_id, WaylandObject::XdgSurface(surface));
                }
            }
            _ => {}
        }
    }

    fn send_toplevel_configure(
        client: &mut ClientConnection,
        toplevel_id: u32,
        width: u32,
        height: u32,
        resizing: bool,
        maximized: bool,
        fullscreen: bool,
    ) {
        let Some(WaylandObject::XdgToplevel(mut top)) = client.objects.get(&toplevel_id).cloned()
        else {
            return;
        };
        let serial = next_serial();
        top.pending_width = width.max(1);
        top.pending_height = height.max(1);
        top.pending_serial = Some(serial);
        client
            .objects
            .insert(toplevel_id, WaylandObject::XdgToplevel(top.clone()));

        if let Some(WaylandObject::XdgSurface(mut surface)) =
            client.objects.get(&top.xdg_surface_id).cloned()
        {
            surface.pending_configure = Some(serial);
            client
                .objects
                .insert(top.xdg_surface_id, WaylandObject::XdgSurface(surface));
        }

        let mut states = Vec::new();
        states.extend_from_slice(&XDG_TOPLEVEL_STATE_ACTIVATED.to_ne_bytes());
        if maximized {
            states.extend_from_slice(&XDG_TOPLEVEL_STATE_MAXIMIZED.to_ne_bytes());
        }
        if fullscreen {
            states.extend_from_slice(&XDG_TOPLEVEL_STATE_FULLSCREEN.to_ne_bytes());
        }
        if resizing {
            states.extend_from_slice(&XDG_TOPLEVEL_STATE_RESIZING.to_ne_bytes());
        }

        let mut top_msg = MessageBuilder::new(toplevel_id, 0);
        top_msg.push_i32(width as i32);
        top_msg.push_i32(height as i32);
        top_msg.push_array(&states);
        client.out_buf.extend_from_slice(&top_msg.build());

        let mut surf_msg = MessageBuilder::new(top.xdg_surface_id, 0);
        surf_msg.push_u32(serial);
        client.out_buf.extend_from_slice(&surf_msg.build());
        client.flush_read_if_pending();
    }

    fn send_popup_configure(
        client: &mut ClientConnection,
        popup_id: u32,
        rel_x: i32,
        rel_y: i32,
        width: u32,
        height: u32,
    ) {
        let Some(WaylandObject::XdgPopup(mut popup)) = client.objects.get(&popup_id).cloned()
        else {
            return;
        };
        let serial = next_serial();
        popup.pending_rel_x = rel_x;
        popup.pending_rel_y = rel_y;
        popup.pending_width = width.max(1);
        popup.pending_height = height.max(1);
        popup.pending_serial = Some(serial);
        client
            .objects
            .insert(popup_id, WaylandObject::XdgPopup(popup.clone()));

        if let Some(WaylandObject::XdgSurface(mut surface)) =
            client.objects.get(&popup.xdg_surface_id).cloned()
        {
            surface.pending_configure = Some(serial);
            client
                .objects
                .insert(popup.xdg_surface_id, WaylandObject::XdgSurface(surface));
        }

        let mut popup_msg = MessageBuilder::new(popup_id, 0);
        popup_msg.push_i32(rel_x);
        popup_msg.push_i32(rel_y);
        popup_msg.push_i32(width as i32);
        popup_msg.push_i32(height as i32);
        client.out_buf.extend_from_slice(&popup_msg.build());

        let mut surf_msg = MessageBuilder::new(popup.xdg_surface_id, 0);
        surf_msg.push_u32(serial);
        client.out_buf.extend_from_slice(&surf_msg.build());
        client.flush_read_if_pending();
    }

    fn compute_popup_geometry(
        client: &ClientConnection,
        parent_xdg_surface_id: u32,
        positioner_id: u32,
    ) -> (i32, i32, u32, u32) {
        let Some(WaylandObject::XdgPositioner(pos)) = client.objects.get(&positioner_id).cloned()
        else {
            return (16, 16, 160, 96);
        };
        let width = pos.width.max(1);
        let height = pos.height.max(1);
        let mut rel_x = pos.anchor_rect.x + pos.offset_x;
        let mut rel_y = pos.anchor_rect.y + pos.anchor_rect.height + pos.offset_y;

        if pos.gravity == 8 || pos.gravity == 5 {
            rel_x -= width as i32;
        }
        if pos.gravity == 2 || pos.gravity == 6 {
            rel_y -= height as i32;
        }

        if let Some(parent_geo) = Self::parent_content_size(client, parent_xdg_surface_id) {
            if rel_x + width as i32 > parent_geo.0 {
                rel_x = (parent_geo.0 - width as i32).max(0);
            }
            if rel_y + height as i32 > parent_geo.1 {
                rel_y = (parent_geo.1 - height as i32).max(0);
            }
        }
        (rel_x.max(0), rel_y.max(0), width, height)
    }

    fn parent_content_size(client: &ClientConnection, parent_xdg_surface_id: u32) -> Option<(i32, i32)> {
        let parent_role = match client.objects.get(&parent_xdg_surface_id)? {
            WaylandObject::XdgSurface(surface) => surface.role,
            _ => return None,
        };
        match parent_role {
            XdgRole::Toplevel(role_id) => {
                let WaylandObject::XdgToplevel(top) = client.objects.get(&role_id)? else {
                    return None;
                };
                Some((top.content_width as i32, top.content_height as i32))
            }
            XdgRole::Popup(role_id) => {
                let WaylandObject::XdgPopup(popup) = client.objects.get(&role_id)? else {
                    return None;
                };
                Some((popup.width as i32, popup.height as i32))
            }
            XdgRole::None => None,
        }
    }

    fn popup_absolute_position(client: &ClientConnection, popup: &XdgPopupState) -> (i32, i32) {
        let Some(WaylandObject::XdgSurface(parent_surface)) =
            client.objects.get(&popup.parent_xdg_surface_id)
        else {
            return (popup.rel_x, popup.rel_y);
        };
        match parent_surface.role {
            XdgRole::Toplevel(role_id) => {
                let Some(WaylandObject::XdgToplevel(parent)) = client.objects.get(&role_id) else {
                    return (popup.rel_x, popup.rel_y);
                };
                (
                    parent.x + XDG_TOPLEVEL_BORDER + popup.rel_x,
                    parent.y + XDG_TOPLEVEL_TITLEBAR + popup.rel_y,
                )
            }
            XdgRole::Popup(role_id) => {
                let Some(WaylandObject::XdgPopup(parent)) = client.objects.get(&role_id) else {
                    return (popup.rel_x, popup.rel_y);
                };
                let (px, py) = Self::popup_absolute_position(client, parent);
                (px + popup.rel_x, py + popup.rel_y)
            }
            XdgRole::None => (popup.rel_x, popup.rel_y),
        }
    }

    fn update_popups_for_parent(&mut self, handle: u64, parent_xdg_surface_id: u32) {
        let Some(mut client) = self.clients.remove(&handle) else {
            return;
        };
        self.update_popups_for_parent_in_client(handle, &mut client, parent_xdg_surface_id);
        self.clients.insert(handle, client);
    }

    fn update_popups_for_parent_in_client(
        &mut self,
        _handle: u64,
        client: &mut ClientConnection,
        parent_xdg_surface_id: u32,
    ) {
        let popup_ids: Vec<u32> = client
            .objects
            .iter()
            .filter_map(|(id, obj)| match obj {
                WaylandObject::XdgPopup(popup) if popup.parent_xdg_surface_id == parent_xdg_surface_id => Some(*id),
                _ => None,
            })
            .collect();
        for popup_id in popup_ids {
            let Some(WaylandObject::XdgPopup(popup)) = client.objects.get(&popup_id).cloned() else {
                continue;
            };
            self.events.push(WaylandServerEvent::WindowChanged(popup.scene_id));
        }
    }
}

fn next_serial() -> u32 {
    NEXT_SERIAL.fetch_add(1, Ordering::SeqCst)
}

fn scene_id_for(handle: u64, object_id: u32) -> ThingId {
    ThingId::from_u64((handle << 32) | object_id as u64)
}

fn read_u32(payload: &[u8], offset: usize) -> u32 {
    u32::from_ne_bytes(payload[offset..offset + 4].try_into().unwrap())
}

fn read_i32(payload: &[u8], offset: usize) -> i32 {
    i32::from_ne_bytes(payload[offset..offset + 4].try_into().unwrap())
}

fn read_string(payload: &[u8]) -> String {
    if payload.len() < 4 {
        return String::new();
    }
    let len = read_u32(payload, 0) as usize;
    if len == 0 || payload.len() < 4 + len {
        return String::new();
    }
    let bytes = &payload[4..4 + len.saturating_sub(1)];
    core::str::from_utf8(bytes).unwrap_or("").to_string()
}
