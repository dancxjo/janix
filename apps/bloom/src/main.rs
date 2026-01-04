#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use thing_std::graph::*;
use thing_std::*;

mod cursor;

use cursor::{CursorAnimator, CursorAsset, CursorFrame};

static mut BACK_BUFFER: Option<Vec<u32>> = None;
static mut WALLPAPER_CACHE: Option<Vec<u32>> = None;

// Mouse ring buffer state
static mut MOUSE_RING_PTR: Option<*const u8> = None;
static mut MOUSE_READ_IDX: u32 = 0;
static mut POINTER_X: i32 = 640;
static mut POINTER_Y: i32 = 360;
static mut POINTER_BUTTONS: u16 = 0;
static mut SCREEN_WIDTH: u32 = 1280;
static mut SCREEN_HEIGHT: u32 = 720;

#[derive(Clone, Copy)]
struct Rect {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
}

impl Rect {
    fn union(a: Rect, b: Rect) -> Rect {
        let x1 = a.x.min(b.x);
        let y1 = a.y.min(b.y);
        let x2 = (a.x + a.w as i32).max(b.x + b.w as i32);
        let y2 = (a.y + a.h as i32).max(b.y + b.h as i32);
        Rect {
            x: x1,
            y: y1,
            w: (x2 - x1) as u32,
            h: (y2 - y1) as u32,
        }
    }
}

#[no_mangle]
pub extern "C" fn main() {
    thing_std::init(0);
    log_info("BLOOM: alive");

    loop {
        if let Some(display_id) = thing_find("device.display0") {
            let mut buf = [0u8; 20];
            let len = thing_std::graph::thing_get_payload(display_id, &mut buf);

            let (width, height) = if len >= 8 {
                let w = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
                let h = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);
                (w, h)
            } else {
                (1280u32, 720u32)
            };

            // Store screen dimensions for mouse clamping
            unsafe {
                SCREEN_WIDTH = width;
                SCREEN_HEIGHT = height;
                POINTER_X = (width / 2) as i32;
                POINTER_Y = (height / 2) as i32;
            }

            let fb_base = 0xA000_0000u64;
            let fb_size: u64 = (width as u64) * (height as u64) * 4;

            let bs_id = thing_find("bytespace.display0").expect("bytespace not found");
            let _mapped = thing_std::memory::space_map(bs_id, fb_base, 0, fb_size);

            // Map mouse input bytespace
            if let Some(mouse_bs_id) = thing_find("bytespace.mouse_input") {
                let mouse_vaddr = 0x8300_0000u64;
                let mouse_size = 8192u64;
                thing_std::memory::space_map(mouse_bs_id, mouse_vaddr, 0, mouse_size);
                unsafe {
                    MOUSE_RING_PTR = Some(mouse_vaddr as *const u8);
                }
                log_info("BLOOM: mapped bytespace.mouse_input");
            }

            let buffer_size = (width * height) as usize;
            unsafe {
                BACK_BUFFER = Some(alloc::vec![0u32; buffer_size]);
                WALLPAPER_CACHE = Some(alloc::vec![0u32; buffer_size]);
            }

            let wallpaper = thing_find("bytespace.asset.clouds.bmp")
                .and_then(|bs_id| load_bmp(bs_id, 0x8100_0000));

            let cursor_asset = load_cursor_asset();

            if let Some(ref wp) = wallpaper {
                unsafe {
                    if let Some(ref mut cache) = WALLPAPER_CACHE {
                        render_wallpaper_full(cache.as_mut_ptr(), width, height, wp);
                    }
                }
            }

            log_info("BLOOM: initialized");

            let fb_ptr = fb_base as *mut u32;
            let (screen_cx, screen_cy) = ((width / 2) as i32, (height / 2) as i32);
            let mut prev_cursor_rect: Option<Rect> = None;
            let mut prev_px: i32 = screen_cx;
            let mut prev_py: i32 = screen_cy;

            // Use milliseconds for animation timing (monotonic_now returns nanoseconds)
            let mut animator = cursor_asset.map(|asset| CursorAnimator::new(asset, 1));

            loop {
                // Get current time in milliseconds
                let now_ms = time::monotonic_now() / 1_000_000;

                // Consume pending mouse samples from ring buffer
                consume_mouse_samples();

                let (px, py, _buttons) = read_pointer_state();
                let frame_changed = animator
                    .as_mut()
                    .map(|a| a.advance(now_ms))
                    .unwrap_or(false);

                // Force redraw every 100ms even if nothing changed, to ensure animation plays
                static mut LAST_REDRAW_MS: u64 = 0;
                let force_redraw = unsafe {
                    if now_ms - LAST_REDRAW_MS > 50 {
                        LAST_REDRAW_MS = now_ms;
                        true
                    } else {
                        false
                    }
                };

                if px != prev_px
                    || py != prev_py
                    || frame_changed
                    || prev_cursor_rect.is_none()
                    || force_redraw
                {
                    unsafe {
                        if let (Some(ref mut back_buf), Some(ref cache)) =
                            (&mut BACK_BUFFER, &WALLPAPER_CACHE)
                        {
                            let cursor_frame = animator.as_ref().and_then(|a| a.current_frame());

                            let cursor_rect = if let Some(frame) = cursor_frame {
                                Rect {
                                    x: px - frame.hotspot_x,
                                    y: py - frame.hotspot_y,
                                    w: frame.width + frame.shadow_offset_x.max(0) as u32,
                                    h: frame.height + frame.shadow_offset_y.max(0) as u32,
                                }
                            } else {
                                Rect {
                                    x: px,
                                    y: py,
                                    w: 8,
                                    h: 8,
                                }
                            };

                            let dirty = match prev_cursor_rect {
                                Some(prev) => Rect::union(prev, cursor_rect),
                                None => Rect {
                                    x: 0,
                                    y: 0,
                                    w: width,
                                    h: height,
                                },
                            };

                            redraw_region(
                                back_buf.as_mut_ptr(),
                                cache.as_ptr(),
                                width,
                                height,
                                dirty,
                            );

                            if let Some(frame) = cursor_frame {
                                draw_cursor_shadow(
                                    back_buf.as_mut_ptr(),
                                    width,
                                    height,
                                    frame,
                                    px,
                                    py,
                                );
                                draw_cursor_frame(
                                    back_buf.as_mut_ptr(),
                                    width,
                                    height,
                                    frame,
                                    px,
                                    py,
                                );
                            } else {
                                draw_fallback_cursor(back_buf.as_mut_ptr(), width, height, px, py);
                            }

                            copy_region_to_fb(fb_ptr, back_buf.as_ptr(), width, height, dirty);
                            prev_cursor_rect = Some(cursor_rect);
                            prev_px = px;
                            prev_py = py;
                        }
                    }
                }
                sched_yield();
            }
        }
        sched_yield();
    }
}

