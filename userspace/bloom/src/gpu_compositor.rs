//! GPU Compositor using Virgl 3D
//!
//! This module provides GPU-accelerated composition using virgl (VirtIO GPU 3D).
//! It composites pre-rasterized window content as textured quads.
//!
//! # Architecture
//!
//! - Window content is rasterized on CPU (same as before)
//! - Raster images are uploaded as GPU textures
//! - Composition happens as textured quad draws on GPU
//! - Result is presented to scanout

#![allow(dead_code)]

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use stem::info;

// ============================================================================
// Geometry Types
// ============================================================================

/// Rectangle for positioning and clipping
#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }
}

/// A quad to be composited on the GPU
#[derive(Debug, Clone)]
pub struct Quad {
    /// Texture resource ID (from GpuTexture)
    pub texture_id: u32,
    /// Destination rectangle in screen coordinates
    pub dst_rect: Rect,
    /// Source rectangle in texture coordinates (None = full texture)
    pub src_rect: Option<Rect>,
    /// Opacity (0.0 = transparent, 1.0 = opaque)
    pub opacity: f32,
    /// Z-order (higher = on top)
    pub z: u32,
}

// ============================================================================
// Texture Management
// ============================================================================

/// Tracks a GPU texture resource
#[derive(Debug)]
pub struct GpuTexture {
    /// Virgl resource ID
    pub resource_id: u32,
    /// Texture dimensions
    pub width: u32,
    pub height: u32,
    /// Generation counter for cache invalidation
    pub generation: u64,
    /// PixelBuffer handle for framebuffer binding
    pub surface_handle: u32,
}

// ============================================================================
// Virgl Command Protocol
// ============================================================================

/// Virgl command opcodes (from virgl_protocol.h)
pub mod virgl_cmd {
    pub const NOP: u32 = 0;
    pub const CREATE_OBJECT: u32 = 1;
    pub const BIND_OBJECT: u32 = 2;
    pub const DESTROY_OBJECT: u32 = 3;
    pub const SET_VIEWPORT_STATE: u32 = 4;
    pub const SET_FRAMEBUFFER_STATE: u32 = 5;
    pub const CLEAR: u32 = 7;
    pub const DRAW_VBO: u32 = 8;
    pub const RESOURCE_INLINE_WRITE: u32 = 9;
    pub const SET_VERTEX_BUFFERS: u32 = 6;
    pub const SET_INDEX_BUFFER: u32 = 11;
    pub const SET_CONSTANT_BUFFER: u32 = 12;
    pub const SET_SCISSOR_STATE: u32 = 15;
    pub const BLIT: u32 = 16;
}

/// Object types for CREATE_OBJECT
pub mod virgl_object {
    pub const BLEND: u32 = 1;
    pub const RASTERIZER: u32 = 2;
    pub const DSA: u32 = 3; // Depth-stencil-alpha
    pub const SHADER: u32 = 4;
    pub const VERTEX_ELEMENTS: u32 = 5;
    pub const SAMPLER_VIEW: u32 = 6;
    pub const SAMPLER_STATE: u32 = 7;
    pub const SURFACE: u32 = 8;
}

/// CLEAR buffer bits
pub mod clear_bits {
    pub const DEPTH: u32 = 1 << 0;
    pub const STENCIL: u32 = 1 << 1;
    pub const COLOR: u32 = 1 << 2;
}

/// Build virgl command header word
fn cmd_header(cmd: u32, object_type: u32, payload_len: u32) -> u32 {
    (cmd & 0xff) | ((object_type & 0xff) << 8) | ((payload_len & 0xffff) << 16)
}

/// Virgl command stream builder
pub struct VirglCommandBuilder {
    cmds: Vec<u32>,
}

