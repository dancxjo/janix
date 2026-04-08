extern crate alloc;

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use abi::errors::Errno;
use abi::pixel::PixelFormat;
use abi::vfs_watch::{flags as watch_flags, mask as watch_mask};
use stem::syscall::vfs::{vfs_close, vfs_read, vfs_watch_path};
use stem::thing::ThingId;

use crate::asset::Image;
use crate::geometry::{Color, Rect};
use crate::scene_graph::SceneGraph;
use crate::session_fs::{self, AttachedBuffer};
use crate::surface::PixelBuffer;

const WATCH_BUFFER_BYTES: usize = 512;

pub struct WindowHit {
    pub id: ThingId,
    pub rect: Rect,
    pub z: i32,
}

#[derive(Clone)]
struct WindowPaintState {
    name: String,
    rect: Rect,
    z: i32,
    hidden: bool,
    surface_name: Option<String>,
    title: String,
    app_id: String,
    mapped: bool,
    focused: bool,
    maximized: bool,
    fullscreen: bool,
    geometry_gen: u64,
    content_gen: u64,
    last_commit: u64,
}

#[derive(Clone, Copy)]
struct WindowWatchState {
    shell_fd: u32,
    requested_fd: u32,
    bind_fd: u32,
}

pub struct PaintResult {
    pub damage: Vec<Rect>,
    pub pending_rebuilds: bool,
}

pub struct PaintPipeline {
    windows: BTreeMap<ThingId, WindowPaintState>,
    dirty_windows: BTreeSet<ThingId>,
    scan_required: bool,
    focused: Option<ThingId>,
    next_configure_serial: u64,
    windows_watch: Option<u32>,
    surfaces_watch: Option<u32>,
    window_watches: BTreeMap<ThingId, WindowWatchState>,
    surface_watches: BTreeMap<String, u32>,
}

impl PaintPipeline {
    pub fn new() -> Self {
        session_fs::ensure_session_roots();
        Self {
            windows: BTreeMap::new(),
            dirty_windows: BTreeSet::new(),
            scan_required: true,
            focused: None,
            next_configure_serial: 1,
            windows_watch: open_watch(
                session_fs::WINDOWS_ROOT,
                watch_mask::CREATE | watch_mask::REMOVE | watch_mask::MOVE,
            ),
            surfaces_watch: open_watch(
                session_fs::SURFACES_ROOT,
                watch_mask::CREATE | watch_mask::REMOVE | watch_mask::MOVE,
            ),
            window_watches: BTreeMap::new(),
            surface_watches: BTreeMap::new(),
        }
    }

    pub fn poll_watch_activity(&mut self) -> bool {
        let mut changed = false;

        if drain_watch(self.windows_watch) {
            self.scan_required = true;
            changed = true;
        }
        if drain_watch(self.surfaces_watch) {
            self.scan_required = true;
            changed = true;
        }

        let window_ids: Vec<ThingId> = self.window_watches.keys().copied().collect();
        for id in window_ids {
            let Some(watch) = self.window_watches.get(&id).copied() else {
                continue;
            };
            if drain_watch(Some(watch.shell_fd))
                || drain_watch(Some(watch.requested_fd))
                || drain_watch(Some(watch.bind_fd))
            {
                self.dirty_windows.insert(id);
                changed = true;
            }
        }

        let surface_names: Vec<String> = self.surface_watches.keys().cloned().collect();
        for name in surface_names {
            let Some(fd) = self.surface_watches.get(&name).copied() else {
                continue;
            };
            if drain_watch(Some(fd)) {
                for (id, win) in self.windows.iter() {
                    if win.surface_name.as_deref() == Some(name.as_str()) {
                        self.dirty_windows.insert(*id);
                    }
                }
                changed = true;
            }
        }

        changed
    }