fn load_cursor_asset() -> Option<CursorAsset> {
    // Try animated cursor first for testing
    if let Some(bs_id) = thing_find("bytespace.asset.Normal.cur") {
        if let Some(asset) = load_cur_asset(bs_id, 0x8200_0000) {
            log_info("BLOOM: loaded Normal.cur");
            return Some(asset);
        }
    }
    if let Some(bs_id) = thing_find("bytespace.asset.Working.ani") {
        if let Some(asset) = load_ani_asset(bs_id, 0x8200_0000) {
            log_info("BLOOM: loaded Working.ani");
            return Some(asset);
        }
    }
    log_info("BLOOM: no cursor asset found, using fallback");
    None
}

fn load_cur_asset(bs_id: ThingId, vaddr: u64) -> Option<CursorAsset> {
    let len: u64 = 256 * 1024;
    thing_std::memory::space_map(bs_id, vaddr, 0, len);
    let buf = unsafe { core::slice::from_raw_parts(vaddr as *const u8, len as usize) };
    cursor::cur::load_cur(buf).map(CursorAsset::static_cursor)
}

fn load_ani_asset(bs_id: ThingId, vaddr: u64) -> Option<CursorAsset> {
    let len: u64 = 1024 * 1024;
    thing_std::memory::space_map(bs_id, vaddr, 0, len);
    let buf = unsafe { core::slice::from_raw_parts(vaddr as *const u8, len as usize) };
    cursor::ani::load_ani(buf)
}

