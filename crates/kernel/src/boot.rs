use crate::machine::BootColor;
use crate::memory::bytespace::Bytespace as MemBytespace;
use crate::PreBootInfo;
use abi::cap::CapOp;
use abi::bodies::{ThingEnvelopeV1, BYTESPACE_FLAG_HAS_PHYS_BASE};
use graph::store;
use graph::symbols::{self, sym};
use models::{
    Bytespace, DisplayDevice, Framebuffer, HardwareInfo, Module, MouseStream, Pointer, Service,
    Surface,
};

use alloc::vec::Vec;
#[cfg(target_arch = "x86_64")]
use models::EventStream;

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
    red: 140,
    green: 181,
    blue: 220,
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
    let scale = |component: u8| -> u8 { ((component as u32 * clamped) / max_step) as u8 };
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
        #[cfg(feature = "boot-progress-text")]
        boot_progress_text::draw(&fb, color, message);
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

    // Register IRQ hooks for graph store to prevent deadlocks
    graph::store::register_irq_callbacks(graph_irq_disable, graph_irq_restore);

    graph::init();
    graph::seed_minimal();
    indicate_progress(2, "🛰️ Graph seeded");
    crate::platform::init();
    indicate_progress(3, "⚙ Platform online");
    seed_bloom_ontology();
    seed_service_plan(ctx);
    indicate_progress(4, "🎨 Bloom scaffolded");
    crate::sched::init();
    let sprout_id = spawn_module_by_name(ctx, "sprout");
    indicate_progress(5, "🚀 Sprout ignited");
    seed_kernel_permissions();

    if let Some(id) = sprout_id {
        inject_root_caps(id);
    }

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

fn inject_root_caps(task_id: abi::ids::ThingId) {
    use abi::cap::{Cap, CapOp, CapScope};
    let ops = [
        CapOp::Log,
        CapOp::MemManage,
        CapOp::GrantCaps,
        CapOp::GraphCreate,
        CapOp::GraphLink,
        CapOp::GraphUnlink,
        CapOp::GraphRead,
        CapOp::GraphWrite,
        CapOp::GraphWatch,
        CapOp::Hardware,
    ];
    for op in ops {
        crate::syscall::cap::inject_cap(
            task_id,
            Cap {
                op,
                scope: CapScope::Global,
            },
        );
    }
}

fn seed_kernel_permissions() {
    // Ensure graph.permissions exists
    let perm_graph =
        if let Some(p) = store::find_thing_by_name(symbols::intern(b"graph.permissions")) {
            p
        } else {
            let p = store::thing_create(sym::KIND_GRAPH);
            store::thing_register_name(p, symbols::intern(b"graph.permissions"));
            // Link to root if possible
            if let Some(root) = store::find_thing_by_name(sym::GRAPH_ROOT) {
                store::relationship_create(sym::PRED_CONTAINS, root, p);
            }
            p
        };

    let permissions = [
        "perm.log",
        "perm.create",
        "perm.link",
        "perm.unlink",
        "perm.read",
        "perm.write",
        "perm.watch",
        "perm.mem",
        "perm.dictator",
    ];

    for perm_name in &permissions {
        let sym = symbols::intern(perm_name.as_bytes());
        if store::find_thing_by_name(sym).is_none() {
            let perm = store::thing_create(symbols::intern(b"kind.permission"));
            store::thing_register_name(perm, sym);
            store::relationship_create(sym::PRED_CONTAINS, perm_graph, perm);
        }
    }
}

