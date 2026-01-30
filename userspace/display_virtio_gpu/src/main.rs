#![no_std]
#![no_main]

extern crate alloc;

use abi::display_driver_protocol as drvproto;
use abi::driver_frame::FrameReader;
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

struct PresentStats {
    frame_count: u32,
    total_rects_in: u32,
    total_transfers: u32,
    total_flushes: u32,
    union_flush_count: u32,
    per_rect_flush_count: u32,
    using_swapchain: bool,
}

/// Entry in the texture registry mapping client IDs to GPU resource IDs
struct TextureEntry {
    resource_id: u32,
    width: u32,
    height: u32,
}

/// Next resource ID for texture allocation
static NEXT_TEXTURE_RESOURCE_ID: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(1000);

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
    // VIRGL 3D BRING-UP TEST: Verify 3D command path works
    // =========================================================================
    if gpu.has_3d_feature() {
        info!("display_virtio_gpu: Virgl 3D supported - running bring-up test...");
        let test_ctx_id = 1u32;
        let render_target_id = 100u32;
        
        // Step 1: Create virgl context
        match gpu.create_context(test_ctx_id, b"virgl_test") {
            Ok(()) => {
                info!("display_virtio_gpu: [VIRGL 1/5] Context created");
            }
            Err(e) => {
                info!("display_virtio_gpu: [VIRGL FAIL] Context creation failed: {}", e);
                // Continue with 2D fallback
            }
        }
        
        // Step 2: Create 3D render target resource
        let (w, h, _, _) = get_display_dimensions();
        let target_type = 2;   // PIPE_TEXTURE_2D
        let format = 2;        // PIPE_FORMAT_B8G8R8X8_UNORM
        let bind = 2;          // PIPE_BIND_RENDER_TARGET
        
        if let Err(e) = gpu.create_resource_3d(render_target_id, target_type, format, bind, w, h, 1) {
            info!("display_virtio_gpu: [VIRGL FAIL] create_resource_3d failed: {}", e);
        } else {
            info!("display_virtio_gpu: [VIRGL 2/5] 3D resource created {}x{}", w, h);
        }
        
        // Step 3: Attach resource to context
        if let Err(e) = gpu.ctx_attach_resource(test_ctx_id, render_target_id) {
            info!("display_virtio_gpu: [VIRGL FAIL] ctx_attach_resource failed: {}", e);
        } else {
            info!("display_virtio_gpu: [VIRGL 3/5] Resource attached to context");
        }
        
        // Step 4: Build and submit CLEAR command
        // Virgl command format: cmd_header(cmd, obj_type, payload_len), then payload words
        // CLEAR command: buffers, r, g, b, a, depth_lo, depth_hi, stencil
        let mut cmds: [u32; 32] = [0; 32];
        let mut idx = 0;
        
        // Create surface object for render target (handle=1)
        let surface_handle = 1u32;
        cmds[idx] = (1 & 0xff) | ((8 & 0xff) << 8) | ((5 & 0xffff) << 16); // CREATE_OBJECT=1, SURFACE=8, len=5
        idx += 1;
        cmds[idx] = surface_handle;  // handle
        idx += 1;
        cmds[idx] = render_target_id; // resource handle
        idx += 1;
        cmds[idx] = format;           // format
        idx += 1;
        cmds[idx] = 0;                // first_element
        idx += 1;
        cmds[idx] = 0;                // last_element
        idx += 1;
        
        // Set framebuffer state (nr_cbufs=1, zsurf=0, cbuf[0]=surface_handle)
        cmds[idx] = (5 & 0xff) | ((0 & 0xff) << 8) | ((3 & 0xffff) << 16); // SET_FRAMEBUFFER_STATE=5, len=3
        idx += 1;
        cmds[idx] = 1;               // nr_cbufs
        idx += 1;
        cmds[idx] = 0;               // zsurf_handle (no depth buffer)
        idx += 1;
        cmds[idx] = surface_handle;  // cbuf[0]
        idx += 1;
        
        // Set viewport state for full screen
        let scale_x = (w as f32) / 2.0;
        let scale_y = -(h as f32) / 2.0;
        let translate_x = (w as f32) / 2.0;
        let translate_y = (h as f32) / 2.0;
        cmds[idx] = (4 & 0xff) | ((0 & 0xff) << 8) | ((7 & 0xffff) << 16); // SET_VIEWPORT_STATE=4, len=7
        idx += 1;
        cmds[idx] = 0;  // start_slot
        idx += 1;
        cmds[idx] = scale_x.to_bits();
        idx += 1;
        cmds[idx] = scale_y.to_bits();
        idx += 1;
        cmds[idx] = 0.5f32.to_bits();  // scale_z
        idx += 1;
        cmds[idx] = translate_x.to_bits();
        idx += 1;
        cmds[idx] = translate_y.to_bits();
        idx += 1;
        cmds[idx] = 0.5f32.to_bits();  // translate_z
        idx += 1;
        
        // CLEAR command (clear to cyan color)
        let depth: f64 = 1.0;
        let depth_bits = depth.to_bits();
        cmds[idx] = (7 & 0xff) | ((0 & 0xff) << 8) | ((8 & 0xffff) << 16); // CLEAR=7, len=8
        idx += 1;
        cmds[idx] = 4;  // buffers = PIPE_CLEAR_COLOR
        idx += 1;
        cmds[idx] = 0.0f32.to_bits();   // r
        idx += 1;
        cmds[idx] = 0.8f32.to_bits();   // g (cyan)
        idx += 1;
        cmds[idx] = 0.8f32.to_bits();   // b (cyan)
        idx += 1;
        cmds[idx] = 1.0f32.to_bits();   // a
        idx += 1;
        cmds[idx] = depth_bits as u32;       // depth_lo
        idx += 1;
        cmds[idx] = (depth_bits >> 32) as u32; // depth_hi
        idx += 1;
        cmds[idx] = 0;  // stencil
        idx += 1;
        
        // Convert to bytes for submit_3d
        let cmd_bytes = unsafe {
            core::slice::from_raw_parts(
                cmds.as_ptr() as *const u8,
                idx * 4,
            )
        };
        
        match gpu.submit_3d(test_ctx_id, cmd_bytes) {
            Ok(()) => info!("display_virtio_gpu: [VIRGL 4/5] CLEAR command submitted ({} bytes)", idx * 4),
            Err(e) => info!("display_virtio_gpu: [VIRGL FAIL] submit_3d failed: {}", e),
        }
        
        // Step 5: Set scanout to show the 3D render target and flush
        if let Err(e) = gpu.set_scanout(render_target_id, w, h) {
            info!("display_virtio_gpu: [VIRGL FAIL] set_scanout failed: {}", e);
        } else {
            info!("display_virtio_gpu: [VIRGL 5/5] Scanout set to 3D render target");
        }
        
         // Flush to display
        let rect = virtio_gpu::Rect { x: 0, y: 0, w: w, h: h };
        if let Err(e) = gpu.flush_resource(render_target_id, rect) {
            info!("display_virtio_gpu: [VIRGL FAIL] flush_resource failed: {}", e);
        } else {
            info!("display_virtio_gpu: [VIRGL OK] Virgl clear test complete! Screen should be cyan.");
        }

        // =====================================================================
        // TEXTURE UPLOAD TEST: Test CPU→GPU texture transfer
        // =====================================================================
        info!("display_virtio_gpu: [TEXTURE] Starting texture upload test...");
        
        let texture_id = 101u32;
        let tex_w = 100u32;
        let tex_h = 100u32;
        let tex_stride = tex_w * 4; // BGRA32
        let tex_size = (tex_h * tex_stride) as usize;
        
        // Create a 3D texture resource (SAMPLER bind for textures)
        let tex_target = 2;   // PIPE_TEXTURE_2D
        let tex_format = 2;   // PIPE_FORMAT_B8G8R8X8_UNORM
        let tex_bind = 8;     // PIPE_BIND_SAMPLER_VIEW
        
        if let Err(e) = gpu.create_resource_3d(texture_id, tex_target, tex_format, tex_bind, tex_w, tex_h, 1) {
            info!("display_virtio_gpu: [TEXTURE FAIL] create_resource_3d failed: {}", e);
        } else {
            info!("display_virtio_gpu: [TEXTURE 1/4] Texture resource created {}x{}", tex_w, tex_h);
            
            // Attach texture to context
            if let Err(e) = gpu.ctx_attach_resource(test_ctx_id, texture_id) {
                info!("display_virtio_gpu: [TEXTURE FAIL] ctx_attach_resource failed: {}", e);
            } else {
                info!("display_virtio_gpu: [TEXTURE 2/4] Texture attached to context");
            }
            
            // Allocate DMA memory for texture data
            use stem::syscall::{device_alloc_dma, device_dma_phys};
            let pages = (tex_size + 4095) / 4096;
            match device_alloc_dma(gpu.claim_handle(), pages) {
                Ok(tex_virt) => {
                    match device_dma_phys(tex_virt) {
                        Ok(tex_phys) => {
                            // Fill texture with a magenta color (B=255, G=0, R=255)
                            let ptr = tex_virt as *mut u32;
                            for i in 0..(tex_w * tex_h) as usize {
                                unsafe { core::ptr::write_volatile(ptr.add(i), 0x00FF00FF) }; // BGRA magenta
                            }
                            info!("display_virtio_gpu: [TEXTURE 2.5/4] Texture data filled (magenta)");
                            
                            // Attach backing memory
                            if let Err(e) = gpu.attach_backing_3d(texture_id, tex_phys, tex_size) {
                                info!("display_virtio_gpu: [TEXTURE FAIL] attach_backing_3d failed: {}", e);
                            } else {
                                info!("display_virtio_gpu: [TEXTURE 3/4] Backing memory attached");
                                
                                // Transfer texture data to GPU
                                if let Err(e) = gpu.transfer_to_host_3d(test_ctx_id, texture_id, tex_w, tex_h, 0, tex_stride) {
                                    info!("display_virtio_gpu: [TEXTURE FAIL] transfer_to_host_3d failed: {}", e);
                                } else {
                                    info!("display_virtio_gpu: [TEXTURE 4/4] Texture data uploaded to GPU!");
                                    info!("display_virtio_gpu: [TEXTURE OK] Texture upload test complete!");
                                    
                                    // =========================================
                                    // BLIT TEST: Copy texture to render target
                                    // =========================================
                                    info!("display_virtio_gpu: [BLIT] Testing texture-to-framebuffer BLIT...");
                                    
                                    // Build BLIT command manually (21 dwords as per virgl protocol)
                                    let mut blit_cmds: [u32; 32] = [0; 32];
                                    let mut idx = 0;
                                    
                                    // BLIT command header: cmd=16, obj_type=0, len=21
                                    blit_cmds[idx] = (16 & 0xff) | ((0 & 0xff) << 8) | ((21 & 0xffff) << 16);
                                    idx += 1;
                                    
                                    // s0: mask=0xf (RGBA), filter=0 (NEAREST), no scissor
                                    blit_cmds[idx] = 0xf;
                                    idx += 1;
                                    
                                    // Destination box (x=100, y=100, w=100, h=100)
                                    blit_cmds[idx] = 100; idx += 1;  // dst.box.x
                                    blit_cmds[idx] = 100; idx += 1;  // dst.box.y
                                    blit_cmds[idx] = 0;   idx += 1;  // dst.box.z
                                    blit_cmds[idx] = tex_w; idx += 1; // dst.box.width
                                    blit_cmds[idx] = tex_h; idx += 1; // dst.box.height
                                    blit_cmds[idx] = 1;   idx += 1;  // dst.box.depth
                                    
                                    // Source box (full texture)
                                    blit_cmds[idx] = 0;   idx += 1;  // src.box.x
                                    blit_cmds[idx] = 0;   idx += 1;  // src.box.y
                                    blit_cmds[idx] = 0;   idx += 1;  // src.box.z
                                    blit_cmds[idx] = tex_w; idx += 1; // src.box.width
                                    blit_cmds[idx] = tex_h; idx += 1; // src.box.height
                                    blit_cmds[idx] = 1;   idx += 1;  // src.box.depth
                                    
                                    // Destination resource (render target)
                                    blit_cmds[idx] = render_target_id; idx += 1; // dst.resource
                                    blit_cmds[idx] = 0;   idx += 1;  // dst.level
                                    blit_cmds[idx] = 2;   idx += 1;  // dst.format (BGRA)
                                    
                                    // Source resource (texture)
                                    blit_cmds[idx] = texture_id; idx += 1; // src.resource
                                    blit_cmds[idx] = 0;   idx += 1;  // src.level
                                    blit_cmds[idx] = 2;   idx += 1;  // src.format (BGRA)
                                    
                                    // Sample0 mask
                                    blit_cmds[idx] = 0;   idx += 1;
                                    
                                    let blit_bytes = unsafe {
                                        core::slice::from_raw_parts(
                                            blit_cmds.as_ptr() as *const u8,
                                            idx * 4,
                                        )
                                    };
                                    
                                    match gpu.submit_3d(test_ctx_id, blit_bytes) {
                                        Ok(()) => info!("display_virtio_gpu: [BLIT OK] Texture blitted to (100,100)! Magenta square should appear."),
                                        Err(e) => info!("display_virtio_gpu: [BLIT FAIL] submit_3d failed: {}", e),
                                    }
                                    
                                    // Flush to display
                                    let blit_rect = virtio_gpu::Rect { x: 0, y: 0, w: w, h: h };
                                    let _ = gpu.flush_resource(render_target_id, blit_rect);
                                }
                            }
                        }
                        Err(e) => info!("display_virtio_gpu: [TEXTURE FAIL] dma_phys failed: {:?}", e),
                    }
                }
                Err(e) => info!("display_virtio_gpu: [TEXTURE FAIL] alloc_dma failed: {:?}", e),
            }
        }
    } else {
        info!("display_virtio_gpu: Virgl 3D not supported, using 2D only");
    }

    // =========================================================================
    // FRAME POOL SETUP: Create multiple GPU resources and bytespaces
    // =========================================================================
    let (disp_width, disp_height, disp_stride, disp_format) = get_display_dimensions();
    let disp_size = (disp_height as usize) * (disp_stride as usize);
    
    // Use triple-buffering for smooth presentation
    const FRAME_COUNT: usize = 3;
    
    info!(
        "display_virtio_gpu: creating frame pool {}x {}x{} stride={} format={}",
        FRAME_COUNT, disp_width, disp_height, disp_stride, disp_format
    );
    
    let mut frame_resources = alloc::vec::Vec::new();
    for i in 0..FRAME_COUNT {
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
        let virt = match thingsys::bytespace_map(bs_id) {
            Ok(v) => v,
            Err(_) => core::ptr::null_mut(),
        };

        let res_id = (i + 1) as u32;
        gpu.set_dimensions(disp_width, disp_height);
        if let Err(e) = gpu.create_resource_2d(res_id) {
            info!("display_virtio_gpu: create_resource_2d failed: {}", e);
            loop { stem::yield_now(); }
        }
        if let Err(e) = gpu.attach_backing(res_id, phys, disp_size, disp_stride) {
            info!("display_virtio_gpu: attach_backing failed: {}", e);
            loop { stem::yield_now(); }
        }

        frame_resources.push(virtio_gpu::FrameResource::new(
            res_id,
            phys,
            bs_id.to_u64_lossy(),
            disp_size,
            disp_width,
            disp_height,
            disp_stride,
            disp_format,
            virt as u64,
        ));
    }

    // Create frame pool and present queue
    let mut frame_pool = virtio_gpu::FramePool::new(frame_resources);
    let mut present_queue = virtio_gpu::PresentQueue::new(2); // Conservative: max 2 in-flight
    let mut surface = virtio_gpu::DisplaySurface::new(0, disp_width, disp_height, disp_format);
    
    // Set initial scanout to first resource
    if let Some(first_frame) = frame_pool.get_frame(&virtio_gpu::FrameHandle { index: 0 }) {
        if let Err(e) = gpu.set_scanout(first_frame.resource_id, disp_width, disp_height) {
            info!("display_virtio_gpu: set_scanout failed: {}", e);
            loop { stem::yield_now(); }
        }
        surface.current_resource = Some(first_frame.resource_id);
    }
    
    info!(
        "display_virtio_gpu: frame pool ready ({} buffers)",
        frame_pool.frame_count()
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
    
    // Track current acquired frame
    let mut current_frame_handle: Option<virtio_gpu::FrameHandle> = None;
    
    let mut stats = PresentStats::new(false); // Not using old swapchain
    const STATS_LOG_INTERVAL: u32 = 120;

    // Texture registry for 3D textures (client_id → TextureEntry)
    let mut texture_registry: alloc::collections::BTreeMap<u64, TextureEntry> = alloc::collections::BTreeMap::new();

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
                    // Try to acquire a free frame from the pool
                    if let Some(handle) = frame_pool.acquire_frame() {
                        if let Some(frame) = frame_pool.get_frame(&handle) {
                            // Calculate buffer age
                            let buffer_age = frame_pool.buffer_age(handle.index, present_queue.current_sequence());
                            
                            info!(
                                "display_virtio_gpu: acquired frame idx={} res={} age={}",
                                handle.index, frame.resource_id, buffer_age
                            );
                            
                            let acquired = drvproto::AcquiredPayload {
                                bytespace_id: frame.bytespace_id,
                                width: frame.width,
                                height: frame.height,
                                stride: frame.stride,
                                format: frame.format,
                                buffer_age,
                                _pad: 0,
                            };
                            
                            // Store the handle for when we receive PRESENT
                            current_frame_handle = Some(handle);
                            
                            let mut acq_bytes = [0u8; drvproto::ACQUIRED_PAYLOAD_WIRE_SIZE];
                            if let Some(len) = drvproto::encode_acquired_payload_le(&acquired, &mut acq_bytes) {
                                send_msg(drv_resp_write, drvproto::MSG_ACQUIRED, &acq_bytes[..len]);
                            }
                        }
                    } else {
                        // No frames available - all are in-flight
                        // Complete oldest to make room
                        info!("display_virtio_gpu: all frames in-flight, completing oldest");
                        if present_queue.pending_count() > 0 {
                            present_queue.complete_oldest(&mut frame_pool);
                        }
                        // Retry immediately
                        if let Some(handle) = frame_pool.acquire_frame() {
                            if let Some(frame) = frame_pool.get_frame(&handle) {
                                let buffer_age = frame_pool.buffer_age(handle.index, present_queue.current_sequence());
                                
                                info!(
                                    "display_virtio_gpu: acquired frame (retry) idx={} res={} age={}",
                                    handle.index, frame.resource_id, buffer_age
                                );
                                
                                let acquired = drvproto::AcquiredPayload {
                                    bytespace_id: frame.bytespace_id,
                                    width: frame.width,
                                    height: frame.height,
                                    stride: frame.stride,
                                    format: frame.format,
                                    buffer_age,
                                    _pad: 0,
                                };
                                
                                current_frame_handle = Some(handle);
                                
                                let mut acq_bytes = [0u8; drvproto::ACQUIRED_PAYLOAD_WIRE_SIZE];
                                if let Some(len) = drvproto::encode_acquired_payload_le(&acquired, &mut acq_bytes) {
                                    send_msg(drv_resp_write, drvproto::MSG_ACQUIRED, &acq_bytes[..len]);
                                }
                            }
                        }
                    }
                }
                drvproto::MSG_BIND => {
                    // Legacy BIND is no longer supported with frame pool
                    // Client must use ACQUIRE instead
                    let err = drvproto::ErrResp { code: 255 }; // Unsupported
                    let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                    if let Some(len) = drvproto::encode_err_resp_le(&err, &mut err_bytes) {
                        send_msg(drv_resp_write, drvproto::MSG_ERR, &err_bytes[..len]);
                    }
                }
                drvproto::MSG_PRESENT => {
                    // We must have an acquired frame to present
                    let handle = match current_frame_handle.take() {
                        Some(h) => h,
                        None => {
                            let err = drvproto::ErrResp { code: 1 };
                            let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                            if let Some(len) = drvproto::encode_err_resp_le(&err, &mut err_bytes) {
                                send_msg(drv_resp_write, drvproto::MSG_ERR, &err_bytes[..len]);
                            }
                            continue;
                        }
                    };

                    if let Some(present) = drvproto::decode_present_header_le(payload) {
                        let rects_payload = &payload[drvproto::PRESENT_HEADER_WIRE_SIZE..];
                        
                        // Get the frame resource
                        let resource_id = if let Some(frame) = frame_pool.get_frame(&handle) {
                            frame.resource_id
                        } else {
                            continue;
                        };
                        
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
                            let _ = gpu.present_rect(resource_id, full_rect);
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
                                    let _ = gpu.transfer_to_host(resource_id, rect);
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
                                        let _ = gpu.flush_resource(resource_id, rect);
                                    }
                                    stats.total_flushes += valid_rects.len() as u32;
                                    stats.per_rect_flush_count += 1;
                                } else {
                                    // Common case: single union flush
                                    let _ = gpu.flush_resource(resource_id, union_rect);
                                    stats.total_flushes += 1;
                                    stats.union_flush_count += 1;
                                }
                            }
                            stats.frame_count += 1;
                        }
                        
                        // ============================================================
                        // FLIP SCANOUT and ENQUEUE PRESENT
                        // ============================================================
                        // Flip the hardware scanout to this resource
                        // Only set scanout if it's a new resource
                        let needs_scanout = surface.current_resource != Some(resource_id);
                        if needs_scanout {
                            let _ = gpu.set_scanout(resource_id, disp_width, disp_height);
                            surface.current_resource = Some(resource_id);
                            info!(
                                "display_virtio_gpu: set_scanout to resource {}",
                                resource_id
                            );
                        }
                        
                        // Enqueue the frame in the present queue (marks it in-flight)
                        let seq = present_queue.enqueue_present(&mut frame_pool, handle);
                        
                        info!(
                            "display_virtio_gpu: present seq={} res={} rects={} pending={}",
                            seq, resource_id, present.rect_count, present_queue.pending_count()
                        );
                        
                        // Conservative completion: immediately complete oldest present
                        // to simulate fence completion (in a real impl, this would be driven by GPU IRQ)
                        if present_queue.pending_count() > 1 {
                            if let Some(completed_idx) = present_queue.complete_oldest(&mut frame_pool) {
                                info!(
                                    "display_virtio_gpu: completed frame idx={} (conservative)",
                                    completed_idx
                                );
                            }
                        }

                        // Rate-limited stats logging
                        if stats.frame_count >= STATS_LOG_INTERVAL {
                            stats.log_and_reset();
                        }
                    }
                    send_msg(drv_resp_write, drvproto::MSG_ACK, &[]);
                }
                drvproto::MSG_SUBMIT_3D => {
                    // Parse Submit3d header
                    if let Some(hdr) = drvproto::decode_submit_3d_header_le(payload) {
                        let cmd_buf = &payload[drvproto::SUBMIT_3D_HEADER_WIRE_SIZE..];
                        if cmd_buf.len() >= hdr.cmd_len as usize {
                            // Submit the virgl commands to the GPU
                            match gpu.submit_3d(hdr.ctx_id, &cmd_buf[..hdr.cmd_len as usize]) {
                                Ok(()) => {
                                    send_msg(drv_resp_write, drvproto::MSG_ACK, &[]);
                                }
                                Err(_e) => {
                                    let err = drvproto::ErrResp { code: 3 };
                                    let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                                    if let Some(len) = drvproto::encode_err_resp_le(&err, &mut err_bytes) {
                                        send_msg(drv_resp_write, drvproto::MSG_ERR, &err_bytes[..len]);
                                    }
                                }
                            }
                        } else {
                            // Buffer too short
                            let err = drvproto::ErrResp { code: 2 };
                            let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                            if let Some(len) = drvproto::encode_err_resp_le(&err, &mut err_bytes) {
                                send_msg(drv_resp_write, drvproto::MSG_ERR, &err_bytes[..len]);
                            }
                        }
                    }
                }
                drvproto::MSG_CREATE_TEXTURE_3D => {
                    // Create a GPU texture resource
                    if let Some(hdr) = drvproto::decode_create_texture_3d_header_le(payload) {
                        let resource_id = NEXT_TEXTURE_RESOURCE_ID.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
                        
                        // Create 3D resource via VirtIO GPU
                        // target=0 (PIPE_TEXTURE_2D), format=2 (B8G8R8X8), bind=2 (RENDER_TARGET) | 8 (SAMPLER)
                        let tex_target = 0; // PIPE_TEXTURE_2D
                        let tex_format = hdr.format; // Usually 2 for BGRA
                        let tex_bind = 2 | 8; // RENDER_TARGET | SAMPLER_VIEW
                        
                        let result = gpu.create_resource_3d(
                            resource_id,
                            tex_target,
                            tex_format,
                            tex_bind,
                            hdr.width,
                            hdr.height,
                            1, // depth
                        );
                        
                        let status = if result.is_ok() {
                            // Attach resource to virgl context
                            let ctx_id = 1; // Main virgl context
                            if gpu.ctx_attach_resource(ctx_id, resource_id).is_ok() {
                                texture_registry.insert(hdr.client_id, TextureEntry {
                                    resource_id,
                                    width: hdr.width,
                                    height: hdr.height,
                                });
                                0 // Success
                            } else {
                                2 // Attach failed
                            }
                        } else {
                            1 // Create failed
                        };
                        
                        // Send response
                        let resp = drvproto::TextureCreatedResponse {
                            client_id: hdr.client_id,
                            resource_id,
                            status,
                        };
                        let mut resp_bytes = [0u8; drvproto::TEXTURE_CREATED_RESPONSE_WIRE_SIZE];
                        if let Some(len) = drvproto::encode_texture_created_response_le(&resp, &mut resp_bytes) {
                            send_msg(drv_resp_write, drvproto::MSG_TEXTURE_CREATED, &resp_bytes[..len]);
                        }
                    }
                }
                drvproto::MSG_UPLOAD_TEXTURE_3D => {
                    // Upload pixel data to an existing texture
                    if let Some(hdr) = drvproto::decode_upload_texture_3d_header_le(payload) {
                        let pixel_data = &payload[drvproto::UPLOAD_TEXTURE_3D_HEADER_WIRE_SIZE..];
                        
                        if pixel_data.len() >= hdr.data_len as usize {
                            let data_slice = &pixel_data[..hdr.data_len as usize];
                            
                            // Allocate DMA-accessible memory for texture data
                            match thingsys::bytespace_create(hdr.data_len as usize, 0, 0) {
                                Ok(bs_id) => {
                                    // Map the bytespace to get a writable pointer
                                    match thingsys::bytespace_map(bs_id) {
                                        Ok(ptr) => {
                                            // Copy pixel data to DMA buffer
                                            unsafe {
                                                core::ptr::copy_nonoverlapping(
                                                    data_slice.as_ptr(),
                                                    ptr as *mut u8,
                                                    hdr.data_len as usize,
                                                );
                                            }
                                            
                                            // Get physical address for attach_backing_3d
                                            match thingsys::bytespace_phys(bs_id) {
                                                Ok(phys_addr) => {
                                                    // Attach backing and transfer
                                                    if gpu.attach_backing_3d(hdr.resource_id, phys_addr, hdr.data_len as usize).is_ok() {
                                                        if gpu.transfer_to_host_3d(1, hdr.resource_id, hdr.width, hdr.height, hdr.x as u64, hdr.stride).is_ok() {
                                                            send_msg(drv_resp_write, drvproto::MSG_ACK, &[]);
                                                        } else {
                                                            let err = drvproto::ErrResp { code: 4 };
                                                            let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                                                            if let Some(len) = drvproto::encode_err_resp_le(&err, &mut err_bytes) {
                                                                send_msg(drv_resp_write, drvproto::MSG_ERR, &err_bytes[..len]);
                                                            }
                                                        }
                                                    } else {
                                                        let err = drvproto::ErrResp { code: 5 };
                                                        let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                                                        if let Some(len) = drvproto::encode_err_resp_le(&err, &mut err_bytes) {
                                                            send_msg(drv_resp_write, drvproto::MSG_ERR, &err_bytes[..len]);
                                                        }
                                                    }
                                                }
                                                Err(_) => {
                                                    let err = drvproto::ErrResp { code: 6 };
                                                    let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                                                    if let Some(len) = drvproto::encode_err_resp_le(&err, &mut err_bytes) {
                                                        send_msg(drv_resp_write, drvproto::MSG_ERR, &err_bytes[..len]);
                                                    }
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            let err = drvproto::ErrResp { code: 7 };
                                            let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                                            if let Some(len) = drvproto::encode_err_resp_le(&err, &mut err_bytes) {
                                                send_msg(drv_resp_write, drvproto::MSG_ERR, &err_bytes[..len]);
                                            }
                                        }
                                    }
                                }
                                Err(_) => {
                                    let err = drvproto::ErrResp { code: 8 };
                                    let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                                    if let Some(len) = drvproto::encode_err_resp_le(&err, &mut err_bytes) {
                                        send_msg(drv_resp_write, drvproto::MSG_ERR, &err_bytes[..len]);
                                    }
                                }
                            }
                        } else {
                            let err = drvproto::ErrResp { code: 2 };
                            let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                            if let Some(len) = drvproto::encode_err_resp_le(&err, &mut err_bytes) {
                                send_msg(drv_resp_write, drvproto::MSG_ERR, &err_bytes[..len]);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        stem::yield_now();
    }
}