/// Consume pending mouse samples from the ring buffer
fn consume_mouse_samples() {
    unsafe {
        let ring_ptr = match MOUSE_RING_PTR {
            Some(p) => p,
            None => return,
        };

        // Ring header: magic(4), version(4), capacity(4), sample_size(4), write(4), dropped(4), reserved(8)
        const HEADER_SIZE: usize = 32;
        const SAMPLE_SIZE: usize = 16; // sizeof(MouseSample)

        let magic = u32::from_le_bytes([
            *ring_ptr,
            *ring_ptr.add(1),
            *ring_ptr.add(2),
            *ring_ptr.add(3),
        ]);
        if magic != 0x4D4F5553 {
            return;
        } // "MOUS"

        let capacity = u32::from_le_bytes([
            *ring_ptr.add(8),
            *ring_ptr.add(9),
            *ring_ptr.add(10),
            *ring_ptr.add(11),
        ]);
        let write_ptr = ring_ptr.add(16) as *const u32;
        let write_idx = core::ptr::read_volatile(write_ptr);

        let samples_base = ring_ptr.add(HEADER_SIZE);

        while MOUSE_READ_IDX != write_idx {
            let slot = (MOUSE_READ_IDX % capacity) as usize;
            let sample_ptr = samples_base.add(slot * SAMPLE_SIZE);

            // MouseSample: t_ns(8), dx(2), dy(2), wheel(2), buttons(2)
            let dx = i16::from_le_bytes([*sample_ptr.add(8), *sample_ptr.add(9)]);
            let dy = i16::from_le_bytes([*sample_ptr.add(10), *sample_ptr.add(11)]);
            let buttons = u16::from_le_bytes([*sample_ptr.add(14), *sample_ptr.add(15)]);

            POINTER_X = (POINTER_X + dx as i32).clamp(0, SCREEN_WIDTH as i32 - 1);
            POINTER_Y = (POINTER_Y + dy as i32).clamp(0, SCREEN_HEIGHT as i32 - 1);
            POINTER_BUTTONS = buttons;

            MOUSE_READ_IDX = MOUSE_READ_IDX.wrapping_add(1);
        }
    }
}

fn read_pointer_state() -> (i32, i32, u8) {
    unsafe { (POINTER_X, POINTER_Y, POINTER_BUTTONS as u8) }
}

unsafe fn redraw_region(dest: *mut u32, src: *const u32, w: u32, h: u32, region: Rect) {
    let x1 = region.x.max(0) as u32;
    let y1: u32 = region.y.max(0) as u32;
    let x2 = ((region.x + region.w as i32) as u32).min(w);
    let y2 = ((region.y + region.h as i32) as u32).min(h);
    for y in y1..y2 {
        let row_start = (y * w + x1) as usize;
        let row_len = (x2 - x1) as usize;
        core::ptr::copy_nonoverlapping(src.add(row_start), dest.add(row_start), row_len);
    }
}

unsafe fn draw_cursor_shadow(
    dest: *mut u32,
    screen_w: u32,
    screen_h: u32,
    frame: &CursorFrame,
    px: i32,
    py: i32,
) {
    let cx = px - frame.hotspot_x + frame.shadow_offset_x;
    let cy = py - frame.hotspot_y + frame.shadow_offset_y;

    for row in 0..frame.height {
        let screen_y = cy + row as i32;
        if screen_y < 0 || screen_y >= screen_h as i32 {
            continue;
        }

        for col in 0..frame.width {
            let screen_x = cx + col as i32;
            if screen_x < 0 || screen_x >= screen_w as i32 {
                continue;
            }

            let shadow_idx = (row * frame.width + col) as usize;
            let shadow_pixel = frame.shadow_pixels[shadow_idx];
            let alpha = (shadow_pixel >> 24) & 0xFF;

            if alpha == 0 {
                continue;
            }

            let dest_idx = (screen_y as u32 * screen_w + screen_x as u32) as usize;
            let dst_pixel = *dest.add(dest_idx);
            *dest.add(dest_idx) = blend_pixel(shadow_pixel, dst_pixel);
        }
    }
}

unsafe fn draw_cursor_frame(
    dest: *mut u32,
    screen_w: u32,
    screen_h: u32,
    frame: &CursorFrame,
    px: i32,
    py: i32,
) {
    let cx = px - frame.hotspot_x;
    let cy = py - frame.hotspot_y;

    for row in 0..frame.height {
        let screen_y = cy + row as i32;
        if screen_y < 0 || screen_y >= screen_h as i32 {
            continue;
        }

        for col in 0..frame.width {
            let screen_x = cx + col as i32;
            if screen_x < 0 || screen_x >= screen_w as i32 {
                continue;
            }

            let cursor_idx = (row * frame.width + col) as usize;
            let src_pixel = frame.pixels[cursor_idx];
            let alpha = (src_pixel >> 24) & 0xFF;

            if alpha == 0 {
                continue;
            }

            let dest_idx = (screen_y as u32 * screen_w + screen_x as u32) as usize;

            if alpha == 255 {
                *dest.add(dest_idx) = src_pixel;
            } else {
                let dst_pixel = *dest.add(dest_idx);
                *dest.add(dest_idx) = blend_pixel(src_pixel, dst_pixel);
            }
        }
    }
}