    pub fn process_updates<F>(
        &mut self,
        scene: &mut SceneGraph,
        screen_w: i32,
        screen_h: i32,
        rescan_windows: bool,
        _refresh_paint: bool,
        mut on_progress: F,
    ) -> PaintResult
    where
        F: FnMut(),
    {
        session_fs::ensure_session_roots();

        let mut damage = Vec::new();
        if rescan_windows || self.scan_required {
            self.sync_windows(scene, screen_w, screen_h, &mut damage, &mut on_progress);
        }

        let dirty_ids: Vec<ThingId> = self.dirty_windows.iter().copied().collect();
        for id in dirty_ids {
            on_progress();
            if let Some(rect) = self.refresh_window_surface(scene, id) {
                damage.push(rect);
            }
            self.dirty_windows.remove(&id);
        }

        damage.extend(scene.collect_damage());
        PaintResult {
            damage,
            pending_rebuilds: false,
        }
    }

    pub fn contains_window(&self, id: ThingId) -> bool {
        self.windows.contains_key(&id)
    }

    pub fn set_focus_target(&mut self, focused: Option<ThingId>) {
        if self.focused == focused {
            return;
        }

        if let Some(prev) = self.focused.take() {
            if let Some(win) = self.windows.get_mut(&prev) {
                win.focused = false;
                write_window_focus(win);
            }
        }

        self.focused = focused;
        if let Some(id) = focused {
            if let Some(win) = self.windows.get_mut(&id) {
                win.focused = true;
                write_window_focus(win);
            }
        }
    }

    pub fn move_window(&mut self, id: ThingId, x: i32, y: i32) -> bool {
        let Some(win) = self.windows.get_mut(&id) else {
            return false;
        };
        let old_rect = win.rect;
        win.rect.origin.x = x;
        win.rect.origin.y = y;
        win.geometry_gen = win.geometry_gen.wrapping_add(1);
        write_window_geometry(win);
        self.dirty_windows.insert(id);
        old_rect != win.rect
    }

    pub fn resize_window(&mut self, id: ThingId, width: i32, height: i32) -> bool {
        let Some(win) = self.windows.get_mut(&id) else {
            return false;
        };
        let old_rect = win.rect;
        win.rect.size.width = width.max(1);
        win.rect.size.height = height.max(1);
        win.geometry_gen = win.geometry_gen.wrapping_add(1);
        win.content_gen = win.content_gen.wrapping_add(1);
        win.last_commit = 0;
        let serial = self.next_configure_serial;
        self.next_configure_serial = self.next_configure_serial.wrapping_add(1);
        send_configure(serial, win);
        write_window_geometry(win);
        self.dirty_windows.insert(id);
        old_rect != win.rect
    }

    pub fn set_window_z(&mut self, id: ThingId, z: i32) -> bool {
        let Some(win) = self.windows.get_mut(&id) else {
            return false;
        };
        if win.z == z {
            return false;
        }
        win.z = z;
        let _ = session_fs::write_text(
            &format!("{}/shell/current/z", session_fs::window_path(&win.name)),
            &format!("{}\n", z),
        );
        self.dirty_windows.insert(id);
        true
    }

    pub fn top_window_at_point(&self, x: i32, y: i32) -> Option<WindowHit> {
        self.windows
            .iter()
            .filter(|(_, state)| !state.hidden && state.rect.contains(x, y))
            .max_by(|(id_a, state_a), (id_b, state_b)| {
                state_a.z.cmp(&state_b.z).then_with(|| id_b.cmp(id_a))
            })
            .map(|(id, state)| WindowHit {
                id: *id,
                rect: state.rect,
                z: state.z,
            })
    }

    pub fn max_z_excluding(&self, exclude_id: ThingId) -> i32 {
        self.windows
            .iter()
            .filter(|(id, _)| **id != exclude_id)
            .map(|(_, state)| state.z)
            .max()
            .unwrap_or(0)
    }

    pub fn build_window_cycle_order(&self) -> (Vec<ThingId>, i32) {
        let mut list: Vec<(ThingId, i32)> = self
            .windows
            .iter()
            .filter(|(_, state)| !state.hidden)
            .map(|(id, state)| (*id, state.z))
            .collect();

        if list.is_empty() {
            return (Vec::new(), 0);
        }

        list.sort_by(|(a_id, a_z), (b_id, b_z)| {
            b_z.cmp(a_z)
                .then(a_id.to_u64_lossy().cmp(&b_id.to_u64_lossy()))
        });

        let max_z = list.iter().map(|(_, z)| *z).max().unwrap_or(0);
        let order = list.into_iter().map(|(id, _)| id).collect();
        (order, max_z)
    }