fn seed_service_plan(ctx: &BootContext) {
    let root = store::find_thing_by_name(sym::GRAPH_ROOT);
    let ensure_graph =
        |name: abi::ids::SymbolId, parent: Option<abi::ids::ThingId>| -> abi::ids::ThingId {
            if let Some(existing) = store::find_thing_by_name(name) {
                existing
            } else {
                let g = store::thing_create(sym::KIND_GRAPH);
                store::thing_register_name(g, name);
                if let Some(p) = parent {
                    store::relationship_create(sym::PRED_CONTAINS, p, g);
                }
                g
            }
        };

    let services_root = ensure_graph(symbols::intern(b"graph.services"), root);
    let time_graph = ensure_graph(sym::GRAPH_SERVICES_TIME, Some(services_root));
    let clock_graph = ensure_graph(sym::GRAPH_APPS_CLOCK, Some(services_root));
    let core_graph = ensure_graph(symbols::intern(b"graph.services.core"), Some(services_root));

    let svc_kind = symbols::intern(b"kind.Service");
    let module_kind = symbols::intern(b"kind.Module");

    let mut create_module =
        |name: &str, parent: abi::ids::ThingId| -> Option<abi::ids::ThingId> {
            let module_sym = symbols::intern(alloc::format!("module.{}", name).as_bytes());
            if let Some(existing) = store::find_thing_by_name(module_sym) {
                return Some(existing);
            }

            let info = ctx.modules.iter().find(|m| m.path.contains(name))?;
            let bs = MemBytespace::new_module(info.phys_addr, info.size as usize);
            let module_thing = store::thing_create(module_kind);
            store::thing_register_name(module_thing, module_sym);

            let mut caps = [CapOp::Log; 8];
            let mut cap_count = 0u8;
            for op in default_caps_for(name).into_iter().flatten() {
                caps[cap_count as usize] = op;
                cap_count += 1;
            }

            let mut deps = [symbols::intern(b""); 8];
            let mut dep_count = 0u8;
            for dep in default_deps_for(name).into_iter().flatten() {
                deps[dep_count as usize] = dep;
                dep_count += 1;
            }

            let module_body = Module {
                name: symbols::intern(name.as_bytes()),
                bytespace: bs.id,
                size: info.size,
                caps,
                cap_count,
                deps,
                dep_count,
                _pad: 0,
            };
            let _ = store::thing_set_body(module_thing, &module_body.encode_full());
            store::relationship_create(sym::PRED_CONTAINS, parent, module_thing);
            Some(module_thing)
        };

    let mut create_service =
        |graph: abi::ids::ThingId, name: &str| -> Option<abi::ids::ThingId> {
            let sym_name = symbols::intern(alloc::format!("service.{}", name).as_bytes());
            if let Some(existing) = store::find_thing_by_name(sym_name) {
                return Some(existing);
            }

            let s = store::thing_create(svc_kind);
            store::thing_register_name(s, sym_name);
            let svc_body = Service {
                name: sym_name,
                pid: 0,
                state: 0,
            };
            let _ = store::thing_set_body(s, &svc_body.encode_full());
            store::relationship_create(sym::PRED_CONTAINS, graph, s);
            Some(s)
        };

    let mut wire_service =
        |graph: abi::ids::ThingId, name: &str| -> Option<(abi::ids::ThingId, Option<abi::ids::ThingId>)> {
            let svc = create_service(graph, name)?;
            let module = create_module(name, graph);
            if let Some(m) = module {
                let _ = store::relationship_create(sym::PRED_OWNS, svc, m);
            }
            Some((svc, module))
        };

    // Core services visible to Sprout
    let _ = wire_service(core_graph, "bloom");
    let _ = wire_service(core_graph, "inputd");
    let _ = wire_service(core_graph, "thingcheck");
    let _ = wire_service(core_graph, "hello_window");

    // Time pipeline: RTC driver -> timed -> clock app
    let rtc_dep = {
        #[cfg(target_arch = "x86_64")]
        {
            wire_service(time_graph, "rtc_cmos").map(|(svc, _)| svc)
        }
        #[cfg(target_arch = "aarch64")]
        {
            wire_service(time_graph, "rtc_pl031").map(|(svc, _)| svc)
        }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            None
        }
    };

    let timed = wire_service(time_graph, "timed").map(|(svc, _)| svc);
    if let (Some(timed_id), Some(rtc_id)) = (timed, rtc_dep) {
        let _ = store::relationship_create(sym::PRED_REFERENCES, timed_id, rtc_id);
    }

    let clock = wire_service(clock_graph, "clock").map(|(svc, _)| svc);
    if let (Some(clock_id), Some(timed_id)) = (clock, timed) {
        let _ = store::relationship_create(sym::PRED_REFERENCES, clock_id, timed_id);
    }

    crate::log::kprintln("BOOT: seeded service plan");
}

