use crate::machine::BootColor;
use crate::PreBootInfo;
#[derive(Clone, Copy, Debug)]
pub struct BootContext {
    pub hhdm_offset: u64,
    pub physical_memory: u64,
    pub cmdline: Option<&'static str>,
    pub framebuffer: Option<FramebufferInfo>,
    pub modules: &'static [ModuleInfo],
    pub early_putc: Option<fn(u8)>,
    pub kernel_phys_base: u64,
    pub kernel_virt_base: u64,
    pub heap_phys_base: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct FramebufferInfo {
    pub addr: u64,
    pub width: u64,
    pub height: u64,
    pub pitch: u64,
    pub bpp: u16,
    pub red_mask_size: u8,
    pub red_mask_shift: u8,
    pub green_mask_size: u8,
    pub green_mask_shift: u8,
    pub blue_mask_size: u8,
    pub blue_mask_shift: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct ModuleInfo {
    pub cmdline: &'static str,
    pub index: usize,
    pub path: &'static str,
    pub phys_addr: u64,
    pub size: u64,
}

static mut BOOT_CTX: Option<BootContext> = None;
const BOOT_COLOR_STEPS: usize = 6;
const BLOOM_WALLPAPER_DOMINANT: BootColor = BootColor {
    red: 0x2E,
    green: 0x80,
    blue: 0xD2,
};

pub fn get_boot_ctx() -> &'static BootContext {
    unsafe {
        #[allow(static_mut_refs)]
        BOOT_CTX.as_ref().expect("boot ctx not init")
    }
}

fn boot_progress_color(step: usize) -> BootColor {
    let max_step = BOOT_COLOR_STEPS.saturating_sub(1) as u32;
    if max_step == 0 {
        return BootColor {
            red: 0,
            green: 0,
            blue: 0,
        };
    }
    let clamped = step.min(BOOT_COLOR_STEPS - 1) as u32;
    let scale = |component: u8| -> u8 { ((component as u32 * clamped) / (max_step * 2)) as u8 };
    BootColor {
        red: scale(BLOOM_WALLPAPER_DOMINANT.red),
        green: scale(BLOOM_WALLPAPER_DOMINANT.green),
        blue: scale(BLOOM_WALLPAPER_DOMINANT.blue),
    }
}

#[cfg(feature = "boot-progress-text")]
mod boot_progress_text {
    use super::{get_boot_ctx, BootColor, FramebufferInfo};
    use alloc::collections::{BTreeMap, BTreeSet};
    use core::ptr;

    const UNIFONT_HEX: &[u8] =
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/fonts/unifont.hex"));
    const UNIFONT_GLYPH_HEIGHT: usize = 16;
    const UNIFONT_GLYPH_MAX_BYTES: usize = 32;
    const BOOT_GLYPH_SPACING: usize = 2;
    const BOOT_SPRITE_MAX_PIXELS: usize = 16384;

    #[derive(Clone, Copy)]
    struct GlyphBuffer {
        bytes: [u8; UNIFONT_GLYPH_MAX_BYTES],
        bytes_used: usize,
        bytes_per_row: usize,
    }

    impl GlyphBuffer {
        const fn new() -> Self {
            Self {
                bytes: [0; UNIFONT_GLYPH_MAX_BYTES],
                bytes_used: 0,
                bytes_per_row: 0,
            }
        }

        fn width(&self) -> usize {
            self.bytes_per_row * 8
        }
    }

    #[derive(Clone, Copy)]
    struct TextSprite<'a> {
        width: usize,
        height: usize,
        offset_x: usize,
        offset_y: usize,
        pixels: &'a [u32],
    }

    fn pack_color(fb: &FramebufferInfo, color: BootColor) -> Option<u32> {
        let pack = |component: u8, size: u8, shift: u8| -> Option<u32> {
            if size == 0 || size > 24 {
                return None;
            }
            let mask = (1u32 << size) - 1;
            let scaled = (component as u32 * mask + 127) / 255;
            Some(scaled << shift)
        };

        Some(
            pack(color.red, fb.red_mask_size, fb.red_mask_shift)?
                | pack(color.green, fb.green_mask_size, fb.green_mask_shift)?
                | pack(color.blue, fb.blue_mask_size, fb.blue_mask_shift)?,
        )
    }

