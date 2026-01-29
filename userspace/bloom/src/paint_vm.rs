extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use abi::schema::{keys, kinds};
use abi::types::HandleId;
use abi::ui_paint::{PaintOpTag, PaintReader};
use stem::thing::sys::{bytespace_info, bytespace_read, find, prop_get};
use stem::thing::ThingId;

use crate::asset::Image;
use crate::damage;
use crate::drawlist::{DrawCmd, DrawList};
use crate::frame::AssetGeneration;
use crate::geometry::{Color, EdgeAA, Point, Rect}; // Added Point import if needed, already used in damage logic but good to align
use crate::raster;
use crate::surface::Surface;
use alloc::sync::Arc;
use core::cmp::{max, min};

pub struct WindowHit {
    pub id: ThingId,
    pub rect: Rect,
    pub z: i32,
}

struct WindowPaintState {
    rect: Rect,
    z: i32,
    hidden: bool,
    paint_gen: u64,
    paint_bs: u64,
    image: Option<Image>,
    buffer: Vec<u32>,
}

pub struct PaintResult {
    pub damage: Vec<Rect>,
}

pub struct PaintPipeline {
    windows: BTreeMap<ThingId, WindowPaintState>,
}

impl PaintPipeline {
    pub fn new() -> Self {
        Self {
            windows: BTreeMap::new(),
        }
    }

    pub fn process_updates(&mut self, screen_w: i32, screen_h: i32) -> PaintResult {
        let mut damage = Vec::new();
        let mut window_ids = [ThingId::default(); 128];
        let count = find(kinds::UI_WINDOW, &mut window_ids).unwrap_or(0);
        let mut active: Vec<ThingId> = Vec::new();

        for id in window_ids.iter().take(count) {
            active.push(*id);
            let rect = window_rect(*id, screen_w, screen_h);
            if rect.width() <= 0 || rect.height() <= 0 {
                continue;
            }

            let paint_gen = prop_get(*id, keys::UI_PAINT_GEN).unwrap_or(0);
            let paint_bs = prop_get(*id, keys::UI_PAINT_BYTESPACE).unwrap_or(0);
            let z = prop_get(*id, keys::UI_Z_INDEX).unwrap_or(0) as i32;
            let hidden = prop_get(*id, keys::UI_HIDDEN).unwrap_or(0) != 0;
            let mut needs_rebuild = false;
            let entry = self.windows.entry(*id).or_insert_with(|| WindowPaintState {
                rect: Rect::new(0, 0, 0, 0),
                z: 0,
                hidden: false,
                paint_gen: 0,
                paint_bs: 0,
                image: None,
                buffer: Vec::new(),
            });

            if entry.paint_gen != paint_gen || entry.paint_bs != paint_bs {
                needs_rebuild = true;
            }
            if entry.rect != rect || entry.z != z || entry.hidden != hidden {
                needs_rebuild = true;
                if entry.rect != rect || entry.hidden != hidden {
                    damage.push(Rect::new(
                        entry.rect.x(),
                        entry.rect.y(),
                        entry.rect.width(),
                        entry.rect.height(),
                    ));
                }
            }

            if needs_rebuild {
                entry.rect = rect;
                entry.z = z;
                entry.hidden = hidden;
                entry.paint_gen = paint_gen;
                entry.paint_bs = paint_bs;

                // Rasterize window content into cached surface
                let w = rect.width() as usize;
                let h = rect.height() as usize;
                let len = w * h;

                // Resize buffer if needed
                if entry.buffer.len() != len {
                    entry.buffer.resize(len, 0);
                }

                // Execute drawlist into buffer
                if w > 0 && h > 0 {
                    // Create wrapper surface for the buffer
                    // SAFETY: buffer is valid for len, valid dimensions
                    let mut surface = unsafe {
                        Surface::new(
                            entry.buffer.as_mut_ptr() as *mut u8,
                            len * 4,
                            w as u32,
                            h as u32,
                            w as u32 * 4,
                        )
                    };

                    // Clear (using transparent black or window background logic - assuming transparent)
                    // Windows usually clear themselves, but let's ensure a clean slate.
                    for i in 0..len {
                        entry.buffer[i] = 0; // Transparent
                    }

                    // Build and execute local drawlist
                    let local_rect = Rect::new(0, 0, rect.width(), rect.height());
                    let list = build_drawlist(paint_bs, local_rect);
                    raster::execute(&mut surface, &list, false);

                    // Update Cached Image
                    entry.image = Some(Image {
                        width: w as u32,
                        height: h as u32,
                        pixels: Arc::from(entry.buffer.clone().into_boxed_slice()),
                        gen: AssetGeneration::ZERO,
                        name: Arc::from("window_cache"),
                        id: None,
                    });
                } else {
                    entry.image = None;
                }

                damage.push(Rect::new(rect.x(), rect.y(), rect.width(), rect.height()));
            }
        }

        self.windows.retain(|id, _| active.contains(id));

        PaintResult { damage }
    }

