use graph::symbols::{self, sym};
use graph::store;
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
    // Color format info from Limine
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

pub fn get_boot_ctx() -> &'static BootContext {
    unsafe {
        #[allow(static_mut_refs)]
        BOOT_CTX.as_ref().expect("boot ctx not init")
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
    
    let heap_config = crate::memory::heap::HeapConfig {
        phys_base: ctx.heap_phys_base,
        virt_base: ctx.heap_phys_base + ctx.hhdm_offset,
        size: 64 * 1024 * 1024,
    };
    crate::memory::heap::init(heap_config).expect("failed to init heap");
    
    crate::log::init(get_boot_ctx());
    
    graph::init();
    graph::seed_minimal();
    
    seed_bloom_ontology();
    
    crate::sched::init();
    
    spawn_module_by_name(ctx, "sprout");
    
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
    let ctx = get_boot_ctx();
    
    // Create Asset Root
    let asset_root = store::thing_create(sym::KIND_PLACE);
    store::thing_register_name(asset_root, sym::PLACE_ASSETS);
    if let Some(root) = store::find_thing_by_name(sym::PLACE_ROOT) {
        store::relationship_create(sym::PRED_CONTAINS, root, asset_root);
    }

    for m in ctx.modules {
        if m.path.is_empty() { continue; }
        
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
        
        let fb_bs_name = symbols::intern(b"bytespace.display0");
        let fb_bytespace = store::thing_create(sym::KIND_BYTESPACE_FRAMEBUFFER);
        store::thing_register_name(fb_bytespace, fb_bs_name);
        
        let surface = store::thing_create(sym::KIND_SURFACE);
        store::relationship_create(sym::PRED_PRIMARY, fb_thing, surface);
        store::relationship_create(sym::PRED_BACKS, surface, fb_bytespace);

        // Set PRED_BASE_PHYS and PRED_SIZE for framebuffer
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
        
        // Store display dimensions and color format in graph as payload on display device
        // Format: width(u32), height(u32), pitch(u32), bpp(u16), r_shift(u8), g_shift(u8), b_shift(u8)
        let mut display_payload = alloc::vec::Vec::new();
        display_payload.extend_from_slice(&(fb.width as u32).to_le_bytes());  // 0-3
        display_payload.extend_from_slice(&(fb.height as u32).to_le_bytes()); // 4-7
        display_payload.extend_from_slice(&(fb.pitch as u32).to_le_bytes());  // 8-11
        display_payload.extend_from_slice(&fb.bpp.to_le_bytes());             // 12-13
        display_payload.push(fb.red_mask_shift);                               // 14
        display_payload.push(fb.green_mask_shift);                             // 15
        display_payload.push(fb.blue_mask_shift);                              // 16
        display_payload.push(fb.red_mask_size);                                // 17
        display_payload.push(fb.green_mask_size);                              // 18
        display_payload.push(fb.blue_mask_size);                               // 19
        store::thing_set_inline_payload(fb_thing, &display_payload);
        
        crate::log::kprintln(&alloc::format!(
            "BOOT: display0 {}x{} r_shift={} g_shift={} b_shift={}",
            fb.width, fb.height, fb.red_mask_shift, fb.green_mask_shift, fb.blue_mask_shift
        ));

        if let Some(devices) = store::find_thing_by_name(sym::PLACE_DEVICES) {
             store::relationship_create(sym::PRED_CONTAINS, devices, fb_thing);
        }
    }
}