#[inline]
fn blend_pixel(src: u32, dst: u32) -> u32 {
    let sa = (src >> 24) & 0xFF;
    let inv_a = 255 - sa;
    let sr = (src >> 16) & 0xFF;
    let sg = (src >> 8) & 0xFF;
    let sb = src & 0xFF;
    let dr = (dst >> 16) & 0xFF;
    let dg = (dst >> 8) & 0xFF;
    let db = dst & 0xFF;
    let or = sr + ((dr * inv_a + 127) / 255);
    let og = sg + ((dg * inv_a + 127) / 255);
    let ob = sb + ((db * inv_a + 127) / 255);
    0xFF000000 | (or.min(255) << 16) | (og.min(255) << 8) | ob.min(255)
}

unsafe fn draw_fallback_cursor(dest: *mut u32, w: u32, h: u32, px: i32, py: i32) {
    for dy in 0..8i32 {
        for dx in 0..8i32 {
            let x = px + dx + 2;
            let y = py + dy + 3;
            if x >= 0 && x < w as i32 && y >= 0 && y < h as i32 {
                let idx = (y as u32 * w + x as u32) as usize;
                let dst = *dest.add(idx);
                *dest.add(idx) = blend_pixel(0x66000000, dst);
            }
        }
    }
    for dy in 0..8i32 {
        for dx in 0..8i32 {
            let x = px + dx;
            let y = py + dy;
            if x >= 0 && x < w as i32 && y >= 0 && y < h as i32 {
                let idx = (y as u32 * w + x as u32) as usize;
                let pixel = if dx == 0 || dy == 0 || dx == 7 || dy == 7 {
                    0xFF000000
                } else {
                    0xFFFFFFFF
                };
                *dest.add(idx) = pixel;
            }
        }
    }
}

unsafe fn copy_region_to_fb(fb: *mut u32, src: *const u32, w: u32, h: u32, region: Rect) {
    let x1 = region.x.max(0) as u32;
    let y1 = region.y.max(0) as u32;
    let x2 = ((region.x + region.w as i32) as u32).min(w);
    let y2 = ((region.y + region.h as i32) as u32).min(h);
    for y in y1..y2 {
        let row_start = (y * w + x1) as usize;
        let row_len = (x2 - x1) as usize;
        core::ptr::copy_nonoverlapping(src.add(row_start), fb.add(row_start), row_len);
    }
}

struct Wallpaper {
    data_ptr: *const u8,
    width: u32,
    height: u32,
    row_stride: usize,
    bytes_per_pixel: usize,
    bottom_up: bool,
}

fn load_bmp(bs_id: ThingId, vaddr: u64) -> Option<Wallpaper> {
    let len: u64 = 4 * 1024 * 1024;
    thing_std::memory::space_map(bs_id, vaddr, 0, len);
    let buf = unsafe { core::slice::from_raw_parts(vaddr as *const u8, len as usize) };
    if buf.len() < 54 || &buf[0..2] != b"BM" {
        return None;
    }
    let data_offset = u32::from_le_bytes([buf[10], buf[11], buf[12], buf[13]]) as usize;
    let width_i = i32::from_le_bytes([buf[18], buf[19], buf[20], buf[21]]);
    let height_i = i32::from_le_bytes([buf[22], buf[23], buf[24], buf[25]]);
    let bpp = u16::from_le_bytes([buf[28], buf[29]]);
    if width_i <= 0 {
        return None;
    }
    let bytes_per_pixel = (bpp as usize + 7) / 8;
    let row_stride = ((width_i as usize * bytes_per_pixel + 3) / 4) * 4;
    let data_ptr = unsafe { (vaddr as *const u8).add(data_offset) };
    Some(Wallpaper {
        data_ptr,
        width: width_i.abs() as u32,
        height: height_i.abs() as u32,
        row_stride,
        bytes_per_pixel,
        bottom_up: height_i > 0,
    })
}

fn render_wallpaper_full(dest: *mut u32, dest_w: u32, dest_h: u32, wp: &Wallpaper) {
    for y in 0..dest_h {
        let src_y = y % wp.height;
        let actual_src_y = if wp.bottom_up {
            wp.height - 1 - src_y
        } else {
            src_y
        };
        let row_ptr = unsafe { wp.data_ptr.add(actual_src_y as usize * wp.row_stride) };
        let dest_row = unsafe { dest.add((y * dest_w) as usize) };
        unsafe {
            for x in 0..dest_w {
                let src_x = x % wp.width;
                let src_ptr = row_ptr.add(src_x as usize * wp.bytes_per_pixel);
                let b = *src_ptr;
                let g = *src_ptr.add(1);
                let r = *src_ptr.add(2);
                *dest_row.add(x as usize) =
                    0xFF000000u32 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            }
        }
    }
}
