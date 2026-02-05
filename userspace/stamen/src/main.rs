//! Stamen: Cursor-Only Overlay Daemon
//!
//! A tiny, purpose-built daemon for "butter smooth" cursor rendering.
//! Receives mouse events from Bristle and renders cursor overlay
//! independently of Bloom's compositing cadence.
//!
//! ## Architecture
//!
//! - **Input**: Bristle mouse events via IPC port
//! - **Output**: Direct framebuffer overlay via display driver
//! - **Fast path**: Save-under + blit for minimal CPU work

#![no_std]
#![no_main]

extern crate alloc;

use abi::hid::{
    BristleEventHeader, PointerButtonPayload, PointerMovePayload,
    BRISTLE_EVENT_MAGIC, BRISTLE_EVENT_VERSION,
};
use abi::ids::HandleId;
use stamen::cursor::CursorState;
use stamen::sprite::ArrowSprite;
use stem::info;
use stem::syscall::{port_recv, PortHandle};
use stem::thing::sys as thingsys;
use stem::thing::ThingId;

/// Cursor metrics for performance tracking.
#[derive(Default)]
struct CursorMetrics {
    moves: u64,
    updates: u64,
    frame_count: u64,
}

impl CursorMetrics {
    fn record_move(&mut self) {
        self.moves += 1;
    }

    fn record_update(&mut self) {
        self.updates += 1;
    }

    fn maybe_log(&mut self) {
        self.frame_count += 1;
        if self.frame_count % 120 == 0 {
            info!(
                "[stamen] metrics: moves={} updates={} fps~{}",
                self.moves / 2,
                self.updates / 2,
                60
            );
            self.moves = 0;
            self.updates = 0;
        }
    }
}

/// Blit cursor sprite to framebuffer with alpha blending.
fn blit_cursor(
    fb: &mut [u32],
    stride: u32,
    fb_w: i32,
    fb_h: i32,
    x: i32,
    y: i32,
) {
    let sprite = &ArrowSprite::PIXELS;
    let sw = ArrowSprite::WIDTH as i32;
    let sh = ArrowSprite::HEIGHT as i32;
    let hx = ArrowSprite::HOTSPOT_X;
    let hy = ArrowSprite::HOTSPOT_Y;

    let dx = x - hx;
    let dy = y - hy;

    for sy in 0..sh {
        for sx in 0..sw {
            let px = dx + sx;
            let py = dy + sy;

            if px < 0 || py < 0 || px >= fb_w || py >= fb_h {
                continue;
            }

            let src_px = sprite[(sy * sw + sx) as usize];
            let alpha = (src_px >> 24) & 0xFF;

            if alpha == 0 {
                continue;
            }

            let dst_idx = (py as u32 * stride + px as u32) as usize;
            if dst_idx >= fb.len() {
                continue;
            }

            if alpha == 255 {
                // Opaque: direct write
                fb[dst_idx] = src_px;
            } else {
                // Alpha blend
                let dst_px = fb[dst_idx];
                let inv_alpha = 255 - alpha;

                let sr = (src_px >> 16) & 0xFF;
                let sg = (src_px >> 8) & 0xFF;
                let sb = src_px & 0xFF;

                let dr = (dst_px >> 16) & 0xFF;
                let dg = (dst_px >> 8) & 0xFF;
                let db = dst_px & 0xFF;

                let r = (sr * alpha + dr * inv_alpha) / 255;
                let g = (sg * alpha + dg * inv_alpha) / 255;
                let b = (sb * alpha + db * inv_alpha) / 255;

                fb[dst_idx] = 0xFF000000 | (r << 16) | (g << 8) | b;
            }
        }
    }
}

/// Save pixels under cursor rect for later restore.
fn save_under(
    fb: &[u32],
    stride: u32,
    fb_w: i32,
    fb_h: i32,
    x: i32,
    y: i32,
    save_buf: &mut [u32; 256],
) {
    let sw = ArrowSprite::WIDTH as i32;
    let sh = ArrowSprite::HEIGHT as i32;
    let hx = ArrowSprite::HOTSPOT_X;
    let hy = ArrowSprite::HOTSPOT_Y;

    let dx = x - hx;
    let dy = y - hy;

    for sy in 0..sh {
        for sx in 0..sw {
            let px = dx + sx;
            let py = dy + sy;

            let save_idx = (sy * sw + sx) as usize;
            if px < 0 || py < 0 || px >= fb_w || py >= fb_h {
                save_buf[save_idx] = 0;
                continue;
            }

            let fb_idx = (py as u32 * stride + px as u32) as usize;
            if fb_idx < fb.len() {
                save_buf[save_idx] = fb[fb_idx];
            } else {
                save_buf[save_idx] = 0;
            }
        }
    }
}

/// Restore saved pixels to framebuffer.
fn restore_under(
    fb: &mut [u32],
    stride: u32,
    fb_w: i32,
    fb_h: i32,
    x: i32,
    y: i32,
    save_buf: &[u32; 256],
) {
    let sw = ArrowSprite::WIDTH as i32;
    let sh = ArrowSprite::HEIGHT as i32;
    let hx = ArrowSprite::HOTSPOT_X;
    let hy = ArrowSprite::HOTSPOT_Y;

    let dx = x - hx;
    let dy = y - hy;

    for sy in 0..sh {
        for sx in 0..sw {
            let px = dx + sx;
            let py = dy + sy;

            if px < 0 || py < 0 || px >= fb_w || py >= fb_h {
                continue;
            }

            let save_idx = (sy * sw + sx) as usize;
            let fb_idx = (py as u32 * stride + px as u32) as usize;
            if fb_idx < fb.len() {
                fb[fb_idx] = save_buf[save_idx];
            }
        }
    }
}

