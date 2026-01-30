#![no_std]
#![no_main]

extern crate alloc;

use abi::display_driver_protocol as drvproto;
use abi::driver_frame::FrameReader;
use abi::ids::HandleId;
use abi::schema::{keys, kinds};
use stem::abi::module_manifest::{ManifestHeader, ModuleKind, MANIFEST_MAGIC};
use stem::info;
use stem::syscall::{port_recv, port_send, PortHandle};
use stem::thing::{sys as thingsys, ThingId};
use virtio_gpu::{Rect, VirtioGpu};

// ============================================================================
// Rect Utilities - no allocations, fast inline helpers
// ============================================================================

/// Compute the bounding box (union) of two rectangles
#[inline]
fn rect_union(a: Rect, b: Rect) -> Rect {
    let x1 = a.x.min(b.x);
    let y1 = a.y.min(b.y);
    let x2 = (a.x + a.w).max(b.x + b.w);
    let y2 = (a.y + a.h).max(b.y + b.h);
    Rect {
        x: x1,
        y: y1,
        w: x2.saturating_sub(x1),
        h: y2.saturating_sub(y1),
    }
}

/// Compute the area of a rectangle
#[inline]
fn rect_area(r: Rect) -> u64 {
    (r.w as u64) * (r.h as u64)
}

/// Check if a rectangle is empty (zero width or height)
#[inline]
fn rect_is_empty(r: Rect) -> bool {
    r.w == 0 || r.h == 0
}

/// Clamp a rectangle to screen bounds
#[inline]
fn rect_clamp_to_bounds(r: Rect, w: u32, h: u32) -> Rect {
    // Clamp origin to screen
    let x = r.x.min(w);
    let y = r.y.min(h);
    // Clamp extent to remaining screen space
    let max_w = w.saturating_sub(x);
    let max_h = h.saturating_sub(y);
    Rect {
        x,
        y,
        w: r.w.min(max_w),
        h: r.h.min(max_h),
    }
}

// ============================================================================
// Instrumentation counters for verification
// ============================================================================

struct Buffer {
    bs_id: ThingId,
    res_id: u32,
    phys: u64,
    last_present_seq: u64,
}

struct PresentStats {
    frame_count: u32,
    total_rects_in: u32,
    total_transfers: u32,
    total_flushes: u32,
    union_flush_count: u32,
    per_rect_flush_count: u32,
    using_swapchain: bool,
}

impl PresentStats {
    const fn new(swapchain: bool) -> Self {
        Self {
            frame_count: 0,
            total_rects_in: 0,
            total_transfers: 0,
            total_flushes: 0,
            union_flush_count: 0,
            per_rect_flush_count: 0,
            using_swapchain: swapchain,
        }
    }
    
    fn log_and_reset(&mut self) {
        if self.frame_count > 0 {
            info!(
                "display_virtio_gpu stats: frames={}, rects_in={}, transfers={}, flushes={}, union_flush={}, per_rect_flush={}, swapchain={}",
                self.frame_count,
                self.total_rects_in,
                self.total_transfers,
                self.total_flushes,
                self.union_flush_count,
                self.per_rect_flush_count,
                self.using_swapchain
            );
        }
        let sc = self.using_swapchain;
        *self = Self::new(sc);
    }
}

#[unsafe(link_section = ".thing_manifest")]
#[unsafe(no_mangle)]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Driver,
    device_kind: *b"dev.display.Gpu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    version: 1,
    _reserved: 0,
};

fn unpack_handle(arg: usize, index: u32) -> PortHandle {
    ((arg >> (index * 16)) & 0xFFFF) as PortHandle
}

fn send_msg(handle: PortHandle, msg_type: u16, payload: &[u8]) {
    let mut buf = [0u8; 256];
    if let Some(len) = drvproto::encode_message(&mut buf, msg_type, payload) {
        let _ = port_send(handle, &buf[..len]);
    }
}

fn find_gpu() -> Option<ThingId> {
    let mut buf = [ThingId::default(); 1];
    let count = thingsys::find(kinds::DEV_DISPLAY_GPU, &mut buf).ok()?;
    if count == 0 {
        return None;
    }
    Some(buf[0])
}