    #[cfg(feature = "gpu")]
    pub fn build_gpu_quads(&self) -> Vec<crate::gpu_compositor::Quad> {
        use crate::gpu_compositor::{Quad, Rect as GpuRect};

        let mut quads: Vec<Quad> = self
            .windows
            .iter()
            .filter(|(_, w)| !w.hidden && w.rect.width() > 0 && w.rect.height() > 0)
            .map(|(id, w)| Quad {
                texture_id: id.to_u64_lossy() as u32,
                dst_rect: GpuRect {
                    x: w.rect.x(),
                    y: w.rect.y(),
                    w: w.rect.width() as u32,
                    h: w.rect.height() as u32,
                },
                src_rect: None,
                opacity: 1.0,
                z: w.z as u32,
            })
            .collect();
        quads.sort_by_key(|q| q.z);
        quads
    }

    #[cfg(feature = "gpu")]
    pub fn windows_for_gpu_upload(&self) -> impl Iterator<Item = (ThingId, Rect, u64, u64)> + '_ {
        self.windows
            .iter()
            .filter(|(_, w)| !w.hidden && w.rect.width() > 0 && w.rect.height() > 0)
            .map(move |(id, w)| (*id, w.rect, w.content_gen, w.geometry_gen))
    }

    #[cfg(not(feature = "gpu"))]
    pub fn windows_for_gpu_upload(&self) -> core::iter::Empty<(ThingId, Rect, u64, u64)> {
        core::iter::empty()
    }

    pub fn compose(
        &self,
        scene: &SceneGraph,
        surface: &mut PixelBuffer,
        damage: &[Rect],
        wallpaper: Option<&Image>,
        bg_color: Color,
    ) {
        let mut ordered_surfaces = scene.ordered_surfaces.clone();
        ordered_surfaces.reverse();

        crate::trace_span!("bloom.compose");

        for damage_rect in damage {
            let d_rect = Rect::new(
                damage_rect.x(),
                damage_rect.y(),
                damage_rect.width(),
                damage_rect.height(),
            );
            let mut remaining: Vec<Rect> = vec![d_rect];

            for surf_id in &ordered_surfaces {
                if remaining.is_empty() {
                    break;
                }

                let surf = scene.surfaces.get(surf_id).unwrap();
                if !surf.visible {
                    continue;
                }

                let mut next_remaining = Vec::with_capacity(remaining.len() * 2);
                let w_rect = surf.rect();

                for r in remaining {
                    let inter = Rect::intersection(&r, &w_rect);
                    if let Some(vis) = inter {
                        let src_x = vis.x() - w_rect.x();
                        let src_y = vis.y() - w_rect.y();

                        let pixels: &[u32] = unsafe {
                            core::slice::from_raw_parts(
                                surf.buffer.as_ptr() as *const u32,
                                surf.width as usize * surf.height as usize,
                            )
                        };

                        blit_rect(
                            surface,
                            vis,
                            pixels,
                            surf.width as usize,
                            Rect::new(src_x, src_y, vis.width(), vis.height()),
                        );

                        next_remaining.extend(subtract_rect(r, vis));
                    } else {
                        next_remaining.push(r);
                    }
                }
                remaining = next_remaining;
            }

            for r in remaining {
                if let Some(wp) = wallpaper {
                    blit_wallpaper_tiled(surface, r, wp);
                } else {
                    fill_rect(surface, r, bg_color);
                }
            }
        }
    }

    fn sync_windows<F>(
        &mut self,
        scene: &mut SceneGraph,
        screen_w: i32,
        screen_h: i32,
        damage: &mut Vec<Rect>,
        on_progress: &mut F,
    ) where
        F: FnMut(),
    {
        let names = session_fs::list_dir(session_fs::WINDOWS_ROOT);
        let mut active = BTreeSet::new();

        for name in names {
            on_progress();
            session_fs::ensure_window_tree(&name);
            let id = session_fs::scene_id_from_name(&name);
            active.insert(id);

            let mut loaded =
                load_window_state(&name, screen_w, screen_h, self.windows.len() as i32);
            loaded.focused = self.focused == Some(id);
            self.install_window_watches(id, &name);
            if let Some(surface_name) = loaded.surface_name.as_ref() {
                session_fs::ensure_surface_tree(surface_name);
                self.install_surface_watch(surface_name);
            }

            let old = self.windows.get(&id).cloned();
            let is_new = old.is_none();
            if let Some(prev) = old {
                loaded.geometry_gen = if prev.rect != loaded.rect || prev.z != loaded.z {
                    prev.geometry_gen.wrapping_add(1)
                } else {
                    prev.geometry_gen
                };
                loaded.content_gen = prev.content_gen;
                loaded.last_commit = prev.last_commit;
                loaded.focused = self.focused == Some(id);

                if prev.rect != loaded.rect {
                    damage.push(prev.rect);
                    damage.push(loaded.rect);
                }
                if prev.surface_name != loaded.surface_name
                    || prev.maximized != loaded.maximized
                    || prev.fullscreen != loaded.fullscreen
                {
                    loaded.content_gen = prev.content_gen.wrapping_add(1);
                    self.dirty_windows.insert(id);
                    let serial = self.next_configure_serial;
                    self.next_configure_serial = self.next_configure_serial.wrapping_add(1);
                    send_configure(serial, &mut loaded);
                }
            } else {
                loaded.geometry_gen = 1;
                loaded.content_gen = 1;
                self.dirty_windows.insert(id);
                let serial = self.next_configure_serial;
                self.next_configure_serial = self.next_configure_serial.wrapping_add(1);
                send_configure(serial, &mut loaded);
            }

            use crate::surface::Surface;
            if scene.get_surface(id).is_none() {
                scene.insert_surface(
                    id,
                    Surface::new(
                        loaded.rect.width().max(1),
                        loaded.rect.height().max(1),
                        PixelFormat::Bgra8888,
                    ),
                );
            }

            if let Some(surf) = scene.get_surface_mut(id) {
                surf.x = loaded.rect.x();
                surf.y = loaded.rect.y();
                surf.z_index = loaded.z;
                surf.visible = loaded.mapped && !loaded.hidden;
            }

            if is_new {
                damage.push(loaded.rect);
            }
            write_window_runtime(&loaded);
            self.windows.insert(id, loaded);
        }

        let removed: Vec<ThingId> = self
            .windows
            .keys()
            .copied()
            .filter(|id| !active.contains(id))
            .collect();

        for id in removed {
            if let Some(prev) = self.windows.remove(&id) {
                damage.push(prev.rect);
                scene.remove_surface(id);
            }
            if let Some(watch) = self.window_watches.remove(&id) {
                let _ = vfs_close(watch.shell_fd);
                let _ = vfs_close(watch.requested_fd);
                let _ = vfs_close(watch.bind_fd);
            }
        }

        self.scan_required = false;
        scene.resort();
    }

    fn refresh_window_surface(&mut self, scene: &mut SceneGraph, id: ThingId) -> Option<Rect> {
        let window = self.windows.get_mut(&id)?;
        let Some(surface_name) = window.surface_name.clone() else {
            window.mapped = false;
            write_window_runtime(window);
            if let Some(surf) = scene.get_surface_mut(id) {
                surf.visible = false;
            }
            return Some(window.rect);
        };

        session_fs::ensure_surface_tree(&surface_name);
        let surface_base = session_fs::surface_path(&surface_name);
        let commit = session_fs::read_u64(&format!("{}/status/last_commit", surface_base))
            .or_else(|| session_fs::read_u64(&format!("{}/commit", surface_base)))
            .unwrap_or(0);

        if commit == 0 && window.last_commit == 0 {
            window.mapped = false;
            write_window_runtime(window);
            return Some(window.rect);
        }

        if commit == window.last_commit && window.mapped {
            return None;
        }

        let attach_text = session_fs::read_text(&format!("{}/attach", surface_base))?;
        let attached = session_fs::parse_attach_payload(&attach_text)?;
        if let Some((pixels, width, height)) = read_attached_pixels(attached) {
            if let Some(surf) = scene.get_surface_mut(id) {
                if surf.width != width as i32 || surf.height != height as i32 {
                    surf.resize(width as i32, height as i32);
                }
                let max_pixels = surf.width as usize * surf.height as usize;
                let copy_pixels = max_pixels.min(pixels.len());
                for (dst, src) in surf.buffer[..copy_pixels]
                    .chunks_exact_mut(4)
                    .zip(pixels[..copy_pixels].iter())
                {
                    dst.copy_from_slice(&src.to_le_bytes());
                }
                surf.visible = true;
                surf.add_damage(Rect::new(0, 0, width as i32, height as i32));
                surf.x = window.rect.x();
                surf.y = window.rect.y();
                surf.z_index = window.z;
            }

            window.last_commit = commit.max(window.last_commit.saturating_add(1));
            window.mapped = true;
            window.content_gen = window.content_gen.wrapping_add(1);
            window.rect.size.width = width as i32;
            window.rect.size.height = height as i32;
            write_window_runtime(window);
            let _ = session_fs::write_text(&format!("{}/status/mapped", surface_base), "1\n");
            let _ = session_fs::write_text(
                &format!("{}/status/last_commit", surface_base),
                &format!("{}\n", window.last_commit),
            );
            let _ = session_fs::write_text(
                &format!("{}/status/width", surface_base),
                &format!("{}\n", width),
            );
            let _ = session_fs::write_text(
                &format!("{}/status/height", surface_base),
                &format!("{}\n", height),
            );
            let _ =
                session_fs::write_text(&format!("{}/status/buffer_attached", surface_base), "1\n");
            return Some(window.rect);
        }

        None
    }

    fn install_window_watches(&mut self, id: ThingId, name: &str) {
        if self.window_watches.contains_key(&id) {
            return;
        }
        let base = session_fs::window_path(name);
        let shell_fd = open_watch(
            &format!("{}/shell", base),
            watch_mask::MODIFY | watch_mask::CREATE | watch_mask::REMOVE,
        )
        .unwrap_or(0);
        let requested_fd = open_watch(
            &format!("{}/shell/requested", base),
            watch_mask::MODIFY | watch_mask::CREATE | watch_mask::REMOVE,
        )
        .unwrap_or(0);
        let bind_fd = open_watch(
            &format!("{}/bind", base),
            watch_mask::MODIFY | watch_mask::CREATE | watch_mask::REMOVE,
        )
        .unwrap_or(0);
        self.window_watches.insert(
            id,
            WindowWatchState {
                shell_fd,
                requested_fd,
                bind_fd,
            },
        );
    }

    fn install_surface_watch(&mut self, surface_name: &str) {
        if self.surface_watches.contains_key(surface_name) {
            return;
        }
        if let Some(fd) = open_watch(
            &session_fs::surface_path(surface_name),
            watch_mask::MODIFY | watch_mask::CREATE | watch_mask::REMOVE,
        ) {
            self.surface_watches.insert(surface_name.to_string(), fd);
        }
    }
}