/// Poll for mouse events from Bristle.
fn poll_bristle_events(handle: PortHandle, cursor: &mut CursorState, metrics: &mut CursorMetrics) -> bool {
    let mut buf = [0u8; 256];
    let mut moved = false;

    loop {
        let n = match port_recv(handle, &mut buf) {
            Ok(n) => n,
            Err(_) => return moved,
        };

        if n == 0 {
            return moved;
        }

        if n < BristleEventHeader::SIZE {
            continue;
        }

        let header: BristleEventHeader =
            unsafe { core::ptr::read_unaligned(buf.as_ptr() as *const BristleEventHeader) };
        let magic = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(header.magic)) };
        let version = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(header.version)) };
        let event_type = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(header.event_type)) };
        let payload_len = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(header.payload_len)) } as usize;

        if magic != BRISTLE_EVENT_MAGIC || version != BRISTLE_EVENT_VERSION {
            continue;
        }

        let total = BristleEventHeader::SIZE + payload_len;
        if n < total {
            continue;
        }

        match event_type {
            3 => {
                // PointerMove
                if payload_len >= PointerMovePayload::SIZE {
                    let payload: PointerMovePayload = unsafe {
                        core::ptr::read_unaligned(
                            buf.as_ptr().add(BristleEventHeader::SIZE) as *const PointerMovePayload
                        )
                    };
                    let dx = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(payload.dx)) };
                    let dy = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(payload.dy)) };
                    cursor.apply_move(dx, dy);
                    metrics.record_move();
                    moved = true;
                }
            }
            4 => {
                // PointerButtonDown
                if payload_len >= PointerButtonPayload::SIZE {
                    let payload: PointerButtonPayload = unsafe {
                        core::ptr::read_unaligned(
                            buf.as_ptr().add(BristleEventHeader::SIZE) as *const PointerButtonPayload
                        )
                    };
                    let btn = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(payload.button)) };
                    cursor.button_down(btn);
                }
            }
            5 => {
                // PointerButtonUp
                if payload_len >= PointerButtonPayload::SIZE {
                    let payload: PointerButtonPayload = unsafe {
                        core::ptr::read_unaligned(
                            buf.as_ptr().add(BristleEventHeader::SIZE) as *const PointerButtonPayload
                        )
                    };
                    let btn = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(payload.button)) };
                    cursor.button_up(btn);
                }
            }
            _ => {}
        }
    }
}

#[stem::main]
fn main(packed_arg: usize) -> ! {
    info!("[stamen] Cursor daemon starting...");

    // Unpack handles from argument
    // Layout: (cursor_evt_read: u16) | (display_bs_lo: u16) << 16 | (display_bs_hi: u16) << 32
    let cursor_evt_read = (packed_arg & 0xFFFF) as u32;
    let display_bs_lo = ((packed_arg >> 16) & 0xFFFF) as u32;
    let display_bs_hi = ((packed_arg >> 32) & 0xFFFF) as u32;
    let display_bs_id = ThingId::from_u64(((display_bs_hi as u64) << 16) | (display_bs_lo as u64));

    info!(
        "[stamen] cursor_evt={} display_bs={:?}",
        cursor_evt_read, display_bs_id
    );

    // Get display dimensions from bytespace properties
    let screen_w = thingsys::prop_get(display_bs_id, "width").unwrap_or(1024) as i32;
    let screen_h = thingsys::prop_get(display_bs_id, "height").unwrap_or(768) as i32;
    let stride = thingsys::prop_get(display_bs_id, "stride").unwrap_or((screen_w as u64) * 4) as u32 / 4;

    info!(
        "[stamen] screen={}x{} stride={}",
        screen_w, screen_h, stride
    );

    // Map the display bytespace
    let fb_ptr = match thingsys::bytespace_map(display_bs_id) {
        Ok(ptr) => ptr,
        Err(e) => {
            stem::error!("[stamen] Failed to map display bytespace: {:?}", e);
            loop {
                stem::time::sleep(1000);
            }
        }
    };

    let fb_size = (screen_h as u32 * stride) as usize;
    let fb: &mut [u32] = unsafe { core::slice::from_raw_parts_mut(fb_ptr as *mut u32, fb_size) };

    // Initialize cursor state
    let mut cursor = CursorState::new(screen_w, screen_h);
    let mut prev_x = cursor.x;
    let mut prev_y = cursor.y;
    let mut save_buf: [u32; 256] = [0; 256];
    let mut metrics = CursorMetrics::default();

    // Draw initial cursor
    save_under(fb, stride, screen_w, screen_h, cursor.x, cursor.y, &mut save_buf);
    blit_cursor(fb, stride, screen_w, screen_h, cursor.x, cursor.y);

    info!("[stamen] Cursor daemon ready, entering event loop");

    let bristle_handle = cursor_evt_read;

    loop {
        // Yield for a bit so we don't spin (no port_wait with timeout currently)
        stem::thread::yield_now();

        // Poll all pending events
        let moved = poll_bristle_events(bristle_handle, &mut cursor, &mut metrics);

        if moved && cursor.moved_since(prev_x, prev_y) {
            // Restore old position
            restore_under(fb, stride, screen_w, screen_h, prev_x, prev_y, &save_buf);

            // Save new position and draw cursor
            save_under(fb, stride, screen_w, screen_h, cursor.x, cursor.y, &mut save_buf);
            blit_cursor(fb, stride, screen_w, screen_h, cursor.x, cursor.y);

            prev_x = cursor.x;
            prev_y = cursor.y;
            metrics.record_update();
        }

        metrics.maybe_log();
    }
}