impl VirglCommandBuilder {
    pub fn new() -> Self {
        Self { cmds: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.cmds.clear();
    }

    /// Get command bytes for SUBMIT_3D
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.cmds.as_ptr() as *const u8, self.cmds.len() * 4) }
    }

    /// Clear color buffer to RGBA color
    pub fn cmd_clear(
        &mut self,
        buffers: u32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
        depth: f64,
        stencil: u32,
    ) {
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

    /// Set framebuffer state
    pub fn cmd_set_framebuffer(&mut self, nr_cbufs: u32, zsurf_handle: u32, cbuf_handles: &[u32]) {
        let len = 2 + nr_cbufs;
        self.cmds
            .push(cmd_header(virgl_cmd::SET_FRAMEBUFFER_STATE, 0, len));
        self.cmds.push(nr_cbufs);
        self.cmds.push(zsurf_handle);
        for handle in cbuf_handles {
            self.cmds.push(*handle);
        }
    }

    /// Create a surface object
    pub fn cmd_create_surface(
        &mut self,
        handle: u32,
        res_handle: u32,
        format: u32,
        first_element: u32,
        last_element: u32,
    ) {
        self.cmds.push(cmd_header(
            virgl_cmd::CREATE_OBJECT,
            virgl_object::SURFACE,
            5,
        ));
        self.cmds.push(handle);
        self.cmds.push(res_handle);
        self.cmds.push(format);
        self.cmds.push(first_element);
        self.cmds.push(last_element);
    }

    /// Create a sampler view object for texture sampling
    pub fn cmd_create_sampler_view(
        &mut self,
        handle: u32,
        res_handle: u32,
        format: u32,
        first_element: u32,
        last_element: u32,
        swizzle: u32,
    ) {
        self.cmds.push(cmd_header(
            virgl_cmd::CREATE_OBJECT,
            virgl_object::SAMPLER_VIEW,
            6,
        ));
        self.cmds.push(handle);
        self.cmds.push(res_handle);
        self.cmds.push(format);
        self.cmds.push(first_element);
        self.cmds.push(last_element);
        self.cmds.push(swizzle); // RGBA identity = 0x03020100
    }

    /// Destroy an object
    pub fn cmd_destroy_object(&mut self, handle: u32, object_type: u32) {
        self.cmds
            .push(cmd_header(virgl_cmd::DESTROY_OBJECT, object_type, 1));
        self.cmds.push(handle);
    }

    /// Set viewport state  
    pub fn cmd_set_viewport(&mut self, start_slot: u32, scale: [f32; 3], translate: [f32; 3]) {
        self.cmds
            .push(cmd_header(virgl_cmd::SET_VIEWPORT_STATE, 0, 7));
        self.cmds.push(start_slot);
        self.cmds.push(scale[0].to_bits());
        self.cmds.push(scale[1].to_bits());
        self.cmds.push(scale[2].to_bits());
        self.cmds.push(translate[0].to_bits());
        self.cmds.push(translate[1].to_bits());
        self.cmds.push(translate[2].to_bits());
    }

    /// Set scissor state
    pub fn cmd_set_scissor(&mut self, start_slot: u32, minx: u16, miny: u16, maxx: u16, maxy: u16) {
        self.cmds
            .push(cmd_header(virgl_cmd::SET_SCISSOR_STATE, 0, 3));
        self.cmds.push(start_slot);
        self.cmds.push((minx as u32) | ((miny as u32) << 16));
        self.cmds.push((maxx as u32) | ((maxy as u32) << 16));
    }

    /// Blit (copy) a region from source texture to destination
    /// This uses virgl's pipe_blit_info structure which has many fields
    pub fn cmd_blit(
        &mut self,
        dst_res: u32,
        dst_level: u32,
        dst_format: u32,
        dst_x: i32,
        dst_y: i32,
        dst_z: i32,
        dst_w: u32,
        dst_h: u32,
        dst_d: u32,
        src_res: u32,
        src_level: u32,
        src_format: u32,
        src_x: i32,
        src_y: i32,
        src_z: i32,
        src_w: u32,
        src_h: u32,
        src_d: u32,
        mask: u32,   // PIPE_MASK_RGBA = 0xf for color
        filter: u32, // 0 = NEAREST, 1 = LINEAR
    ) {
        // virgl BLIT has a complex structure - 21 dwords
        // Based on virgl_protocol.h pipe_blit_info encoding
        self.cmds.push(cmd_header(virgl_cmd::BLIT, 0, 21));

        // s0: blend mask, filter, scissor_enable, render_condition_enable
        self.cmds.push(mask | (filter << 4)); // No scissor or render condition

        // Destination box
        self.cmds.push(dst_x as u32); // dst.box.x
        self.cmds.push(dst_y as u32); // dst.box.y
        self.cmds.push(dst_z as u32); // dst.box.z
        self.cmds.push(dst_w); // dst.box.width
        self.cmds.push(dst_h); // dst.box.height
        self.cmds.push(dst_d); // dst.box.depth

        // Source box
        self.cmds.push(src_x as u32); // src.box.x
        self.cmds.push(src_y as u32); // src.box.y
        self.cmds.push(src_z as u32); // src.box.z
        self.cmds.push(src_w); // src.box.width
        self.cmds.push(src_h); // src.box.height
        self.cmds.push(src_d); // src.box.depth

        // Destination resource info
        self.cmds.push(dst_res); // dst.resource
        self.cmds.push(dst_level); // dst.level
        self.cmds.push(dst_format); // dst.format

        // Source resource info
        self.cmds.push(src_res); // src.resource
        self.cmds.push(src_level); // src.level
        self.cmds.push(src_format); // src.format

        // Sample0 mask (for MSAA, 0 = use all samples)
        self.cmds.push(0);
    }
}

