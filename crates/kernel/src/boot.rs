use crate::machine::BootColor;
use crate::PreBootInfo;
use abi::bodies::BYTESPACE_FLAG_HAS_PHYS_BASE;
use graph::store;
use graph::symbols::{self, sym};
use models::{
    BytespaceBody, DisplayDeviceBody, FramebufferBody, EventStreamBody, MouseStreamBody, PointerStateBody,
    SurfaceBody, Thing,
};

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
// Dominant tone sampled from assets/wallpapers/clouds.bmp (approximate average color).
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
    let scale = |component: u8| -> u8 {
        ((component as u32 * clamped) / max_step) as u8
    };

    BootColor {
        red: scale(BLOOM_WALLPAPER_DOMINANT.red),
        green: scale(BLOOM_WALLPAPER_DOMINANT.green),
        blue: scale(BLOOM_WALLPAPER_DOMINANT.blue),
    }
}

fn indicate_progress(step: usize) {
    if let Some(fb) = get_boot_ctx().framebuffer {
        crate::machine::machine().set_boot_color(&fb, boot_progress_color(step));
    }
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

    indicate_progress(0);

    let heap_config = crate::memory::heap::HeapConfig {
        phys_base: ctx.heap_phys_base,
        virt_base: ctx.heap_phys_base + ctx.hhdm_offset,
        size: 64 * 1024 * 1024,
    };
    crate::memory::heap::init(heap_config).expect("failed to init heap");

    indicate_progress(1);

    crate::log::init(get_boot_ctx());

    // Initialize PS/2 mouse (requires heap)
    crate::machine::input::init_mouse();

    graph::init();
    graph::seed_minimal();

    indicate_progress(2);

    // Platform layer handles arch-specific wiring and graph seeding
    crate::platform::init();

    indicate_progress(3);

    seed_bloom_ontology();

    indicate_progress(4);

    crate::sched::init();

    spawn_module_by_name(ctx, "sprout");

    indicate_progress(5);

    crate::log::kprintln("BOOT: Handing off to scheduler");
    crate::sched::run();
}

pub fn spawn_module_by_name(ctx: &BootContext, name: &str) {
    for module in ctx.modules {
        if module.path.contains(name) {
            if let Err(_) = crate::proc::spawn_kernel_module(module) {
                crate::log::kprintln(&alloc::format!("BOOT: Failed to spawn module {}", name));
            }
            return;
        }
    }
    crate::log::kprintln(&alloc::format!("BOOT: Module {} not found", name));
}