    pub fn top_window_at_point(&self, x: i32, y: i32) -> Option<WindowHit> {
        self.windows
            .iter()
            .filter(|(_, state)| !state.hidden && state.rect.contains(x, y))
            .max_by_key(|(_, state)| state.z)
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

    /// Compose the scene into the framebuffer surface using occlusion culling.
    pub fn compose(
        &self,
        surface: &mut Surface,
        damage: &[Rect],
        wallpaper: Option<&Image>,
        bg_color: Color,
    ) {
        let mut ordered: Vec<&WindowPaintState> =
            self.windows.values().filter(|w| !w.hidden).collect();
        // Sort by Z descending (top to bottom) for occlusion
        ordered.sort_by_key(|w| -w.z);

        crate::trace_span!("bloom.compose");

        for damage_rect in damage {
            let d_rect: Rect = Rect::new(
                damage_rect.x(),
                damage_rect.y(),
                damage_rect.width(),
                damage_rect.height(),
            );
            let mut remaining: Vec<Rect> = vec![d_rect];

            for win in &ordered {
                if remaining.is_empty() {
                    break;
                }

                let mut next_remaining = Vec::with_capacity(remaining.len() * 2);
                let w_rect = win.rect;

                for r in remaining {
                    let inter = Rect::intersection(&r, &w_rect);
                    if let Some(vis) = inter {
                        // This part of 'r' is covered by 'win'.
                        // Blit 'win' portion to 'vis'.
                        // Use intersection with valid image area (0,0,w,h) relative to window
                        if let Some(img) = &win.image {
                            if let Some(win_buffer) = &win.image.as_ref().map(|_| &win.buffer) {
                                // Calculate src rect in window coordinates
                                let src_x = vis.x() - w_rect.x();
                                let src_y = vis.y() - w_rect.y();

                                // Blit logic
                                blit_rect(
                                    surface,
                                    vis,
                                    win_buffer,
                                    img.width as usize, // stride
                                    Rect::new(src_x, src_y, vis.width(), vis.height()),
                                );
                            }
                        }

                        // Subtract vis from r
                        next_remaining.extend(subtract_rect(r, vis));
                    } else {
                        next_remaining.push(r);
                    }
                }
                remaining = next_remaining;
            }

            // Fill background for remaining
            for r in remaining {
                if let Some(wp) = wallpaper {
                    // Blit wallpaper tiled
                    blit_wallpaper_tiled(surface, r, wp);
                } else {
                    fill_rect(surface, r, bg_color);
                }
            }
        }
    }
}

fn subtract_rect(base: Rect, cut: Rect) -> Vec<Rect> {
    // assumes cut intersects base (guaranteed by caller logic usually, but intersection check handles subset)
    // cut must be within base for this simple logic? No, cut is intersection(base, win), so cut IS within base.

    let mut out = Vec::with_capacity(4);

    // Top
    if cut.y() > base.y() {
        out.push(Rect::new(
            base.x(),
            base.y(),
            base.width(),
            cut.y() - base.y(),
        ));
    }
    // Bottom
    if cut.y() + cut.height() < base.y() + base.height() {
        let y1 = cut.y() + cut.height();
        out.push(Rect::new(
            base.x(),
            y1,
            base.width(),
            (base.y() + base.height()) - y1,
        ));
    }

    // Left (be careful with y range - middle strip)
    let y0 = max(base.y(), cut.y());
    let y1 = min(base.y() + base.height(), cut.y() + cut.height());
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
    dst: &mut Surface,
    dst_rect: Rect,
    src_pixels: &[u32],
    src_stride: usize,
    src_rect: Rect,
) {
    // Simple copy/blend
    // src_pixels are assumed to be 0xAARRGGBB

    let dw = dst.width();
    let dh = dst.height();

    // Clip dst_rect to surface
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
        // src_pixels is u32 slice, stride is u32 count

        for ix in 0..w {
            let sx = src_rect.x() + ix;
            let d_x = dx + ix;

            if d_x < 0 || d_x >= dw {
                d_off += 4;
                s_off += 1;
                continue;
            }

            let src_px = src_pixels[s_off];
            // Blend
            let output = blend_pixel(src_px, unsafe {
                // Read current dst
                let ptr = dst.ptr.add(d_off);
                let b = *ptr;
                let g = *ptr.add(1);
                let r = *ptr.add(2);
                // DST is BGRX (ignore alpha/assume 255)
                Color::rgb(r, g, b).to_u32()
            });

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

    // SrcOver
    // out = src * α + dst * (1 - α)
    let inv_a = 255 - sa;

    let r = (sr * sa + dr * inv_a) / 255;
    let g = (sg * sa + dg * inv_a) / 255;
    let b = (sb * sa + db * inv_a) / 255;

    (0xFF << 24) | (r << 16) | (g << 8) | b
}

fn fill_rect(dst: &mut Surface, rect: Rect, color: Color) {
    let c = color.to_u32();
    let dw = dst.width();
    let dh = dst.height();

    let x0 = max(0, rect.x());
    let y0 = max(0, rect.y());
    let x1 = min(dw, rect.x() + rect.width());
    let y1 = min(dh, rect.y() + rect.height());

    if x1 <= x0 || y1 <= y0 {
        return;
    }

    for y in y0..y1 {
        for x in x0..x1 {
            dst.put_px(x, y, c);
        }
    }
}

fn blit_wallpaper_tiled(dst: &mut Surface, rect: Rect, wp: &Image) {
    // Similar to fill_rect logic but sampling wp
    let dw = dst.width();
    let dh = dst.height();
    let ww = wp.width as i32;
    let wh = wp.height as i32;

    if ww == 0 || wh == 0 {
        return;
    }

    let x0 = max(0, rect.x());
    let y0 = max(0, rect.y());
    let x1 = min(dw, rect.x() + rect.width());
    let y1 = min(dh, rect.y() + rect.height());

    if x1 <= x0 || y1 <= y0 {
        return;
    }

    // WP pixels are likely packed u32 or u8? Image has `pixels: Arc<[u32]>`. Wait, Image struct in asset.rs: `pixels: Arc<Box<[u32]>>` ?
    // Let's check Image struct def.
    // asset.rs: `pub pixels: Arc<[u32]>` (lines not fully shown but likely u32 based on usage in raster).
    // Actually, `asset::Image` usually stores u32 pixels.
    // I need to assume it is u32 slice.

    let pixels: &[u32] = unsafe { core::mem::transmute(&*wp.pixels) }; // Safety: Should already be castable or access methods?
                                                                       // Wait, let's verify Image struct.

    for y in y0..y1 {
        let wy = y % wh;
        for x in x0..x1 {
            let wx = x % ww;
            let p = pixels[(wy * ww + wx) as usize];
            dst.put_px(x, y, p); // WP usually opaque
        }
    }
}

fn window_rect(window_id: ThingId, screen_w: i32, screen_h: i32) -> Rect {
    let mut w = prop_get(window_id, keys::UI_WIDTH).unwrap_or(0) as i32;
    let mut h = prop_get(window_id, keys::UI_HEIGHT).unwrap_or(0) as i32;
    if w <= 0 || h <= 0 {
        return Rect::new(0, 0, 0, 0);
    }
    let mut x = prop_get(window_id, keys::UI_X).unwrap_or(0) as i32;
    let mut y = prop_get(window_id, keys::UI_Y).unwrap_or(0) as i32;
    let inset_right = prop_get(window_id, keys::UI_INSET_RIGHT).unwrap_or(0) as i32;
    let inset_bottom = prop_get(window_id, keys::UI_INSET_BOTTOM).unwrap_or(0) as i32;
    if inset_right > 0 {
        x = screen_w - inset_right - w;
    }
    if inset_bottom > 0 {
        y = screen_h - inset_bottom - h;
    }
    Rect::new(x, y, w, h)
}

fn build_drawlist(paint_bs: u64, rect: Rect) -> DrawList {
    let mut list = DrawList::new();
    if paint_bs == 0 {
        return list;
    }
    if rect.width() <= 0 || rect.height() <= 0 {
        return list;
    }
    list.commands().push(DrawCmd::PushClip { rect });
    let origin_x = rect.x();
    let origin_y = rect.y();
    let bytes = match read_bytespace(ThingId::from_u64(paint_bs)) {
        Ok(bytes) => bytes,
        Err(_) => return list,
    };
    let mut reader = match PaintReader::new(&bytes) {
        Some(reader) => reader,
        None => return list,
    };
    while let Some(op) = reader.next() {
        match op.tag {
            PaintOpTag::PushClip => {
                if let Some((x, y, w, h)) = decode_rect(op.payload) {
                    list.commands().push(DrawCmd::PushClip {
                        rect: Rect::new(x + origin_x, y + origin_y, w, h),
                    });
                }
            }
            PaintOpTag::PopClip => {
                list.commands().push(DrawCmd::PopClip);
            }
            PaintOpTag::FillRect => {
                if let Some((x, y, w, h, color)) = decode_fill_rect(op.payload) {
                    list.commands().push(DrawCmd::FillRect {
                        rect: Rect::new(x + origin_x, y + origin_y, w, h),
                        color: Color::from_u32(color),
                        aa: EdgeAA::None,
                    });
                }
            }
            PaintOpTag::DrawTextRun => {
                if let Some(text) = decode_text_run(op.payload) {
                    list.commands().push(DrawCmd::Text {
                        text: text.text,
                        font: Some(text.font),
                        rect: Rect::new(text.x + origin_x, text.y + origin_y, text.w, text.h),
                        size: text.size as f32,
                        color: Color::from_u32(text.color),
                        font_debug: false,
                    });
                }
            }
            PaintOpTag::BlitImage => {
                // TODO: hook into asset/image cache by key
            }
            PaintOpTag::StrokeLine => {
                if let Some((x1, y1, x2, y2, width, color)) = decode_line(op.payload) {
                    list.commands().push(DrawCmd::Line {
                        from: crate::isa::PointF::new(
                            (x1 + origin_x) as f32,
                            (y1 + origin_y) as f32,
                        ),
                        to: crate::isa::PointF::new((x2 + origin_x) as f32, (y2 + origin_y) as f32),
                        color: Color::from_u32(color),
                        width: width as f32,
                    });
                }
            }
            PaintOpTag::DrawIcon => {
                if let Some((x, y, w, h, name)) = decode_icon(op.payload) {
                    if let Ok(id) = stem::thing::sys::intern(&name) {
                        list.commands().push(DrawCmd::Icon {
                            icon_name_id: id,
                            dest: Rect::new(x + origin_x, y + origin_y, w, h),
                        });
                    }
                }
            }
            PaintOpTag::FillLinearGradient => {
                if let Some((x, y, w, h, c1, c2)) =
                    abi::ui_paint::decode_fill_linear_gradient(op.payload)
                {
                    list.commands().push(DrawCmd::FillLinearGradient {
                        rect: Rect::new(x + origin_x, y + origin_y, w, h),
                        color1: Color::from_u32(c1),
                        color2: Color::from_u32(c2),
                    });
                }
            }
            _ => {}
        }
    }
    list.commands().push(DrawCmd::PopClip);
    list
}

fn decode_rect(payload: &[u8]) -> Option<(i32, i32, i32, i32)> {
    if payload.len() < 16 {
        return None;
    }
    let x = i32::from_le_bytes(payload[0..4].try_into().ok()?);
    let y = i32::from_le_bytes(payload[4..8].try_into().ok()?);
    let w = i32::from_le_bytes(payload[8..12].try_into().ok()?);
    let h = i32::from_le_bytes(payload[12..16].try_into().ok()?);
    Some((x, y, w, h))
}

fn decode_fill_rect(payload: &[u8]) -> Option<(i32, i32, i32, i32, u32)> {
    if payload.len() < 20 {
        return None;
    }
    let x = i32::from_le_bytes(payload[0..4].try_into().ok()?);
    let y = i32::from_le_bytes(payload[4..8].try_into().ok()?);
    let w = i32::from_le_bytes(payload[8..12].try_into().ok()?);
    let h = i32::from_le_bytes(payload[12..16].try_into().ok()?);
    let color = u32::from_le_bytes(payload[16..20].try_into().ok()?);
    Some((x, y, w, h, color))
}

fn decode_line(payload: &[u8]) -> Option<(i32, i32, i32, i32, i32, u32)> {
    if payload.len() < 24 {
        return None;
    }
    let x1 = i32::from_le_bytes(payload[0..4].try_into().ok()?);
    let y1 = i32::from_le_bytes(payload[4..8].try_into().ok()?);
    let x2 = i32::from_le_bytes(payload[8..12].try_into().ok()?);
    let y2 = i32::from_le_bytes(payload[12..16].try_into().ok()?);
    let width = i32::from_le_bytes(payload[16..20].try_into().ok()?);
    let color = u32::from_le_bytes(payload[20..24].try_into().ok()?);
    Some((x1, y1, x2, y2, width, color))
}

fn decode_icon(payload: &[u8]) -> Option<(i32, i32, i32, i32, String)> {
    if payload.len() < 20 {
        return None;
    }
    let x = i32::from_le_bytes(payload[0..4].try_into().ok()?);
    let y = i32::from_le_bytes(payload[4..8].try_into().ok()?);
    let w = i32::from_le_bytes(payload[8..12].try_into().ok()?);
    let h = i32::from_le_bytes(payload[12..16].try_into().ok()?);
    let name_len = u32::from_le_bytes(payload[16..20].try_into().ok()?);
    let start = 20;
    let end = start + name_len as usize;
    if end > payload.len() {
        return None;
    }
    let name = String::from(core::str::from_utf8(&payload[start..end]).ok()?);
    Some((x, y, w, h, name))
}

struct TextRunDecoded {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    size: i32,
    color: u32,
    font: String,
    text: String,
}

fn decode_text_run(payload: &[u8]) -> Option<TextRunDecoded> {
    if payload.len() < 36 {
        return None;
    }
    let x = i32::from_le_bytes(payload[0..4].try_into().ok()?);
    let y = i32::from_le_bytes(payload[4..8].try_into().ok()?);
    let w = i32::from_le_bytes(payload[8..12].try_into().ok()?);
    let h = i32::from_le_bytes(payload[12..16].try_into().ok()?);
    let _baseline = i32::from_le_bytes(payload[16..20].try_into().ok()?);
    let size = i32::from_le_bytes(payload[20..24].try_into().ok()?);
    let color = u32::from_le_bytes(payload[24..28].try_into().ok()?);
    let font_len = u32::from_le_bytes(payload[28..32].try_into().ok()?);
    let text_len = u32::from_le_bytes(payload[32..36].try_into().ok()?);
    let start = 36;
    let font_end = start + font_len as usize;
    let text_end = font_end + text_len as usize;
    if text_end > payload.len() {
        return None;
    }
    let font = String::from(core::str::from_utf8(&payload[start..font_end]).ok()?);
    let text = String::from(core::str::from_utf8(&payload[font_end..text_end]).ok()?);
    Some(TextRunDecoded {
        x,
        y,
        w,
        h,
        size,
        color,
        font,
        text,
    })
}

fn read_bytespace(bs_id: ThingId) -> Result<Vec<u8>, abi::errors::Errno> {
    let size = bytespace_info(bs_id)?;
    if size == 0 {
        return Ok(Vec::new());
    }
    let mut out = Vec::with_capacity(size);
    out.resize(size, 0);
    let mut offset = 0usize;
    while offset < size {
        let end = core::cmp::min(offset + 4096, size);
        let read = bytespace_read(bs_id, offset, &mut out[offset..end])?;
        if read == 0 {
            break;
        }
        offset = offset.saturating_add(read);
    }
    Ok(out)
}