impl Default for VirglCommandBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// GPU Compositor
// ============================================================================

/// GPU-accelerated compositor using virgl
pub struct GpuCompositor {
    /// Virgl context ID
    ctx_id: u32,
    /// Render target resource ID
    scanout_resource: u32,
    /// Render target surface handle
    scanout_surface: u32,
    /// Display dimensions
    width: u32,
    height: u32,
    /// Pixel format (PIPE_FORMAT_B8G8R8X8_UNORM = 2)
    format: u32,
    /// Textures by window ID
    textures: BTreeMap<u64, GpuTexture>,
    /// Next texture handle to allocate
    next_texture_handle: u32,
    /// Command builder (reused to avoid allocations)
    cmd_builder: VirglCommandBuilder,
    /// Is the compositor initialized?
    initialized: bool,
}

impl GpuCompositor {
    /// Create a new GPU compositor (call init() after to set up virgl context)
    pub fn new() -> Self {
        Self {
            ctx_id: 1, // Use context ID 1 for compositor
            scanout_resource: 0,
            scanout_surface: 1, // PixelBuffer handle 1 for scanout
            width: 0,
            height: 0,
            format: 2, // PIPE_FORMAT_B8G8R8X8_UNORM
            textures: BTreeMap::new(),
            next_texture_handle: 100, // Start texture handles at 100
            cmd_builder: VirglCommandBuilder::new(),
            initialized: false,
        }
    }

    /// Check if compositor is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Get context ID for submit_3d calls
    pub fn context_id(&self) -> u32 {
        self.ctx_id
    }

    /// Get scanout resource ID for set_scanout calls  
    pub fn scanout_resource_id(&self) -> u32 {
        self.scanout_resource
    }

    /// Get display dimensions
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Set the scanout resource (created externally by VirtioGpu)
    pub fn set_scanout_resource(&mut self, resource_id: u32, width: u32, height: u32) {
        self.scanout_resource = resource_id;
        self.width = width;
        self.height = height;
        info!(
            "GpuCompositor: scanout resource {} ({}x{})",
            resource_id, width, height
        );
    }

    /// Mark as initialized
    pub fn mark_initialized(&mut self) {
        self.initialized = true;
    }

    /// Build command stream to set up the framebuffer
    pub fn build_setup_commands(&mut self) -> &[u8] {
        self.cmd_builder.clear();

        // Create surface for render target
        self.cmd_builder.cmd_create_surface(
            self.scanout_surface,
            self.scanout_resource,
            self.format,
            0, // first_element
            0, // last_element
        );

        // Set framebuffer to use our surface
        self.cmd_builder
            .cmd_set_framebuffer(1, 0, &[self.scanout_surface]);

        // Set viewport for full screen
        let w = self.width as f32;
        let h = self.height as f32;
        self.cmd_builder.cmd_set_viewport(
            0,
            [w / 2.0, -h / 2.0, 0.5], // scale
            [w / 2.0, h / 2.0, 0.5],  // translate
        );

        self.cmd_builder.as_bytes()
    }