impl Drop for PaintPipeline {
    fn drop(&mut self) {
        if let Some(fd) = self.windows_watch.take() {
            let _ = vfs_close(fd);
        }
        if let Some(fd) = self.surfaces_watch.take() {
            let _ = vfs_close(fd);
        }
        for (_, watch) in self.window_watches.iter() {
            let _ = vfs_close(watch.shell_fd);
            let _ = vfs_close(watch.requested_fd);
            let _ = vfs_close(watch.bind_fd);
        }
        for (_, fd) in self.surface_watches.iter() {
            let _ = vfs_close(*fd);
        }
    }
}

fn send_configure(serial: u64, window: &mut WindowPaintState) {
    let mut states = Vec::new();
    if window.focused {
        states.push("activated");
    }
    if window.maximized {
        states.push("maximized");
    }
    if window.fullscreen {
        states.push("fullscreen");
    }

    let line = session_fs::encode_configure_event(
        serial,
        window.rect.width(),
        window.rect.height(),
        &states,
    );
    let _ = session_fs::append_line(
        &format!("{}/events", session_fs::window_path(&window.name)),
        &line,
    );
    let _ = session_fs::write_text(
        &format!(
            "{}/status/last_configure_serial",
            session_fs::window_path(&window.name)
        ),
        &format!("{}\n", serial),
    );
}