/// Query the boot framebuffer for display dimensions.
/// Falls back to 1024x768 if not found.
fn get_display_dimensions() -> (u32, u32, u32, u32) {
    let mut fb_buf = [ThingId::default(); 1];
    if let Ok(count) = thingsys::find(kinds::DEV_DISPLAY_FRAMEBUFFER, &mut fb_buf) {
        if count > 0 {
            let fb = fb_buf[0];
            let width = thingsys::prop_get(fb, keys::WIDTH).unwrap_or(1024) as u32;
            let height = thingsys::prop_get(fb, keys::HEIGHT).unwrap_or(768) as u32;
            let stride = width * 4;
            let format = thingsys::prop_get(fb, keys::FORMAT).unwrap_or(1) as u32;
            return (width, height, stride, format);
        }
    }
    // Fallback defaults
    (1024, 768, 1024 * 4, 1)
}

#[stem::main]
fn main(arg: usize) -> ! {
    let drv_req_read = unpack_handle(arg, 0);
    let drv_resp_write = unpack_handle(arg, 1);

    info!(
        "display_virtio_gpu: starting (drv_req_r={}, drv_resp_w={})",
        drv_req_read, drv_resp_write
    );

    // Find and initialize GPU
    let gpu_id = match find_gpu() {
        Some(id) => id,
        None => {
            info!("display_virtio_gpu: GPU device not found");
            loop {
                stem::yield_now();
            }
        }
    };

    let mut gpu = match VirtioGpu::new(gpu_id.to_u64_lossy()) {
        Ok(g) => g,
        Err(e) => {
            info!("display_virtio_gpu: Failed to initialize GPU: {:?}", e);
            loop {
                stem::yield_now();
            }
        }
    };

    if let Err(e) = gpu.init_virtio() {
        info!("display_virtio_gpu: Virtio init failed: {}", e);
        loop {
            stem::yield_now();
        }
    }

    info!("display_virtio_gpu: GPU initialized successfully");

    // =========================================================================
    // SWAPCHAIN SETUP: Create multiple GPU resources and bytespaces
    // =========================================================================
    let (disp_width, disp_height, disp_stride, disp_format) = get_display_dimensions();
    let disp_size = (disp_height as usize) * (disp_stride as usize);
    
    info!(
        "display_virtio_gpu: creating swapchain 1x {}x{} stride={} format={}",
        disp_width, disp_height, disp_stride, disp_format
    );
    
    // Single buffer for now - multi-buffer requires cross-process bytespace access
    let swapchain_count = 1;
    let mut swapchain_buffers = alloc::vec::Vec::new();
    for i in 0..swapchain_count {
        let bs_id = match thingsys::bytespace_create(disp_size, 0, disp_format as u64) {
            Ok(id) => id,
            Err(e) => {
                info!("display_virtio_gpu: bytespace_create failed: {:?}", e);
                loop { stem::yield_now(); }
            }
        };
        
        let phys = match thingsys::bytespace_phys(bs_id) {
            Ok(phys) => phys,
            Err(e) => {
                info!("display_virtio_gpu: bytespace_phys failed: {:?}", e);
                loop { stem::yield_now(); }
            }
        };

        // Explicitly map it locally so it stays pinned/resident
        let _ = thingsys::bytespace_map(bs_id);

        let res_id = (i + 1) as u32;
        gpu.set_dimensions(disp_width, disp_height);
        if let Err(e) = gpu.create_resource_2d(res_id) {
            info!("display_virtio_gpu: create_resource_2d failed: {}", e);
            loop { stem::yield_now(); }
        }
        if let Err(e) = gpu.attach_backing(res_id, phys, disp_size) {
            info!("display_virtio_gpu: attach_backing failed: {}", e);
            loop { stem::yield_now(); }
        }

        swapchain_buffers.push(Buffer {
            bs_id,
            res_id,
            phys,
            last_present_seq: 0,
        });
    }

    // Set initial scanout to first buffer
    if let Err(e) = gpu.set_scanout(swapchain_buffers[0].res_id, disp_width, disp_height) {
        info!("display_virtio_gpu: set_scanout failed: {}", e);
        loop { stem::yield_now(); }
    }
    
    info!(
        "display_virtio_gpu: swapchain ready (3 buffers)"
    );
    
    // Send MSG_REGISTER
    let register = drvproto::RegisterPayload {
        driver_kind: drvproto::DRIVER_KIND_VIRTIO_GPU,
        caps: drvproto::CAP_DIRTY_RECTS | drvproto::CAP_FULLFRAME,
    };
    let mut register_bytes = [0u8; drvproto::REGISTER_PAYLOAD_WIRE_SIZE];
    if let Some(len) = drvproto::encode_register_payload_le(&register, &mut register_bytes) {
        send_msg(
            drv_resp_write,
            drvproto::MSG_REGISTER,
            &register_bytes[..len],
        );
    }

    let mut buf = [0u8; 512];
    let mut frames = FrameReader::<4096>::new();
    
    let mut current_bs_id: Option<ThingId> = None;
    let mut current_res_id: u32 = 1;
    let mut next_buffer_idx = 0;
    let mut present_seq: u64 = 0;
    let mut last_presented_idx: Option<usize> = None;
    
    let mut stats = PresentStats::new(true);
    const STATS_LOG_INTERVAL: u32 = 120;

    loop {
        if let Ok(n) = port_recv(drv_req_read, &mut buf) {
            if n > 0 {
                frames.push(&buf[..n]);
            }
        }

        while let Some((header, payload)) = frames.next_message() {
            match header.msg_type {
                drvproto::MSG_HELLO => {
                    let want_caps = drvproto::decode_hello_payload_le(payload)
                        .map(|hello| hello.want_caps)
                        .unwrap_or(0);
                    let supported_caps = drvproto::CAP_DIRTY_RECTS | drvproto::CAP_FULLFRAME;
                    let welcome = drvproto::WelcomePayload {
                        proto_major: drvproto::PROTO_MAJOR,
                        proto_minor: drvproto::PROTO_MINOR,
                        have_caps: supported_caps & want_caps,
                        max_rects: 8,
                        reserved: 0,
                    };
                    let mut welcome_bytes = [0u8; drvproto::WELCOME_PAYLOAD_WIRE_SIZE];
                    if let Some(len) =
                        drvproto::encode_welcome_payload_le(&welcome, &mut welcome_bytes)
                    {
                        send_msg(drv_resp_write, drvproto::MSG_WELCOME, &welcome_bytes[..len]);
                    }
                }
                drvproto::MSG_ACQUIRE => {
                    let mut buffer_age = 0;
                    let idx = next_buffer_idx;
                    
                    if let Some(last_idx) = last_presented_idx {
                        let age = present_seq.saturating_sub(swapchain_buffers[idx].last_present_seq);
                        buffer_age = if swapchain_buffers[idx].last_present_seq == 0 {
                            0 // Never presented
                        } else {
                            age as u32
                        };
                    }

                    next_buffer_idx = (next_buffer_idx + 1) % swapchain_buffers.len();
                    
                    let acquired = drvproto::AcquiredPayload {
                        bytespace_id: swapchain_buffers[idx].bs_id.to_u64_lossy(),
                        width: disp_width,
                        height: disp_height,
                        stride: disp_stride,
                        format: disp_format,
                        buffer_age,
                        _pad: 0,
                    };
                    
                    current_bs_id = Some(swapchain_buffers[idx].bs_id);
                    current_res_id = swapchain_buffers[idx].res_id;
                    
                    let mut acq_bytes = [0u8; drvproto::ACQUIRED_PAYLOAD_WIRE_SIZE];
                    if let Some(len) = drvproto::encode_acquired_payload_le(&acquired, &mut acq_bytes) {
                        send_msg(drv_resp_write, drvproto::MSG_ACQUIRED, &acq_bytes[..len]);
                    }
                }
                drvproto::MSG_BIND => {
                    // MSG_BIND legacy fallback
                    if let Some(bind) = drvproto::decode_bind_payload_le(payload) {
                        let bs_id = ThingId({
                            let mut b = [0u8; 16];
                            b[0..8].copy_from_slice(&bind.bytespace_id.to_le_bytes());
                            b
                        });
                        current_bs_id = Some(bs_id);
                        // In legacy mode, we just stay on the first buffer's resource
                        current_res_id = swapchain_buffers[0].res_id;
                        send_msg(drv_resp_write, drvproto::MSG_ACK, &[]);
                    }
                }
                drvproto::MSG_PRESENT => {
                    if current_bs_id.is_none() {
                        let err = drvproto::ErrResp { code: 1 };
                        let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                        if let Some(len) = drvproto::encode_err_resp_le(&err, &mut err_bytes) {
                            send_msg(drv_resp_write, drvproto::MSG_ERR, &err_bytes[..len]);
                        }
                        continue;
                    }

                    if let Some(present) = drvproto::decode_present_header_le(payload) {
                        let rects_payload = &payload[drvproto::PRESENT_HEADER_WIRE_SIZE..];
                        
                        // Handle full-frame present (rect_count==0 or FULLFRAME flag)
                        if present.rect_count == 0
                            || (present._pad & drvproto::PRESENT_FLAG_FULLFRAME != 0)
                        {
                            let full_rect = Rect {
                                x: 0,
                                y: 0,
                                w: disp_width,
                                h: disp_height,
                            };
                            let _ = gpu.present_rect(current_res_id, full_rect);
                            stats.frame_count += 1;
                            stats.total_transfers += 1;
                            stats.total_flushes += 1;
                        } else {
                            // ============================================================
                            // GPU-Fast Present Path: batch transfers, smart flush
                            // ============================================================
                            
                            // Phase 1: Decode and clamp all rects, skip empty ones
                            let mut valid_rects: alloc::vec::Vec<Rect> = alloc::vec::Vec::new();
                            let rect_size = drvproto::RECT_WIRE_SIZE;
                            
                            for i in 0..present.rect_count as usize {
                                let off = i * rect_size;
                                if rects_payload.len() < off + rect_size {
                                    break;
                                }
                                if let Some(rect) =
                                    drvproto::decode_rect_le(&rects_payload[off..off + rect_size])
                                {
                                    let gpu_rect = Rect {
                                        x: rect.x,
                                        y: rect.y,
                                        w: rect.w,
                                        h: rect.h,
                                    };
                                    // Clamp to screen bounds and skip empty rects
                                    let clamped = rect_clamp_to_bounds(gpu_rect, disp_width, disp_height);
                                    if !rect_is_empty(clamped) {
                                        valid_rects.push(clamped);
                                    }
                                }
                            }
                            
                            stats.total_rects_in += valid_rects.len() as u32;
                            
                            if !valid_rects.is_empty() {
                                // Phase 2: Transfer all rects (bandwidth follows true damage)
                                for &rect in &valid_rects {
                                    let _ = gpu.transfer_to_host(current_res_id, rect);
                                }
                                stats.total_transfers += valid_rects.len() as u32;
                                
                                // Phase 3: Compute union and sum of areas for flush policy
                                let mut union_rect = valid_rects[0];
                                let mut sum_area: u64 = 0;
                                for &rect in &valid_rects {
                                    union_rect = rect_union(union_rect, rect);
                                    sum_area += rect_area(rect);
                                }
                                let union_area = rect_area(union_rect);
                                
                                // Phase 4: Smart flush policy
                                // If union is much larger than sum of individual rects,
                                // flush each rect separately to avoid giant flush area
                                if valid_rects.len() > 1 && union_area > sum_area * 2 {
                                    // Distant rects case: per-rect flush
                                    for &rect in &valid_rects {
                                        let _ = gpu.flush_resource(current_res_id, rect);
                                    }
                                    stats.total_flushes += valid_rects.len() as u32;
                                    stats.per_rect_flush_count += 1;
                                } else {
                                    // Common case: single union flush
                                    let _ = gpu.flush_resource(current_res_id, union_rect);
                                    stats.total_flushes += 1;
                                    stats.union_flush_count += 1;
                                }
                            }
                            stats.frame_count += 1;
                        }
                        
                        // ============================================================
                        // FLIP SCANOUT
                        // ============================================================
                        // Now that transfers and flushes for THIS resource are done,
                        // flip the hardware scanout to this resource ID.
                        let _ = gpu.set_scanout(current_res_id, disp_width, disp_height);
                        
                        // Update sequence and age bookkeeping
                        present_seq += 1;
                        let mut presented_idx = 0;
                        for (i, buf) in swapchain_buffers.iter_mut().enumerate() {
                            if buf.res_id == current_res_id {
                                buf.last_present_seq = present_seq;
                                presented_idx = i;
                                break;
                            }
                        }
                        last_presented_idx = Some(presented_idx);

                        // Rate-limited stats logging
                        if stats.frame_count >= STATS_LOG_INTERVAL {
                            stats.log_and_reset();
                        }
                    }
                    send_msg(drv_resp_write, drvproto::MSG_ACK, &[]);
                }
                _ => {}
            }
        }
        stem::yield_now();
    }
}
