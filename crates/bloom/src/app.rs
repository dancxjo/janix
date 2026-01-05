use alloc::vec::Vec;
use thing_std::graph::*;
use thing_std::*;
use models::*;
use crate::input::PointerInput;
use crate::scene::Rect;
use crate::pixels::*;

use crate::assets::cursor::{CursorAnimator, CursorAsset, CursorFrame};
use crate::backend::*;

pub fn run() {
    thing_std::init(0);
    log_info("BLOOM: alive");

    loop {
        if let Some(display_id) = thing_find("device.display0") {
            let (width, height) = if let Ok(display) = DisplayDevice::read(&SyscallGraphClient, display_id) {
                (display.width, display.height)
            } else {
                (1280u32, 720u32)
            };

            let mut backend = CpuBytespaceBackend::new(
                 thing_find("bytespace.display0").expect("bytespace not found"),
                 0xA000_0000u64
            );
            backend.configure(SurfaceDesc {
                width: width,
                height: height,
                stride_pixels: width, // Assuming stride == width for now
            });

            // Map mouse input bytespace
            let mut input = if let Some(mouse_bs_id) = thing_find("bytespace.mouse_input") {
                let mouse_vaddr = 0x8820_0000u64;
                let mouse_size = 8192u64;
                thing_std::memory::space_map(mouse_bs_id, mouse_vaddr, 0, mouse_size);
                log_info("BLOOM: mapped bytespace.mouse_input");
                PointerInput::new(mouse_vaddr as *const u8, mouse_size as u32, width, height)
            } else {
                 PointerInput::new(core::ptr::null(), 0, width, height)
            };

            let buffer_size = (width * height) as usize;
            let mut back_buffer = alloc::vec![0u32; buffer_size];
            let mut wallpaper_cache = alloc::vec![0u32; buffer_size];

            let wallpaper = thing_find("bytespace.asset.clouds.bmp")
                .and_then(|id| {
                    let len = 128 * 1024 * 1024; // 128MB to support 4K+
                    let buf = crate::assets::map_bytespace(id, 0x8000_0000, len);
                    crate::assets::bmp::parse_bmp(buf)
                });

            if let Some(ref wp) = wallpaper {
                render_wallpaper_full(wallpaper_cache.as_mut_ptr(), width, height, wp);
                // Fix: Copy the rendered wallpaper to the back buffer so it's ready for the first frame
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        wallpaper_cache.as_ptr(),
                        back_buffer.as_mut_ptr(),
                        buffer_size,
                    );
                }
            }

            // Load cursor assets
            let cursor_asset = load_cursor_asset();
            
            // Mouse state
            let mut prev_px = 0;
            let mut prev_py = 0;
            let mut prev_cursor_rect: Option<Rect> = None;

            // Use milliseconds for animation timing (monotonic_now returns nanoseconds)
            let mut animator = cursor_asset.map(|asset| CursorAnimator::new(asset, 1));
            let mut last_redraw_ms: u64 = 0;

            loop {
                // Get current time in milliseconds
                let now_ms = (monotonic_now() / 1_000_000) as u64;

                let (px, py, buttons) = input.poll();

                // Animation update
                let anim_changed = if let Some(anim) = animator.as_mut() {
                    anim.advance(now_ms)
                } else {
                    false
                };

                let force_redraw = if now_ms - last_redraw_ms > 50 {
                    last_redraw_ms = now_ms;
                    true
                } else {
                    false
                };

                if px != prev_px
                    || py != prev_py
                    || buttons != 0
                    || anim_changed
                    || force_redraw
                    || prev_cursor_rect.is_none() // Fix: Force redraw on first frame
                {
                    unsafe {
                        let back_buf = &mut back_buffer;
                        let cache = &wallpaper_cache;
                        let cursor_frame =
                            if let Some(anim) = animator.as_ref() {
                                anim.current_frame()
                            } else {
                                None
                            };
                        
                        // Calculate cursor rect
                        let (cw, ch, hot_x, hot_y) = if let Some(frame) = cursor_frame {
                            (frame.width, frame.height, frame.hotspot_x, frame.hotspot_y)
                        } else {
                            (16, 24, 0, 0)
                        };

                        let cursor_rect = Rect {
                            x: px as i32 - hot_x as i32,
                            y: py as i32 - hot_y as i32,
                            w: cw,
                            h: ch
                        };

                        // Clear previous cursor position
                        if let Some(prev) = prev_cursor_rect {
                           // Logic to restore from cache would involve DirtyRect
                           // For now, simpler redraw strategy:
                           // If we have separate layers, we'd redraw the under-layer.
                           // Here we have wallpaper_cache.
                         
                           // Optimization: Union of old and new rect?
                           // Actually, let's just redraw the union of separate rects.
                           
                           let clear_rect = Rect::union(prev, cursor_rect);
                           
                           // Redraw from wallpaper cache to backbuffer
                            redraw_region(back_buf.as_mut_ptr(), cache.as_ptr(), width, height, clear_rect);

                           // Draw cursor shadow
                           // draw_cursor_shadow(back_buf.as_mut_ptr(), width, height, px, py);

                            // Draw cursor
                           if let Some(frame) = cursor_frame {
                               draw_cursor_frame(back_buf.as_mut_ptr(), width, height, frame, px, py);
                           } else {
                               // Fallback cursor?
                           }
                           
                           // Present
                           backend.present(back_buf, DirtyRect {
                               x: clear_rect.x,
                               y: clear_rect.y,
                               w: clear_rect.w,
                               h: clear_rect.h
                           });
                        } else {
                             // First frame or full redraw
                             // Only if we want to draw cursor initially
                             
                             // Draw cursor
                           if let Some(frame) = cursor_frame {
                               draw_cursor_frame(back_buf.as_mut_ptr(), width, height, frame, px, py);
                           }

                           // Fix: resent full screen on first frame
                           backend.present(back_buf, DirtyRect {
                               x: 0,
                               y: 0,
                               w: width,
                               h: height
                           });
                        }
                        
                        prev_cursor_rect = Some(cursor_rect);
                        prev_px = px;
                        prev_py = py;
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
        if let Some(asset) = load_cur_asset(bs_id, 0x8800_0000) {
            log_info("BLOOM: loaded Normal.cur");
            return Some(asset);
        }
    }
    if let Some(bs_id) = thing_find("bytespace.asset.Working.ani") {
        if let Some(asset) = load_ani_asset(bs_id, 0x8810_0000) {
            log_info("BLOOM: loaded Working.ani");
            return Some(asset);
        }
    }
    log_info("BLOOM: no cursor asset found, using fallback");
    None
}

fn load_cur_asset(bs_id: ThingId, vaddr: u64) -> Option<CursorAsset> {
    let len: u64 = 256 * 1024;
    let buf = crate::assets::map_bytespace(bs_id, vaddr, len);
    crate::assets::cursor::cur::load_cur(buf).map(CursorAsset::static_cursor)
}

fn load_ani_asset(bs_id: ThingId, vaddr: u64) -> Option<CursorAsset> {
    let len: u64 = 1024 * 1024;
    let buf = crate::assets::map_bytespace(bs_id, vaddr, len);
    crate::assets::cursor::ani::load_ani(buf)
}


// Pixel functions removed


use crate::assets::bmp::Wallpaper;

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
