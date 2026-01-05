use crate::input::PointerInput;
use crate::pixels::*;
use crate::shadow::{draw_shadow_from_mask, ShadowMask, ShadowParams};
use crate::ui::{collect_window_scenes, render_window_scenes};
use alloc::vec::Vec;
use models::*;
use thing_std::graph::*;
use thing_std::*;

use crate::assets::cursor::{CursorAnimator, CursorAsset, CursorFrame};
use crate::backend::*;

pub fn run() {
    thing_std::init(0);
    log_info("BLOOM: alive");

    let mut graph_client = SyscallGraphClient;

    loop {
        if let Some(display_id) = thing_find("device.display0") {
            let (width, height) =
                if let Ok(display) = DisplayDevice::read(&SyscallGraphClient, display_id) {
                    (display.width, display.height)
                } else {
                    (1280u32, 720u32)
                };

            let mut backend = CpuBytespaceBackend::new(
                thing_find("bytespace.display0").expect("bytespace not found"),
                0xA000_0000u64,
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

            let wallpaper = thing_find("bytespace.asset.clouds.bmp").and_then(|id| {
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

            // Use milliseconds for animation timing (monotonic_now returns nanoseconds)
            let mut animator = cursor_asset.map(|asset| CursorAnimator::new(asset, 1));
            let mut last_redraw_ms: u64 = 0;
            let mut last_scene_refresh_ms: u64 = 0;
            let mut window_scenes = collect_window_scenes(&mut graph_client);
            let mut logged_shared_shadow = false;
            let mut prev_px = 0;
            let mut prev_py = 0;
            let mut logged_window_once = false;

            loop {
                // Get current time in milliseconds
                let now_ms = (monotonic_now() / 1_000_000) as u64;

                let (px, py, buttons) = input.poll();
                let moved = px != prev_px || py != prev_py;

                // Animation update
                let anim_changed = if let Some(anim) = animator.as_mut() {
                    anim.advance(now_ms)
                } else {
                    false
                };

                if now_ms.saturating_sub(last_scene_refresh_ms) > 250 {
                    window_scenes = collect_window_scenes(&mut graph_client);
                    last_scene_refresh_ms = now_ms;
                }

                let force_redraw = if now_ms - last_redraw_ms > 50 {
                    last_redraw_ms = now_ms;
                    true
                } else {
                    false
                };

                if force_redraw || anim_changed || moved || buttons != 0 {
                    unsafe {
                        // Rebuild frame: wallpaper -> windows -> cursor
                        core::ptr::copy_nonoverlapping(
                            wallpaper_cache.as_ptr(),
                            back_buffer.as_mut_ptr(),
                            buffer_size,
                        );

                        render_window_scenes(
                            back_buffer.as_mut_ptr(),
                            width,
                            height,
                            &window_scenes,
                        );
                        if !window_scenes.is_empty() && !logged_window_once {
                            let win_id = window_scenes[0].id.low();
                            log_info(&alloc::format!("BLOOM: rendered window {}", win_id));
                            logged_window_once = true;
                        }

                        let cursor_frame = if let Some(anim) = animator.as_ref() {
                            anim.current_frame()
                        } else {
                            None
                        };

                        if let Some(frame) = cursor_frame {
                            draw_shadow_from_mask(
                                back_buffer.as_mut_ptr(),
                                width,
                                height,
                                px - frame.hotspot_x,
                                py - frame.hotspot_y,
                                ShadowMask::SpriteAlpha {
                                    pixels: &frame.shadow_pixels,
                                    width: frame.width,
                                    height: frame.height,
                                },
                                ShadowParams {
                                    offset_x: frame.shadow_offset_x,
                                    offset_y: frame.shadow_offset_y,
                                    blur_radius: 0, // Frame already prerendered with blur
                                    color: 0xAA000000,
                                },
                            );
                            draw_cursor_frame(
                                back_buffer.as_mut_ptr(),
                                width,
                                height,
                                frame,
                                px,
                                py,
                            );
                            if !logged_shared_shadow {
                                log_info("BLOOM: shadow kernel: shared");
                                logged_shared_shadow = true;
                            }
                        } else {
                            draw_fallback_cursor(back_buffer.as_mut_ptr(), width, height, px, py);
                        }

                        backend.present(
                            back_buffer.as_mut_slice(),
                            DirtyRect {
                                x: 0,
                                y: 0,
                                w: width,
                                h: height,
                            },
                        );

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
