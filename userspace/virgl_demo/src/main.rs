//! Virgl 3D Demo - Hello Triangle
//!
//! Minimal virgl demo that:
//! 1. Creates virgl context
//! 2. Creates a render target resource
//! 3. Clears to a solid color (simpler than triangle for initial bring-up)
//! 4. Presents to scanout
//!
//! ## Debugging Virgl Errors
//!
//! Virglrenderer errors are logged by QEMU to the **host console**, not the guest.
//! If you see "Illegal command buffer" or similar errors in the QEMU output,
//! check the command buffer diagnostics logged on the first frame.
//!
//! Common issues:
//! - Incorrect command header format (cmd, obj_type, length)
//! - Surface handles not created before use
//! - Resource IDs not attached to context
//! - Format mismatches between resources and surfaces

#![feature(restricted_std)]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use core::time::Duration;
use stem::info;
use virtio_gpu::VirtioGpu;

// ============================================================================
// Virgl Protocol Commands (sent via SUBMIT_3D)
// ============================================================================
// References: Mesa's virgl_protocol.h

/// Virgl command opcodes (VIRGL_CCMD_*)
mod virgl_cmd {
    pub const NOP: u32 = 0;
    pub const CREATE_OBJECT: u32 = 1;
    pub const BIND_OBJECT: u32 = 2;
    pub const DESTROY_OBJECT: u32 = 3;
    pub const SET_VIEWPORT_STATE: u32 = 4;
    pub const SET_FRAMEBUFFER_STATE: u32 = 5;
    pub const SET_VERTEX_BUFFERS: u32 = 6;
    pub const CLEAR: u32 = 7;
    pub const DRAW_VBO: u32 = 8;
    pub const RESOURCE_INLINE_WRITE: u32 = 9;
    pub const SET_SAMPLER_VIEWS: u32 = 10;
    pub const SET_INDEX_BUFFER: u32 = 11;
    pub const SET_CONSTANT_BUFFER: u32 = 12;
    pub const SET_STENCIL_REF: u32 = 13;
    pub const SET_BLEND_COLOR: u32 = 14;
    pub const SET_SCISSOR_STATE: u32 = 15;
    pub const BLIT: u32 = 16;
    pub const RESOURCE_COPY_REGION: u32 = 17;
    pub const BIND_SAMPLER_STATES: u32 = 18;
    pub const BEGIN_QUERY: u32 = 19;
    pub const END_QUERY: u32 = 20;
    pub const GET_QUERY_RESULT: u32 = 21;
    pub const SET_POLYGON_STIPPLE: u32 = 22;
    pub const SET_CLIP_STATE: u32 = 23;
    pub const SET_SAMPLE_MASK: u32 = 24;
    pub const SET_STREAMOUT_TARGETS: u32 = 25;
    pub const SET_RENDER_CONDITION: u32 = 26;
    pub const SET_UNIFORM_BUFFER: u32 = 27;
    pub const SET_SUB_CTX: u32 = 28;
    pub const CREATE_SUB_CTX: u32 = 29;
    pub const DESTROY_SUB_CTX: u32 = 30;
    pub const BIND_SHADER: u32 = 31;
    pub const SET_TESS_STATE: u32 = 32;
    pub const SET_MIN_SAMPLES: u32 = 33;
    pub const SET_SHADER_BUFFERS: u32 = 34;
    pub const SET_SHADER_IMAGES: u32 = 35;
    pub const MEMORY_BARRIER: u32 = 36;
    pub const LAUNCH_GRID: u32 = 37;
    pub const SET_FRAMEBUFFER_STATE_NO_ATTACH: u32 = 38;
    pub const TEXTURE_BARRIER: u32 = 39;
    pub const SET_ATOMIC_BUFFERS: u32 = 40;
    pub const SET_DEBUG_FLAGS: u32 = 41;
    pub const GET_QBO_RESULT: u32 = 42;
    pub const TRANSFER3D: u32 = 43;
    pub const END_TRANSFERS: u32 = 44;
    pub const COPY_TRANSFER3D: u32 = 45;
    pub const SET_TWEAKS: u32 = 46;
    pub const CLEAR_TEXTURE: u32 = 47;
    pub const PIPE_RESOURCE_CREATE: u32 = 48;
    pub const PIPE_RESOURCE_SET_TYPE: u32 = 49;
    pub const GET_MEMORY_INFO: u32 = 50;
    pub const SEND_STRING_MARKER: u32 = 51;
    pub const LINK_SHADER: u32 = 52;
}