fn default_caps_for(name: &str) -> [Option<CapOp>; 8] {
    match name {
        "bloom" => [
            Some(CapOp::Log),
            Some(CapOp::MemManage),
            Some(CapOp::GraphCreate),
            Some(CapOp::GraphLink),
            Some(CapOp::GraphUnlink),
            Some(CapOp::GraphRead),
            Some(CapOp::GraphWrite),
            Some(CapOp::GraphWatch),
        ],
        "inputd" => [
            Some(CapOp::Log),
            Some(CapOp::MemManage),
            Some(CapOp::Hardware),
            Some(CapOp::GraphCreate),
            Some(CapOp::GraphLink),
            Some(CapOp::GraphRead),
            Some(CapOp::GraphWrite),
            None,
        ],
        "clock" => [Some(CapOp::Log), Some(CapOp::MemManage), Some(CapOp::GraphRead), None, None, None, None, None],
        "timed" => [
            Some(CapOp::Log),
            Some(CapOp::MemManage),
            Some(CapOp::GraphRead),
            Some(CapOp::GraphCreate),
            Some(CapOp::GraphLink),
            Some(CapOp::GraphWrite),
            None,
            None,
        ],
        "rtc_cmos" => [
            Some(CapOp::Log),
            Some(CapOp::MemManage),
            Some(CapOp::GraphRead),
            Some(CapOp::GraphCreate),
            Some(CapOp::GraphLink),
            Some(CapOp::GraphWrite),
            Some(CapOp::IoPort),
            None,
        ],
        "rtc_pl031" => [
            Some(CapOp::Log),
            Some(CapOp::MemManage),
            Some(CapOp::Hardware),
            Some(CapOp::GraphRead),
            Some(CapOp::GraphCreate),
            Some(CapOp::GraphLink),
            Some(CapOp::GraphWrite),
            None,
        ],
                "hello_window" => [
            Some(CapOp::Log),
            Some(CapOp::MemManage),
            Some(CapOp::GraphCreate),
            Some(CapOp::GraphLink),
            Some(CapOp::GraphRead),
            Some(CapOp::GraphWrite),
            None,
            None,
        ],
        _ => [Some(CapOp::Log), None, None, None, None, None, None, None],
    }
}