fn write_window_runtime(window: &WindowPaintState) {
    write_window_geometry(window);
    write_window_focus(window);
    let base = session_fs::window_path(&window.name);
    let _ = session_fs::write_text(
        &format!("{}/status/mapped", base),
        if window.mapped { "1\n" } else { "0\n" },
    );
    let _ = session_fs::write_text(
        &format!("{}/shell/current/maximized", base),
        if window.maximized { "1\n" } else { "0\n" },
    );
    let _ = session_fs::write_text(
        &format!("{}/shell/current/fullscreen", base),
        if window.fullscreen { "1\n" } else { "0\n" },
    );
    let _ = session_fs::write_text(
        &format!("{}/shell/current/activated", base),
        if window.focused { "1\n" } else { "0\n" },
    );
}

fn write_window_geometry(window: &WindowPaintState) {
    let base = session_fs::window_path(&window.name);
    let _ = session_fs::write_text(
        &format!("{}/shell/current/x", base),
        &format!("{}\n", window.rect.x()),
    );
    let _ = session_fs::write_text(
        &format!("{}/shell/current/y", base),
        &format!("{}\n", window.rect.y()),
    );
    let _ = session_fs::write_text(
        &format!("{}/shell/current/width", base),
        &format!("{}\n", window.rect.width()),
    );
    let _ = session_fs::write_text(
        &format!("{}/shell/current/height", base),
        &format!("{}\n", window.rect.height()),
    );
    let _ = session_fs::write_text(
        &format!("{}/shell/current/z", base),
        &format!("{}\n", window.z),
    );
}