/// Object types for CREATE_OBJECT
mod virgl_object {
    pub const BLEND: u32 = 1;
    pub const RASTERIZER: u32 = 2;
    pub const DSA: u32 = 3; // Depth-stencil-alpha
    pub const SHADER: u32 = 4;
    pub const VERTEX_ELEMENTS: u32 = 5;
    pub const SAMPLER_VIEW: u32 = 6;
    pub const SAMPLER_STATE: u32 = 7;
    pub const SURFACE: u32 = 8;
    pub const QUERY: u32 = 9;
    pub const STREAMOUT_TARGET: u32 = 10;
}

/// CLEAR buffer bits
mod clear_bits {
    pub const COLOR: u32 = 1 << 2;
    pub const DEPTH: u32 = 1 << 0;
    pub const STENCIL: u32 = 1 << 1;
}

/// Build virgl command header word
fn cmd_header(cmd: u32, object_type: u32, payload_len: u32) -> u32 {
    // Format: [cmd:8][obj:8][len:16]
    (cmd & 0xff) | ((object_type & 0xff) << 8) | ((payload_len & 0xffff) << 16)
}

/// Virgl command stream builder
struct VirglCommandBuilder {
    cmds: Vec<u32>,
}

impl VirglCommandBuilder {
    fn new() -> Self {
        Self { cmds: Vec::new() }
    }

    /// Clear color buffer to RGBA color
    fn clear(&mut self, buffers: u32, r: f32, g: f32, b: f32, a: f32, depth: f64, stencil: u32) {
        // VIRGL_CCMD_CLEAR: header + buffers + color(4 floats) + depth(2 u32s) + stencil
        let depth_bits = depth.to_bits();
        let depth_lo = depth_bits as u32;
        let depth_hi = (depth_bits >> 32) as u32;

        self.cmds.push(cmd_header(virgl_cmd::CLEAR, 0, 8));
        self.cmds.push(buffers);
        self.cmds.push(r.to_bits());
        self.cmds.push(g.to_bits());
        self.cmds.push(b.to_bits());
        self.cmds.push(a.to_bits());
        self.cmds.push(depth_lo);
        self.cmds.push(depth_hi);
        self.cmds.push(stencil);
    }

    /// Set framebuffer state (minimal: just bind a surface)
    fn set_framebuffer_state(
        &mut self,
        width: u32,
        height: u32,
        nr_cbufs: u32,
        zsurf_handle: u32,
        cbuf_handles: &[u32],
    ) {
        // VIRGL_CCMD_SET_FRAMEBUFFER_STATE
        let len = 2 + nr_cbufs;
        self.cmds
            .push(cmd_header(virgl_cmd::SET_FRAMEBUFFER_STATE, 0, len));
        self.cmds.push(nr_cbufs);
        self.cmds.push(zsurf_handle);
        for handle in cbuf_handles {
            self.cmds.push(*handle);
        }
    }

    /// Create a surface object pointing to a resource
    fn create_surface(
        &mut self,
        handle: u32,
        res_handle: u32,
        format: u32,
        first_element: u32,
        last_element: u32,
    ) {
        // CREATE_OBJECT with type=SURFACE
        // Payload: res_handle, format, val0 (first_element | level<<16), val1 (last_element | first_layer<<16)
        self.cmds.push(cmd_header(
            virgl_cmd::CREATE_OBJECT,
            virgl_object::SURFACE,
            5,
        ));
        self.cmds.push(handle);
        self.cmds.push(res_handle);
        self.cmds.push(format);
        self.cmds.push(first_element); // first_element | (level << 16)
        self.cmds.push(last_element); // last_element | (first_layer << 16)
    }