    fn parse_hex_value(byte: u8) -> Option<u8> {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            b'A'..=b'F' => Some(byte - b'A' + 10),
            _ => None,
        }
    }

    fn parse_hex_pair(hi: u8, lo: u8) -> Option<u8> {
        Some(parse_hex_value(hi)? << 4 | parse_hex_value(lo)?)
    }

    fn parse_codepoint_hex(hex: &[u8]) -> Option<u32> {
        let mut value = 0u32;
        for &digit in hex {
            value = value.checked_mul(16)? + parse_hex_value(digit)? as u32;
        }
        Some(value)
    }

    fn parse_glyph_hex(glyph_hex: &[u8]) -> Option<GlyphBuffer> {
        if glyph_hex.len() % 2 != 0 {
            return None;
        }

        let glyph_bytes = glyph_hex.len() / 2;
        if glyph_bytes == 0
            || glyph_bytes > UNIFONT_GLYPH_MAX_BYTES
            || glyph_bytes % UNIFONT_GLYPH_HEIGHT != 0
        {
            return None;
        }

        let mut glyph = GlyphBuffer::new();
        for idx in 0..glyph_bytes {
            let byte = parse_hex_pair(glyph_hex[idx * 2], glyph_hex[idx * 2 + 1])?;
            glyph.bytes[idx] = byte;
        }
        glyph.bytes_used = glyph_bytes;
        glyph.bytes_per_row = glyph_bytes / UNIFONT_GLYPH_HEIGHT;
        Some(glyph)
    }

    fn load_glyphs(message: &str) -> BTreeMap<char, GlyphBuffer> {
        let mut targets: BTreeSet<char> = message.chars().collect();
        targets.insert('\u{FFFD}');
        targets.insert('?');
        if targets.is_empty() {
            return BTreeMap::new();
        }

        let mut glyphs = BTreeMap::new();
        let mut remaining = targets.len();
        let mut line_start = 0;

        while line_start < UNIFONT_HEX.len() && remaining > 0 {
            let mut line_end = line_start;
            while line_end < UNIFONT_HEX.len() && UNIFONT_HEX[line_end] != b'\n' {
                line_end += 1;
            }
            let line = &UNIFONT_HEX[line_start..line_end];
            line_start = line_end.saturating_add(1);

            let Some(colon) = line.iter().position(|&b| b == b':') else {
                continue;
            };

            let Some(codepoint) = parse_codepoint_hex(&line[..colon]) else {
                continue;
            };

            let Some(ch) = core::char::from_u32(codepoint) else {
                continue;
            };

            if !targets.contains(&ch) || glyphs.contains_key(&ch) {
                continue;
            }

            if let Some(glyph) = parse_glyph_hex(&line[colon + 1..]) {
                glyphs.insert(ch, glyph);
                remaining = remaining.saturating_sub(1);
            }
        }

        glyphs
    }

    fn choose_fg(bg: BootColor) -> BootColor {
        let brightness =
            (u32::from(bg.red) * 299 + u32::from(bg.green) * 587 + u32::from(bg.blue) * 114)
                / 1000;
        // Stay light-on-dark longer for readability during boot splash.
        if brightness > 220 {
            BootColor {
                red: 36,
                green: 48,
                blue: 64,
            }
        } else {
            BootColor {
                red: 245,
                green: 250,
                blue: 255,
            }
        }
    }

    fn draw_glyph_into(
        glyph: &GlyphBuffer,
        fg_pixel: u32,
        shadow_pixel: u32,
        buffer: &mut [u32],
        buf_width: usize,
        x: usize,
        y: usize,
    ) {
        if glyph.bytes_used == 0 {
            return;
        }

        let mut row_offset = 0;
        for row in 0..UNIFONT_GLYPH_HEIGHT {
            for byte_idx in 0..glyph.bytes_per_row {
                let byte = glyph.bytes[row_offset + byte_idx];
                if byte == 0 {
                    continue;
                }
                for bit in 0..8 {
                    if (byte & (0x80 >> bit)) != 0 {
                        let px = x + byte_idx * 8 + bit;
                        let py = y + row;
                        let idx_shadow = (py + 1) * buf_width + (px + 1);
                        let idx_fg = py * buf_width + px;
                        if idx_shadow < buffer.len() {
                            buffer[idx_shadow] = shadow_pixel;
                        }
                        if idx_fg < buffer.len() {
                            buffer[idx_fg] = fg_pixel;
                        }
                    }
                }
            }
            row_offset += glyph.bytes_per_row;
        }
    }

    fn pick_glyph<'a>(
        glyphs: &'a BTreeMap<char, GlyphBuffer>,
        ch: char,
    ) -> Option<&'a GlyphBuffer> {
        glyphs
            .get(&ch)
            .or_else(|| glyphs.get(&'\u{FFFD}'))
            .or_else(|| glyphs.get(&'?'))
    }

    fn prepare_text_sprite<'a>(
        fb: &FramebufferInfo,
        bg: BootColor,
        message: &str,
        scratch: &'a mut [u32],
    ) -> Option<TextSprite<'a>> {
        let glyphs = load_glyphs(message);
        if glyphs.is_empty() {
            return None;
        }

        let fg = choose_fg(bg);
        let fg_pixel = pack_color(fb, fg)?;
        let shadow_pixel = pack_color(
            fb,
            BootColor {
                red: 0,
                green: 0,
                blue: 0,
            },
        )?;

        let mut total_width = 0usize;
        let mut has_glyph = false;
        for ch in message.chars() {
            if let Some(glyph) = pick_glyph(&glyphs, ch) {
                if has_glyph {
                    total_width += BOOT_GLYPH_SPACING;
                }
                total_width += glyph.width();
                has_glyph = true;
            }
        }
        if !has_glyph {
            return None;
        }

        let sprite_w = total_width + 1;
        let sprite_h = UNIFONT_GLYPH_HEIGHT + 1;
        let needed = sprite_w * sprite_h;
        if needed > scratch.len() {
            return None;
        }
        let (pixels, _rest) = scratch.split_at_mut(needed);
        pixels.fill(0);

        let mut cursor_x = 0usize;
        let mut first_drawn = false;
        for ch in message.chars() {
            let Some(glyph) = pick_glyph(&glyphs, ch) else {
                continue;
            };
            if first_drawn {
                cursor_x += BOOT_GLYPH_SPACING;
            }
            draw_glyph_into(glyph, fg_pixel, shadow_pixel, pixels, sprite_w, cursor_x, 0);
            cursor_x += glyph.width();
            first_drawn = true;
        }

        let offset_x = (fb.width as usize).saturating_sub(sprite_w) / 2;
        let offset_y = (fb.height as usize).saturating_sub(sprite_h) / 2;

        Some(TextSprite {
            width: sprite_w,
            height: sprite_h,
            offset_x,
            offset_y,
            pixels,
        })
    }

    fn blit_text_sprite(fb: &FramebufferInfo, sprite: &TextSprite) {
        let bytes_per_pixel = (fb.bpp / 8) as usize;
        let width = fb.width as usize;
        let height = fb.height as usize;
        let pitch = fb.pitch as usize;
        if bytes_per_pixel < 4 || width == 0 || height == 0 || pitch < width * bytes_per_pixel {
            return;
        }

        let base = fb.addr + get_boot_ctx().hhdm_offset;
        for row in 0..sprite.height {
            let dst_y = sprite.offset_y + row;
            if dst_y >= height {
                continue;
            }
            for col in 0..sprite.width {
                let dst_x = sprite.offset_x + col;
                if dst_x >= width {
                    continue;
                }
                let src_pixel = sprite.pixels[row * sprite.width + col];
                if src_pixel == 0 {
                    continue;
                }
                let offset =
                    dst_y as u64 * pitch as u64 + dst_x as u64 * bytes_per_pixel as u64;
                let ptr = (base + offset) as *mut u32;
                unsafe {
                    ptr::write_unaligned(ptr, src_pixel);
                }
            }
        }
    }

    pub fn draw(fb: &FramebufferInfo, color: BootColor, message: &str) {
        let mut scratch = [0u32; BOOT_SPRITE_MAX_PIXELS];
        if let Some(sprite) = prepare_text_sprite(fb, color, message, &mut scratch) {
            blit_text_sprite(fb, &sprite);
        }
    }
}