fn write_window_focus(window: &WindowPaintState) {
    let base = session_fs::window_path(&window.name);
    let _ = session_fs::write_text(
        &format!("{}/status/focused", base),
        if window.focused { "1\n" } else { "0\n" },
    );
}

fn load_window_state(name: &str, screen_w: i32, screen_h: i32, ordinal: i32) -> WindowPaintState {
    let base = session_fs::window_path(name);
    let x = read_i32(&format!("{}/shell/current/x", base))
        .unwrap_or(48 + (ordinal % 8) * 36)
        .clamp(0, screen_w.saturating_sub(64));
    let y = read_i32(&format!("{}/shell/current/y", base))
        .unwrap_or(48 + (ordinal % 6) * 28)
        .clamp(0, screen_h.saturating_sub(64));
    let width = read_i32(&format!("{}/shell/current/width", base))
        .unwrap_or(640)
        .max(1);
    let height = read_i32(&format!("{}/shell/current/height", base))
        .unwrap_or(480)
        .max(1);
    let z = read_i32(&format!("{}/shell/current/z", base)).unwrap_or(ordinal + 1);
    let title = session_fs::read_text(&format!("{}/shell/title", base)).unwrap_or_default();
    let app_id = session_fs::read_text(&format!("{}/shell/app_id", base)).unwrap_or_default();
    let surface_name =
        session_fs::read_text(&format!("{}/bind/surface", base)).filter(|s| !s.is_empty());
    let maximized = session_fs::read_bool(&format!("{}/shell/requested/maximize", base));
    let fullscreen = session_fs::read_bool(&format!("{}/shell/requested/fullscreen", base));
    let hidden = session_fs::read_bool(&format!("{}/status/closing", base));
    let mapped = session_fs::read_bool(&format!("{}/status/mapped", base));

    WindowPaintState {
        name: name.to_string(),
        rect: Rect::new(x, y, width, height),
        z,
        hidden,
        surface_name,
        title,
        app_id,
        mapped,
        focused: false,
        maximized,
        fullscreen,
        geometry_gen: 0,
        content_gen: 0,
        last_commit: 0,
    }
}

fn open_watch(path: &str, mask: u32) -> Option<u32> {
    vfs_watch_path(path, mask, watch_flags::NONBLOCK | watch_flags::ONLYDIR).ok()
}