fn seed_bloom_ontology() {
    let root = store::find_thing_by_name(sym::PLACE_ROOT);

    let ensure_place =
        |name: abi::ids::SymbolId, root: Option<abi::ids::ThingId>| -> abi::ids::ThingId {
            if let Some(existing) = store::find_thing_by_name(name) {
                existing
            } else {
                let p = store::thing_create(sym::KIND_PLACE);
                store::thing_register_name(p, name);
                if let Some(parent) = root {
                    store::relationship_create(sym::PRED_CONTAINS, parent, p);
                }
                p
            }
        };

    let display_place = ensure_place(symbols::intern(b"place.display"), root);
    let _windows_place = ensure_place(symbols::intern(b"place.windows"), root);

    // Create place.input if not exists and seed pointer.0
    let input_place = if let Some(p) = store::find_thing_by_name(sym::PLACE_INPUT) {
        p
    } else {
        let p = store::thing_create(sym::KIND_PLACE);
        store::thing_register_name(p, sym::PLACE_INPUT);
        if let Some(root) = store::find_thing_by_name(sym::PLACE_ROOT) {
            store::relationship_create(sym::PRED_CONTAINS, root, p);
        }
        p
    };

    // Create bytespace.mouse_input for high-throughput mouse events
    let mut mouse_stream_id: Option<abi::ids::ThingId> = None;
    #[cfg(target_arch = "x86_64")]
    {
        use crate::machine::x86_64::ps2_mouse;
        let mouse_bs = store::thing_create(sym::KIND_BYTE_SPACE);
        let mouse_bs_name = symbols::intern(b"bytespace.mouse_input");
        store::thing_register_name(mouse_bs, mouse_bs_name);
        store::relationship_create(sym::PRED_CONTAINS, input_place, mouse_bs);

        // Attach physical address and size
        let phys_addr = ps2_mouse::get_ring_phys_addr();
        let ring_size = ps2_mouse::get_ring_size() as u64;

        let bs_payload = BytespaceBody {
            len: ring_size,
            flags: BYTESPACE_FLAG_HAS_PHYS_BASE,
            _pad: 0,
            phys_base: phys_addr,
        };
        store::thing_set_inline_payload(mouse_bs, &bs_payload.encode());

        let phys_thing = store::thing_create(sym::KIND_PLACE);
        let mut phys_payload = alloc::vec::Vec::new();
        phys_payload.extend_from_slice(&phys_addr.to_le_bytes());
        store::thing_set_inline_payload(phys_thing, &phys_payload);
        store::relationship_create(sym::PRED_BASE_PHYS, mouse_bs, phys_thing);

        let size_thing = store::thing_create(sym::KIND_PLACE);
        let mut size_payload = alloc::vec::Vec::new();
        size_payload.extend_from_slice(&ring_size.to_le_bytes());
        store::thing_set_inline_payload(size_thing, &size_payload);
        store::relationship_create(sym::PRED_SIZE, mouse_bs, size_thing);

        let mouse_stream = store::thing_create(symbols::intern(b"kind.MouseStream"));
        store::thing_register_name(mouse_stream, symbols::intern(b"mouse.stream.0"));
        let ms_payload = MouseStreamBody {
            bytespace: mouse_bs,
            capacity: ps2_mouse::RING_CAPACITY,
            sample_size: core::mem::size_of::<abi::mouse_ring::MouseSample>() as u32,
            write_index: 0,
            dropped: 0,
        };
        store::thing_set_inline_payload(mouse_stream, &ms_payload.encode());
        store::relationship_create(sym::PRED_CONTAINS, input_place, mouse_stream);
        store::relationship_create(sym::PRED_REFERENCES, mouse_stream, mouse_bs);

        // Create event_stream.mouse (new unified EventStream format)
        let event_stream = store::thing_create(symbols::intern(b"kind.EventStream"));
        store::thing_register_name(event_stream, symbols::intern(b"event_stream.mouse"));
        let es_payload = EventStreamBody {
            bytespace: mouse_bs,
            capacity_bytes: ps2_mouse::RING_CAPACITY,
            max_record_bytes: 64,
            flags: 0, // single-producer
            name: symbols::intern(b"mouse"),
        };
        store::thing_set_inline_payload(event_stream, &es_payload.encode());
        store::relationship_create(sym::PRED_CONTAINS, input_place, event_stream);
        store::relationship_create(symbols::intern(b"stream.bytespace"), event_stream, mouse_bs);
        mouse_stream_id = Some(mouse_stream);

        crate::log::kprintln(&alloc::format!(
            "BOOT: Created bytespace.mouse_input phys={:#x} size={}",
            phys_addr,
            ring_size
        ));
    }

    if mouse_stream_id.is_none() {
        let mouse_stream = store::thing_create(symbols::intern(b"kind.MouseStream"));
        store::thing_register_name(mouse_stream, symbols::intern(b"mouse.stream.0"));
        let ms_payload = MouseStreamBody {
            bytespace: abi::ids::ThingId(0),
            capacity: 0,
            sample_size: 0,
            write_index: 0,
            dropped: 0,
        };
        store::thing_set_inline_payload(mouse_stream, &ms_payload.encode());
        store::relationship_create(sym::PRED_CONTAINS, input_place, mouse_stream);
        mouse_stream_id = Some(mouse_stream);
    }

    // Create pointer.0 Thing (state mirror, updated at low rate)
    let pointer_thing = store::thing_create(sym::KIND_POINTER);
    let pointer_name = symbols::intern(b"pointer.0");
    store::thing_register_name(pointer_thing, pointer_name);
    store::thing_register_name(pointer_thing, symbols::intern(b"pointer.state"));
    store::relationship_create(sym::REL_HAS_POINTER, input_place, pointer_thing);
    store::relationship_create(sym::PRED_CONTAINS, input_place, pointer_thing);
    let pointer_payload = PointerStateBody {
        stream: mouse_stream_id.unwrap_or(abi::ids::ThingId(0)),
        x: 0,
        y: 0,
        buttons: 0,
        updated_at_ns: 0,
    };
    store::thing_set_inline_payload(pointer_thing, &pointer_payload.encode());
    crate::log::kprintln("BOOT: Created pointer.0");

    let ctx = get_boot_ctx();

    // Create Asset Root
    let asset_root = store::thing_create(sym::KIND_PLACE);
    store::thing_register_name(asset_root, sym::PLACE_ASSETS);
    if let Some(root) = store::find_thing_by_name(sym::PLACE_ROOT) {
        store::relationship_create(sym::PRED_CONTAINS, root, asset_root);
    }

    for m in ctx.modules {
        if m.path.is_empty() {
            continue;
        }

        crate::log::kprintln(&alloc::format!("MOD: {} {}", m.path, m.cmdline));

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

            let phys_thing = store::thing_create(sym::KIND_PLACE);
            let mut phys_payload = alloc::vec::Vec::new();
            phys_payload.extend_from_slice(&m.phys_addr.to_le_bytes());
            store::thing_set_inline_payload(phys_thing, &phys_payload);
            store::relationship_create(sym::PRED_BASE_PHYS, bs, phys_thing);

            let size_thing = store::thing_create(sym::KIND_PLACE);
            let mut size_payload = alloc::vec::Vec::new();
            size_payload.extend_from_slice(&m.size.to_le_bytes());
            store::thing_set_inline_payload(size_thing, &size_payload);
            store::relationship_create(sym::PRED_SIZE, bs, size_thing);

            crate::log::kprintln(&alloc::format!("BOOT: Registered asset {}", thing_name_str));
        }
    }

    if let Some(fb) = ctx.framebuffer {
        let fb_thing = store::thing_create(sym::KIND_DEVICE_DISPLAY);
        let fb_name = symbols::intern(b"device.display0");
        store::thing_register_name(fb_thing, fb_name);
        store::thing_register_name(fb_thing, symbols::intern(b"display.0"));
        store::relationship_create(sym::PRED_CONTAINS, display_place, fb_thing);

        let fb_bs_name = symbols::intern(b"bytespace.display0");
        let fb_bytespace = store::thing_create(sym::KIND_BYTESPACE_FRAMEBUFFER);
        store::thing_register_name(fb_bytespace, fb_bs_name);
        store::relationship_create(sym::PRED_CONTAINS, display_place, fb_bytespace);

        let surface = store::thing_create(sym::KIND_SURFACE);
        store::thing_register_name(surface, symbols::intern(b"surface.display0"));
        store::relationship_create(sym::PRED_PRIMARY, fb_thing, surface);
        store::relationship_create(sym::PRED_BACKS, surface, fb_bytespace);
        store::relationship_create(sym::PRED_CONTAINS, display_place, surface);

        let framebuffer_thing = store::thing_create(symbols::intern(b"kind.Framebuffer"));
        store::thing_register_name(framebuffer_thing, symbols::intern(b"framebuffer.display0"));
        store::relationship_create(sym::PRED_REFERENCES, framebuffer_thing, fb_bytespace);
        store::relationship_create(sym::PRED_CONTAINS, display_place, framebuffer_thing);

        let fb_phys_thing = store::thing_create(sym::KIND_PLACE);
        let mut fb_phys_payload = alloc::vec::Vec::new();
        fb_phys_payload.extend_from_slice(&fb.addr.to_le_bytes());
        store::thing_set_inline_payload(fb_phys_thing, &fb_phys_payload);
        store::relationship_create(sym::PRED_BASE_PHYS, fb_bytespace, fb_phys_thing);

        let fb_size = fb.height * fb.pitch;
        let fb_size_thing = store::thing_create(sym::KIND_PLACE);
        let mut fb_size_payload = alloc::vec::Vec::new();
        fb_size_payload.extend_from_slice(&fb_size.to_le_bytes());
        store::thing_set_inline_payload(fb_size_thing, &fb_size_payload);
        store::relationship_create(sym::PRED_SIZE, fb_bytespace, fb_size_thing);

        let fb_bytespace_payload = BytespaceBody {
            len: fb_size,
            flags: BYTESPACE_FLAG_HAS_PHYS_BASE,
            _pad: 0,
            phys_base: fb.addr,
        };
        store::thing_set_inline_payload(fb_bytespace, &fb_bytespace_payload.encode());

        let framebuffer_payload = FramebufferBody {
            bytespace: fb_bytespace,
            width: fb.width as u32,
            height: fb.height as u32,
            stride_bytes: fb.pitch as u32,
            format: symbols::intern(b"format.bgra8888"),
        };
        store::thing_set_inline_payload(framebuffer_thing, &framebuffer_payload.encode());

        let surface_payload = SurfaceBody {
            width: fb.width as u32,
            height: fb.height as u32,
            stride_bytes: fb.pitch as u32,
            format: symbols::intern(b"format.bgra8888"),
            bytespace: fb_bytespace,
        };
        store::thing_set_inline_payload(surface, &surface_payload.encode());

        let display_payload = DisplayDeviceBody {
            framebuffer: framebuffer_thing,
            width: fb.width as u32,
            height: fb.height as u32,
            stride_bytes: fb.pitch as u32,
            format: symbols::intern(b"format.bgra8888"),
            refresh_hz: 60,
        };
        store::thing_set_inline_payload(fb_thing, &display_payload.encode());

        crate::log::kprintln(&alloc::format!(
            "BOOT: display0 {}x{} r_shift={} g_shift={} b_shift={}",
            fb.width,
            fb.height,
            fb.red_mask_shift,
            fb.green_mask_shift,
            fb.blue_mask_shift
        ));

        // Set mouse bounds to match framebuffer dimensions
        crate::machine::input::set_mouse_bounds(fb.width as u32, fb.height as u32);

        if let Some(devices) = store::find_thing_by_name(sym::PLACE_DEVICES) {
            store::relationship_create(sym::PRED_CONTAINS, devices, fb_thing);
        }
    }
}