fn indicate_progress(step: usize, message: &str) {
    if let Some(fb) = get_boot_ctx().framebuffer {
        let color = boot_progress_color(step);
        crate::machine::machine().set_boot_color(&fb, color);
        // #[cfg(feature = "boot-progress-text")]
        //boot_progress_text::draw(&fb, color, message);
    }
}

fn graph_irq_disable() -> usize {
    crate::machine::irq_disable() as usize
}

fn graph_irq_restore(flags: usize) {
    crate::machine::irq_restore(flags as u64)
}

pub fn pre_boot(info: PreBootInfo) {
    unsafe {
        crate::machine::install(crate::machine::ARCH_MACHINE);
    }
    crate::machine::machine().init(info);
}

pub unsafe fn boot(ctx_ptr: *mut BootContext) -> ! {
    let ctx = unsafe { &*ctx_ptr };
    BOOT_CTX = Some(*ctx);
    indicate_progress(0, "🌱 Booting ThingOS");
    let heap_config = crate::memory::heap::HeapConfig {
        phys_base: ctx.heap_phys_base,
        virt_base: ctx.heap_phys_base + ctx.hhdm_offset,
        size: 64 * 1024 * 1024,
    };
    crate::memory::heap::init(heap_config).expect("failed to init heap");
    indicate_progress(1, "🧰 Heap ready");
    crate::log::init(get_boot_ctx());
    crate::machine::input::init_mouse();

    // Enable SIMD for userspace (SSE/AVX)
    let simd = crate::machine::simd();
    if simd.enable() {
        crate::log::kprintln("BOOT: SIMD enabled");
    } else {
        crate::log::kprintln("BOOT: SIMD not available");
    }

    // Register IRQ hooks for graph store to prevent deadlocks

    crate::seeding::init_graph_system();
    indicate_progress(2, "🛰️ Graph seeded");
    crate::platform::init();
    indicate_progress(3, "⚙ Platform online");
    crate::seeding::seed_bloom_ontology();
    crate::seeding::seed_service_plan(ctx);
    indicate_progress(4, "🎨 Bloom scaffolded");
    crate::sched::init();

    // --- Launch Bloom ---

    // --- Launch Sprout ---
    // Sprout is PID 1 and spawned directly, not via the service plan
    // Register its boot grants manually
    {
        use abi::cap::{Cap, CapOp, CapScope};
        let sprout_caps = alloc::vec![
            Cap { op: CapOp::Log, scope: CapScope::Global },
            Cap { op: CapOp::MemManage, scope: CapScope::Global },
            Cap { op: CapOp::GrantCaps, scope: CapScope::Global },
            Cap { op: CapOp::GraphCreate, scope: CapScope::Global },
            Cap { op: CapOp::GraphLink, scope: CapScope::Global },
            Cap { op: CapOp::GraphUnlink, scope: CapScope::Global },
            Cap { op: CapOp::GraphRead, scope: CapScope::Global },
            Cap { op: CapOp::GraphWrite, scope: CapScope::Global },
        ];
        // Use a dummy ThingId since we register by name
        crate::boot_grants::register_module_grants(
            abi::ids::ThingId(0),
            Some(alloc::string::String::from("sprout")),
            sprout_caps
        );
    }
    let sprout_id = spawn_module_by_name(ctx, "sprout");
    indicate_progress(5, "🚀 Sprout ignited");
    crate::seeding::seed_kernel_permissions();

    crate::log::kprintln("BOOT: Handing off to scheduler");
    crate::sched::run();
}

pub fn spawn_module_by_name(ctx: &BootContext, name: &str) -> Option<abi::ids::ThingId> {
    for module in ctx.modules {
        if module.path.contains(name) {
            match crate::proc::spawn_kernel_module(module) {
                Ok(id) => return Some(id),
                Err(_) => {
                    crate::log::kprintln(&alloc::format!("BOOT: Failed to spawn module {}", name));
                    return None;
                }
            }
        }
    }
    crate::log::kprintln(&alloc::format!("BOOT: Module {} not found", name));
    None
}