fn drain_watch(fd: Option<u32>) -> bool {
    let Some(fd) = fd else {
        return false;
    };
    if fd == 0 {
        return false;
    }
    let mut saw_event = false;
    loop {
        let mut buf = [0u8; WATCH_BUFFER_BYTES];
        match vfs_read(fd, &mut buf) {
            Ok(0) => break,
            Ok(_) => saw_event = true,
            Err(Errno::EAGAIN) => break,
            Err(_) => break,
        }
    }
    saw_event
}

fn read_attached_pixels(attached: AttachedBuffer) -> Option<(Vec<u32>, u32, u32)> {
    use abi::vm::{VmBacking, VmMapReq, VmProt};

    let len = attached.stride as usize * attached.height as usize;
    let req = VmMapReq {
        addr_hint: 0,
        len,
        prot: VmProt::READ | VmProt::USER,
        flags: abi::vm::VmMapFlags::empty(),
        backing: VmBacking::File {
            fd: attached.fd,
            offset: 0,
        },
    };
    let resp = stem::thing::sys::vm_map(&req).ok()?;
    let src = unsafe { core::slice::from_raw_parts(resp.addr as *const u8, len) };
    let width = attached.width.max(1);
    let height = attached.height.max(1);
    let row_pixels = width as usize;
    let mut out = vec![0u32; row_pixels * height as usize];

    for y in 0..height as usize {
        let src_off = y * attached.stride as usize;
        let src_row = &src[src_off..src_off + row_pixels * 4];
        for x in 0..row_pixels {
            let off = x * 4;
            out[y * row_pixels + x] = u32::from_le_bytes([
                src_row[off],
                src_row[off + 1],
                src_row[off + 2],
                src_row[off + 3],
            ]);
        }
    }

    Some((out, width, height))
}

fn read_i32(path: &str) -> Option<i32> {
    let text = session_fs::read_text(path)?;
    if text.is_empty() {
        return None;
    }
    if let Some(hex) = text.strip_prefix("0x") {
        i32::from_str_radix(hex, 16).ok()
    } else {
        text.parse::<i32>().ok()
    }
}

fn subtract_rect(base: Rect, cut: Rect) -> Vec<Rect> {
    let mut out = Vec::with_capacity(4);

    if cut.y() > base.y() {
        out.push(Rect::new(
            base.x(),
            base.y(),
            base.width(),
            cut.y() - base.y(),
        ));
    }
    if cut.y() + cut.height() < base.y() + base.height() {
        let y1 = cut.y() + cut.height();
        out.push(Rect::new(
            base.x(),
            y1,
            base.width(),
            (base.y() + base.height()) - y1,
        ));
    }

    let y0 = core::cmp::max(base.y(), cut.y());
    let y1 = core::cmp::min(base.y() + base.height(), cut.y() + cut.height());
    let h = y1 - y0;

    if h > 0 {
        if cut.x() > base.x() {
            out.push(Rect::new(base.x(), y0, cut.x() - base.x(), h));
        }
        if cut.x() + cut.width() < base.x() + base.width() {
            let x1 = cut.x() + cut.width();
            out.push(Rect::new(x1, y0, (base.x() + base.width()) - x1, h));
        }
    }

    out
}

fn blit_rect(
    dst: &mut PixelBuffer,
    dst_rect: Rect,
    src_pixels: &[u32],
    src_stride: usize,
    src_rect: Rect,
) {
    let dw = dst.width();
    let dh = dst.height();
    let dx = dst_rect.x();
    let dy = dst_rect.y();
    let w = dst_rect.width();
    let h = dst_rect.height();

    for iy in 0..h {
        let sy = src_rect.y() + iy;
        let d_y = dy + iy;
        if d_y < 0 || d_y >= dh {
            continue;
        }

        let mut d_off = (d_y as usize * dst.stride_bytes) + (dx as usize * 4);
        let mut s_off = (sy as usize * src_stride) + (src_rect.x() as usize);

        for ix in 0..w {
            let d_x = dx + ix;
            if d_x < 0 || d_x >= dw {
                d_off += 4;
                s_off += 1;
                continue;
            }

            let src_px = src_pixels[s_off];
            let sa = (src_px >> 24) & 0xFF;
            if sa == 0 {
                d_off += 4;
                s_off += 1;
                continue;
            }

            let output = if sa == 0xFF {
                src_px
            } else {
                blend_pixel(src_px, unsafe {
                    let ptr = dst.ptr.add(d_off);
                    let b = *ptr;
                    let g = *ptr.add(1);
                    let r = *ptr.add(2);
                    Color::rgb(r, g, b).to_u32()
                })
            };

            unsafe {
                let bytes = output.to_le_bytes();
                core::ptr::copy_nonoverlapping(bytes.as_ptr(), dst.ptr.add(d_off), 4);
            }

            d_off += 4;
            s_off += 1;
        }
    }
}

