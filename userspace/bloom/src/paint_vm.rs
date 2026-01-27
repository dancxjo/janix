extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use abi::ui_paint::{PaintOpTag, PaintReader};
use abi::schema::{keys, kinds};
use abi::types::HandleId;
use stem::thing::ThingId;
use stem::thing::sys::{bytespace_info, bytespace_read, find, prop_get};

use crate::damage;
use crate::drawlist::{DrawCmd, DrawList};
use crate::geometry::{Color, Rect, EdgeAA};

struct WindowPaintState {
    rect: Rect,
    z: i32,
    paint_gen: u64,
    paint_bs: u64,
    list: DrawList,
}

pub struct PaintResult {
    pub damage: Vec<damage::Rect>,
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

    pub fn run(&mut self, screen_w: i32, screen_h: i32, list: &mut DrawList) -> PaintResult {
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
            let mut needs_rebuild = false;
            let entry = self.windows.entry(*id).or_insert_with(|| WindowPaintState {
                rect: Rect::new(0, 0, 0, 0),
                z: 0,
                paint_gen: 0,
                paint_bs: 0,
                list: DrawList::new(),
            });

            if entry.paint_gen != paint_gen || entry.paint_bs != paint_bs {
                needs_rebuild = true;
            }
            if entry.rect != rect || entry.z != z {
                needs_rebuild = true;
            }

            if needs_rebuild {
                entry.rect = rect;
                entry.z = z;
                entry.paint_gen = paint_gen;
                entry.paint_bs = paint_bs;
                entry.list = build_drawlist(paint_bs, rect);
                damage.push(damage::Rect::new(
                    rect.x(),
                    rect.y(),
                    rect.width(),
                    rect.height(),
                ));
            }
        }

        self.windows.retain(|id, _| active.contains(id));

        // Draw in z-order
        let mut ordered: Vec<&WindowPaintState> = self.windows.values().collect();
        ordered.sort_by_key(|w| w.z);
        for win in ordered {
            win.list.append_to(list);
        }

        PaintResult { damage }
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
                        rect: Rect::new(
                            text.x + origin_x,
                            text.y + origin_y,
                            text.w,
                            text.h,
                        ),
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
                        from: crate::isa::PointF::new((x1 + origin_x) as f32, (y1 + origin_y) as f32),
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
