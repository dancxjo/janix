use alloc::string::String;
use alloc::vec::Vec;
use crate::machine::{Machine, PreBootInfo};

#[derive(Clone, Copy)]
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
}

#[derive(Clone, Copy, Debug)]
pub struct ModuleInfo {
    pub index: usize,
    pub path: &'static str,
    pub phys_addr: u64,
    pub size: u64,
}

static mut BOOT_CTX: Option<BootContext> = None;

pub fn get_boot_ctx() -> &'static BootContext {
    unsafe { BOOT_CTX.as_ref().expect("boot ctx not init") }
}

pub fn spawn_module_by_name(ctx: &BootContext, name: &str) {
    for m in ctx.modules {
        if m.path == name || m.path.contains(name) {
             crate::proc::spawn_kernel_module(m).ok();
             return;
        }
    }
}

pub fn pre_boot(info: PreBootInfo) {
    unsafe {
        crate::machine::install(
            #[cfg(target_arch = "x86_64")]
            crate::machine::x86_64::ARCH_MACHINE,
            #[cfg(target_arch = "aarch64")]
            crate::machine::aarch64::ARCH_MACHINE,
            #[cfg(target_arch = "riscv64")]
            crate::machine::riscv64::ARCH_MACHINE,
            #[cfg(target_arch = "loongarch64")]
            crate::machine::loongarch64::ARCH_MACHINE,
        );
    }
    crate::machine::machine().init(info);
}

pub unsafe fn boot(ctx: *mut BootContext) -> ! {
    let ctx = &mut *ctx;
    BOOT_CTX = Some(*ctx);
    
    // 1. Memory Init
    let heap_size = 64 * 1024 * 1024;
    let config = crate::memory::heap::HeapConfig {
        phys_base: ctx.heap_phys_base,
        virt_base: ctx.heap_phys_base.wrapping_add(ctx.hhdm_offset),
        size: heap_size,
    };
    crate::memory::init_heap_raw(config).expect("heap init failed");
    
    // 2. Graph Init
    crate::serial::write(b"BOOT: init graph...\n");
    graph::store::init();
    
    // 3. Seed Ontology (Display, place.tasks, etc.)
    seed_bloom_ontology(ctx);
    
    // 4. Scheduler Init
    crate::serial::write(b"BOOT: init sched...\n");
    crate::sched::init();

    // 5. Load Modules (Sprout)
    crate::serial::write(b"BOOT: loading modules...\n");
    for m in ctx.modules {
        crate::serial::write(b"MOD: ");
        crate::serial::write(m.path.as_bytes());
        crate::serial::write(b"\n");
        // Spawn Sprout (init) and Bloom (compositor)
        if m.path.contains("sprout") || m.path.contains("bloom") {
             if let Err(_) = crate::proc::spawn_kernel_module(m) {
                 crate::serial::write(b"PROC: failed to spawn module\n");
             }
        }
    }

    // 6. Start Scheduler
    crate::serial::write(b"Booted.\n");
    crate::sched::run()
}

fn seed_bloom_ontology(ctx: &BootContext) {
    use graph::store;
    use graph::symbols::{self, sym};
    
    store::with_store(|s| {
        // Ensure root places exist
        let place_root = s.create_thing(sym::KIND_PLACE).expect("place.root");
        s.register_name(place_root, sym::PLACE_ROOT);
        
        let place_devices = s.create_thing(sym::KIND_PLACE).expect("place.devices");
        s.register_name(place_devices, sym::PLACE_DEVICES);
        let _ = s.create_relationship(sym::PRED_CONTAINS, place_root, place_devices);
        
        let place_tasks = s.create_thing(sym::KIND_PLACE).expect("place.tasks");
        s.register_name(place_tasks, sym::PLACE_TASKS);
        let _ = s.create_relationship(sym::PRED_CONTAINS, place_root, place_tasks);

        // Display
        if let Some(fb) = ctx.framebuffer {
            let dev = s.create_thing(sym::KIND_DEVICE_DISPLAY).expect("dev.display");
            s.register_name(dev, symbols::intern(b"device.display0"));
            let _ = s.create_relationship(sym::PRED_CONTAINS, place_devices, dev);
            
            let surf = s.create_thing(sym::KIND_SURFACE).expect("surface");
            s.register_name(surf, symbols::intern(b"surface.display0"));
            
            let _ = s.create_relationship(sym::PRED_PRIMARY, dev, surf);
            
            // Backing Bytespace
            let bs = s.create_thing(sym::KIND_BYTE_SPACE).expect("fb.bs");
            s.register_name(bs, symbols::intern(b"bytespace.display0")); // Register Name
            let _ = s.create_relationship(sym::PRED_BACKS, surf, bs);
            
            // Properties for Bytespace (Required for sys_space_map)
            // 1. Size
            let size_thing = s.create_thing(sym::KIND_BYTESLICE).expect("size");
            let mut buf_size = [0u8; 8];
            // Use pitch * height as implicit size (or use explicit size if available in FB info)
            // We use pitch * height to safely cover the framebuffer
            let linear_size = fb.pitch as u64 * fb.height as u64; 
            buf_size.copy_from_slice(&linear_size.to_le_bytes());
            s.set_payload(size_thing, &buf_size);
            let _ = s.create_relationship(sym::PRED_SIZE, bs, size_thing);
            
            // 2. Base Phys
            let phys_thing = s.create_thing(sym::KIND_BYTESLICE).expect("phys");
            let mut buf_phys = [0u8; 8];
            buf_phys.copy_from_slice(&fb.addr.to_le_bytes());
            s.set_payload(phys_thing, &buf_phys);
            let _ = s.create_relationship(sym::PRED_BASE_PHYS, bs, phys_thing);

            // Optional: Surface Properties (Width, Height, Stride) for Userland Metadata
            let width_thing = s.create_thing(sym::KIND_BYTESLICE).expect("width");
            let mut buf_w = [0u8; 8];
            buf_w.copy_from_slice(&(fb.width as u64).to_le_bytes());
            s.set_payload(width_thing, &buf_w);
            let _ = s.create_relationship(sym::PRED_SIZE, surf, width_thing); // Reusing PRED_SIZE constraint? Or separate? 
            // Standard ontology suggests PRED_WIDTH, PRED_HEIGHT. Using PRED_SIZE is ambiguous on Surface.
            // But bloom doesn't read it yet. So skipping to avoid confusion.
            // Bloom currently hardcodes 1024x768.
        }
    });
}