fn blend_pixel(src: u32, dst: u32) -> u32 {
    let sa = (src >> 24) & 0xFF;
    if sa == 0 {
        return dst;
    }
    if sa == 255 {
        return src;
    }

    let sr = (src >> 16) & 0xFF;
    let sg = (src >> 8) & 0xFF;
    let sb = src & 0xFF;

    let dr = (dst >> 16) & 0xFF;
    let dg = (dst >> 8) & 0xFF;
    let db = dst & 0xFF;

    let inv_a = 255 - sa;
    let r = (sr * sa + dr * inv_a) / 255;
    let g = (sg * sa + dg * inv_a) / 255;
    let b = (sb * sa + db * inv_a) / 255;

    (0xFF << 24) | (r << 16) | (g << 8) | b
}

fn fill_rect(dst: &mut PixelBuffer, rect: Rect, color: Color) {
    let c = color.to_u32();
    let dw = dst.width();
    let dh = dst.height();

    let x0 = core::cmp::max(0, rect.x());
    let y0 = core::cmp::max(0, rect.y());
    let x1 = core::cmp::min(dw, rect.x() + rect.width());
    let y1 = core::cmp::min(dh, rect.y() + rect.height());

    if x1 <= x0 || y1 <= y0 {
        return;
    }

    for y in y0..y1 {
        for x in x0..x1 {
            dst.put_px(x, y, c);
        }
    }
}

fn blit_wallpaper_tiled(dst: &mut PixelBuffer, rect: Rect, wp: &Image) {
    let dw = dst.width();
    let dh = dst.height();
    let ww = wp.width as i32;
    let wh = wp.height as i32;

    if ww == 0 || wh == 0 {
        return;
    }

    let x0 = core::cmp::max(0, rect.x());
    let y0 = core::cmp::max(0, rect.y());
    let x1 = core::cmp::min(dw, rect.x() + rect.width());
    let y1 = core::cmp::min(dh, rect.y() + rect.height());

    if x1 <= x0 || y1 <= y0 {
        return;
    }

    let pixels: &[u32] = unsafe { core::mem::transmute(&*wp.pixels) };
    for y in y0..y1 {
        let wy = y % wh;
        for x in x0..x1 {
            let wx = x % ww;
            let p = pixels[(wy * ww + wx) as usize];
            dst.put_px(x, y, p);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_id(n: u8) -> ThingId {
        let mut b = [0u8; 16];
        b[0] = n;
        ThingId(b)
    }

    #[test]
    fn test_top_window_z_priority() {
        let mut pipeline = PaintPipeline::new();
        pipeline.windows.insert(
            make_id(1),
            WindowPaintState {
                name: "a".into(),
                rect: Rect::new(0, 0, 100, 100),
                z: 5,
                hidden: false,
                surface_name: None,
                title: String::new(),
                app_id: String::new(),
                mapped: true,
                focused: false,
                maximized: false,
                fullscreen: false,
                geometry_gen: 0,
                content_gen: 0,
                last_commit: 0,
            },
        );
        pipeline.windows.insert(
            make_id(2),
            WindowPaintState {
                name: "b".into(),
                rect: Rect::new(0, 0, 100, 100),
                z: 10,
                hidden: false,
                surface_name: None,
                title: String::new(),
                app_id: String::new(),
                mapped: true,
                focused: false,
                maximized: false,
                fullscreen: false,
                geometry_gen: 0,
                content_gen: 0,
                last_commit: 0,
            },
        );

        let hit = pipeline.top_window_at_point(50, 50).expect("hit");
        assert_eq!(hit.id, make_id(2));
    }
}