    /// Get command bytes for SUBMIT_3D
    fn finish(&self) -> &[u32] {
        &self.cmds
    }

    /// Get command bytes as u8 slice
    fn as_bytes(&self) -> &[u8] {
        // Safe: u32 slice to u8 slice
        unsafe { core::slice::from_raw_parts(self.cmds.as_ptr() as *const u8, self.cmds.len() * 4) }
    }
}

// ============================================================================
// Demo Application
// ============================================================================

#[stem::main]
fn main() -> ! {
    info!("Virgl demo starting...");

    // Wait for VirtIO GPU device to appear
    let gpu_id = loop {
        use abi::schema::kinds;
        use stem::thing::sys::find;
        use stem::thing::ThingId;

        let mut devs = [ThingId::default(); 1];
        if let Ok(1) = find(kinds::DEV_DISPLAY_GPU, &mut devs) {
            break devs[0].to_u64_lossy();
        }
        stem::sleep(Duration::from_millis(100));
    };

    info!("virgl_demo: Found VirtIO GPU device: {}", gpu_id);

    // Initialize GPU
    let mut gpu = match VirtioGpu::new(gpu_id) {
        Ok(g) => g,
        Err(e) => {
            info!("virgl_demo: Failed to create GPU driver: {:?}", e);
            loop {
                stem::sleep(Duration::from_secs(1));
            }
        }
    };

    if let Err(e) = gpu.init_virtio() {
        info!("virgl_demo: Failed to init virtio: {}", e);
        loop {
            stem::sleep(Duration::from_secs(1));
        }
    }

    // Check for virgl support
    if !gpu.has_3d_feature() {
        info!("virgl_demo: No virgl support, falling back to 2D clear");
        demo_2d_clear(&mut gpu);
    } else {
        info!("virgl_demo: Virgl 3D supported! Running 3D demo...");
        demo_3d_clear(&mut gpu);
    }
}

/// 2D fallback - just clear the framebuffer with CPU composition
fn demo_2d_clear(gpu: &mut VirtioGpu) -> ! {
    info!("virgl_demo: 2D fallback - displaying solid color");

    // Use default display dimensions (driver initializes to 800x600 if not set)
    let (width, height) = gpu.get_dimensions();
    let (width, height) = if width == 0 {
        (1920, 1080)
    } else {
        (width, height)
    };
    info!("virgl_demo: Display size {}x{}", width, height);

    // Create a resource and fill with solid color
    let resource_id = 1;
    gpu.set_dimensions(width, height);

    if let Err(e) = gpu.create_resource_2d(resource_id) {
        info!("virgl_demo: Failed to create resource: {}", e);
        loop {
            stem::sleep(Duration::from_secs(1));
        }
    }

    // We would need backing memory to actually draw - for now just loop
    info!("virgl_demo: 2D resource created. (No backing memory impl yet)");
    loop {
        stem::sleep(Duration::from_secs(1));
    }
}