    /// Build command stream to clear the screen
    pub fn build_clear_commands(&mut self, r: f32, g: f32, b: f32, a: f32) -> &[u8] {
        self.cmd_builder.clear();
        self.cmd_builder
            .cmd_clear(clear_bits::COLOR, r, g, b, a, 1.0, 0);
        self.cmd_builder.as_bytes()
    }

    /// Allocate a new texture handle
    pub fn alloc_texture_handle(&mut self) -> u32 {
        let handle = self.next_texture_handle;
        self.next_texture_handle += 1;
        handle
    }

    /// Register a texture (after external creation via VirtioGpu)
    pub fn register_texture(
        &mut self,
        window_id: u64,
        resource_id: u32,
        width: u32,
        height: u32,
        generation: u64,
    ) {
        let surface_handle = self.alloc_texture_handle();
        self.textures.insert(
            window_id,
            GpuTexture {
                resource_id,
                width,
                height,
                generation,
                surface_handle,
            },
        );
    }

    /// Check if texture needs update  
    pub fn texture_needs_update(&self, window_id: u64, generation: u64) -> bool {
        match self.textures.get(&window_id) {
            Some(tex) => tex.generation < generation,
            None => true,
        }
    }

    /// Get texture info
    pub fn get_texture(&self, window_id: u64) -> Option<&GpuTexture> {
        self.textures.get(&window_id)
    }

    /// Remove texture
    pub fn remove_texture(&mut self, window_id: u64) -> Option<GpuTexture> {
        self.textures.remove(&window_id)
    }

    /// Render a frame with quads using BLIT commands
    ///
    /// This composites each quad by blitting the source texture to the render target.
    /// Quads are sorted by z-order (painter's algorithm - lowest z first).
    pub fn render_quads(&mut self, quads: &[Quad]) -> &[u8] {
        self.cmd_builder.clear();

        // Sort quads by z-order (lowest first = drawn first = behind)
        let mut sorted_quads: Vec<&Quad> = quads.iter().collect();
        sorted_quads.sort_by_key(|q| q.z);

        // BLIT each quad's texture to the render target
        for quad in sorted_quads {
            // Look up texture to get resource ID
            if let Some(tex) = self
                .textures
                .values()
                .find(|t| t.resource_id == quad.texture_id)
            {
                let src_rect = quad
                    .src_rect
                    .unwrap_or(Rect::new(0, 0, tex.width, tex.height));

                self.cmd_builder.cmd_blit(
                    // Destination (render target)
                    self.scanout_resource,
                    0, // level
                    self.format,
                    quad.dst_rect.x,
                    quad.dst_rect.y,
                    0, // z
                    quad.dst_rect.w,
                    quad.dst_rect.h,
                    1, // depth
                    // Source (texture)
                    tex.resource_id,
                    0, // level
                    self.format,
                    src_rect.x,
                    src_rect.y,
                    0, // z
                    src_rect.w,
                    src_rect.h,
                    1,   // depth
                    0xf, // PIPE_MASK_RGBA
                    0,   // NEAREST filter
                );
            }
        }

        self.cmd_builder.as_bytes()
    }

    /// Build commands to blit a single texture to the framebuffer (for testing)
    pub fn build_blit_texture(
        &mut self,
        tex_resource: u32,
        tex_width: u32,
        tex_height: u32,
        dst_x: i32,
        dst_y: i32,
    ) -> &[u8] {
        self.cmd_builder.clear();

        self.cmd_builder.cmd_blit(
            self.scanout_resource,
            0,
            self.format,
            dst_x,
            dst_y,
            0,
            tex_width,
            tex_height,
            1,
            tex_resource,
            0,
            self.format,
            0,
            0,
            0,
            tex_width,
            tex_height,
            1,
            0xf,
            0,
        );

        self.cmd_builder.as_bytes()
    }
}

impl Default for GpuCompositor {
    fn default() -> Self {
        Self::new()
    }
}
