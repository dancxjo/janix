#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use thing_std::graph::*;
use thing_std::*;

// Global state
static mut BACK_BUFFER: Option<Vec<u32>> = None;
static mut WALLPAPER_CACHE: Option<Vec<u32>> = None;

// Cursor state
struct CursorState {
    pixels: Vec<u32>,      // ARGB pixels
    width: u32,
    height: u32,
    hotspot_x: i32,
    hotspot_y: i32,
}

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

            let fb_base = 0xA000_0000u64;
            let fb_size: u64 = (width as u64) * (height as u64) * 4;
            
            let bs_id = thing_find("bytespace.display0").expect("bytespace not found");
            let _mapped = thing_std::memory::space_map(bs_id, fb_base, 0, fb_size);

            let buffer_size = (width * height) as usize;
            unsafe {
                BACK_BUFFER = Some(alloc::vec![0u32; buffer_size]);
                WALLPAPER_CACHE = Some(alloc::vec![0u32; buffer_size]);
            }

            // Load wallpaper
            let wallpaper = thing_find("bytespace.asset.clouds.bmp")
                .and_then(|bs_id| load_bmp(bs_id, 0x8100_0000));

            // Load cursor
            let cursor = thing_find("bytespace.asset.cursor.bmp")
                .and_then(|bs_id| load_cursor_bmp(bs_id, 0x8200_0000));

            // Render initial wallpaper to cache
            if let Some(ref wp) = wallpaper {
                unsafe {
                    if let Some(ref mut cache) = WALLPAPER_CACHE {
                        render_wallpaper_full(cache.as_mut_ptr(), width, height, wp);
                    }
                }
            }

            log_info("BLOOM: initialized");

            // Main compositor loop
            let fb_ptr = fb_base as *mut u32;
            let mut prev_cursor_rect: Option<Rect> = None;
            let mut prev_px: i32 = -1;
            let mut prev_py: i32 = -1;

            loop {
                // Read pointer state from graph
                let (px, py, _buttons) = read_pointer_state();

                // Only update if pointer moved
                if px != prev_px || py != prev_py || prev_cursor_rect.is_none() {
                    unsafe {
                        if let (Some(ref mut back_buf), Some(ref cache)) = 
                            (&mut BACK_BUFFER, &WALLPAPER_CACHE) 
                        {
                            // Calculate cursor rect
                            let cursor_rect = if let Some(ref c) = cursor {
                                Rect {
                                    x: px - c.hotspot_x,
                                    y: py - c.hotspot_y,
                                    w: c.width,
                                    h: c.height,
                                }
                            } else {
                                // Fallback: simple 8x8 cursor
                                Rect { x: px, y: py, w: 8, h: 8 }
                            };

                            // Calculate dirty region
                            let dirty = match prev_cursor_rect {
                                Some(prev) => Rect::union(prev, cursor_rect),
                                None => Rect {
                                    x: 0, y: 0, w: width, h: height
                                },
                            };

                            // Redraw dirty region from wallpaper cache
                            redraw_region(
                                back_buf.as_mut_ptr(), 
                                cache.as_ptr(), 
                                width, height, 
                                dirty
                            );

                            // Draw cursor
                            if let Some(ref c) = cursor {
                                draw_cursor(
                                    back_buf.as_mut_ptr(),
                                    width, height,
                                    c, px, py
                                );
                            } else {
                                // Fallback: draw simple white square
                                draw_fallback_cursor(
                                    back_buf.as_mut_ptr(),
                                    width, height,
                                    px, py
                                );
                            }

                            // Copy dirty region to framebuffer
                            copy_region_to_fb(
                                fb_ptr,
                                back_buf.as_ptr(),
                                width, height,
                                dirty
                            );

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

/// Read pointer state from the graph
fn read_pointer_state() -> (i32, i32, u8) {
    if let Some(ptr_id) = thing_find("pointer.0") {
        let mut buf = [0u8; 12];
        let len = thing_get_payload(ptr_id, &mut buf);
        if len >= 9 {
            let x = i32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
            let y = i32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);
            let buttons = buf[8];
            return (x, y, buttons);
        }
    }
    // Fallback: center of screen
    (640, 360, 0)
}

/// Redraw a region from the wallpaper cache
unsafe fn redraw_region(dest: *mut u32, src: *const u32, w: u32, h: u32, region: Rect) {
    // Clip to screen bounds
    let x1 = region.x.max(0) as u32;
    let y1 = region.y.max(0) as u32;
    let x2 = ((region.x + region.w as i32) as u32).min(w);
    let y2 = ((region.y + region.h as i32) as u32).min(h);

    for y in y1..y2 {
        for x in x1..x2 {
            let idx = (y * w + x) as usize;
            *dest.add(idx) = *src.add(idx);
        }
    }
}

/// Draw cursor with alpha blending
unsafe fn draw_cursor(dest: *mut u32, screen_w: u32, screen_h: u32, cursor: &CursorState, px: i32, py: i32) {
    let cx = px - cursor.hotspot_x;
    let cy = py - cursor.hotspot_y;

    for row in 0..cursor.height {
        let screen_y = cy + row as i32;
        if screen_y < 0 || screen_y >= screen_h as i32 { continue; }

        for col in 0..cursor.width {
            let screen_x = cx + col as i32;
            if screen_x < 0 || screen_x >= screen_w as i32 { continue; }

            let cursor_idx = (row * cursor.width + col) as usize;
            let src_pixel = cursor.pixels[cursor_idx];
            let alpha = (src_pixel >> 24) & 0xFF;

            if alpha == 0 { continue; } // Fully transparent

            let dest_idx = (screen_y as u32 * screen_w + screen_x as u32) as usize;

            if alpha == 255 {
                // Fully opaque - just copy
                *dest.add(dest_idx) = src_pixel;
            } else {
                // Alpha blend
                let dst_pixel = *dest.add(dest_idx);
                *dest.add(dest_idx) = blend_pixel(src_pixel, dst_pixel);
            }
        }
    }
}

/// Alpha blend with premultiplied alpha formula
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

    // out = src + dst * (1 - src_alpha)
    let or = sr + ((dr * inv_a + 127) / 255);
    let og = sg + ((dg * inv_a + 127) / 255);
    let ob = sb + ((db * inv_a + 127) / 255);

    0xFF000000 | (or.min(255) << 16) | (og.min(255) << 8) | ob.min(255)
}

/// Draw fallback cursor (simple white square)
unsafe fn draw_fallback_cursor(dest: *mut u32, w: u32, h: u32, px: i32, py: i32) {
    for dy in 0..8i32 {
        for dx in 0..8i32 {
            let x = px + dx;
            let y = py + dy;
            if x >= 0 && x < w as i32 && y >= 0 && y < h as i32 {
                let idx = (y as u32 * w + x as u32) as usize;
                // White with black outline
                let pixel = if dx == 0 || dy == 0 || dx == 7 || dy == 7 {
                    0xFF000000 // Black border
                } else {
                    0xFFFFFFFF // White fill
                };
                *dest.add(idx) = pixel;
            }
        }
    }
}

/// Copy a region from backbuffer to framebuffer
unsafe fn copy_region_to_fb(fb: *mut u32, src: *const u32, w: u32, h: u32, region: Rect) {
    let x1 = region.x.max(0) as u32;
    let y1 = region.y.max(0) as u32;
    let x2 = ((region.x + region.w as i32) as u32).min(w);
    let y2 = ((region.y + region.h as i32) as u32).min(h);

    for y in y1..y2 {
        let row_start = (y * w + x1) as usize;
        let row_len = (x2 - x1) as usize;
        let src_ptr = src.add(row_start);
        let dst_ptr = fb.add(row_start);
        core::ptr::copy_nonoverlapping(src_ptr, dst_ptr, row_len);
    }
}

// ============================================================================
// Asset Loading
// ============================================================================

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
    
    if buf.len() < 54 || &buf[0..2] != b"BM" { return None; }
    
    let data_offset = u32::from_le_bytes([buf[10], buf[11], buf[12], buf[13]]) as usize;
    let width_i = i32::from_le_bytes([buf[18], buf[19], buf[20], buf[21]]);
    let height_i = i32::from_le_bytes([buf[22], buf[23], buf[24], buf[25]]);
    let bpp = u16::from_le_bytes([buf[28], buf[29]]);
    
    if width_i <= 0 { return None; }
    
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

/// Load cursor BMP and convert to ARGB with transparency
fn load_cursor_bmp(bs_id: ThingId, vaddr: u64) -> Option<CursorState> {
    let len: u64 = 256 * 1024; // Cursor is small
    thing_std::memory::space_map(bs_id, vaddr, 0, len);
    let buf = unsafe { core::slice::from_raw_parts(vaddr as *const u8, len as usize) };
    
    if buf.len() < 54 || &buf[0..2] != b"BM" { return None; }
    
    let data_offset = u32::from_le_bytes([buf[10], buf[11], buf[12], buf[13]]) as usize;
    let width_i = i32::from_le_bytes([buf[18], buf[19], buf[20], buf[21]]);
    let height_i = i32::from_le_bytes([buf[22], buf[23], buf[24], buf[25]]);
    let bpp = u16::from_le_bytes([buf[28], buf[29]]);
    
    if width_i <= 0 { return None; }
    
    let width = width_i.abs() as u32;
    let height = height_i.abs() as u32;
    let bottom_up = height_i > 0;
    let bytes_per_pixel = (bpp as usize + 7) / 8;
    let row_stride = ((width as usize * bytes_per_pixel + 3) / 4) * 4;
    
    let mut pixels = Vec::with_capacity((width * height) as usize);
    
    for y in 0..height {
        let src_y = if bottom_up { height - 1 - y } else { y };
        let row_ptr = unsafe { (vaddr as *const u8).add(data_offset + src_y as usize * row_stride) };
        
        for x in 0..width {
            let pixel_ptr = unsafe { row_ptr.add(x as usize * bytes_per_pixel) };
            
            let (r, g, b, a) = if bytes_per_pixel >= 4 {
                // BGRA
                unsafe {
                    (
                        *pixel_ptr.add(2),
                        *pixel_ptr.add(1),
                        *pixel_ptr,
                        *pixel_ptr.add(3),
                    )
                }
            } else {
                // BGR - use magenta (255, 0, 255) as transparency key
                unsafe {
                    let b = *pixel_ptr;
                    let g = *pixel_ptr.add(1);
                    let r = *pixel_ptr.add(2);
                    let a = if r == 255 && g == 0 && b == 255 { 0 } else { 255 };
                    (r, g, b, a)
                }
            };
            
            // Convert to premultiplied alpha
            let (pr, pg, pb) = if a == 255 {
                (r, g, b)
            } else if a == 0 {
                (0, 0, 0)
            } else {
                (
                    ((r as u32 * a as u32 + 127) / 255) as u8,
                    ((g as u32 * a as u32 + 127) / 255) as u8,
                    ((b as u32 * a as u32 + 127) / 255) as u8,
                )
            };
            
            let pixel = ((a as u32) << 24) | ((pr as u32) << 16) | ((pg as u32) << 8) | (pb as u32);
            pixels.push(pixel);
        }
    }
    
    Some(CursorState {
        pixels,
        width,
        height,
        hotspot_x: 0, // Arrow tip at top-left
        hotspot_y: 0,
    })
}

/// Render full wallpaper (tiled) to buffer
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
                
                let pixel = 0xFF000000u32 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                *dest_row.add(x as usize) = pixel;
            }
        }
    }
}