/// 3D demo - use virgl to clear to a solid color
fn demo_3d_clear(gpu: &mut VirtioGpu) -> ! {
    info!("virgl_demo: Creating virgl context...");

    // Create virgl context
    let ctx_id = 1;
    if let Err(e) = gpu.create_context(ctx_id, b"virgl_demo") {
        info!("virgl_demo: Failed to create context: {}", e);
        loop {
            stem::sleep(Duration::from_secs(1));
        }
    }
    info!("virgl_demo: Context created successfully!");

    // Use display dimensions (or reasonable default)
    let (width, height) = gpu.get_dimensions();
    let (width, height) = if width == 0 {
        (1920, 1080)
    } else {
        (width, height)
    };
    info!("virgl_demo: Display {}x{}", width, height);

    // Create a 3D render target resource
    let rt_resource_id = 100;
    let format = 2; // PIPE_FORMAT_B8G8R8X8_UNORM
    let bind =
        virtio_gpu::commands::PIPE_BIND_RENDER_TARGET | virtio_gpu::commands::PIPE_BIND_SCANOUT;

    if let Err(e) = gpu.create_resource_3d(
        rt_resource_id,
        virtio_gpu::commands::PIPE_TEXTURE_2D,
        format,
        bind,
        width,
        height,
        1, // depth
    ) {
        info!("virgl_demo: Failed to create 3D resource: {}", e);
        loop {
            stem::sleep(Duration::from_secs(1));
        }
    }
    info!("virgl_demo: 3D render target created!");

    // Attach resource to context
    if let Err(e) = gpu.ctx_attach_resource(ctx_id, rt_resource_id) {
        info!("virgl_demo: Failed to attach resource: {}", e);
        loop {
            stem::sleep(Duration::from_secs(1));
        }
    }
    info!("virgl_demo: Resource attached to context!");

    // Build virgl command stream to clear
    let mut frame = 0u32;
    loop {
        // Cycle through colors
        let r = ((frame % 256) as f32) / 255.0;
        let g = (((frame / 2) % 256) as f32) / 255.0;
        let b = (((frame / 4) % 256) as f32) / 255.0;

        let mut cmds = VirglCommandBuilder::new();

        // Create a surface for our render target (handle=1)
        cmds.create_surface(1, rt_resource_id, format, 0, 0);

        // Set framebuffer state to use our surface
        cmds.set_framebuffer_state(width, height, 1, 0, &[1]);

        // Clear to cycling color
        cmds.clear(clear_bits::COLOR, r, g, b, 1.0, 1.0, 0);

        // Log command details on first frame for diagnostics
        if frame == 0 {
            let cmd_words = cmds.finish();
            info!("virgl_demo: Submitting virgl command stream:");
            info!(
                "  - ctx_id={}, resource_id={}, format={}",
                ctx_id, rt_resource_id, format
            );
            info!(
                "  - command count={} words ({} bytes)",
                cmd_words.len(),
                cmds.as_bytes().len()
            );
            info!(
                "  - First 8 command words: {:08x?}",
                &cmd_words[..8.min(cmd_words.len())]
            );
        }

        // Submit command stream
        match gpu.submit_3d(ctx_id, cmds.as_bytes()) {
            Ok(_) => {
                if frame == 0 {
                    info!("virgl_demo: submit_3d succeeded (check host console for virgl errors)");
                }
            }
            Err(e) => {
                let cmd_words = cmds.finish();
                info!("virgl_demo: FAIL - submit_3d returned error: {}", e);
                info!("virgl_demo: Command buffer details:");
                info!(
                    "  - ctx_id={}, resource_id={}, format={}",
                    ctx_id, rt_resource_id, format
                );
                // Limit output to first 32 words to avoid spam
                let words_to_show = 32.min(cmd_words.len());
                info!(
                    "  - First {} command words: {:08x?}",
                    words_to_show,
                    &cmd_words[..words_to_show]
                );
                loop {
                    stem::sleep(Duration::from_secs(1));
                }
            }
        }

        // Set scanout to show result (resource_id, width, height)
        if let Err(e) = gpu.set_scanout(rt_resource_id, width, height) {
            if frame == 0 {
                info!("virgl_demo: set_scanout failed: {}", e);
            }
        }

        // Flush to display
        let rect = virtio_gpu::Rect {
            x: 0,
            y: 0,
            w: width,
            h: height,
        };
        if let Err(e) = gpu.flush_resource(rt_resource_id, rect) {
            if frame == 0 {
                info!("virgl_demo: flush failed: {}", e);
            }
        }

        if frame % 60 == 0 {
            info!(
                "virgl_demo: Frame {} - color ({:.2}, {:.2}, {:.2})",
                frame, r, g, b
            );
        }

        frame = frame.wrapping_add(1);
        stem::sleep(Duration::from_millis(16)); // ~60 FPS
    }
}