fn default_deps_for(name: &str) -> [Option<abi::ids::SymbolId>; 8] {
    match name {
        "timed" => [
            Some(symbols::intern(b"service.rtc_cmos")),
            Some(symbols::intern(b"service.rtc_pl031")),
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        "clock" => [
            Some(symbols::intern(b"service.timed")),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        _ => [None; 8],
    }
}

fn wrap_raw(kind: abi::ids::SymbolId, payload: &[u8]) -> Vec<u8> {
    let header = ThingEnvelopeV1 {
        magic: ThingEnvelopeV1::MAGIC,
        env_version: ThingEnvelopeV1::VERSION,
        flags: 0,
        kind: kind.0,
        schema_hash: 0,
        schema_version: 0,
        schema_str_len: 0,
        payload_format: 0,
        reserved0: 0,
        payload_len: payload.len() as u32,
        body_len: (core::mem::size_of::<ThingEnvelopeV1>() + payload.len()) as u32,
        integrity: 0,
    };
    let mut bytes = Vec::with_capacity(header.body_len as usize);
    unsafe {
        let ptr = &header as *const _ as *const u8;
        bytes.extend_from_slice(core::slice::from_raw_parts(
            ptr,
            core::mem::size_of::<ThingEnvelopeV1>(),
        ));
    }
    bytes.extend_from_slice(payload);
    bytes
}

fn seed_bloom_ontology() {
    let root = store::find_thing_by_name(sym::GRAPH_ROOT);

    let ensure_graph =
        |name: abi::ids::SymbolId, root: Option<abi::ids::ThingId>| -> abi::ids::ThingId {
            if let Some(existing) = store::find_thing_by_name(name) {
                existing
            } else {
                let p = store::thing_create(sym::KIND_GRAPH);
                store::thing_register_name(p, name);
                if let Some(parent) = root {
                    store::relationship_create(sym::PRED_CONTAINS, parent, p);
                }
                p
            }
        };

    let display_graph = ensure_graph(symbols::intern(b"graph.display"), root);
    let _windows_graph = ensure_graph(symbols::intern(b"graph.windows"), root);

    let input_graph = if let Some(p) = store::find_thing_by_name(sym::GRAPH_INPUT) {
        p
    } else {
        let p = store::thing_create(sym::KIND_GRAPH);
        store::thing_register_name(p, sym::GRAPH_INPUT);
        if let Some(root) = store::find_thing_by_name(sym::GRAPH_ROOT) {
            store::relationship_create(sym::PRED_CONTAINS, root, p);
        }
        p
    };

    let mut mouse_stream_id: Option<abi::ids::ThingId> = None;
    #[cfg(target_arch = "x86_64")]
    {
        use crate::machine::x86_64::ps2_mouse;
        let mouse_bs = store::thing_create(sym::KIND_BYTE_SPACE);
        let mouse_bs_name = symbols::intern(b"bytespace.mouse_input");
        store::thing_register_name(mouse_bs, mouse_bs_name);
        store::relationship_create(sym::PRED_CONTAINS, input_graph, mouse_bs);

        let phys_addr = ps2_mouse::get_ring_phys_addr();
        let ring_size = ps2_mouse::get_ring_size() as u64;

        let bs_payload = Bytespace {
            len: ring_size,
            flags: BYTESPACE_FLAG_HAS_PHYS_BASE,
            _pad: 0,
            phys_base: phys_addr,
        };
        let _ = store::thing_set_body(mouse_bs, &bs_payload.encode_full());

        let phys_thing = store::thing_create(sym::KIND_GRAPH);
        let phys_payload = phys_addr.to_le_bytes();
        let _ = store::thing_set_body(phys_thing, &wrap_raw(sym::KIND_GRAPH, &phys_payload));
        store::relationship_create(sym::PRED_BASE_PHYS, mouse_bs, phys_thing);

        let size_thing = store::thing_create(sym::KIND_GRAPH);
        let size_payload = ring_size.to_le_bytes();
        let _ = store::thing_set_body(size_thing, &wrap_raw(sym::KIND_GRAPH, &size_payload));
        store::relationship_create(sym::PRED_SIZE, mouse_bs, size_thing);

        let mouse_stream = store::thing_create(symbols::intern(b"kind.MouseStream"));
        store::thing_register_name(mouse_stream, symbols::intern(b"mouse.stream.0"));
        let ms_payload = MouseStream {
            bytespace: mouse_bs,
            capacity: ps2_mouse::RING_CAPACITY,
            sample_size: core::mem::size_of::<abi::mouse_ring::MouseSample>() as u32,
            write_index: 0,
            dropped: 0,
        };
        let _ = store::thing_set_body(mouse_stream, &ms_payload.encode_full());
        store::relationship_create(sym::PRED_CONTAINS, input_graph, mouse_stream);
        store::relationship_create(sym::PRED_REFERENCES, mouse_stream, mouse_bs);

        let event_stream = store::thing_create(symbols::intern(b"kind.EventStream"));
        store::thing_register_name(event_stream, symbols::intern(b"event_stream.mouse"));
        let es_payload = EventStream {
            bytespace: mouse_bs,
            capacity_bytes: ps2_mouse::RING_CAPACITY,
            max_record_bytes: 64,
            flags: 0,
            name: symbols::intern(b"mouse"),
        };
        let _ = store::thing_set_body(event_stream, &es_payload.encode_full());
        store::relationship_create(sym::PRED_CONTAINS, input_graph, event_stream);
        store::relationship_create(symbols::intern(b"stream.bytespace"), event_stream, mouse_bs);
        mouse_stream_id = Some(mouse_stream);
    }

    if mouse_stream_id.is_none() {
        let mouse_stream = store::thing_create(symbols::intern(b"kind.MouseStream"));
        store::thing_register_name(mouse_stream, symbols::intern(b"mouse.stream.0"));
        let ms_payload = MouseStream {
            bytespace: abi::ids::ThingId(0),
            capacity: 0,
            sample_size: 0,
            write_index: 0,
            dropped: 0,
        };
        let _ = store::thing_set_body(mouse_stream, &ms_payload.encode_full());
        store::relationship_create(sym::PRED_CONTAINS, input_graph, mouse_stream);
        mouse_stream_id = Some(mouse_stream);
    }

    let pointer_thing = store::thing_create(sym::KIND_POINTER);
    store::thing_register_name(pointer_thing, symbols::intern(b"pointer.0"));
    store::thing_register_name(pointer_thing, symbols::intern(b"pointer.state"));
    store::relationship_create(sym::REL_HAS_POINTER, input_graph, pointer_thing);
    store::relationship_create(sym::PRED_CONTAINS, input_graph, pointer_thing);
    let pointer_payload = Pointer {
        stream: mouse_stream_id.unwrap_or(abi::ids::ThingId(0)),
        x: 0,
        y: 0,
        buttons: 0,
        updated_at_ns: 0,
    };
    let _ = store::thing_set_body(pointer_thing, &pointer_payload.encode_full());

    let ctx = get_boot_ctx();
    let asset_root = store::thing_create(sym::KIND_GRAPH);
    store::thing_register_name(asset_root, sym::GRAPH_ASSETS);
    if let Some(root) = store::find_thing_by_name(sym::GRAPH_ROOT) {
        store::relationship_create(sym::PRED_CONTAINS, root, asset_root);
    }

    for m in ctx.modules {
        if m.path.is_empty() {
            continue;
        }
        if m.path.contains("/assets/") {
            let filename = m.path.rsplit('/').next().unwrap_or(m.path);
            let thing_name_str = alloc::format!("asset.{}", filename);
            let bs_name_str = alloc::format!("bytespace.asset.{}", filename);
            let asset_thing = store::thing_create(sym::KIND_ASSET);
            store::thing_register_name(asset_thing, symbols::intern(thing_name_str.as_bytes()));
            let bs = store::thing_create(sym::KIND_BYTESPACE_MODULE);
            store::thing_register_name(bs, symbols::intern(bs_name_str.as_bytes()));
            store::relationship_create(sym::PRED_BACKS, asset_thing, bs);
            store::relationship_create(sym::PRED_CONTAINS, asset_root, asset_thing);

            let phys_thing = store::thing_create(sym::KIND_GRAPH);
            let phys_payload = m.phys_addr.to_le_bytes();
            let _ = store::thing_set_body(phys_thing, &wrap_raw(sym::KIND_GRAPH, &phys_payload));
            store::relationship_create(sym::PRED_BASE_PHYS, bs, phys_thing);

            let size_thing = store::thing_create(sym::KIND_GRAPH);
            let size_payload = m.size.to_le_bytes();
            let _ = store::thing_set_body(size_thing, &wrap_raw(sym::KIND_GRAPH, &size_payload));
            store::relationship_create(sym::PRED_SIZE, bs, size_thing);
        }
    }

    if let Some(fb) = ctx.framebuffer {
        let fb_thing = store::thing_create(sym::KIND_DEVICE_DISPLAY);
        store::thing_register_name(fb_thing, symbols::intern(b"device.display0"));
        store::thing_register_name(fb_thing, symbols::intern(b"display.0"));
        store::relationship_create(sym::PRED_CONTAINS, display_graph, fb_thing);

        let fb_bytespace = store::thing_create(sym::KIND_BYTESPACE_FRAMEBUFFER);
        store::thing_register_name(fb_bytespace, symbols::intern(b"bytespace.display0"));
        store::relationship_create(sym::PRED_CONTAINS, display_graph, fb_bytespace);

        let surface = store::thing_create(sym::KIND_SURFACE);
        store::thing_register_name(surface, symbols::intern(b"surface.display0"));
        store::relationship_create(sym::PRED_PRIMARY, fb_thing, surface);
        store::relationship_create(sym::PRED_BACKS, surface, fb_bytespace);
        store::relationship_create(sym::PRED_CONTAINS, display_graph, surface);

        let framebuffer_thing = store::thing_create(symbols::intern(b"kind.Framebuffer"));
        store::thing_register_name(framebuffer_thing, symbols::intern(b"framebuffer.display0"));
        store::relationship_create(sym::PRED_REFERENCES, framebuffer_thing, fb_bytespace);
        store::relationship_create(sym::PRED_CONTAINS, display_graph, framebuffer_thing);

        let fb_phys_thing = store::thing_create(sym::KIND_GRAPH);
        let fb_phys_payload = fb.addr.to_le_bytes();
        let _ = store::thing_set_body(fb_phys_thing, &wrap_raw(sym::KIND_GRAPH, &fb_phys_payload));
        store::relationship_create(sym::PRED_BASE_PHYS, fb_bytespace, fb_phys_thing);

        let fb_size = fb.height * fb.pitch;
        let fb_size_thing = store::thing_create(sym::KIND_GRAPH);
        let fb_size_payload = fb_size.to_le_bytes();
        let _ = store::thing_set_body(fb_size_thing, &wrap_raw(sym::KIND_GRAPH, &fb_size_payload));
        store::relationship_create(sym::PRED_SIZE, fb_bytespace, fb_size_thing);

        let fb_bytespace_payload = Bytespace {
            len: fb_size,
            flags: BYTESPACE_FLAG_HAS_PHYS_BASE,
            _pad: 0,
            phys_base: fb.addr,
        };
        let _ = store::thing_set_body(fb_bytespace, &fb_bytespace_payload.encode_full());

        let framebuffer_payload = Framebuffer {
            bytespace: fb_bytespace,
            width: fb.width as u32,
            height: fb.height as u32,
            stride_bytes: fb.pitch as u32,
            format: symbols::intern(b"format.bgra8888"),
        };
        let _ = store::thing_set_body(framebuffer_thing, &framebuffer_payload.encode_full());

        let surface_payload = Surface {
            width: fb.width as u32,
            height: fb.height as u32,
            stride_bytes: fb.pitch as u32,
            format: symbols::intern(b"format.bgra8888"),
            bytespace: fb_bytespace,
        };
        let _ = store::thing_set_body(surface, &surface_payload.encode_full());

        let display_payload = DisplayDevice {
            framebuffer: framebuffer_thing,
            width: fb.width as u32,
            height: fb.height as u32,
            stride_bytes: fb.pitch as u32,
            format: symbols::intern(b"format.bgra8888"),
            refresh_hz: 60,
        };
        let _ = store::thing_set_body(fb_thing, &display_payload.encode_full());

        crate::machine::input::set_mouse_bounds(fb.width as u32, fb.height as u32);
        if let Some(devices) = store::find_thing_by_name(sym::GRAPH_DEVICES) {
            store::relationship_create(sym::PRED_CONTAINS, devices, fb_thing);
        }
    }

    #[cfg(target_arch = "x86_64")]
    {
        let rtc_hw = store::thing_create(symbols::intern(b"kind.HardwareResource"));
        store::thing_register_name(rtc_hw, symbols::intern(b"hw.rtc0"));
        let rtc_info = HardwareInfo {
            name: symbols::intern(b"cmos-rtc"),
            resource_type: symbols::intern(b"ioport"),
            start: 0x70,
            end: 0x71,
            irq: 8,
            _pad: 0,
        };
        let _ = store::thing_set_body(rtc_hw, &rtc_info.encode_full());
        if let Some(devices) = store::find_thing_by_name(sym::GRAPH_DEVICES) {
            store::relationship_create(sym::PRED_CONTAINS, devices, rtc_hw);
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        let rtc_hw = store::thing_create(symbols::intern(b"kind.HardwareResource"));
        store::thing_register_name(rtc_hw, symbols::intern(b"hw.rtc0"));
        let rtc_info = HardwareInfo {
            name: symbols::intern(b"pl031"),
            resource_type: symbols::intern(b"mmio"),
            start: 0x09010000,
            end: 0x09011000,
            irq: 34,
            _pad: 0,
        };
        let _ = store::thing_set_body(rtc_hw, &rtc_info.encode_full());
        if let Some(devices) = store::find_thing_by_name(sym::GRAPH_DEVICES) {
            store::relationship_create(sym::PRED_CONTAINS, devices, rtc_hw);
        }
    }
}
